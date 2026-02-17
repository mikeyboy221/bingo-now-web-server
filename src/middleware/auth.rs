use axum::{
    http::{Request, StatusCode}, 
    middleware::Next,
    response::Response, 
    body::Body,
    extract::State,
};
use super::super::AppState;

pub async fn auth_middleware(
    State(app_state): State<AppState>,
    request: Request<Body>,
    next: Next

) -> Result<Response, StatusCode> {
    let api_key = request
        .headers()
        .get("x-api-key")
        .and_then(|value| value.to_str().ok());

    match api_key{
        Some(key) if key == app_state.api_key => Ok(next.run(request).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
