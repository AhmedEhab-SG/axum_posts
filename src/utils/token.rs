use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode,
    errors::{Error as JwtError, ErrorKind as JwtErrorKind},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

impl TokenClaims {
    pub fn encode(id: &str, secret: &[u8], expires_in_sec: i64) -> Result<String, JwtError> {
        if id.is_empty() || secret.is_empty() {
            return Err(JwtErrorKind::InvalidToken.into());
        }

        let now = Utc::now();
        let iat = now.timestamp();
        let exp = (now + Duration::minutes(expires_in_sec)).timestamp();
        let claims = Self {
            sub: id.to_string(),
            exp,
            iat,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret),
        )
    }

    pub fn decode<T: Into<String>>(token: T, secert: &[u8]) -> Result<Self, JwtError> {
        match decode::<Self>(
            &token.into(),
            &DecodingKey::from_secret(secert),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(token_data) => Ok(token_data.claims),
            Err(e) => Err(e),
        }
    }

    pub fn validate(self) -> Result<Self, JwtError> {
        if self.sub.is_empty() {
            return Err(JwtErrorKind::InvalidToken.into());
        }

        let now = Utc::now().timestamp();
        if self.exp < now {
            return Err(JwtErrorKind::ExpiredSignature.into());
        }

        Ok(self)
    }
}
