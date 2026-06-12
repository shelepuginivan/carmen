use super::collections;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(collections::tasks_retry_failed,),
    components(schemas(
        crate::service::collections::CollectionTaskMetaOut,
        super::error::ErrorWithDetail,
    ))
)]
pub struct ApiDoc;
