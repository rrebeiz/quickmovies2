use axum::Router;
use axum::routing::{delete, get, post, put};

use crate::AppState;
use crate::handlers::movies::get_movie_by_id;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/movies/{id}", get(get_movie_by_id))
        .with_state(state)
}
