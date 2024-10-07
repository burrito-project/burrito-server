#![allow(unreachable_patterns)]
use dotenvy::dotenv;
use lazy_static::lazy_static;
use rocket::{
    config::Ident,
    data::{Limits, ToByteUnit},
    fs, Config,
};
use serde_json::json;

#[macro_use]
extern crate rocket;

mod api;
mod core;
mod db;
mod env;

mod features {
    pub mod analytics;
    pub mod auth;
    pub mod bot;
    pub mod bus_driver;
    pub mod bus_status;
    pub mod bus_stops;
    pub mod cdn;
    pub mod flags;
    pub mod identities;
    pub mod mock;
    pub mod notifications;
    pub mod updates;
}

lazy_static! {
    pub static ref startup: std::time::SystemTime = std::time::SystemTime::now();
    pub static ref startup_unix_timestamp: u64 = startup
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
}

pub const PORT: u16 = 6969;
/// For making requests to our own endpoints
pub const SELF_URL: &str = "http://localhost:6969";
/// Production host url
pub const HOST_URL: &str = "https://api.contigosanmarcos.com";

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = *startup; // forcing evaluation

    dotenv().expect("No .env file");

    let config = Config {
        port: PORT,
        address: [0, 0, 0, 0].into(),
        ident: Ident::none(),
        limits: Limits::new()
            .limit("forms", 16.mebibytes())
            .limit("json", 32.mebibytes()),
        ..Config::default()
    };

    let pool = crate::db::create_pool().await.unwrap();

    crate::features::mock::rc::initialize_mocks();
    crate::features::flags::rc::setup_base_flags(&pool)
        .await
        .expect("Failed to setup base flags");

    rocket::build()
        .configure(config)
        .mount("/", api::index::routes())
        .mount("/ws", api::ws::routes())
        .mount("/map", api::map::routes())
        .mount("/help", routes![api::index::help_index])
        .mount("/auth", api::auth::routes())
        .mount("/ping", api::ping::routes())
        .mount("/hooks", api::hooks::routes())
        .mount("/flags", api::flags::routes())
        .mount("/health", api::ping::routes())
        .mount("/status", api::status::routes())
        .mount("/driver", api::driver::routes())
        .mount("/battery", api::battery::routes())
        .mount("/session", api::session::routes())
        .mount("/versions", api::versions::routes())
        .mount("/analytics", api::analytics::routes())
        .mount("/notifications", api::notifications::routes())
        .mount("/pending_updates", api::pending_updates::routes())
        .mount("/public", fs::FileServer::from(fs::relative!("public")))
        .register("/", catchers![not_found])
        .attach(core::fairings::Cors)
        .manage(crate::core::AppState::from_db(pool))
        .launch()
        .await?;

    Ok(())
}

#[catch(404)]
fn not_found() -> serde_json::Value {
    json!({
        "message": "That's a certified 404 classic. Lost? Try /help",
    })
}
