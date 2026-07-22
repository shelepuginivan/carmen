use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub size: u32,
}
