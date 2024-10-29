use leptos::*;
use models::projections::jobsite::Jobsite;

use crate::views::components::jobsite::JobsiteRow;

#[server]
pub async fn fetch_jobsites() -> Result<Vec<Jobsite>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use sqlx::PgPool;
    
    let db_pool: Data<PgPool> = extract().await?;

    let mut transaction = db_pool.begin().await?;

    let jobsites = Jobsite::get_list(&mut transaction).await?;

    transaction.commit().await?;

    Ok(jobsites)
}

#[component]
pub fn JobsiteList() -> impl IntoView {
    let jobsites = create_resource(
        || (),
        move |_| async move {
            fetch_jobsites().await.ok()
        },
    );

    view! {
        <div class="w-11/12 mx-auto rounded-md p-4" id="jobsite-list">
            <Suspense>
                <For
                    each=move || jobsites.get().unwrap_or_default().unwrap_or_default()
                    key=|jobsite| jobsite.id
                    let:jobsite
                >

                    <JobsiteRow jobsite=jobsite.clone() jobsite_id=jobsite.id.to_string() />
                </For>
            </Suspense>
        </div>
    }
}
