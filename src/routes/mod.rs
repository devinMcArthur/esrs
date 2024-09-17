use actix_web::HttpResponse;

mod health_check;
mod jobsite;
mod websocket;

pub use health_check::*;
pub use jobsite::*;
use uuid::Uuid;
pub use websocket::*;

use crate::{
    utils::RouteError,
    views::{pages, TemplateRenderer},
};

#[tracing::instrument(name = "Landing page")]
pub async fn get_landing_page() -> Result<HttpResponse, RouteError> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(TemplateRenderer::render(pages::Landing)))
}

#[tracing::instrument(name = "Not Found (404) page")]
pub async fn get_not_found_page(req: actix_web::HttpRequest) -> Result<HttpResponse, RouteError> {
    Ok(HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(TemplateRenderer::render(pages::NotFound)))
}

pub struct ApiRoutes;

impl ApiRoutes {
    /// Route: `GET /jobsites`
    /// Get jobsite list
    pub fn get_jobsite_list() -> String {
        String::from("/jobsites")
    }

    /// Route: `GET /jobsite/:id`
    /// Get jobsite list
    pub fn get_jobsite(jobsite_id: Uuid) -> String {
        format!("/jobsite/{jobsite_id}")
    }

    /// Route: `POST /jobsite`
    /// Create a new jobsite
    pub fn post_jobsite() -> String {
        String::from("/jobsite")
    }

    /// Route: `PUT /jobsite/:id`
    /// Update a jobsite
    pub fn put_jobsite(jobsite_id: Uuid) -> String {
        format!("/jobsite/{jobsite_id}")
    }
}
