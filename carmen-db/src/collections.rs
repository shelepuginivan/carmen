use sqlx::PgPool;
use sqlx::types::Uuid;

#[derive(sqlx::FromRow)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

impl Collection {
    pub async fn insert(
        pool: &PgPool,
        name: &str,
        description: Option<&str>,
    ) -> sqlx::Result<Self> {
        sqlx::query_as("INSERT INTO collections (name, description) VALUES ($1, $2) RETURNING *")
            .bind(name)
            .bind(description)
            .fetch_one(pool)
            .await
    }

    pub async fn get(pool: &PgPool, id: Uuid) -> sqlx::Result<Self> {
        sqlx::query_as("SELECT * FROM collections WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn get_all(pool: &PgPool) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as("SELECT * FROM collections")
            .fetch_all(pool)
            .await
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
    ) -> sqlx::Result<Self> {
        sqlx::query_as(
            r#"
            UPDATE collections
            SET name = COALESCE($1, name), description = $2
            WHERE id = $3
            RETURNING *
            "#,
        )
        .bind(name)
        .bind(description)
        .bind(id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> sqlx::Result<Self> {
        sqlx::query_as("DELETE FROM collections WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(pool)
            .await
    }
}
