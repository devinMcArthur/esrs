use leptos::*;

use models::projections::jobsite::Jobsite;

#[component]
pub fn JobsiteEdit(jobsite: Option<Jobsite>) -> impl IntoView {
    match jobsite {
        None => view! {
            <div class="h-full w-full">
                <div class="text-center align-center m-auto">
                    <span class="text-orange-700 text-3xl font-bold italic">
                        No Jobsite Selected
                    </span>
                </div>
            </div>
        },
        Some(jobsite) => {
            view! {
                <div id=format!("jobsite_edit_{}", jobsite.id) class="h-full w-full mx-4">
                    <div class="text-center mb-4">
                        <span
                            class="text-orange-700 text-3xl font-bold"
                            style="font-family: 'Roboto Slab', serif;"
                        >
                            {jobsite.name.clone()}
                        </span>
                    </div>
                    <form class="w-full">
                        <div class="mb-4">
                            <label class="block text-sm font-medium text-white">Name</label>
                            <input
                                name="name"
                                value=jobsite.name
                                class="mt-1 p-2 w-full border rounded-md text-black"
                                autofocus
                            />
                        </div>
                        <button
                            id="jobsite-edit-submit"
                            class="w-full bg-orange-600 disabled:bg-orange-300 text-white p-2 rounded-md hover:bg-orange-700"
                        >
                            Update
                        </button>
                    </form>
                </div>
            }
        }
    }
}
