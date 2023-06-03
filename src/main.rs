mod handlers;
pub mod data;

use axum::http::Method;
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use shuttle_runtime::tracing_subscriber;
use std::net::SocketAddr;

use tower_http::cors::{Any, CorsLayer};
use crate::data::REPOS;

//todo: add Nginx ssl reverse proxy when its done

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    // build our application with a route
    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/query", get(handlers::query))

        .layer(CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 0448));
    println!("listening on http://{:#?}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
