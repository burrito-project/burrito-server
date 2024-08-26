use parking_lot::RwLock;

use crate::bus_stops::BusStopInfo;
use crate::entities::state_record::BurritoStateRecord;

pub struct AppState {
    pub pool: sqlx::Pool<sqlx::Postgres>,
    pub messages: RwLock<Vec<BurritoStateRecord>>,
    pub last_stop: RwLock<Option<BusStopInfo>>,
}

impl AppState {
    pub fn from_db(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self {
            pool,
            messages: RwLock::new(Vec::with_capacity(*crate::env::MAX_MEMORY_RECORDS)),
            last_stop: RwLock::new(None),
        }
    }
}
