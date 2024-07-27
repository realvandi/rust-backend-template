use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use rand::{thread_rng, Rng};
use serde::Deserialize;

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

pub fn create_a_routes() -> Router {
    Router::new()
        .route("/", get(page_a_controller))
        .nest("/foo", create_foo_routes())
}

pub fn create_b_routes() -> Router {
    Router::new().route("/", get(page_b_controller))
}

pub fn create_c_routes() -> Router {
    Router::new().route("/", get(page_c_controller))
}

pub fn create_root_routes() -> Router {
    Router::new().route("/", get(page_root_controller))
}
