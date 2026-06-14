#[derive(sqlx::Type)]
#[sqlx(type_name = "status", rename_all = "snake_case")]
pub enum Status {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}
