use axum::{
    routing::get,
    Router,
};

use crate::handlers::{health_check, root, UserHandler};
use crate::middleware::logging::logging_middleware;

pub fn create_routes(user_handler: UserHandler) -> Router {
    // Health routes
    let health_routes = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check));

    // User routes
    let user_routes = Router::new()
        .route("/", get(UserHandler::get_all).post(UserHandler::create))
        .route(
            "/{id}",
            get(UserHandler::get_by_id)
                .put(UserHandler::update)
                .delete(UserHandler::delete),
        )
        .with_state(user_handler);

    // Combine all routes
    Router::new()
        .merge(health_routes)
        .nest("/api/users", user_routes)
        .layer(axum::middleware::from_fn(logging_middleware))
}
