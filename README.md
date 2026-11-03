# Rust Job Finder

## Project Overview

Vibe Reach の公開案件取得 API を利用して、公開中の業務委託案件を検索・閲覧する Web アプリケーション（Rust 製、Axum + Askama による SSR）。

現在は初期構築段階です。

## Features

| 機能 | 状態 |
| --- | --- |
| 案件一覧 | 未実装（予定） |
| キーワード検索 | 未実装（予定） |
| AI 案件フィルター | 未実装（予定） |
| 案件詳細 | 未実装（予定） |
| ページネーション | 未実装（予定） |

実装済み: Axum サーバー起動、Askama テンプレート描画、静的ファイル配信。

## Tech Stack

- Rust / Axum / Tokio
- reqwest（HTTP クライアント。API 連携 PR で使用）
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

`http://127.0.0.1:3000/` にアクセスします。待ち受けアドレスは環境変数 `BIND_ADDR` で変更できます。

## Project Structure

```
src/
├── main.rs        # エントリポイント
├── lib.rs         # Router 構築
├── config.rs      # 設定
├── api/           # 外部 API クライアント（今後実装）
├── models/        # データモデル（今後実装）
├── routes/        # ハンドラ
└── templates/     # Askama テンプレート
static/            # CSS
```

## API Information

利用予定: Vibe Reach Jobs API（認証不要）

- 一覧: `GET https://vibe-reach.net/api/v1/jobs?limit=20`（`category=ai`、`cursor` 指定可）
- 詳細: `GET https://vibe-reach.net/api/v1/jobs/{JOB_ID}`

現時点では未接続です。

## Testing

```sh
cargo fmt --check
cargo clippy --all-targets
cargo test
```

外部 API に依存しないテストのみを CI で実行します。
