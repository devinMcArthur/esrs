use std::{fmt, rc::Rc};

use actix_web::{HttpResponse, ResponseError};
use anyhow::bail;
use leptos::*;

use crate::views::{FormError, TemplateRenderer};

#[derive(thiserror::Error)]
pub enum RouteError {
    #[error("Not found")]
    NotFound,
    #[error("Database error")]
    DbError(#[from] sqlx::Error),
    #[error("Unsuccessful request to source")]
    RequestError(#[from] reqwest::Error),
    #[error("Unexpected error")]
    UnexpectedError(#[from] anyhow::Error),
}

impl fmt::Debug for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for RouteError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            RouteError::NotFound => HttpResponse::NotFound().finish(),
            RouteError::DbError(e) => HttpResponse::InternalServerError()
                .json(format!("Database Error: {}", e.to_string())),
            RouteError::RequestError(e) => HttpResponse::InternalServerError()
                .json(format!("Reqwuest Error: {}", e.to_string())),
            RouteError::UnexpectedError(e) => HttpResponse::InternalServerError()
                .insert_header(("HX-Retarget", "#flash-error"))
                .insert_header(("HX-Reswap", "innerHTML"))
                .body(format!("Unexpected server error: {}", e)),
        }
    }
}

fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

#[derive(Clone)]
pub enum ErrorContent {
    Text(String),
    View(ChildrenFn),
}

impl fmt::Debug for ErrorContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(text) => write!(f, "ErrorContent::Text({})", text),
            Self::View(_) => write!(f, "ErrorContent::View"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ErrorProps {
    pub text: Option<ErrorContent>,
    pub id: String,
}

impl ErrorProps {
    pub fn new(id: String) -> Self {
        Self { text: None, id }
    }

    /// Set the text of the error message
    pub fn set_text(&mut self, text: String) {
        self.text = Some(ErrorContent::Text(text));
    }

    /// Set the view of the error message
    pub fn set_view<F, N>(&mut self, f: F)
    where
        F: Fn() -> N + 'static,
        N: IntoView,
    {
        self.text = Some(ErrorContent::View(Rc::new(move || {
            Fragment::new(vec![f().into_view()])
        })));
    }

    /// Clear the text of the error message
    pub fn clear_text(&mut self) {
        self.text = None;
    }

    /// Render the error message to HTML string
    pub fn render(&self) -> String {
        let id = self.id.clone();
        let text = self.text.clone();
        TemplateRenderer::render(|| match text {
            Some(ErrorContent::Text(text)) => view! {
                <FormError id={id}>
                    <span>{text}</span>
                </FormError>
            },
            Some(ErrorContent::View(view)) => {
                view! {
                    <FormError id={id}>
                        {view()}
                    </FormError>
                }
            }
            // Used to reset errors to empty
            None => view! {<FormError id={id} />},
        })
    }
}

pub trait ErrorPropsCollection {
    fn set_error(&mut self, id: &str, text: &str) -> anyhow::Result<()>;
    fn set_error_view<F, N>(&mut self, id: &str, f: F) -> anyhow::Result<()>
    where
        F: Fn() -> N + 'static,
        N: IntoView;
    fn clear_error(&mut self, id: &str);
    fn has_errors(&self) -> bool;
    fn render_errors(&self) -> anyhow::Result<String>;
}

impl ErrorPropsCollection for Vec<ErrorProps> {
    /// Set the text of an error message by its ID
    fn set_error(&mut self, id: &str, text: &str) -> anyhow::Result<()> {
        if let Some(error) = self.iter_mut().find(|e| e.id == id) {
            error.set_text(text.to_string());
        } else {
            bail!("Error ID not found: {}", id);
        }

        Ok(())
    }

    /// Set the view of an error message by its ID
    fn set_error_view<F, N>(&mut self, id: &str, f: F) -> anyhow::Result<()>
    where
        F: Fn() -> N + 'static,
        N: IntoView,
    {
        if let Some(error) = self.iter_mut().find(|e| e.id == id) {
            error.set_view(f);
        } else {
            bail!("Error ID not found: {}", id);
        }

        Ok(())
    }

    /// Clears the text of an error message by its ID
    fn clear_error(&mut self, id: &str) {
        if let Some(error) = self.iter_mut().find(|e| e.id == id) {
            error.clear_text();
        }
    }

    /// Check if any error messages have been set
    fn has_errors(&self) -> bool {
        self.iter().any(|e| e.text.is_some())
    }

    fn render_errors(&self) -> anyhow::Result<String> {
        Ok(self
            .iter()
            .map(|error| error.render())
            .collect::<Vec<String>>()
            .join(""))
    }
}
