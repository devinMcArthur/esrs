use leptos::*;
use uuid::Uuid;

use crate::{
    models::jobsite::Jobsite,
    routes::{ApiRoutes, JobsiteClientMessage},
};

#[component]
pub fn JobsiteRow(#[prop(optional)] jobsite: Option<Jobsite>, jobsite_id: Uuid) -> impl IntoView {
    let ws_vals = if jobsite.is_some() {
        JobsiteClientMessage::JobsiteRegister { jobsite_id }
    } else {
        JobsiteClientMessage::JobsiteLoading { jobsite_id }
    };
    let ws_vals = serde_json::to_string(&ws_vals).unwrap();

    view! {
        <div
            class="jobsite-row cursor-pointer flex items-center justify-between p-2 my-2 bg-gray-400 rounded-md"
            id=format!("jobsite_row_{}", jobsite_id)
            hx-get=ApiRoutes::get_jobsite(jobsite_id)
            hx-swap="innerHTML"
            hx-target="#jobsite-edit"
            hx-trigger="click"
        >
            <div
                ws-send
                hx-vals=ws_vals
                hx-trigger="load"
            >
                {match jobsite {
                    Some(jobsite) => view! {
                        <span class="text-lg">{jobsite.name}</span>
                    }.into_view(),
                    None => view! {
                        <loading-spinner />
                    }.into_view()
                }}
            </div>
        </div>
    }
}
