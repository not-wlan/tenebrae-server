#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
use rocket::{get, http::Status, routes};
use rocket_contrib::json::Json;

use serde::Serialize;

mod database;
mod schema;
mod sql_types;

use crate::database::Connection;

#[derive(Serialize)]
struct ServerStatus {
    motd: Option<String>,
    entries: i64,
    version: String,
    errors: Vec<String>,
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
        .mount("/", routes![index])
        .launch();
}
