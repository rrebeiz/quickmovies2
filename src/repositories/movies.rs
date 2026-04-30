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
            INSERT INTO movies (title, year, runtime, genres)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(&movie.title)
        .bind(&movie.year)
        .bind(&movie.runtime)
        .bind(&movie.genres)
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
    pub async fn delete_movie_by_id(&self, id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM movies WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
