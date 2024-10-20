use projections::jobsite::Jobsite;
use tokio::sync::broadcast;

pub mod events;
pub mod projections;

#[derive(Clone)]
pub enum JobsiteBroadcast {
    JobsiteCreated(Jobsite),
    JobsiteUpdated(Jobsite),
}

#[derive(Clone)]
pub struct AppState {
    pub jobsite_tx: broadcast::Sender<JobsiteBroadcast>,
}
