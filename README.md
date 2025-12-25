# Management Bookmarks

タグ管理＆ステルス機能付きブックマーク管理アプリ
Rust (Axum) + SQLite + Next.js で構築。

## 技術スタック

- **Frontend:** Next.js (App Router), Tailwind CSS
- **Backend:** Rust (Axum, Tokio), SQLx
- **Database:** SQLite

## 開発環境のセットアップ (Setup)

### 1. Prerequisites (事前準備)

以下のツールがインストールされている必要があります。

- **Node.js** (v18 以上推奨) & **pnpm**
- **Rust** & **Cargo** (最新版)

### 2. Backend Setup (Rust API)

バックエンドの依存関係をインストールし、データベースをセットアップします。

```bash
# 1. SQLx CLIツールのインストール（DB操作用）
cargo install sqlx-cli --no-default-features --features rustls,sqlite

# 2. APIディレクトリへ移動
cd apps/api

# 3. 環境変数ファイルの作成
# (Windows PowerShellの場合、エンコーディングに注意。エラーが出る場合は手動でUTF-8で作成してください)
echo "DATABASE_URL=sqlite:stealth.db" > .env

# 4. データベース作成 & マイグレーション適用
sqlx database create
sqlx migrate run
```

### 3. Frontend Setup (Web Client)

フロントエンドの依存関係をインストールします。

```bash
# ルートディレクトリで実行
pnpm install
```

## 起動方法 (How to Run)

ターミナルを 2 つ開いて実行します。

### Terminal 1: バックエンド (Rust)

```bash
cd apps/api
cargo run
# Server listening on http://127.0.0.1:3001
```

### Terminal 2: フロントエンド (Next.js)

```bash
cd apps/web
pnpm dev
# App running on http://localhost:3000
```

もしくは

```bash
pnpm --filter web dev
```

VScode で Rust を編集する際は `.vscode/settings.json` の設定により `apps/api/Cargo.toml` がルートとして認識されます。

## Stealth Features (ステルス機能について)

このアプリには、背後から画面を覗かれた際のリスク管理機能（開発中）が搭載されています。

- Blur Mode: 特定のタグ（`#Private`など）がついた項目のタイトルを自動でぼかします。
- Fake Mode: 瞬時に「業務用の Excel ファイル」や「要件定義書 PDF」に見た目を偽装します。
