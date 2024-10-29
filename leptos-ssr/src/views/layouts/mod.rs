use leptos::*;

#[component]
pub fn GradientBody(children: Children) -> impl IntoView {
    view! {
        <body
            id="body"
            class="bg-gradient-to-br from-gray-900 to-gray-700 h-screen flex justify-center items-center"
            style="font-family: 'Roboto'"
        >
            {children()}
        </body>
    }
}
