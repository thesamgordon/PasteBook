use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;
use ::entity::{paste_metadata, paste_content, user_metadata};
use tracing::info;
use migration::{Migrator, MigratorTrait};

pub struct PostgresService {
    db: DatabaseConnection,
}

impl PostgresService {
    pub async fn new(uri: &str) -> Result<Self, DbErr> {
        info!("Connecting to PostgreSQL...");
        let db = Database::connect(uri).await?;
        info!("Connected to PostgreSQL.");

        info!("Running migrations...");
        Migrator::up(&db, None).await?;

        Ok(Self { db })
    }

    pub async fn increment_requests(&self, ip_str: &str) -> Result<(), DbErr> {
        let user = self.get_user(ip_str).await?;
        let mut active_user: user_metadata::ActiveModel = user.into_active_model();
        
        let requests = active_user.requests.take().unwrap_or(0);
        active_user.requests = Set(requests + 1);
        
        active_user.update(&self.db).await?;
        
        Ok(())
    }

    pub async fn put_paste(&self, paste_metadata: paste_metadata::ActiveModel, paste_content: paste_content::ActiveModel) -> Result<(), DbErr> {
        paste_metadata.insert(&self.db).await?;
        paste_content.insert(&self.db).await?;

        Ok(())
    }

    pub async fn get_paste_content(&self, id_str: &str) -> Result<Option<paste_content::Model>, DbErr> {
        paste_content::Entity::find_by_id(id_str).one(&self.db).await
    }
    
    pub async fn delete_paste(&self, id_str: &str) -> Result<(), DbErr> {
        use paste_metadata::Entity as PasteMetadata;
        use paste_content::Entity as PasteContent;
        
        PasteMetadata::delete_by_id(id_str).exec(&self.db).await?;
        PasteContent::delete_by_id(id_str).exec(&self.db).await?;

        Ok(())
    }

    pub async fn is_user_banned(&self, ip: &str) -> Result<bool, DbErr> {
        let user = self.get_user(ip).await?;

        Ok(user.banned)
    }

    pub async fn get_user(&self, ip: &str) -> Result<user_metadata::Model, DbErr> {
        let user = user_metadata::Entity::find_by_id(ip).one(&self.db).await;

        if let Ok(Some(user)) = user {
            return Ok(user);
        }
        
        let user = user_metadata::ActiveModel {
            ip: Set(ip.to_string()),
            id: Set(Uuid::new_v4().to_string()),
            created_at: Set(Utc::now().timestamp_millis()),
            banned: Set(false),
            requests: Set(0),
        };
        
        user.clone().insert(&self.db).await?;
        user.try_into_model()
    }

    pub async fn get_paste_metadata(&self, id: &str) -> Result<Option<paste_metadata::Model>, DbErr> {
        paste_metadata::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn get_all_pastes_metadata(&self) -> Result<Vec<paste_metadata::Model>, DbErr> {
        paste_metadata::Entity::find().all(&self.db).await
    }
}
