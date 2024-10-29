use leptos::*;

use crate::views::{
    components::jobsite::{JobsiteCreate, JobsiteEdit, JobsiteList},
    layouts,
};

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <layouts::GradientBody>
            <div id="ws-status-indicator" class="fixed top-4 right-4 h-3 w-3">
                <div
                    id="ws-status-ping"
                    class="animate-ping absolute inline-flex h-full w-full rounded-full bg-red-500 opacity-75"
                ></div>
                <div
                    id="ws-status-circle"
                    class="rounded-full relative inline-flex h-3 w-3 bg-red-500"
                ></div>
            </div>
            <div class="relative flex flex-row divide-x divide-orange-500 justify-around border-gray-800 backdrop-blur bg-slate-700/70 p-8 rounded-lg shadow-lg w-3/4 h-3/4">
                <div class="w-1/2 mx-4 flex flex-col">
                    <div class="text-center mb-4">
                        <span
                            class="text-orange-700 text-3xl font-bold"
                            style="font-family: 'Roboto Slab', serif;"
                        >
                            Jobsite
                        </span>
                    </div>
                    <JobsiteCreate />
                    <div class="flex-grow overflow-auto">
                        <JobsiteList />
                    </div>
                </div>
                <div class="w-1/2 flex flex-col" id="jobsite-edit">
                    <JobsiteEdit jobsite=None />
                </div>
            </div>
        </layouts::GradientBody>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <layouts::GradientBody>
            <div class="relative bg-white p-8 rounded-lg shadow-lg w-96">
                <div class="text-center mb-4">
                    <span
                        class="text-orange-700 text-3xl font-bold"
                        style="font-family: 'Roboto Slab', serif;"
                    >
                        ESRS
                    </span>
                </div>
                <h1>Page Not Found</h1>
            </div>
        </layouts::GradientBody>
    }
}
