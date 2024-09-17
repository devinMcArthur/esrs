use leptos::*;

use crate::{routes::ApiRoutes, views::FormError};

#[component]
pub fn JobsiteCreate() -> impl IntoView {
    view! {
        <form
          hx-post=ApiRoutes::post_jobsite()
          hx-swap="none"
          hx-disabled-elt="#jobsite-submit"
          class="w-full"
        >
          <div class="mb-4">
            <label class="block text-sm font-medium text-white">Name</label>
            <input
              name="name"
              class="mt-1 p-2 w-full border rounded-md text-black"
              autofocus
            />
            <FormError id="name-error".to_string() />
          </div>
          <button id="jobsite-submit" class="w-full bg-orange-600 disabled:bg-orange-300 text-white p-2 rounded-md hover:bg-orange-700">
            Submit
          </button>
        </form>
    }
}
