#[cfg(feature = "connect")]
use projections::jobsite::Jobsite;

pub mod events;
pub mod projections;

#[derive(Clone)]
#[cfg(feature = "connect")]
pub enum JobsiteBroadcast {
    JobsiteCreated(Jobsite),
    JobsiteUpdated(Jobsite),
}

#[derive(Clone)]
#[cfg(feature = "connect")]
pub struct AppState {
    pub jobsite_tx: tokio::sync::broadcast::Sender<JobsiteBroadcast>,
}
