use std::collections::HashMap;

use crate::AppState;
use crate::errors::AppError;
use crate::models::movies::{Movie, MovieCreateRequest, MovieResponse, MovieUpdateRequest};
use crate::validator::Validator;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::de::value::Error;

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
    let mut validator = Validator::new();

    if !validate_create_movie(&mut validator, &payload) {
        return Err(AppError::Validation(validator.errors));
    }

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

pub async fn update_movie(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<MovieUpdateRequest>,
) -> Result<Json<MovieResponse>, AppError> {
    let movie = state.movie_service.get_movie_by_id(id).await;
    match movie {
        Ok(movie) => {
            let movie = movie_update_to_movie(movie, payload);
            let updated_movie = state.movie_service.update_movie(id, movie).await;
            match updated_movie {
                Ok(updated_move) => Ok(Json(movie_to_response(updated_move))),
                Err(e) => Err(e),
            }
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

fn movie_update_to_movie(movie: Movie, update: MovieUpdateRequest) -> Movie {
    Movie {
        id: movie.id,
        title: update.title.unwrap_or(movie.title),
        year: update.year.unwrap_or(movie.year),
        runtime: update.runtime.unwrap_or(movie.runtime),
        genres: update.genres.unwrap_or(movie.genres),
        created_at: movie.created_at,
        updated_at: movie.updated_at,
        version: movie.version,
    }
}

fn validate_create_movie(validator: &mut Validator, payload: &MovieCreateRequest) -> bool {
    validator.check(!payload.title.is_empty(), "title", "title cannot be empty");
    validator.check(
        !payload.year.is_negative(),
        "year",
        "year cannot be negative",
    );
    validator.check(
        !payload.runtime.is_negative(),
        "runtime",
        "runtime cannot be empty",
    );
    validator.check(
        !payload.genres.is_empty(),
        "genres",
        "genres cannot be empty",
    );

    let permitted_values: Vec<String> = vec![
        "Action".to_string(),
        "Comedy".to_string(),
        "Drama".to_string(),
        "Sci-Fi".to_string(),
        "Thriller".to_string(),
    ];

    validator.permitted_value(&payload.genres, permitted_values);

    validator.unique(payload.genres.clone());
    validator.valid()
}
