use crate::errors::AppError;
use crate::models::movies::{Movie, MovieCreateRequest};
use crate::repositories::movies::MovieRepository;

pub struct MovieService {
    repo: MovieRepository,
}

impl MovieService {
    pub fn new(repo: MovieRepository) -> Self {
        Self { repo }
    }

    pub async fn create_movie(&self, movie: MovieCreateRequest) -> Result<Movie, AppError> {
        match self.repo.create_movie(&movie).await {
            Ok(movie) => Ok(movie),
            Err(e) => Err(AppError::Internal(e.to_string())),
        }
    }

    pub async fn get_movie_by_id(&self, id: i64) -> Result<Movie, AppError> {
        match self.repo.get_movie_by_id(id).await {
            Ok(movie) => Ok(movie),
            Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound(format!(
                "movie with id {} not found",
                id
            ))),
            Err(e) => Err(AppError::Internal(e.to_string())),
        }
    }
}
