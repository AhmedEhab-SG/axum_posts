use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::error::ErrorMessage;

#[derive(Debug, Clone)]
pub struct PasswordArgon;

impl PasswordArgon {
    const MAX_PASSWORD_LENGTH: usize = 128;

    pub fn hash(password: impl Into<String>) -> Result<String, ErrorMessage> {
        let password = password.into();

        if password.is_empty() {
            return Err(ErrorMessage::EmptyPassword);
        }

        if password.len() > Self::MAX_PASSWORD_LENGTH {
            return Err(ErrorMessage::ExceededMaxPaasswordLength(
                Self::MAX_PASSWORD_LENGTH,
            ));
        }

        let salt = SaltString::generate(&mut OsRng);

        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| ErrorMessage::HashingError)?
            .to_string();

        Ok(hashed_password)
    }

    pub fn compare(password: &str, hash_password: &str) -> Result<bool, ErrorMessage> {
        if password.is_empty() || hash_password.is_empty() {
            return Err(ErrorMessage::EmptyPassword);
        }

        if password.len() > Self::MAX_PASSWORD_LENGTH {
            return Err(ErrorMessage::ExceededMaxPaasswordLength(
                Self::MAX_PASSWORD_LENGTH,
            ));
        }

        let parsed_hash =
            PasswordHash::new(hash_password).map_err(|_| ErrorMessage::InvalidHashForamt)?;

        let password_matched = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true);

        Ok(password_matched)
    }
}
