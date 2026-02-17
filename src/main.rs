use axum::{
    routing::{get, post},
    middleware as axum_middleware,
    Router
};
use arc_swap::ArcSwap;
use dotenvy::dotenv;
use std::{env, sync::Arc};
use tokio::sync::mpsc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod handlers;
mod bingo;
mod middleware;

#[derive(Clone)]
pub struct AppState {
    pub api_key: String,
    pub game: Arc<ArcSwap<bingo::game::Game>>,
    pub win_tx: mpsc::UnboundedSender<bingo::game::WinSubmission>
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap()
        )
        .without_time()
        .compact()
        .init();

    let game_state = Arc::new(ArcSwap::new(Arc::new(bingo::game::new())));
    let (win_tx, win_rx) = mpsc::unbounded_channel();
    tokio::spawn(bingo::game::run(win_rx, Arc::clone(&game_state)));

    let app_state = AppState {
        api_key: env::var("API_KEY")
            .expect("API_KEY must be set"),
        game: game_state,
        win_tx: win_tx
    };

    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/connect", post(handlers::bingo::connnect))
        .route("/disconnect", post(handlers::bingo::disconnect))
        .route("/submit_win", post(handlers::bingo::submit_win))
        .route("/poll", get(handlers::bingo::poll))
        .with_state(app_state.clone())
        .layer(
            ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(axum_middleware::from_fn_with_state(
                    app_state.clone(),
                    middleware::auth::auth_middleware
            ))
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("Server running @ http://{}", "0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}
