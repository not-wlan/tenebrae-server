use rocket::{catch, Request};
use rocket_contrib::json::Json;

#[derive(Serialize)]
pub struct TenebraeError {
    version: String,
    errors: Vec<String>,
}

impl TenebraeError {
    pub fn new(message: &str) -> Self {
        TenebraeError {
            version: env!("CARGO_PKG_VERSION").to_string(),
            errors: vec![message.to_string()],
        }
    }
}

#[catch(422)]
pub fn illformed_request(_: &Request) -> Json<TenebraeError> {
    Json(TenebraeError::new("Malformed request!"))
}

#[catch(500)]
pub fn service_unavailable(_: &Request) -> Json<TenebraeError> {
    Json(TenebraeError::new("Service currently unavailable!"))
}

#[catch(403)]
pub fn access_denied(_: &Request) -> Json<TenebraeError> {
    Json(TenebraeError::new("Access denied!"))
}

#[catch(404)]
pub fn not_found(_: &Request) -> Json<TenebraeError> {
    Json(TenebraeError::new("The requested resource was not found!"))
}
