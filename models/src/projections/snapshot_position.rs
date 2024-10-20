use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SnapshotPosition {
    pub key: SnapshotPositionKey,
    pub value: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SnapshotPositionKey {
    Jobsite,
}

impl SnapshotPositionKey {
    pub fn to_string(&self) -> String {
        match self {
            SnapshotPositionKey::Jobsite => "jobsite".to_string(),
        }
    }
}

impl Into<SnapshotPositionKey> for String {
    fn into(self) -> SnapshotPositionKey {
        match self.as_str() {
            "jobsite" => SnapshotPositionKey::Jobsite,
            _ => panic!("Invalid SnapshotPositionKey"),
        }
    }
}

impl SnapshotPosition {
    pub async fn get_by_key(
        transaction: &mut Transaction<'_, Postgres>,
        key: SnapshotPositionKey,
    ) -> Result<Option<Self>, sqlx::Error> {
        let setting = sqlx::query_as!(
            Self,
            r#"
            SELECT key, value
            FROM snapshot_positions
            WHERE key = $1
            "#,
            key.to_string()
        )
        .fetch_optional(&mut **transaction)
        .await?;

        Ok(setting)
    }

    pub async fn insert(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO snapshot_positions (key, value) VALUES ($1, $2)
                ON CONFLICT (key) DO UPDATE SET value = excluded.value;
            "#,
            self.key.to_string(),
            self.value
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }
}
