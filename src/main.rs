use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::BoxError;
use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::net::SocketAddr;
use std::time::Duration;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber;

#[derive(Deserialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}

async fn page_a_controller() -> impl IntoResponse {
    "Welcome to the A page!"
}

async fn page_b_controller() -> impl IntoResponse {
    "Welcome to the B page!"
}

async fn page_c_controller(Query(range): Query<RangeParameters>) -> Html<String> {
    let random_number = thread_rng().gen_range(range.start..range.end);
    Html(format!("<h1>Random Number: {}</h1>", random_number))
}

async fn page_foo_controller() -> impl IntoResponse {
    "Welcome to the a/foo page!"
}

async fn page_root_controller() -> impl IntoResponse {
    "Welcome to the root page!"
}

fn create_foo_routes() -> Router {
    Router::new().route("/", get(page_foo_controller))
}

fn create_a_routes() -> Router {
    Router::new()
        .route("/", get(page_a_controller))
        .nest("/foo", create_foo_routes())
}

fn create_b_routes() -> Router {
    Router::new().route("/", get(page_b_controller))
}

fn create_c_routes() -> Router {
    Router::new().route("/", get(page_c_controller))
}

fn create_root_routes() -> Router {
    Router::new().route("/", get(page_root_controller))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let app = Router::new()
        .nest("/a", create_a_routes())
        .nest("/b", create_b_routes())
        .nest("/c", create_c_routes())
        .nest("/", create_root_routes())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(4, Duration::from_secs(5))),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
