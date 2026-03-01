use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use rollback2025::app_state::AppState;
use rollback2025::handlers::user_handler::{create_user, get_user};
use rollback2025::services::user_service::UserService;
use rollback2025::repositories::user_repository::UserRepository;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>  {
    tracing_subscriber::fmt().init();

    dotenv().ok();

    let db_url = env::var("AIVEN_POSTGRES_URL").expect("DATABASE_URL is not set in .env file");
    println!("DATABASE_URL: {}", db_url);

    let db_pool = PgPoolOptions::new().max_connections(5).connect(&db_url).await?;
    println!("DB connection established");
    
    // just for testing connection to the database, remove later
    // let user_name: Option<String> = sqlx::query_scalar("SELECT name FROM users Where id = $1")
    // .bind(2) 
    // .fetch_optional(&db_pool)
    // .await?;
    // match user_name {
    //     Some(name) => println!("User name: {}", name),
    //     None => println!("User not found"),
    // }

    // if let Some(name) = user_name {
    //     println!("User name: {}", name);
    // } else {
    //     println!("User not found");
    // }
    

    let user_repository = Arc::new(UserRepository::new(db_pool));
    let user_service = Arc::new(UserService::new(user_repository));

    let app_state = AppState {
        user_service: user_service.clone(),
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, Axum World!" }))
        .route("/users", post(create_user))
        .route("/users/{id}", get(get_user))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8090")
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server running on port 8090");   
    axum::serve(listener, app).await.expect("Server failed");

    return Ok(());
}