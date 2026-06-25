use axum::{routing::get, Json, Router, Extension};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;
use tokio::time::Instant;
use tracing::{info, debug}; // Only importing info and debug since they are the ones we need

#[derive(serde::Serialize, serde::Deserialize, Debug, sqlx::FromRow)] 
struct User {
    id: i32,      // maps to COUNT(Symbol)
    name: String, // maps to Symbol
}

#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber to format logs with timestamps up to TRACE level
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE) 
        .init();

    // Database connection
    let connection_string = r"sqlite://C:\Users\KuchipudiAjayKumar\Desktop\RUST\Rust_Inject.db";
    let pool = SqlitePool::connect(connection_string).await.unwrap();

    info!("Successfully connected to Rust_Inject.db!");
    
    // Setup CORS layer
    let cors = CorsLayer::permissive();
    info!("Google Chrome allows requests from localhost:8080 to localhost:3000");

    // Build routes and attach the database pool via Extension layer
    let app = Router::new()
        .route("/api/user", get(get_user))
        .layer(cors)
        .layer(Extension(pool));

    // Start listening on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("🚀 Backend API running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

// API Handler to fetch user details with debug timing built-in
async fn get_user(Extension(pool): Extension<SqlitePool>) -> Json<User> {
    // 1. Start timer
    let start_time = Instant::now();

    let user = sqlx::query_as::<_, User>("SELECT COUNT(Symbol) as id, Symbol as name FROM Nifty_Data WHERE Symbol = 'NIFTY' GROUP BY Symbol")
        .fetch_one(&pool)
        .await
        .unwrap();

    // 2. Calculate time elapsed
    let duration = start_time.elapsed();

    // 3. Log data using exclusively debug! as requested
    debug!("User data queried: {:?}", user);
    debug!("API handler '/api/user' execution time: {:?}", duration);

    // 4. Return response
    Json(user)
}