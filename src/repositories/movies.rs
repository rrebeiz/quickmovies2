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

    pub async fn get_all_movies(&self) -> Result<Vec<Movie>, sqlx::Error> {
        let query = r#"select * from movies"#;
        let result = sqlx::query_as::<_, Movie>(query)
            .fetch_all(&self.db)
            .await?;
        Ok(result)
    }

    pub async fn update_movie(&self, id: i64, movie: &Movie) -> Result<Movie, sqlx::Error> {
        let query = r#"UPDATE movies SET title = $1, year = $2, runtime = $3, genres = $4, version = version + 1 WHERE id = $5 and version = $6 RETURNING *"#;
        let result = sqlx::query_as::<_, Movie>(query)
            .bind(&movie.title)
            .bind(&movie.year)
            .bind(&movie.runtime)
            .bind(&movie.genres)
            .bind(&id)
            .bind(&movie.version)
            .fetch_one(&self.db)
            .await?;

        Ok(result)
    }
}
