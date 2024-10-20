use leptos::*;

pub mod components;
pub mod layouts;
pub mod pages;

pub trait Renderable {
    fn render(&self, renderer: &TemplateRenderer) -> anyhow::Result<String>;
}

pub struct TemplateRenderer;

impl TemplateRenderer {
    pub fn render<F, N>(f: F) -> String
    where
        F: FnOnce() -> N + 'static,
        N: IntoView,
    {
        format!("{}", leptos::ssr::render_to_string(f).to_string())
    }
}

#[component]
pub fn FormError(id: String, #[prop(optional)] children: Option<Children>) -> impl IntoView {
    view! {
        <div id={id.clone()} hx-select=".error-box" class="error-container">
          <div hx-swap-oob=format!("innerHTML:#{}", id) class="error-box">
            {
                if let Some(children) = children {
                    view! {
                      <div class="my-2 p-2 bg-red-100 text-red-700 rounded-md" role="alert">
                        <span id="error-text" class="block text-base sm:inline">{children()}</span>
                      </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }
          </div>
        </div>
    }
}
