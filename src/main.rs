use std::error::Error;

use axum::{
    http::{
        Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
    serve,
};
use dotenv::dotenv;
use tokio::{main as async_main, net::TcpListener};
use tower_http::cors::{Any, CorsLayer};

use axum_posts::{
    AppState,
    db::DBClient,
    router::ApiRouter,
    utils::{config::Env, print_running},
};

#[async_main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    tracing_subscriber::fmt().init();

    let env = Env::init();
    let db_client = DBClient::new(&env.database_url).await?;

    let app_state = AppState { env, db_client };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        //.allow_credentials(true);
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let listener =
        TcpListener::bind(format!("{}:{}", app_state.env.ip, app_state.env.port)).await?;
    let app = ApiRouter::new(app_state.clone()).with_cors(cors);

    print_running(&app_state.env.local, app_state.env.port, &app_state.env.ip);

    serve(listener, app.router).await.map_err(|e| {
        eprintln!("Failed to start the server: {e}");
        e
    })?;

    Ok(())
}
