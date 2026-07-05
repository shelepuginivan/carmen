use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct CreateCollection {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateCollection {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

impl From<carmen_db::collections::Collection> for Collection {
    fn from(value: carmen_db::collections::Collection) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
        }
    }
}
