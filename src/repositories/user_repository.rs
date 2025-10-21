use crate::models::User;
use sqlx::{PgPool, Error as SqlxError};
use uuid::Uuid;

/// PostgreSQL-based user repository
#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Find all users
    pub async fn find_all(&self) -> Result<Vec<User>, SqlxError> {
        let users = sqlx::query_as::<_, User>("SELECT id, name, email, created_at FROM users ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    /// Find user by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, created_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Find user by email
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, created_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Create a new user
    pub async fn create(&self, name: String, email: String) -> Result<User, SqlxError> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email, created_at"
        )
        .bind(name)
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Update an existing user
    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        email: Option<String>,
    ) -> Result<Option<User>, SqlxError> {
        // First check if user exists
        let existing_user = self.find_by_id(id).await?;

        if existing_user.is_none() {
            return Ok(None);
        }

        let existing = existing_user.unwrap();

        // Use existing values if new ones are not provided
        let new_name = name.unwrap_or(existing.name);
        let new_email = email.unwrap_or(existing.email);

        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING id, name, email, created_at"
        )
        .bind(new_name)
        .bind(new_email)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Some(user))
    }

    /// Delete a user by ID
    pub async fn delete(&self, id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // Count total users
    pub async fn count(&self) -> Result<i64, SqlxError> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }
}
