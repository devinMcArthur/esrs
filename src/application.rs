use std::{net::TcpListener, path::PathBuf, sync::Arc};

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{dev::Server, web, App, HttpServer};
use log::error;
use sqlx::PgPool;
use tokio::sync::broadcast;
use tracing_actix_web::TracingLogger;

use crate::{
    configuration::{get_connection_pool, get_eventstore_client, Settings},
    events::jobsite::JobsiteReadModelHandler,
    models::jobsite::Jobsite,
    routes::{
        get_jobsite, get_jobsites, get_landing_page, get_not_found_page, health_check,
        post_jobsite, put_jobsite, websocket,
    },
};

pub async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    eventstore_client: eventstore::Client,
    app_state: AppState,
    settings: Settings,
) -> Result<Server, anyhow::Error> {
    // Wrap the connection in a smart pointer
    let db_pool_data = web::Data::new(db_pool.clone());
    let eventstore_client_data = web::Data::new(eventstore_client.clone());
    let application_settings_data = web::Data::new(settings.application.clone());
    let app_state_data = web::Data::new(app_state);

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let public_path = PathBuf::from(manifest_dir).join("./public");

    let server = HttpServer::new(move || {
        let domain = settings.application.domain.clone();
        let cors = Cors::default()
            .allowed_origin_fn(move |origin, _req_head| {
                origin.as_bytes().ends_with(domain.as_bytes())
            })
            .allow_any_method()
            .allow_any_header()
            .expose_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default())
            .route("", web::get().to(get_landing_page))
            .route("/", web::get().to(get_landing_page))
            .route("/health-check", web::get().to(health_check))
            .route("/jobsite", web::post().to(post_jobsite))
            .route("/jobsite/{jobsite_id}", web::get().to(get_jobsite))
            .route("/jobsite/{jobsite_id}", web::put().to(put_jobsite))
            .route("/jobsites", web::get().to(get_jobsites))
            .route("/websocket", web::get().to(websocket))
            // Default handler (404)
            .default_service(
                // 404 for GET request
                web::route().to(get_not_found_page),
            )
            // Get a pointer copy and attach it to the application state
            .app_data(db_pool_data.clone())
            .app_data(application_settings_data.clone())
            .app_data(eventstore_client_data.clone())
            .app_data(app_state_data.clone())
            .service(Files::new("/public", public_path.to_str().unwrap()).prefer_utf8(true))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn run_event_handlers(
    eventstore: Arc<eventstore::Client>,
    db_pool: Arc<PgPool>,
    app_state: AppState,
) {
    let jobsite_read_model_handler =
        JobsiteReadModelHandler::new(eventstore.clone(), db_pool.clone(), app_state.clone());
    let jobsite_read_model_event_handler = tokio::spawn(async move {
        jobsite_read_model_handler.subscribe().await;
    });

    tokio::select! {
        _ = jobsite_read_model_event_handler => {
            error!("Jobsite read model event handler stopped");
        }
    }
}

#[derive(Clone)]
pub enum JobsiteBroadcast {
    JobsiteCreated(Jobsite),
    JobsiteUpdated(Jobsite),
}

#[derive(Clone)]
pub struct AppState {
    pub jobsite_tx: broadcast::Sender<JobsiteBroadcast>,
}

pub struct Application {
    server: Server,
    event_handler: tokio::task::JoinHandle<()>,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let mut configuration = configuration.clone();
        let connection_pool = get_connection_pool(&configuration.database);

        let eventstore_client = get_eventstore_client(&configuration.eventstore);

        let (jobsite_tx, _) = broadcast::channel::<JobsiteBroadcast>(16);

        let app_state = AppState { jobsite_tx };

        // sqlx::migrate!("./migrations")
        //     .run(&connection_pool)
        //     .await
        //     .expect("Failed to migrate the database.");

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        configuration.application.port = port;
        let server = run(
            listener,
            connection_pool.clone(),
            eventstore_client.clone(),
            app_state.clone(),
            configuration.clone(),
        )
        .await?;

        let event_store = Arc::new(eventstore_client);
        let db_pool = Arc::new(connection_pool);
        let event_handler = tokio::spawn(async move {
            run_event_handlers(event_store, db_pool, app_state).await;
        });

        Ok(Self {
            server,
            event_handler,
        })
    }

    // A more expiress name that makes it clear that
    // this function only returns when the application is stopped
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        let Application {
            server,
            event_handler,
        } = self;

        tokio::select! {
            _ = server => {
                error!("Server stopped");
            }
            _ = event_handler => {
                error!("Event handler stopped");
            }
        }

        Ok(())
    }
}
