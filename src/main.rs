#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
use rocket::{catch, catchers, get, http::Status, post, put, routes, Request};
use rocket_contrib::json::Json;

use serde::Serialize;

mod database;
mod schema;
mod sql_types;

use crate::{
    database::Connection,
    sql_types::{Signature, SignatureState},
};
use crate::sql_types::ApiKey;

#[derive(Serialize)]
struct ServerStatus {
    motd: Option<String>,
    entries: i64,
    version: String,
    errors: Vec<String>,
}

#[derive(Serialize)]
struct TenebraeError {
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

#[derive(Serialize, Deserialize)]
struct TenebraeEntry {
    name: String,
    signature: String,
}

#[derive(Serialize, Deserialize)]
struct TenebraeAdd {
    name: String,
    signatures: Vec<TenebraeEntry>,
    filename: String,
    filehash: String,
}

#[derive(Serialize, Deserialize)]
struct TenebraeSearch {
    signatures: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct TenebraeResult {
    name: String,
    signature: String,
    filename: String,
    filehash: String,
}

impl From<&sql_types::Signature> for TenebraeResult {
    fn from(sig: &Signature) -> Self {
        TenebraeResult {
            name: sig.name.clone(),
            signature: sig.signature.clone(),
            filename: sig.filename.clone(),
            filehash: sig.filehash.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TenebraeSearchResult {
    signatures: Vec<TenebraeResult>,
}

#[catch(422)]
fn illformed_request(_: &Request) -> Json<TenebraeError> {
    Json(TenebraeError::new("Malformed request!"))
}

#[catch(500)]
fn service_unavailable(_: &Request) -> Json<TenebraeError> {
    Json(TenebraeError::new("Service currently unavailable!"))
}

#[catch(403)]
fn access_denied(_: &Request) -> Json<TenebraeError> {
    Json(TenebraeError::new("Access denied!"))
}

#[catch(404)]
fn not_found(_: &Request) -> Json<TenebraeError> {
    Json(TenebraeError::new("The requested resource was not found!"))
}

#[post("/signature", data = "<query>")]
fn search(
    query: Json<TenebraeSearch>,
    connection: Connection,
) -> Result<Json<TenebraeSearchResult>, Status> {
    let result = sql_types::Signature::search(&query.signatures, &connection)
        .map_err(|_| Status::ServiceUnavailable)?;
    Ok(Json(TenebraeSearchResult {
        signatures: result.iter().map(TenebraeResult::from).collect(),
    }))
}

#[put("/signature", data = "<signature>")]
fn add_signature(signature: Json<TenebraeAdd>, connection: Connection, key: ApiKey) -> Status {

    if let Ok(demokey) = dotenv::var("TENEBRAE_KEY") {
        if key.key != demokey{
            return Status::Forbidden;
        }
    }

    let result = signature
        .signatures
        .iter()
        .map(|sig| {
            Signature::new(
                0,
                &sig.name,
                &sig.signature,
                &signature.filename,
                &signature.filehash,
                SignatureState::Normal,
            )
        })
        .collect::<Vec<_>>();

    Signature::mass_insert(&result, &connection)
        .map(|_| Status::Ok)
        .unwrap_or(Status::ServiceUnavailable)
}

#[get("/signature/<id>")]
fn fetch(id: i32, connection: Connection) -> Result<Json<sql_types::Signature>, Status> {
    sql_types::Signature::fetch(id, &connection)
        .map_err(|_| Status::NotFound)
        .map(Json)
}

#[get("/")]
fn index(connection: Connection) -> Result<Json<ServerStatus>, Status> {
    let signatures = Signature::count(&connection).map_err(|_| Status::ServiceUnavailable)?;
    Ok(Json(ServerStatus {
        motd: None,
        entries: signatures,
        version: env!("CARGO_PKG_VERSION").to_string(),
        errors: vec![],
    }))
}

fn main() {
    rocket::ignite()
        .manage(database::connect())
        .register(catchers![
            not_found,
            service_unavailable,
            illformed_request,
            access_denied
        ])
        .mount("/", routes![index, fetch, search, add_signature])
        .launch();
}
