use dotenvy::dotenv;
use lazy_static::lazy_static;
use rocket::{config::Ident, Config};
use serde_json::json;

use entities::BurritoStateRecord;

#[macro_use]
extern crate rocket;

mod api;
mod auth;
mod bus_stops;
mod cors;
mod db;
mod entities;
mod env;
mod mock;
mod responders;
mod utils;

#[get("/")]
fn index() -> serde_json::Value {
    let routes = vec!["/status/?count=<n>"];

    json!({
        "message": "Welcome to the UNMSM burrito tracker API",
        "routes": routes,
    })
}

#[catch(404)]
fn not_found() -> serde_json::Value {
    json!({
        "message": "That's a certified 404 classic. Lost? Try /help",
    })
}

lazy_static! {
    pub static ref startup: std::time::SystemTime = std::time::SystemTime::now();
}

pub const PORT: u16 = 6969;
pub const SELF_URL: &str = "http://localhost:6969";

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = *startup;

    dotenv().expect("No .env file");

    let config = Config {
        port: PORT,
        address: [0, 0, 0, 0].into(),
        ident: Ident::none(),
        ..Config::default()
    };

    let pool = crate::db::create_pool().await.unwrap();

    crate::mock::initialize_mocks();

    rocket::build()
        .configure(config)
        .mount("/", routes![index])
        .mount("/status", api::status::routes())
        .mount("/help", routes![index])
        .register("/", catchers![not_found])
        .attach(cors::Cors)
        .manage(crate::entities::AppState::from_db(pool))
        .launch()
        .await?;

    Ok(())
}
