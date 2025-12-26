use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use dotenvy::dotenv;
use serde::Deserialize;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::env;
use std::net::SocketAddr;

// フロントエンドから送られてくるデータの形
#[derive(Deserialize)]
struct CreateBookmark {
    url: String,
    title: String,
}

// POST /bookmarks を受け取る関数
async fn create_bookmark(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateBookmark>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    // UUIDを生成
    let id = uuid::Uuid::new_v4().to_string();

    // SQLを実行して保存
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

    // 結果に応じてレスポンスを返す
    match result {
        Ok(_) => Ok((
            StatusCode::CREATED,
            "Bookmark created successfully".to_string(),
        )),
        Err(e) => {
            tracing::error!("Failed to create bookmark: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create bookmark: {}", e),
            ))
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("✅ Database connection successful!");

    // ルーティング設定
    let app = Router::new()
        .route("/", get(|| async { "Hello, Stealth Bookmarks API!" }))
        .route("/bookmarks", post(create_bookmark))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("🚀 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
