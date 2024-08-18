use lazy_static::lazy_static;
use std::env;

lazy_static! {
    // The passphrase to authenticate the bus driver POST requests
    pub static ref AUTH_DRIVER_PASSPHRASE: String =
        env::var("AUTH_DRIVER_PASSPHRASE").expect("AUTH_DRIVER_PASSPHRASE must be set");

    // The postgresql database connection string
    pub static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // The max bus state records that we should keep in memory
    pub static ref MAX_MEMORY_MESSAGES: usize = env::var("MAX_MEMORY_MESSAGES")
        .unwrap_or("1000".into())
        .parse::<usize>()
        .expect("MAX_MEMORY_MESSAGES must be a number");

    /// Whether we should use a mocked bus route to showcase the system. See mock.rs
    pub static ref IS_MOCKED: bool = env::var("IS_MOCKED").unwrap_or("false".into()) == "true";
}
