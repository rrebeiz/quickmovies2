use crate::AppState;
use crate::errors::AppError;
use crate::models::movies::Movie;
use axum::Json;
use axum::extract::{Path, State};

pub async fn get_movie_by_id(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<Movie>, AppError> {
    let movie = state.movie_service.get_movie_by_id(id).await?;
    Ok(Json(movie))
}
