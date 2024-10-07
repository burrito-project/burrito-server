pub mod schemas;
pub mod handlers {
    mod get_bus_status;
    mod get_driver_battery;

    pub use get_bus_status::get_burrito_status_handler;
    pub use get_driver_battery::get_driver_battery_handler;
}
