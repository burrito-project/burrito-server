pub use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env;

macro_rules! env_var {
    ($name:expr) => {
        env::var($name).expect(&format!("{} must be set", $name))
    };
}

macro_rules! env_var_or {
    ($name:expr, $default:expr) => {
        env::var($name).unwrap_or($default.into())
    };
}

lazy_static! {
    // Master passphrase. If someone gets this, we are doomed
    pub static ref ROOT_SECRET: String = env_var!("ROOT_SECRET");

    /// The passphrase to authenticate the bus driver POST requests
    pub static ref AUTH_DRIVER_PASSPHRASE: String = env_var!("AUTH_DRIVER_PASSPHRASE");

    /// The postgresql database connection string
    pub static ref DATABASE_URL: String = env_var!("DATABASE_URL");

    /// The max bus state records that we should keep in memory
    pub static ref MAX_MEMORY_RECORDS: usize = env_var_or!("MAX_MEMORY_RECORDS", "1000")
        .parse::<usize>()
        .expect("MAX_MEMORY_RECORDS must be a number");

    /// Whether we should use a mocked bus route to showcase the system. See mock.rs
    pub static ref IS_MOCKED: bool = env_var_or!("IS_MOCKED", "false") == "true";

    /// Token used to interact with the WhatsApp API
    pub static ref AUTH_WHATSAPP_ACCESS_TOKEN: String = env_var!("AUTH_WHATSAPP_ACCESS_TOKEN");

    /// Cloudinary cloud name <https://console.cloudinary.com/pm/developer-dashboard>
    pub static ref CLOUDINARY_CLOUD_NAME: String = env_var!("CLOUDINARY_CLOUD_NAME");

    /// Cloudinary API key <https://console.cloudinary.com/pm/developer-dashboard>
    pub static ref CLOUDINARY_API_KEY: String = env_var!("CLOUDINARY_API_KEY");

    /// Cloudinary API secret <https://console.cloudinary.com/pm/developer-dashboard>
    pub static ref CLOUDINARY_API_SECRET: String = env_var!("CLOUDINARY_API_SECRET");
}
