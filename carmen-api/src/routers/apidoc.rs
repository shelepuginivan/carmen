use super::collections;
use super::documents;
use super::extractions;
use super::search;

#[derive(utoipa::OpenApi)]
#[openapi(paths(
    collections::create_collection,
    collections::get_all_collections,
    collections::get_collection,
    collections::update_collection,
    collections::delete_collection,
    collections::get_extractions,
    collections::get_documents,
    collections::extract_collection,
    documents::get_document,
    documents::stream_raw_document,
    documents::stream_exported_document,
    documents::delete_document,
    documents::index_document,
    extractions::get_extraction,
    extractions::delete_extraction,
    extractions::cancel_extraction,
    extractions::replay_extraction,
    search::full_text,
    search::semantic,
    search::hybrid,
))]
pub struct ApiDoc;
