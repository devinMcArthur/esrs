use application::Application;
use configuration::get_configuration;
use serde::{Deserialize, Serialize};

mod application;
mod configuration;
pub mod events;
mod models;
mod routes;
pub mod utils;
pub mod views;

#[derive(Serialize, Deserialize, Debug)]
struct MyEvent {
    is_rust_a_nice_language: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TestEvent {
    id: String,
    important_data: String,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
