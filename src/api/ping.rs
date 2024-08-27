use rocket::Route;
use serde_json::json;

use crate::core::utils::get_uptime;

pub fn routes() -> Vec<Route> {
    routes![ping]
}

#[get("/")]
pub fn ping() -> serde_json::Value {
    let uptime_secs = get_uptime().as_secs();

    json!({
        "message": "All systems operational",
        "version": env!("CARGO_PKG_VERSION"),
        "startup": *crate::startup_unix_timestamp,
        "uptime": uptime_secs,
    })
}
