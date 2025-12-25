use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 1. .envファイルを読み込む
    dotenv().ok();

    // 2. ログ設定（エラーが見やすくなる）
    tracing_subscriber::fmt::init();

    // 3. データベースに接続する
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let pool = SqlitePoolOptions::new()
        .max_connections(5) // 同時接続数
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("✅ Database connection successful!");

    // 4. ルーティング設定（"/" にアクセスしたら文字を返すだけ）
    let app = Router::new()
        .route("/", get(|| async { "Hello, Stealth Bookmarks API!" }))
        .with_state(pool); // DB接続をアプリ全体で共有

    // 5. サーバー起動設定 (ポート3001番で待機)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("🚀 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
