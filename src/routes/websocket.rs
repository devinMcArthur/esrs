use std::collections::HashSet;

use actix_web::{rt, web, HttpRequest, HttpResponse};
use actix_ws::{Message, MessageStream, Session};
use leptos::view;
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::{
    application::{AppState, JobsiteBroadcast},
    models::jobsite::Jobsite,
    views::{components, TemplateRenderer},
};

pub async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let (response, session, msg_stream) = actix_ws::handle(&req, stream)?;

    rt::spawn(ws_handler(session, msg_stream, state, db_pool));

    Ok(response)
}

async fn ws_handler(
    mut session: Session,
    mut msg_stream: MessageStream,
    state: web::Data<AppState>,
    db_pool: web::Data<PgPool>,
) {
    // Subscribe to jobsite updates
    let mut jobsite_rx = state.jobsite_tx.subscribe();

    let mut subscribed_jobsites = HashSet::<Uuid>::new();

    loop {
        tokio::select! {
            // Handle incoming messages from client
            Some(Ok(msg)) = msg_stream.next() => {
                match msg {
                    Message::Text(text) => {
                        handle_client_message(&text, &mut session, &mut subscribed_jobsites, &db_pool).await;
                    },
                    Message::Close(_) => {
                        break;
                    },
                    _ => {}
                }
            },
            // Handle jobsite updates from event store
            Ok(jobsite_update) = jobsite_rx.recv() => {
                match jobsite_update {
                    JobsiteBroadcast::JobsiteCreated(jobsite) => {
                        send_jobsite_update(&mut session, jobsite).await;
                    }
                    JobsiteBroadcast::JobsiteUpdated(jobsite) => {
                        send_jobsite_updated_update(&mut session, jobsite).await;
                    }
                }
            },
            else => break,
        }
    }
}

async fn handle_client_message(
    text: &str,
    session: &mut Session,
    subscribed_jobsites: &mut HashSet<Uuid>,
    db_pool: &web::Data<PgPool>,
) {
    if let Ok(message) = serde_json::from_str::<JobsiteClientMessage>(text) {
        match message {
            JobsiteClientMessage::JobsiteLoading { jobsite_id } => {
                let mut transaction = db_pool.begin().await.unwrap();

                if let Ok(Some(jobsite)) = Jobsite::get_by_id(&mut transaction, &jobsite_id).await {
                    send_jobsite_update(session, jobsite).await;
                };

                transaction.commit().await.unwrap();
            }
            JobsiteClientMessage::JobsiteRegister { jobsite_id } => {
                subscribed_jobsites.insert(jobsite_id);
            }
        }
    } else {
        error!("Failed to parse client message: {}", text);
    }
}

async fn send_jobsite_update(session: &mut Session, jobsite: Jobsite) {
    let jobsite_id = jobsite.id;

    let html = TemplateRenderer::render(move || {
        view! {
            <components::jobsite::JobsiteRow jobsite=jobsite.clone() jobsite_id=jobsite_id />
            <components::jobsite::JobsiteList jobsites=vec![jobsite.clone()] append=jobsite_id />
        }
    });

    let _ = session.text(html).await;
}

async fn send_jobsite_updated_update(session: &mut Session, jobsite: Jobsite) {
    let jobsite_id = jobsite.id;

    let html = TemplateRenderer::render(move || {
        view! {
            <components::jobsite::JobsiteRow jobsite=jobsite.clone() jobsite_id=jobsite_id />
            <components::jobsite::JobsiteEdit jobsite=Some(jobsite) />
        }
    });

    let _ = session.text(html).await;
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum JobsiteClientMessage {
    JobsiteLoading { jobsite_id: Uuid },
    JobsiteRegister { jobsite_id: Uuid },
}
