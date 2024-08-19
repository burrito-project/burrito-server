use rocket::Route;
use serde_json::json;

pub fn routes() -> Vec<Route> {
    routes![help_index]
}

#[get("/")]
pub fn help_index() -> serde_json::Value {
    let routes = vec!["/status/?count=<n>"];

    json!({
        "message": "Welcome to the UNMSM burrito tracker API",
        "routes": routes,
    })
}
