use axum::{body::Body, extract::Request, http::HeaderValue, middleware::Next, response::Response};

static APP_VERSION: HeaderValue = HeaderValue::from_static(env!("VERGEN_GIT_SHA"));

pub async fn add_version_header(req: Request<Body>, next: Next) -> Response {
    let mut res = next.run(req).await;
    res.headers_mut().insert(
        "x-app-version",
        APP_VERSION.clone()
    );
    res
}
