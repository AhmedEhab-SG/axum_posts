pub mod db;
pub mod dtos;
pub mod error;
pub mod handlers;
pub mod middlewares;
pub mod router;
pub mod services;
pub mod utils;

use crate::{db::DBClient, utils::config::Env};

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Env,
    pub db_client: DBClient,
}
