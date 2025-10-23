use std::sync::Arc;
use std::time::Duration;
use actix_web::web::Data;
use chrono::Utc;
use tracing::{debug, error, trace};
use migration::async_trait::async_trait;
use crate::scheduler::ScheduledJob;
use crate::State;

pub struct DeleteHandler;

#[async_trait]
impl ScheduledJob for DeleteHandler {
    fn name(&self) -> &str {
        "DeleteHandler"
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(600)
    }

    async fn execute(&self, data: Arc<Data<State>>) -> anyhow::Result<()> {
        match Self::delete_files(data).await {
            Ok(_) => {
                debug!("Deletion process completed successfully.");
                Ok(())
            }
            Err(error) => {
                error!("Error during deletion process: {}", error);

                Err(anyhow::anyhow!(error))
            }
        }
    }
}

impl DeleteHandler {
    async fn delete_files(
        data: Arc<Data<State>>,
    ) -> Result<(), String> {
        let postgres_service = &data.postgres_service;

        trace!("Deleting files...");
        let now = Utc::now().timestamp_millis();
        let mut deletable_pastes = Vec::new();
        let mut all_pastes = Vec::new();

        match postgres_service.get_all_pastes_metadata().await {
            Ok(pastes) => {
                for paste in pastes {
                    if paste.expires_at > 0 && paste.expires_at < now {
                        trace!("Deleting expired paste: {}", paste.id);
                        deletable_pastes.push(paste.clone());
                    }

                    all_pastes.push(paste);
                }
            }
            Err(err) => {
                return Err(format!("Failed to retrieve pastes from database: {}", err));
            }
        }

        for paste in &deletable_pastes {
            let id = &paste.id;

            match postgres_service.delete_paste(id).await { Err(err) => {
                error!("Failed to delete paste file: {}", err);
            } _ => {
                debug!("Deleted paste file: {}", id);
            }}
        }

        Ok(())
    }
}