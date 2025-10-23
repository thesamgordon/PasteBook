use std::sync::Arc;
use actix_web::web::Data;
use log::trace;
use tracing::{error};
use tokio::time;
use crate::scheduler::ScheduledJob;
use crate::State;

pub struct Scheduler {
    jobs: Vec<Arc<dyn ScheduledJob>>,
    state: Arc<Data<State>>,
}

impl Scheduler {
    pub fn new(state: Data<State>) -> Self {
        Scheduler {
            jobs: Vec::new(),
            state: Arc::new(state),
        }
    }

    pub fn add_job<T>(&mut self, job: T) -> &mut Self where T: ScheduledJob + 'static {
        self.jobs.push(Arc::new(job));

        self
    }

    pub fn run(&self) {
        println!("{}", &self.jobs.len());
        for job in &self.jobs {
            let state = self.state.clone();
            let job = job.clone();

            tokio::spawn(async move {
                let mut interval = time::interval(job.interval());

                loop {
                    interval.tick().await;
                    trace!("Executing job: {}", job.name());

                    if let Err(e) = job.execute(state.clone()).await {
                        error!("Error executing job {}: {}", job.name(), e)
                    }
                }
            });
        }
    }
}