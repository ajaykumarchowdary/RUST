use axum::{routing::get, Json, Router, Extension};
use serde::Serialize;
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;

#[derive(Serialize, sqlx::FromRow, Clone)]
struct User {
    id: i32,
    name: String,
}


#[tokio::main]
async fn main() {
    // Initialize an in-memory SQLite database pool
    // 1. Use a raw string (r"...") so Windows paths work perfectly
    let connection_string = r"sqlite://C:\Users\KuchipudiAjayKumar\Desktop\RUST\Rust_Inject.db";
    
    // 2. Connect to your physical DB Browser file
    let pool = SqlitePool::connect(connection_string).await.unwrap();

    println!("Successfully connected to Rust_Inject.db!");
    
    // Create a mock table and insert data
    //sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);")
        //.execute(&pool).await.unwrap();
    //sqlx::query("INSERT INTO users (id, name) VALUES (1, 'Ajay');")
        //.execute(&pool).await.unwrap();

    // Setup CORS layer so Google Chrome allows requests from localhost:8080 to localhost:3000
    let cors = CorsLayer::permissive();

    // Build routes and attach the database pool via Extension layer
    let app = Router::new()
        .route("/api/user", get(get_user))
        .layer(cors)
        .layer(Extension(pool));

    // Start listening on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Backend API running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

// API Handler to fetch user details
async fn get_user(Extension(pool): Extension<SqlitePool>) -> Json<User> {
    let user = sqlx::query_as::<_, User>("SELECT COUNT(Symbol) as id, Symbol as name FROM Nifty_Data WHERE Symbol = 'NIFTY' GROUP BY Symbol")
        .fetch_one(&pool)
        .await
        .unwrap();

    Json(user)
}