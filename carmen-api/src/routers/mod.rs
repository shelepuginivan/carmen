pub mod collections;
pub mod documents;
pub mod error;
pub mod extractions;
pub mod search;

#[derive(utoipa::OpenApi)]
#[openapi(
    nest(
        (path = "/api/v1/collections", api = collections::ApiDoc),
        (path = "/api/v1/documents", api = documents::ApiDoc),
        (path = "/api/v1/extractions", api = extractions::ApiDoc),
        (path = "/api/v1/search", api = search::ApiDoc),
    ),
)]
pub struct ApiDoc;
