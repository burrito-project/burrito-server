use rocket::State;

use crate::core::AppState;
use crate::features::flags;

pub async fn list_flags_handler(state: &State<AppState>) -> Vec<flags::schemas::Flag> {
    

    sqlx::query_as!(
        flags::schemas::Flag,
        "SELECT * FROM flags WHERE internal = false ORDER BY name ASC;",
    )
    .fetch_all(&state.pool)
    .await
    .unwrap()
}
