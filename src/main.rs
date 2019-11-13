#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
use rocket::{catchers, get, http::Status, post, put, routes};
use rocket_contrib::json::Json;

use serde::Serialize;

mod database;
mod error;
mod schema;
mod sql_types;

use crate::{
    database::Connection,
    error::*,
    sql_types::{ApiKey, Signature},
};

#[derive(Serialize)]
struct ServerStatus {
    motd: Option<String>,
    entries: i64,
    version: String,
    errors: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct TenebraeEntry {
    name: String,
    signature: String,
}

#[derive(Serialize, Deserialize)]
struct TenebraeAdd {
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

#[post("/signature", data = "<query>")]
fn search(
    query: Json<TenebraeSearch>,
    connection: Connection,
) -> Result<Json<TenebraeSearchResult>, Status> {
    let result = sql_types::Signature::search(&query.signatures, &connection)
        .map_err(|_| Status::BadRequest)?;
    Ok(Json(TenebraeSearchResult {
        signatures: result.iter().map(TenebraeResult::from).collect(),
    }))
}

#[put("/signature", data = "<signature>")]
fn add_signature(signature: Json<TenebraeAdd>, connection: Connection, key: ApiKey) -> Status {
    let result = signature
        .signatures
        .iter()
        .map(|sig| {
            Signature::new(
                key.id,
                &sig.name,
                &sig.signature,
                &signature.filename,
                &signature.filehash
            )
        })
        .collect::<Vec<_>>();

    Signature::mass_insert(&result, &connection)
        .map(|_| Status::Ok)
        .unwrap_or(Status::BadRequest)
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
