use rocket::serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BurritoStateRecord {
    pub lt: f64,
    pub lg: f64,
    pub sts: i32,
    #[serde(skip_deserializing)]
    pub timestamp: Option<SystemTime>,
    #[serde(skip_deserializing)]
    pub velocity: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RawBurritoStateRecord {
    pub lt: f64,
    pub lg: f64,
    pub sts: i32,
    pub timestamp: Option<SystemTime>,
    pub velocity: f64,
}
