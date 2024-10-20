use application::Application;
use services::configuration::get_configuration;

mod application;
mod routes;
pub mod utils;
pub mod views;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
