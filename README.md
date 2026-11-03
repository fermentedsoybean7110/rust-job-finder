# Rust Job Finder

## Project Overview

Vibe Reach の公開案件取得 API を利用して、公開中の業務委託案件を検索・閲覧する Web アプリケーション（Rust 製、Axum + Askama による SSR）。

案件一覧まで実装済みです。

## Features

| 機能 | 状態 |
| --- | --- |
| 案件一覧（20件、会社名・報酬・概要・詳細/掲載元リンク） | 実装済み |
| キーワード検索 | 未実装（予定） |
| AI 案件フィルター | 未実装（予定） |
| 案件詳細 | 未実装（予定） |
| ページネーション | 未実装（予定） |

案件詳細ページ（`/jobs/{id}`）へのリンクは表示しますが、詳細画面自体は未実装です。

## Tech Stack

- Rust / Axum / Tokio
- reqwest（HTTP クライアント）
- Serde（JSON）
- Askama（HTML テンプレート）
- GitHub Actions（CI）

## Requirements

- Rust 1.99 以降（Cargo 含む）

## Installation

```sh
git clone https://github.com/fermentedsoybean7110/rust-job-finder.git
cd rust-job-finder
cargo build
```

## Usage

```sh
cargo run
```

`http://127.0.0.1:3000/` にアクセスします。インターネット接続が必要です。

| 環境変数 | 既定値 | 説明 |
| --- | --- | --- |
| `BIND_ADDR` | `127.0.0.1:3000` | 待ち受けアドレス |
| `VIBE_REACH_API_BASE` | `https://vibe-reach.net/api/v1` | API のベース URL |

## Project Structure

```
src/
├── main.rs        # エントリポイント
├── lib.rs         # Router 構築
├── config.rs      # 設定
├── api/           # Vibe Reach API クライアント
├── models/        # API レスポンスモデル
├── routes/        # ハンドラ
└── templates/     # Askama テンプレート
static/            # CSS
```

## API Information

利用予定: Vibe Reach Jobs API（認証不要）

- 一覧: `GET https://vibe-reach.net/api/v1/jobs?limit=20`（`category=ai`、`cursor` 指定可）
- 詳細: `GET https://vibe-reach.net/api/v1/jobs/{JOB_ID}`

一覧は `limit=20` で取得しています。`category`・`cursor`・詳細 API は、クライアント側は対応済みですが画面からは未使用です。

## Testing

```sh
cargo fmt --check
cargo clippy --all-targets
cargo test
```

外部 API に依存しないテストのみを CI で実行します。HTTP 通信のテストは、ローカルに立てたモックサーバーを使います。
