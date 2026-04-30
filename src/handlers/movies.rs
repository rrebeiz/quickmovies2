use crate::AppState;
use crate::errors::AppError;
use crate::models::movies::{Movie, MovieCreateRequest};
use axum::Json;
use axum::extract::{Path, State};

pub async fn get_movie_by_id(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<Movie>, AppError> {
    let movie = state.movie_service.get_movie_by_id(id).await;
    match movie {
        Ok(movie) => Ok(Json(movie)),
        Err(e) => Err(e),
    }
}

pub async fn create_movie(
    State(state): State<AppState>,
    Json(payload): Json<MovieCreateRequest>,
) -> Result<Json<Movie>, AppError> {
    let movie = state.movie_service.create_movie(payload).await;
    match movie {
        Ok(movie) => Ok(Json(movie)),
        Err(e) => Err(e),
    }
}

pub async fn delete_movie_by_id(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<String>, AppError> {
    let result = state.movie_service.delete_movie_by_id(id).await;
    match result {
        Ok(()) => Ok(Json(format!("movie with id {} deleted!", id))),
        Err(e) => Err(e),
    }
}
