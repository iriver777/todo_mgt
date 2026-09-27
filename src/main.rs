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

    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    tracing::info!("Running in {} mode", app_env);

    if app_env != "prod" {
        match dotenv() {
            Ok(path) => tracing::info!("Loaded environment variables from .env file: {:?}", path),
            Err(_) => tracing::info!("No .env file found, using system environment variables"),
        }
    } else {
        tracing::info!("prod mode: skipping .env file, using system environment variables only");
    }

    let db_url = env::var("AIVEN_POSTGRES_URL")
        .expect("AIVEN_POSTGRES_URL environment variable is not set. \
                 Either set it as a system environment variable or create a .env file (see .env.example).");
    tracing::debug!("Database URL loaded (credentials hidden for security)");

    let db_pool = PgPoolOptions::new().max_connections(5).connect(&db_url).await?;
    tracing::info!("Database connection established successfully");
    
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