use serde::{Deserialize, Serialize};

// the output to our `create_user` handler
#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
}
