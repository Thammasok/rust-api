mod config;
mod errors;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;

use config::Config;
use handlers::UserHandler;
use repositories::UserRepository;
use routes::create_routes;
use services::UserService;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Load configuration
    let config = Config::from_env();

    println!("🚀 Starting Rust API Server...");
    println!("📝 Version: {}", env!("CARGO_PKG_VERSION"));
    println!("🌐 Server address: {}", config.server_address());

    // Create database connection pool
    println!("🔌 Connecting to database...");
    let pool = config
        .create_pool()
        .await
        .expect("Failed to create database pool");

    println!("✅ Database connection established");

    // Run migrations
    println!("🔄 Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    println!("✅ Migrations completed");

    // Initialize layers
    let repository = UserRepository::new(pool);
    let service = UserService::new(repository);
    let handler = UserHandler::new(service);

    // Create routes
    let app = create_routes(handler);

    // Start server
    let listener = tokio::net::TcpListener::bind(config.server_address())
        .await
        .expect("Failed to bind to address");

    println!("✅ Server running on http://{}", config.server_address());
    println!("\n📚 Available endpoints:");
    println!("  GET    /              - Root");
    println!("  GET    /health        - Health check");
    println!("  GET    /api/users     - Get all users");
    println!("  POST   /api/users     - Create user");
    println!("  GET    /api/users/:id - Get user by ID");
    println!("  PUT    /api/users/:id - Update user");
    println!("  DELETE /api/users/:id - Delete user");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
