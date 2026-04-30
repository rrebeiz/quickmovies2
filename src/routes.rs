use axum::Router;
use axum::routing::{delete, get, patch, post, put};

use crate::AppState;
use crate::handlers::movies::{
    create_movie, delete_movie_by_id, get_all_movies, get_movie_by_id, update_movie,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/v1/movies/{id}", get(get_movie_by_id))
        .route("/v1/movies", post(create_movie))
        .route("/v1/movies/{id}", delete(delete_movie_by_id))
        .route("/v1/movies", get(get_all_movies))
        .route("/v1/movies/{id}", patch(update_movie))
        .with_state(state)
}
