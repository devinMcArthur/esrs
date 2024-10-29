use leptos::*;

#[component]
pub fn JobsiteCreate() -> impl IntoView {
    view! {
        <form class="w-full">
            <div class="mb-4">
                <label class="block text-sm font-medium text-white">Name</label>
                <input name="name" class="mt-1 p-2 w-full border rounded-md text-black" autofocus />
            </div>
            <button
                id="jobsite-submit"
                class="w-full bg-orange-600 disabled:bg-orange-300 text-white p-2 rounded-md hover:bg-orange-700"
            >
                Submit
            </button>
        </form>
    }
}
