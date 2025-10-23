use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use std::sync::Arc;
use crate::utils::data::DataUtils;
use crate::utils::ip::IPUtils;
use crate::database::postgres_service::PostgresService;
use crate::models::content_query;

#[get("/{id}/content")]
async fn get_content(
    postgres_service: web::Data<Arc<PostgresService>>,
    request: HttpRequest,
    path: web::Path<String>,
    query: web::Query<content_query::ContentQuery>,
) -> impl Responder {
    let compress = query.compress.unwrap_or(true);
    let ip = IPUtils::get_ip_from_request(&request).unwrap_or("Unknown".to_string());

    if postgres_service.is_user_banned(&ip).await.expect("Failed to check if user is banned") {
        return HttpResponse::Forbidden().body("Prohibited");
    }

    let postgres_service_clone = Arc::clone(&postgres_service);
    let path = path.into_inner();
    let ip = ip.clone();
    tokio::spawn(async move {
        if let Err(err) = postgres_service_clone.increment_requests(&ip).await {
            tracing::error!("Failed to increment requests: {}", err);
        }
    });

    match postgres_service.get_paste_content(&path).await {
        Ok(data) => {
            if data.is_none() {
                return HttpResponse::NotFound().body("File not found");
            }
            
            if compress {
                let compressed = DataUtils::compress_data(data.unwrap().content.as_bytes());
                HttpResponse::Ok()
                    .content_type("text/plain; charset=utf-8")
                    .append_header(("Content-Encoding", "gzip"))
                    .body(compressed)
            } else {
                HttpResponse::Ok()
                    .content_type("text/plain; charset=utf-8")
                    .body(data.unwrap().content)
            }
        }
        Err(_) => HttpResponse::NotFound().body("File not found"),
    }
}

#[get("/{id}/metadata")]
async fn get_metadata(
    postgres_service: web::Data<Arc<PostgresService>>,
    request: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let ip = IPUtils::get_ip_from_request(&request).unwrap_or("Unknown".to_string());
    if postgres_service.is_user_banned(&ip).await.expect("Failed to check if user is banned") {
        return HttpResponse::Forbidden().body("Prohibited");
    }

    match postgres_service.get_paste_metadata(&path).await {
        Ok(Some(metadata)) => {
            let user = postgres_service.get_user(&metadata.creator_ip).await.unwrap();
            
            if user.ip != ip && metadata.burn {
                tracing::trace!("Burning paste: {}", metadata.id);
                let postgres_service_clone = Arc::clone(&postgres_service);
                let id = metadata.id.clone();
                tokio::spawn(async move {
                    if let Err(err) = postgres_service_clone.delete_paste(&id).await {
                        tracing::error!("Failed to burn paste: {}", err);
                    }
                });
            }

            let public_dto = metadata.to_public_dto(user.to_dto());
            HttpResponse::Ok().json(public_dto)
        }
        Ok(None) => HttpResponse::NotFound().body("Not Found"),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}