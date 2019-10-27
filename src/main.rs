#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]
use serde::Serialize;
use rocket::routes;
use rocket::get;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize)]
struct Status {
    motd: Option<String>,
    entries: usize,
    version: String,
    errors: Vec<String>,
}

#[get("/")]
fn index() -> Json<Status> {
    Json(Status{
        motd: None,
        entries: 0,
        version: "".to_string(),
        errors: vec![]
    })
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
