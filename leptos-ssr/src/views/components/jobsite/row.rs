use leptos::*;
use models::projections::jobsite::Jobsite;

#[component]
pub fn JobsiteRow(#[prop(optional)] jobsite: Option<Jobsite>, jobsite_id: String) -> impl IntoView {
    view! {
        <div
            class="jobsite-row cursor-pointer flex items-center justify-between p-2 my-2 bg-gray-400 rounded-md"
            id=format!("jobsite_row_{}", jobsite_id)
        >
            <div>
                {match jobsite {
                    Some(jobsite) => {
                        view! { <span class="text-lg">{jobsite.name}</span> }.into_view()
                    }
                    None => view! { <loading-spinner /> }.into_view(),
                }}
            </div>
        </div>
    }
}
