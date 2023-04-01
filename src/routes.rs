use crate::handlers::home;
use crate::handlers::users;
use crate::AppState;
use axum::routing::{delete, get, patch, post, Router};

pub fn all() -> Router<AppState> {
    return Router::new()
        .route("/", get(home::root))
        .route("/users", get(users::all_users))
        .route("/users/:id", get(users::get_user))
        .route("/users/:id", delete(users::delete_user))
        .route("/users/:id", patch(users::update_user))
        .route("/users", post(users::create_user));
}
