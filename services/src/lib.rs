use configuration::{DatabaseSettings, EventStoreSettings};
use sqlx::{postgres::PgPoolOptions, PgPool};

pub mod configuration;

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub fn get_eventstore_client(configuration: &EventStoreSettings) -> eventstore::Client {
    let connection_string = configuration
        .url
        .parse()
        .expect("Failed to parse EventStoreDB URL.");

    eventstore::Client::new(connection_string).expect("Failed to create EventStoreDB client.")
}
