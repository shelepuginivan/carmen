use super::collections;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        collections::create,
        collections::get_all,
        collections::get_by_id,
        collections::update,
        collections::delete_collection,
        collections::get_extractions,
        collections::schedule_extraction,
    ),
    components(schemas(
        crate::service::collections::CollectionExtractionIn,
        crate::service::collections::CollectionExtractionOut,
        crate::service::collections::CollectionExtractionTypeOut,
        crate::service::collections::CollectionIn,
        crate::service::collections::CollectionOut,
        crate::service::collections::CollectionUpdate,
        super::error::ErrorWithDetail,
    ))
)]
pub struct ApiDoc;
