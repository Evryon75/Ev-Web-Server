use axum::http::{header, Method, Request, Response};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use shuttle_runtime::tracing_subscriber;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

//todo: add Nginx ssl reverse proxy when its done

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/test", get(test))
        .layer(cors);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 0448));
    println!("listening on http://{:#?}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn test() -> Json<Test> {
    Json::from(Test { test_value: 1 })
}
async fn root() -> &'static str {
    "Server operational"
}

#[derive(Serialize)]
struct Test {
    test_value: i8,
}
