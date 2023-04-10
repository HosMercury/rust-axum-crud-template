use axum::extract::State;
use axum_session::{Session, SessionPgPool};

use crate::AppState;

pub async fn root(State(state): State<AppState>, session: Session<SessionPgPool>) -> String {
    let mut count: usize = session.get("count").unwrap_or(0);

    count += 1;
    session.set("count", count);

    count.to_string()
}
