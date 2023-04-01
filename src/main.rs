mod handlers;
mod models;
mod routes;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
    pub pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("err getting url");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("error connection to db");

    let state = AppState {
        app_name: "Todos".to_string(),
        pool,
    };

    let app = routes::all();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.with_state(state).into_make_service())
        .await
        .unwrap();
}
