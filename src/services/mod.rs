pub mod auth_service;
pub mod posts_service;
pub mod users_service;

use crate::{
    AppState,
    services::{
        auth_service::AuthService, posts_service::PostsService, users_service::UsersService,
    },
};

#[derive(Debug, Clone)]
pub struct Services {
    pub auth_service: AuthService,
    pub users_service: UsersService,
    pub posts_service: PostsService,
}

impl Services {
    pub fn new(app_state: AppState) -> Self {
        Self {
            auth_service: AuthService::new(app_state.clone()),
            users_service: UsersService::new(app_state.db_client.clone()),
            posts_service: PostsService::new(app_state.db_client),
        }
    }
}
