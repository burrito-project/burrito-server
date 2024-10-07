use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync;
use tokio::sync::broadcast;

use crate::features::bus_driver::schemas::{BurritoPosRecord, WsClientMessage};
use crate::features::bus_stops::schemas::BusStopInfo;

pub struct AppState {
    pub pool: sqlx::Pool<sqlx::Postgres>,
    pub records: sync::RwLock<Vec<BurritoPosRecord>>,
    pub channel: Arc<broadcast::Sender<WsClientMessage>>,
    pub drivers_locks: Arc<sync::Mutex<HashMap<String, ()>>>,
    pub last_stop: sync::RwLock<Option<BusStopInfo>>,
}

impl AppState {
    pub fn from_db(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self {
            pool,
            records: sync::RwLock::new(Vec::with_capacity(*crate::env::MAX_MEMORY_RECORDS)),
            channel: Arc::new(broadcast::channel(1).0),
            drivers_locks: Arc::new(sync::Mutex::new(HashMap::new())),
            last_stop: sync::RwLock::new(None),
        }
    }
}
