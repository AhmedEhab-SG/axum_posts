pub mod auth_handler;
pub mod posts_handler;
pub mod root_handler;
pub mod users_handler;

use crate::{
    AppState,
    handlers::{
        auth_handler::AuthHandler, posts_handler::PostsHandler, root_handler::RootHandler,
        users_handler::UsersHandler,
    },
};

#[derive(Debug, Clone)]
pub struct Handlers {
    pub root_handler: RootHandler,
    pub auth_handler: AuthHandler,
    pub users_handler: UsersHandler,
    pub posts_handler: PostsHandler,
}

impl Handlers {
    pub fn new(app_state: AppState) -> Self {
        Self {
            root_handler: RootHandler::new(),
            auth_handler: AuthHandler::new(),
            users_handler: UsersHandler::new(app_state.clone()),
            posts_handler: PostsHandler::new(app_state),
        }
    }
}
