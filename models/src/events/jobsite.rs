#[cfg(feature = "connect")]
use {
    log::error,
    serde_json::Value,
    crate::AppState,
    super::EventParseError
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct JobsiteCreated {
    pub id: Uuid,
    pub name: String,
}

impl JobsiteCreated {
    pub fn event_name() -> String {
        String::from("JobsiteCreated")
    }

    #[cfg(feature = "connect")]
    pub async fn handle_read_model(&self, handler: JobsiteReadModelHandler) -> anyhow::Result<()> {
        use crate::{projections::jobsite::Jobsite, JobsiteBroadcast};

        let mut transaction = handler
            .db_pool
            .begin()
            .await
            .expect("Failed to start transaction");

        match Jobsite::create(&mut transaction, self).await {
            Ok(jobsite) => match transaction.commit().await {
                Ok(_) => {
                    if let Err(e) = handler
                        .app_state
                        .jobsite_tx
                        .send(JobsiteBroadcast::JobsiteCreated(jobsite))
                    {
                        error!("Failed to send jobsite to channel: {}", e);
                    }
                }
                Err(e) => error!("Failed to commit transaction: {}", e),
            },
            Err(e) => {
                error!("Failed to create jobsite in read model: {}", e);
                if let Err(e) = transaction.rollback().await {
                    error!("Failed to rollback transaction: {}", e);
                }
            }
        };

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobsiteUpdated {
    pub id: Uuid,
    pub name: String,
}

impl JobsiteUpdated {
    pub fn event_name() -> String {
        String::from("JobsiteUpdated")
    }

    #[cfg(feature = "connect")]
    pub async fn handle_read_model(&self, handler: JobsiteReadModelHandler) -> anyhow::Result<()> {
        use crate::{projections::jobsite::Jobsite, JobsiteBroadcast};

        let mut transaction = handler
            .db_pool
            .begin()
            .await
            .expect("Failed to start transaction");

        match Jobsite::update(&mut transaction, self).await {
            Ok(jobsite) => match transaction.commit().await {
                Ok(_) => {
                    if let Err(e) = handler
                        .app_state
                        .jobsite_tx
                        .send(JobsiteBroadcast::JobsiteUpdated(jobsite))
                    {
                        error!("Failed to send jobsite to channel: {}", e);
                    }
                }
                Err(e) => {
                    error!("Failed to commit transaction: {}", e);
                }
            },
            Err(e) => {
                error!("Failed to update jobsite in read model: {}", e);
                if let Err(e) = transaction.rollback().await {
                    error!("Failed to rollback transaction: {}", e);
                }
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum JobsiteEvent {
    JobsiteCreated(JobsiteCreated),
    JobsiteUpdated(JobsiteUpdated),
}

#[cfg(feature = "connect")]
impl TryFrom<eventstore::ResolvedEvent> for JobsiteEvent {
    type Error = EventParseError;

    fn try_from(value: eventstore::ResolvedEvent) -> Result<Self, Self::Error> {
        let event_data = value.event.ok_or(EventParseError::MissingEventData)?;
        let event_json: Value = serde_json::from_slice(&event_data.data)
            .map_err(EventParseError::DeserializationError)?;

        match event_data.event_type {
            s if s == JobsiteCreated::event_name() => Ok(JobsiteEvent::JobsiteCreated(
                serde_json::from_value(event_json)
                    .map_err(EventParseError::DeserializationError)?,
            )),
            s if s == JobsiteUpdated::event_name() => Ok(JobsiteEvent::JobsiteUpdated(
                serde_json::from_value(event_json)
                    .map_err(EventParseError::DeserializationError)?,
            )),
            _ => Err(EventParseError::UnknownEventType(event_data.event_type)),
        }
    }
}

impl JobsiteEvent {
    #[cfg(feature = "connect")]
    pub fn subscription_filter() -> eventstore::SubscriptionFilter {
        eventstore::SubscriptionFilter::on_stream_name().add_prefix("jobsite-")
    }

    #[cfg(feature = "connect")]
    pub async fn handle_read_model(&self, handler: JobsiteReadModelHandler) -> anyhow::Result<()> {
        match self {
            JobsiteEvent::JobsiteCreated(event) => event.handle_read_model(handler).await,
            JobsiteEvent::JobsiteUpdated(event) => event.handle_read_model(handler).await,
        }
    }
}

/**
 * Jobsite read model handler
 * Holds all necessary service connections and state to handle jobsite events
 */
#[derive(Clone)]
#[cfg(feature = "connect")]
pub struct JobsiteReadModelHandler {
    eventstore: std::sync::Arc<eventstore::Client>,
    db_pool: std::sync::Arc<sqlx::PgPool>,
    app_state: AppState,
}


#[cfg(feature = "connect")]
impl JobsiteReadModelHandler {
    pub fn new(
        eventstore: std::sync::Arc<eventstore::Client>,
        db_pool: std::sync::Arc<sqlx::PgPool>,
        app_state: AppState,
    ) -> Self {
        Self {
            eventstore,
            db_pool,
            app_state,
        }
    }

    /**
     * Subscribe to all jobsite events and handle them in the read model
     */
    pub async fn subscribe(&self) {
        let snapshot_position = match self.get_snapshot_position().await {
            Ok(position) => position,
            Err(e) => {
                error!("Failed to get snapshot position: {}", e);
                return;
            }
        };

        let mut jobsite_subscription = self
            .eventstore
            .subscribe_to_all(
                &eventstore::SubscribeToAllOptions::default()
                    .position(eventstore::StreamPosition::Position(eventstore::Position {
                        commit: snapshot_position as u64,
                        prepare: snapshot_position as u64,
                    }))
                    .filter(JobsiteEvent::subscription_filter()),
            )
            .await;

        while let Ok(resolved_event) = jobsite_subscription.next().await {
            let position = match &resolved_event.event {
                Some(event) => Some(event.position.commit),
                None => None,
            };

            let event: JobsiteEvent = match resolved_event.try_into() {
                Ok(event) => event,
                Err(e) => {
                    error!("Failed to parse event: {}", e);
                    continue;
                }
            };

            if let Err(e) = event.handle_read_model(self.clone()).await {
                error!("Failed to handle event: {}", e);
                continue;
            }

            if let Err(e) = self.set_snapshot_position(position).await {
                error!("Failed to set snapshot position: {}", e);
                continue;
            }
        }
    }

    /**
     * Get the current snapshot position for jobsites
     */
    pub async fn get_snapshot_position(&self) -> anyhow::Result<i64> {
        let mut transaction = self
            .db_pool
            .begin()
            .await
            .expect("Failed to start transaction for snapshot");

        let snapshot_position =
            crate::projections::snapshot_position::SnapshotPosition::get_by_key(&mut transaction, crate::projections::snapshot_position::SnapshotPositionKey::Jobsite)
                .await
                .expect("Failed to get snapshot position");

        match snapshot_position {
            Some(position) => Ok(position.value),
            None => Ok(0),
        }
    }

    /**
     * Set the current snapshot position for jobsites
     */
    pub async fn set_snapshot_position(&self, position: Option<u64>) -> anyhow::Result<()> {
        let mut transaction = self
            .db_pool
            .begin()
            .await
            .expect("Failed to start transaction for snapshot");

        if let Some(position) = position {
            let snapshot_position = crate::projections::snapshot_position::SnapshotPosition {
                key: crate::projections::snapshot_position::SnapshotPositionKey::Jobsite,
                value: position as i64,
            };

            snapshot_position.insert(&mut transaction).await?;

            transaction.commit().await?;
        }

        Ok(())
    }
}
