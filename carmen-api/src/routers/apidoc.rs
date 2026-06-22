use super::collections;
use super::documents;
use super::search;

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
    documents::get_by_id,
    documents::raw_document,
    documents::exported_document,
    documents::delete_document,
    documents::schedule_indexing,
    search::full_text,
    search::semantic,
    search::hybrid,
))]
pub struct ApiDoc;
