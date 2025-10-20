use axum::{http::StatusCode, Json};
use serde::Serialize;

pub async fn root() -> &'static str {
    "[GET] Hello, World"
}

pub async fn get_foo() -> &'static str {
    "[GET] request to /foo"
}

#[derive(Serialize)]
pub struct PostFooResponse {
    pub message: String,
}

pub async fn post_foo() -> (StatusCode, Json<PostFooResponse>) {
    println!("[POST] request to /foo");

    (
        StatusCode::OK,
        Json(PostFooResponse {
            message: "[POST] request to /foo".to_string(),
        }),
    )
}

pub async fn foo_bar() -> &'static str {
    "[GET] request to /foo/bar"
}
