pub mod scheduler;
pub mod tasks;

use std::sync::Arc;
use core::time::Duration;
use actix_web::web::Data;
use sea_orm::prelude::async_trait::async_trait;
use crate::State;

#[async_trait]
pub trait ScheduledJob: Send + Sync {
    fn name(&self) -> &str;
    fn interval(&self) -> Duration;
    async fn execute(&self, data: Arc<Data<State>>) -> anyhow::Result<()>;
}