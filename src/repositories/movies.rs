use crate::models::movies::{Movie, MovieCreateRequest};
use sqlx::PgPool;

pub struct MovieRepository {
    pub db: PgPool,
}

impl MovieRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn create_movie(&self, movie: &MovieCreateRequest) -> Result<Movie, sqlx::Error> {
        let result = sqlx::query_as::<_, Movie>(
            r#"
            INSERT INTO movies (title, year, runtime, genres, created_at, updated_at)
            VALUES ($1, $2, $3, $4, NOW(), NOW())
            RETURNING *
            "#,
        )
        .bind(&movie.title)
        .bind(movie.year)
        .bind(movie.runtime)
        .bind(movie.genres.join(","))
        .fetch_one(&self.db)
        .await?;

        Ok(result)
    }

    pub async fn get_movie_by_id(&self, id: i64) -> Result<Movie, sqlx::Error> {
        let movie = sqlx::query_as::<_, Movie>("SELECT * FROM movies WHERE id = $1")
            .bind(id)
            .fetch_one(&self.db)
            .await?;

        Ok(movie)
    }
}
