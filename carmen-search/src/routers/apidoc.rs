use super::collections;

#[derive(utoipa::OpenApi)]
#[openapi(paths(
    collections::create,
    collections::get_all,
    collections::get_by_id,
    collections::update,
    collections::delete_collection,
    collections::get_extractions,
    collections::get_documents,
    collections::schedule_extraction,
))]
pub struct ApiDoc;
