use axum::extract::State;

use crate::AppState;

pub async fn root(State(state): State<AppState>) -> String {
    state.app_name
}
