use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::events::jobsite::{JobsiteCreated, JobsiteUpdated};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Jobsite {
    pub id: Uuid,
    pub name: String,
}

impl Jobsite {
    pub async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        created_event: &JobsiteCreated,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            INSERT INTO jobsites (id, name)
            VALUES ($1, $2)
            RETURNING *;
            "#,
            created_event.id,
            created_event.name
        )
        .fetch_one(&mut **transaction)
        .await
    }

    pub async fn update(
        transaction: &mut Transaction<'_, Postgres>,
        updated_event: &JobsiteUpdated,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            UPDATE jobsites
            SET name = $2
            WHERE id = $1
            RETURNING *;
            "#,
            updated_event.id,
            updated_event.name
        )
        .fetch_one(&mut **transaction)
        .await
    }

    pub async fn get_by_id(
        transaction: &mut Transaction<'_, Postgres>,
        id: &Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, name
            FROM jobsites
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&mut **transaction)
        .await
    }

    pub async fn get_by_name(
        transaction: &mut Transaction<'_, Postgres>,
        name: String,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, name
            FROM jobsites
            WHERE name = $1
            "#,
            name
        )
        .fetch_optional(&mut **transaction)
        .await
    }

    pub async fn get_list(
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, name
            FROM jobsites
            "#,
        )
        .fetch_all(&mut **transaction)
        .await
    }
}
