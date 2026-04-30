use crate::AppState;
use crate::errors::AppError;
use crate::models::movies::{Movie, MovieCreateRequest, MovieResponse};
use axum::Json;
use axum::extract::{Path, State};

pub async fn get_movie_by_id(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<MovieResponse>, AppError> {
    let movie = state.movie_service.get_movie_by_id(id).await;
    match movie {
        Ok(movie) => {
            let res = movie_to_response(movie);
            Ok(Json(res))
        }
        Err(e) => Err(e),
    }
}

pub async fn create_movie(
    State(state): State<AppState>,
    Json(payload): Json<MovieCreateRequest>,
) -> Result<Json<MovieResponse>, AppError> {
    let movie = state.movie_service.create_movie(payload).await;
    match movie {
        Ok(movie) => {
            let res = movie_to_response(movie);
            Ok(Json(res))
        }
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

pub async fn get_all_movies(
    State(state): State<AppState>,
) -> Result<Json<Vec<MovieResponse>>, AppError> {
    let result = state.movie_service.get_all_movies().await;
    match result {
        Ok(movies) => {
            let res = movies.into_iter().map(|m| movie_to_response(m)).collect();
            Ok(Json(res))
        }
        Err(e) => Err(e),
    }
}

fn movie_to_response(movie: Movie) -> MovieResponse {
    MovieResponse {
        id: movie.id,
        title: movie.title,
        year: movie.year,
        runtime: movie.runtime,
        genres: movie.genres,
    }
}
