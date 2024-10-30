use leptos::*;
use leptos_router::ActionForm;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct JobsiteCreateData {
    name: String,
}

#[server]
pub async fn create_jobsite(data: JobsiteCreateData) -> Result<Uuid, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use sqlx::PgPool;
    use models::{events::jobsite::JobsiteCreated, projections::jobsite::Jobsite};
    use eventstore::EventData;
    
    let (db_pool, eventstore): (Data<PgPool>, Data<eventstore::Client>) = extract().await?;

    let mut transaction = db_pool.begin().await.unwrap();

    match Jobsite::get_by_name(&mut transaction, data.name.clone()).await? {
        Some(_) => {
            return Err(ServerFnError::new("A Jobsite already exists with this name"));
        }
        None => {}
    };

    transaction.commit().await?;

    let jobsite_id = uuid::Uuid::new_v4();
    let create_event = JobsiteCreated {
        id: jobsite_id,
        name: data.name.clone(),
    };

    let event = EventData::json("JobsiteCreated", &create_event)
        .expect("Unable to serialize")
        .id(uuid::Uuid::new_v4());

    eventstore
        .append_to_stream(
            format!("jobsite-{}", create_event.id),
            &Default::default(),
            event,
        )
        .await
        .expect("Failed to append event");

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    Ok(jobsite_id)
}

#[component]
pub fn JobsiteCreate() -> impl IntoView {
    let create_jobsite = create_server_action::<CreateJobsite>();

    let pending = create_jobsite.pending();

    let (_, error) = match create_jobsite.value().get() {
        Some(value) => match value {
            Ok(value) => (Some(value), None),
            Err(error) => (None, Some(error)),
        },
        None => (None, None),
    };

    logging::log!("Error: {:?}", error);

    view! {
        <ActionForm class="w-full" action=create_jobsite>
            <div class="mb-4">
                <label class="block text-sm font-medium text-white">Name</label>
                <input
                    name="data[name]"
                    class="mt-1 p-2 w-full border rounded-md text-black"
                    autofocus
                />
            </div>
            {match error {
                Some(error) => {
                    view! { <div class="text-red-500 text-sm">{error.to_string()}</div> }
                        .into_view()
                }
                None => view! {}.into_view(),
            }}
            <button
                type="submit"
                id="jobsite-submit"
                disabled=pending.get()
                class="w-full bg-orange-600 disabled:bg-orange-300 text-white p-2 rounded-md hover:bg-orange-700"
            >
                Submit
            </button>
        </ActionForm>
    }
}
