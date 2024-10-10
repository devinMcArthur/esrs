use leptos::*;
use uuid::Uuid;

use crate::{models::jobsite::Jobsite, views::components::jobsite::JobsiteRow};

#[component]
pub fn JobsiteList(
    jobsites: Vec<Jobsite>,
    #[prop(optional)] append: Option<Uuid>,
) -> impl IntoView {
    let hx_swap_oob = if append.is_some() { "afterbegin" } else { "" };
    let data_append = if let Some(append) = append {
        format!("jobsite_row_{}", append)
    } else {
        "".to_string()
    };

    view! {
        <div
            class="w-11/12 mx-auto rounded-md p-4"
            id="jobsite-list"
            hx-swap-oob=hx_swap_oob
            data-append=data_append
        >
            {jobsites.into_iter().map(|jobsite| view! { <JobsiteRow jobsite=jobsite.clone() jobsite_id=jobsite.id /> }).collect::<Vec<_>>().into_view()}
        </div>
    }
}
