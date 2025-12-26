use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::env;
use std::net::SocketAddr;

#[derive(Deserialize)]
struct CreateBookmark {
    url: String,
    title: String,
}

#[derive(Serialize)]
struct Bookmark {
    id: String,
    url: String,
    title: String,
    fake_title: Option<String>,
    // 日付は一旦後回し
}

async fn create_bookmark(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateBookmark>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let id = uuid::Uuid::new_v4().to_string();
    let result = sqlx::query!(
        r#"
        INSERT INTO bookmarks (id, url, title)
        VALUES ($1, $2, $3)
        "#,
        id,
        payload.url,
        payload.title
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok((
            StatusCode::CREATED,
            "Bookmark created successfully".to_string(),
        )),
        Err(e) => {
            tracing::error!("Failed to create bookmark: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

async fn get_bookmarks(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Bookmark>>, (StatusCode, String)> {
    let bookmarks = sqlx::query_as!(
        Bookmark,
        r#"
        SELECT id, url, title, fake_title
        FROM bookmarks
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch bookmarks: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    Ok(Json(bookmarks))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("✅ Database connection successful!");

    let app = Router::new()
        .route("/", get(|| async { "Hello, Stealth Bookmarks API!" }))
        .route("/bookmarks", post(create_bookmark))
        .route("/bookmarks", get(get_bookmarks))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("🚀 Server listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
