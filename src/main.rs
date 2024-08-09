use std::sync::RwLock;
use serde_json::json;
use bus_stops::BusStopInfo;
use entities::burrito_state_record::BurritoStateRecord;

#[macro_use] extern crate rocket;

mod bus_stops;
mod routes;
mod cors;
mod entities;
mod utils;

#[derive(Default)]
pub struct AppState {
    messages: RwLock<Vec<BurritoStateRecord>>,
    last_stop: RwLock<Option<BusStopInfo>>,
}

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/status", routes::status::routes())
        .mount("/help", routes![index])
        .register("/", catchers![not_found])
        .attach(cors::Cors)
        .manage(AppState::default())
}
