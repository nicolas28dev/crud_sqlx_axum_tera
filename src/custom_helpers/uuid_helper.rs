use std::str::FromStr;
use uuid::Uuid;
use sqlx::Error;

pub fn str_to_uuid(guid: &str) -> Result<Uuid, Error> {
    match Uuid::from_str(guid) {
        Ok(uuid) => Ok(uuid),
        Err(e) => {
            eprintln!("Error parsing UUID: {}", e);
            Err(Error::Protocol(format!("Invalid UUID: {}", e).into()))
        },
    }
}