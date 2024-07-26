use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::net::SocketAddr;

// `Deserialize` need be implemented to use with `Query` extractor.
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
    // Generate a random number in range parsed from query.
    let random_number = thread_rng().gen_range(range.start..range.end);

    // Send response in html format.
    Html(format!("<h1>Random Number: {}</h1>", random_number))
}

fn create_a_routes() -> Router {
    Router::new().route("/a", get(page_a_controller))
}

fn create_b_routes() -> Router {
    Router::new().route("/b", get(page_b_controller))
}

fn create_c_routes() -> Router {
    Router::new().route("/c", get(page_c_controller))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(create_a_routes())
        .merge(create_b_routes())
        .merge(create_c_routes());

    //Create socket and bind
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
