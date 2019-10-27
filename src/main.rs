#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
use rocket::{get, http::Status, routes, Request, catch, catchers, post};
use rocket_contrib::json::Json;

use serde::Serialize;

mod database;
mod schema;
mod sql_types;

use crate::database::Connection;
use crate::sql_types::Signature;

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

#[derive(Serialize, Deserialize)]
struct TenebraeSearch {
    signatures: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct TenebraeResult {
    name: String,
    signature: String
}

impl From<&sql_types::Signature> for TenebraeResult {
    fn from(sig: &Signature) -> Self {
        TenebraeResult{
            name: sig.name.clone(),
            signature: sig.signature.clone()
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TenebraeSearchResult {
    signatures: Vec<TenebraeResult>,
}

#[catch(404)]
fn not_found(_: &Request) -> Json<TenebraeError> {
    Json(TenebraeError{
        version: env!("CARGO_PKG_VERSION").to_string(),
        errors: vec!["The requested resource was not found!".to_string()]
    })
}

#[post("/search", data = "<query>")]
fn search(query: Json<TenebraeSearch>, connection: Connection) -> Result<Json<TenebraeSearchResult>, Status> {
    let result = sql_types::Signature::search(&query.signatures, &connection).map_err(|_| Status::ServiceUnavailable)?;
    Ok(Json(TenebraeSearchResult{
        signatures: result.iter().map(TenebraeResult::from).collect()
    }))
}

#[get("/signature/<id>")]
fn fetch(id: i32, connection: Connection) -> Result<Json<sql_types::Signature>, Status> {
    sql_types::Signature::fetch(id, &connection).map_err(|_| Status::NotFound).map(Json)
}

#[get("/")]
fn index(connection: Connection) -> Result<Json<ServerStatus>, Status> {
    let signatures =
        sql_types::Signature::count(&connection).map_err(|_| Status::ServiceUnavailable)?;
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
        .register(catchers![not_found])
        .mount("/", routes![index, fetch, search])
        .launch();
}
