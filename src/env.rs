use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref IS_MOCKED: bool = env::var("IS_MOCKED").unwrap_or("false".into()) == "true";
    pub static ref AUTH_DRIVER_PASSPHRASE: String = env::var("AUTH_DRIVER_PASSPHRASE").unwrap();
}
