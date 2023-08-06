use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthorForm {
    pub first_name: String,
    pub last_name: String
}