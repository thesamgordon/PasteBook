use crate::utils::ip::IPUtils;
use crate::utils::string::StringUtils;
use actix_web::{post, web, HttpRequest, HttpResponse};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;
use sea_orm::IntoActiveModel;
use entity::{paste_content, paste_metadata};
use crate::State;

#[post("")]
async fn upload(
    data: web::Data<State>,
    req: HttpRequest,
    body: String
) -> HttpResponse {
    let title = req
        .headers()
        .get("title")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("Untitled")
        .to_string();
    
    let wrap = req
        .headers()
        .get("wrap")
        .map(|v| v.to_str().unwrap_or("false") == "true")
        .unwrap_or(false);

    let burn = req
        .headers()
        .get("burn")
        .map(|v| v.to_str().unwrap_or("false") == "true")
        .unwrap_or(false);
    
    let lang = req
        .headers()
        .get("lang")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let mut expires = req
        .headers()
        .get("expires")
        .and_then(|v| v.to_str().ok()?.parse::<i64>().ok())
        .unwrap_or(86_400_000);

    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    if expires < 60_000 {
        return HttpResponse::BadRequest().body("Expire time too short");
    }
    if expires < since_the_epoch {
        expires += since_the_epoch;
    }
    if expires > since_the_epoch + 2_765_000_000 {
        return HttpResponse::BadRequest().body("Expire time too long");
    }

    let ip = match IPUtils::get_ip_from_request(&req) {
        Some(ip) => ip,
        None => return HttpResponse::BadRequest().body("Failed to get IP"),
    };

    let file_id = StringUtils::generate_random_string(5);

    let paste = paste_metadata::Model {
        id: file_id.clone(),
        title: title.clone(),
        created: since_the_epoch,
        wrap,
        creator_ip: ip.clone(),
        expires_at: expires,
        burn,
        language: lang.clone(),
    };
    
    let paste_content = paste_content::Model {
        id: file_id.clone(),
        content: body.clone(),
    };
    
    let active_metadata: paste_metadata::ActiveModel = paste.into_active_model();
    let active_content: paste_content::ActiveModel = paste_content.into_active_model();
    
    if let Err(e) = data.postgres_service.put_paste(active_metadata, active_content).await {
        info!("Failed to save to database: {:?}", e);
        return HttpResponse::InternalServerError().body(format!("Failed to save to database: {:?}", e));
    }

    data.postgres_service.increment_requests(&ip).await.expect("Failed to increment requests");
    
    let host_domain = req
        .headers()
        .get("X-Domain-Name")
        .and_then(|v| v.to_str().ok());
    let response_body = if let Some(domain) = host_domain {
        format!("https://{}/p/{}", domain, file_id)
    } else {
        file_id.clone()
    };

    HttpResponse::Ok().body(response_body)
}