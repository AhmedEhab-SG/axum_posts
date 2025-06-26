use std::env::var;

#[derive(Debug, Clone)]
pub struct Env {
    pub database_url: String,
    pub jwt_access_token_secert: String,
    pub jwt_access_token_expires: i64,
    pub jwt_refresh_token_secert: String,
    pub jwt_refresh_token_expires: i64,
    pub port: u16,
    pub ip: String,
    pub local: String,
}

impl Env {
    pub fn init() -> Self {
        let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");

        let jwt_access_token_secert =
            var("JWT_ACCESS_TOKEN_SECRET").expect("JWT_ACCESS_TOKEN_SECRET must be set");

        let jwt_access_token_expires = var("JWT_ACCESS_TOKEN_EXPIRES")
            .expect("JWT_ACCESS_TOKEN_EXPIRES must be set")
            .parse()
            .expect("JWT_ACCESS_TOKEN_EXPIRES must be a valid u64");

        let jwt_refresh_token_secert =
            var("JWT_REFRESH_TOKEN_SECRET").expect("JWT_REFRESH_TOKEN_SECRET must be set");

        let jwt_refresh_token_expires = var("JWT_REFRESH_TOKEN_EXPIRES")
            .expect("JWT_REFRESH_TOKEN_EXPIRES must be set")
            .parse()
            .expect("JWT_REFRESH_TOKEN_EXPIRES must be a valid u64");

        let port: u16 = var("PORT")
            .unwrap_or_else(|_| "7878".to_string())
            .parse()
            .expect("PORT must be a valid u16");

        let ip = var("IP").unwrap_or_else(|_| "127.0.0.1".to_string());

        let local = var("LOCAL").unwrap_or_else(|_| "localhost".to_string());

        println!("Configuration loaded!");

        Self {
            database_url,
            jwt_access_token_secert,
            jwt_access_token_expires,
            jwt_refresh_token_secert,
            jwt_refresh_token_expires,
            port,
            ip,
            local,
        }
    }
}
