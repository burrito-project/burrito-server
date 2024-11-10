use rocket::State;

use crate::core::AppState;
use crate::features::updates::schemas;

pub async fn list_app_versions_handler(state: &State<AppState>) -> Vec<schemas::AppVersion> {
    sqlx::query_as!(schemas::AppVersion, "SELECT * FROM app_versions;")
        .fetch_all(&state.pool)
        .await
        .unwrap()
}
