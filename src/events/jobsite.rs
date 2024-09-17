use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct JobsiteCreated {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobsiteUpdated {
    pub id: Uuid,
    pub name: String,
}
