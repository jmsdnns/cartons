use sqlx::{FromRow, Pool, Postgres, Row};

use crate::errors::DBError;

const INSERT_USER: &str = "INSERT INTO users (username, password_hash) \
    values($1, $2) \
    RETURNING id";

const SELECT_USER: &str = "SELECT id, username, password_hash, created_at, updated_at \
    FROM users \
    WHERE username = $1";

//  const UPDATE_USER_PASS: &str = "UPDATE users \
//      SET password_hash = $2 \
//      WHERE username = $1";

#[derive(FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub async fn create(
        db: &Pool<Postgres>,
        username: &String,
        password_hash: &String,
    ) -> Result<i32, DBError> {
        let result = sqlx::query(INSERT_USER)
            .bind(username)
            .bind(password_hash)
            .fetch_one(db)
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_username_key") => {
                    DBError::Conflict("username taken".to_string())
                }
                sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_email_key") => {
                    DBError::Conflict("email taken".to_string())
                }
                _ => DBError::InternalError(e.to_string()),
            })?;

        Ok(result.get("id"))
    }

    pub async fn load(db: &Pool<Postgres>, username: &String) -> Result<User, DBError> {
        let result: User = sqlx::query_as(SELECT_USER)
            .bind(username)
            .fetch_one(db)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => DBError::NotFound("user not found".to_string()),
                _ => DBError::InternalError(e.to_string()),
            })?;

        Ok(result)
    }
}
