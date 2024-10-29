use leptos::*;

#[component]
pub fn MainLayout(children: Children, title: String) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <Header title=title />
            {children()}
            <Footer />
        </html>
    }
}

#[component]
pub fn GradientBody(children: Children) -> impl IntoView {
    view! {
        <body
            id="body"
            class="bg-gradient-to-br from-gray-900 to-gray-700 h-screen flex justify-center items-center"
            style="font-family: 'Roboto'"
            hx-ext="ws"
            ws-connect="/websocket"
        >
            {children()}
        </body>
    }
}

#[component]
pub fn Header(title: String) -> impl IntoView {
    view! {
        <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>{title}</title>
            <script
                src="https://unpkg.com/htmx.org@2.0.2/dist/htmx.js"
                integrity="sha384-yZq+5izaUBKcRgFbxgkRYwpHhHHCpp5nseXp0MEQ1A4MTWVMnqkmcuFez8x5qfxr"
                crossorigin="anonymous"
            ></script>
            <script src="https://unpkg.com/htmx-ext-ws@2.0.1/ws.js"></script>
            <script src="https://cdn.tailwindcss.com"></script>
            <script src="/public/js/loading-spinner.js" defer></script>
            <link rel="stylesheet" href="/public/css/styles.css" />
            <script>
                document.addEventListener("DOMContentLoaded", () => {
                    document.body.addEventListener("htmx:oobBeforeSwap", (event) => {
                        // If we're doing an append swap, ensure the element is not already in the
                        // DOM, used when a websocket needs to update an existing element in some
                        // clients and append it as new in others

                        var appendId = event.detail.fragment.getAttribute("data-append");

                        // Check if the elemtent is already in the DOM
                        if (appendId && document.getElementById(appendId) !== null) {
                            event.preventDefault();
                        }
                    });
                });
            // If we're doing an append swap, ensure the element is not already in the
            // DOM, used when a websocket needs to update an existing element in some
            // clients and append it as new in others

            // Check if the elemtent is already in the DOM
            </script>
        </head>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="absolute bottom-0 w-full text-center text-white p-4">
            <script src="/public/js/status-indicator.js"></script>
        </footer>
    }
}
