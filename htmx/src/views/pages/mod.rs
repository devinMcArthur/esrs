use leptos::*;

use crate::{
    routes::ApiRoutes,
    views::{
        components::jobsite::{JobsiteCreate, JobsiteEdit},
        layouts,
    },
};

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <layouts::MainLayout title=String::from("ESRS")>
            <layouts::GradientBody>
                <div id="ws-status-indicator" class="fixed top-4 right-4 h-3 w-3">
                    <div id="ws-status-ping" class="animate-ping absolute inline-flex h-full w-full rounded-full bg-red-500 opacity-75"></div>
                    <div id="ws-status-circle" class="rounded-full relative inline-flex h-3 w-3 bg-red-500"></div>
                </div>
                <div class="relative flex flex-row divide-x divide-orange-500 justify-around border-gray-800 backdrop-blur bg-slate-700/70 p-8 rounded-lg shadow-lg w-3/4 h-3/4">
                    <div class="w-1/2 mx-4 flex flex-col">
                        <div class="text-center mb-4">
                            <span class="text-orange-700 text-3xl font-bold" style="font-family: 'Roboto Slab', serif;">Jobsite</span>
                        </div>
                        <JobsiteCreate />
                        <div class="flex-grow overflow-auto" hx-get=ApiRoutes::get_jobsite_list() hx-trigger="load">
                        </div>
                    </div>
                    <div class="w-1/2 flex flex-col" id="jobsite-edit">
                        <JobsiteEdit jobsite=None />
                    </div>
                </div>
            </layouts::GradientBody>
        </layouts::MainLayout>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <layouts::MainLayout title=String::from("Page Not Found")>
            <layouts::GradientBody>
                <div class="relative bg-white p-8 rounded-lg shadow-lg w-96">
                    <div class="text-center mb-4">
                        <span class="text-orange-700 text-3xl font-bold" style="font-family: 'Roboto Slab', serif;">ESRS</span>
                    </div>
                    <h1>Page Not Found</h1>
                </div>
            </layouts::GradientBody>
        </layouts::MainLayout>
    }
}
