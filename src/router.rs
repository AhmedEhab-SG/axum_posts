use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    AppState,
    handlers::Handlers,
    services::Services,
    utils::config::Routes::{Auth, Base, Posts, Users},
};

pub struct ApiRouter {
    pub router: Router,
}

impl ApiRouter {
    pub fn new(app_state: AppState) -> Self {
        let handlers = Handlers::new(app_state.clone());
        let services = Services::new(app_state);
        let router = Router::new()
            .nest(
                &Base.to_string(),
                Router::new()
                    .merge(handlers.root_handler.router())
                    .nest(
                        &Auth.to_string(),
                        handlers.auth_handler.router(services.auth_service),
                    )
                    .nest(
                        &Users.to_string(),
                        handlers.users_handler.router(services.users_service),
                    )
                    .nest(
                        &Posts.to_string(),
                        handlers.posts_handler.router(services.posts_service),
                    ),
            )
            .layer(TraceLayer::new_for_http());
        Self { router }
    }

    pub fn with_cors(self, cors: CorsLayer) -> Self {
        let router = self.router.layer(cors);
        Self { router }
    }
}
