use lazy_static::lazy_static;
use std::env;

lazy_static! {
    /// The passphrase to authenticate the bus driver POST requests
    pub static ref AUTH_DRIVER_PASSPHRASE: String =
        env::var("AUTH_DRIVER_PASSPHRASE").expect("AUTH_DRIVER_PASSPHRASE must be set");

    /// The postgresql database connection string
    pub static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    /// The max bus state records that we should keep in memory
    pub static ref MAX_MEMORY_RECORDS: usize = env::var("MAX_MEMORY_RECORDS")
        .unwrap_or("1000".into())
        .parse::<usize>()
        .expect("MAX_MEMORY_RECORDS must be a number");

    /// Whether we should use a mocked bus route to showcase the system. See mock.rs
    pub static ref IS_MOCKED: bool = env::var("IS_MOCKED").unwrap_or("false".into()) == "true";

    /// Token used to interact with the WhatsApp API
    pub static ref AUTH_WHATSAPP_ACCESS_TOKEN: String =
    env::var("AUTH_WHATSAPP_ACCESS_TOKEN").expect("AUTH_WHATSAPP_ACCESS_TOKEN must be set");

    /// Cloudinary cloud name <https://console.cloudinary.com/pm/developer-dashboard>
    pub static ref CLOUDINARY_CLOUD_NAME: String =
    env::var("CLOUDINARY_CLOUD_NAME").expect("CLOUDINARY_CLOUD_NAME must be set");

    /// Cloudinary API key <https://console.cloudinary.com/pm/developer-dashboard>
    pub static ref CLOUDINARY_API_KEY: String =
    env::var("CLOUDINARY_API_KEY").expect("CLOUDINARY_API_KEY must be set");

    /// Cloudinary API secret <https://console.cloudinary.com/pm/developer-dashboard>
    pub static ref CLOUDINARY_API_SECRET: String =
    env::var("CLOUDINARY_API_SECRET").expect("CLOUDINARY_API_SECRET must be set");
}
