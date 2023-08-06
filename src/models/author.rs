use sqlx::FromRow;
use uuid::Uuid;
use serde::Serialize;

#[derive(Debug, FromRow, Serialize)]
pub struct Author {
    id: Uuid,
    first_name: String,
    last_name: String
}
