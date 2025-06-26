use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{AppState, handlers::Handlers, services::Services};

pub struct ApiRouter {
    pub router: Router,
}

impl ApiRouter {
    pub fn new(app_state: AppState) -> Self {
        let handlers = Handlers::new(app_state.clone());
        let services = Services::new(app_state);
        let router = Router::new()
            .merge(handlers.root_handler.router())
            .nest(
                "/api",
                Router::new()
                    .merge(handlers.root_handler.router())
                    .nest("/auth", handlers.auth_handler.router(services.auth_service))
                    .nest(
                        "/users",
                        handlers.users_handler.router(services.users_service),
                    ),
            )
            .layer(TraceLayer::new_for_http());
        Self { router }
    }

    pub fn with_cors(self, cors: CorsLayer) -> Self {
        let router = self.router.layer(cors);
        Self { router }
    }

    // pub fn with_extension<T: Send + Sync + 'static>(self, extension: T) -> Self {
    //     let router = self.router.layer(Extension(Arc::new(extension)));
    //     Self { router }
    // }
}
