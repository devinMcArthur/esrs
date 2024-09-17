use actix_web::{web, HttpResponse};
use eventstore::EventData;
use sqlx::PgPool;

use crate::{
    events::jobsite::{JobsiteCreated, JobsiteUpdated},
    models::jobsite::Jobsite,
    utils::{ErrorProps, ErrorPropsCollection, RouteError},
    views::{components, TemplateRenderer},
};

#[derive(serde::Deserialize)]
pub struct JobsiteCreateData {
    name: String,
}

pub async fn post_jobsite(
    db_pool: web::Data<PgPool>,
    data: web::Form<JobsiteCreateData>,
    eventstore: web::Data<eventstore::Client>,
) -> Result<HttpResponse, RouteError> {
    let mut errors = vec![ErrorProps {
        id: "name-error".to_string(),
        text: None,
    }];

    let mut transaction = db_pool.begin().await.unwrap();

    match Jobsite::get_by_name(&mut transaction, data.name.clone()).await? {
        Some(_) => {
            errors.set_error("name-error", "A Jobsite already exists with this name")?;

            return Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(errors.render_errors()?));
        }
        None => {}
    };

    transaction.commit().await?;

    let jobsite_id = uuid::Uuid::new_v4();
    let create_event = JobsiteCreated {
        id: jobsite_id,
        name: data.name.clone(),
    };

    let event = EventData::json("JobsiteCreated", &create_event)
        .expect("Unable to serialize")
        .id(uuid::Uuid::new_v4());

    eventstore
        .append_to_stream(
            format!("jobsite-{}", create_event.id),
            &Default::default(),
            event,
        )
        .await
        .expect("Failed to append event");

    Ok(HttpResponse::Created()
        .content_type("text/html; charset=utf-8")
        .body(TemplateRenderer::render(move || {
            components::jobsite::JobsiteRow(components::jobsite::JobsiteRowProps {
                jobsite: None,
                jobsite_id,
            })
        })))
}

pub async fn get_jobsites(db_pool: web::Data<PgPool>) -> Result<HttpResponse, RouteError> {
    let mut transaction = db_pool.begin().await.unwrap();

    let jobsites = Jobsite::get_list(&mut transaction).await?;

    transaction.commit().await?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(TemplateRenderer::render(|| {
            components::jobsite::JobsiteList(components::jobsite::JobsiteListProps {
                jobsites,
                append: None,
            })
        })))
}

pub async fn get_jobsite(
    db_pool: web::Data<PgPool>,
    jobsite_id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, RouteError> {
    let mut transaction = db_pool.begin().await.unwrap();

    let jobsite = Jobsite::get_by_id(&mut transaction, &jobsite_id.into_inner()).await?;

    transaction.commit().await?;

    match jobsite {
        Some(jobsite) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(TemplateRenderer::render(move || {
                components::jobsite::JobsiteEdit(components::jobsite::JobsiteEditProps {
                    jobsite: Some(jobsite),
                })
            }))),
        None => Err(RouteError::NotFound),
    }
}

#[derive(serde::Deserialize)]
pub struct JobsiteUpdateData {
    name: String,
}

pub async fn put_jobsite(
    db_pool: web::Data<PgPool>,
    data: web::Form<JobsiteUpdateData>,
    eventstore: web::Data<eventstore::Client>,
    jobsite_id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, RouteError> {
    let mut errors = vec![ErrorProps {
        id: "name-error".to_string(),
        text: None,
    }];

    let mut transaction = db_pool.begin().await.unwrap();

    let jobsite_id = jobsite_id.into_inner();

    match Jobsite::get_by_id(&mut transaction, &jobsite_id).await? {
        Some(_) => {}
        None => {
            return Err(RouteError::NotFound);
        }
    };

    match Jobsite::get_by_name(&mut transaction, data.name.clone()).await? {
        Some(_) => {
            errors.set_error("name-error", "This name is already taken")?;

            return Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(errors.render_errors()?));
        }
        None => {}
    };

    transaction.commit().await?;

    let update_event = JobsiteUpdated {
        id: jobsite_id,
        name: data.name.clone(),
    };

    let event = EventData::json("JobsiteUpdated", &update_event)
        .expect("Unable to serialize")
        .id(uuid::Uuid::new_v4());

    eventstore
        .append_to_stream(
            format!("jobsite-{}", update_event.id),
            &Default::default(),
            event,
        )
        .await
        .expect("Failed to append event");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .finish())
}
