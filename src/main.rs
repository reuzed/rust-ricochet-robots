use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use lambda_http::{run, tracing, Error};
mod http_handler;
use rust_ricochet_robots::routes::{echo, health, TestRequest};

async fn health_route() -> impl IntoResponse {
    Json(health())
}

async fn echo_route(Json(req): Json<TestRequest>) -> impl IntoResponse {
    Json(echo(req))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let app = Router::new()
        .route("/", get(health_route))
        .route("/echo", post(echo_route));

    run(app).await
}
