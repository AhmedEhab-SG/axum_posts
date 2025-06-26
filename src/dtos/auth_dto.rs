use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Default, Debug, Clone, Deserialize, Serialize, Validate)]
pub struct LoginDto {
    #[validate(email(message = "Email is invalid"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, Validate)]
pub struct RegisterDto {
    #[validate(length(min = 5, message = "name must be at least 5 characters long"))]
    pub name: String,

    #[validate(email(message = "Email is invalid"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,

    #[validate(
        length(
            min = 6,
            message = "Confirm password must be at least 6 characters long"
        ),
        must_match(other = "password", message = "Passwords do not match")
    )]
    pub confirm_password: String,
}
