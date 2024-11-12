#![allow(unreachable_patterns)]
use lazy_static::lazy_static;
use rocket::config::{Ident, LogLevel};
use rocket::data::{Limits, ToByteUnit};
use rocket::Config;

#[macro_use]
extern crate rocket;

mod api;
mod core;
mod db;
mod docs;
mod env;
mod routes;

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

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = *startup; // forcing evaluation

    crate::env::dotenv().expect("No .env file");

    let pool = crate::db::create_pool().await.unwrap();

    crate::features::mock::rc::initialize_mocks();
    crate::features::flags::rc::setup_base_flags(&pool)
        .await
        .expect("Failed to setup base flags");

    let rocket_config = Config {
        port: *crate::env::PORT,
        address: [0, 0, 0, 0].into(),
        ident: Ident::none(),
        log_level: LogLevel::Normal,
        limits: Limits::new()
            .limit("forms", 16.mebibytes())
            .limit("json", 32.mebibytes()),
        ..Config::default()
    };

    let mut rocket = rocket::build()
        .configure(rocket_config)
        .attach(core::fairings::Cors)
        .manage(crate::core::AppState::from_db(pool));

    rocket = routes::mount_routers(rocket);
    rocket.launch().await?;

    Ok(())
}
