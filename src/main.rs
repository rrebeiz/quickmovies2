use crate::{config::Config, repositories::movies::MovieRepository};

mod config;
mod db;
mod errors;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;
mod validator;

use crate::services::movies::MovieService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub movie_service: Arc<MovieService>,
}

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let pool = db::create_pool(&config).await;
    let repo = MovieRepository::new(pool);
    let service = MovieService::new(repo);

    let state = AppState {
        movie_service: Arc::new(service),
    };

    let addr = format!("{}:{}", config.host, config.port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to start the server");

    let router = routes::create_router(state);

    axum::serve(listener, router).await.unwrap();
}
