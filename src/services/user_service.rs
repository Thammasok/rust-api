use crate::errors::AppError;
use crate::models::{CreateUserRequest, UpdateUserRequest, User};
use crate::repositories::UserRepository;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, AppError> {
        self.repository
            .find_all()
            .await
            .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<User, AppError> {
        self.repository
            .find_by_id(id)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?
            .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", id)))
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User, AppError> {
        // Validate email format
        if !self.is_valid_email(&request.email) {
            return Err(AppError::BadRequest("Invalid email format".to_string()));
        }

        // Check if email already exists
        let existing_user = self
            .repository
            .find_by_email(&request.email)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

        if existing_user.is_some() {
            return Err(AppError::Conflict(format!(
                "User with email {} already exists",
                request.email
            )));
        }

        // Validate name
        if request.name.trim().is_empty() {
            return Err(AppError::BadRequest("Name cannot be empty".to_string()));
        }

        let user = self
            .repository
            .create(request.name, request.email)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

        Ok(user)
    }

    pub async fn update_user(
        &self,
        id: Uuid,
        request: UpdateUserRequest,
    ) -> Result<User, AppError> {
        // Check if user exists
        let existing = self
            .repository
            .find_by_id(id)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

        if existing.is_none() {
            return Err(AppError::NotFound(format!("User with id {} not found", id)));
        }

        // Validate email if provided
        if let Some(email) = &request.email {
            if !self.is_valid_email(email) {
                return Err(AppError::BadRequest("Invalid email format".to_string()));
            }

            // Check if email is already taken by another user
            let email_user = self
                .repository
                .find_by_email(email)
                .await
                .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

            if let Some(existing_user) = email_user {
                if existing_user.id != id {
                    return Err(AppError::Conflict(format!(
                        "Email {} is already taken",
                        email
                    )));
                }
            }
        }

        // Validate name if provided
        if let Some(name) = &request.name {
            if name.trim().is_empty() {
                return Err(AppError::BadRequest("Name cannot be empty".to_string()));
            }
        }

        let updated_user = self
            .repository
            .update(id, request.name, request.email)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?
            .ok_or_else(|| AppError::InternalServerError("Failed to update user".to_string()))?;

        Ok(updated_user)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        let deleted = self
            .repository
            .delete(id)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

        if deleted {
            Ok(())
        } else {
            Err(AppError::NotFound(format!("User with id {} not found", id)))
        }
    }

    // Helper method for email validation
    fn is_valid_email(&self, email: &str) -> bool {
        email.contains('@') && email.contains('.')
    }
}
