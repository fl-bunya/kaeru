# Kaeru - Rust版

Yahoo乗換案内からルート情報を取得するRustアプリケーションです。

## 機能

- 現在時刻を基にYahoo乗換案内のURLを生成
- 新富町(東京都)から上福岡までのルート情報を取得
- 発車時刻、到着時刻、ルート詳細を表示

## セットアップ

1. Rustがインストールされていることを確認:
```bash
rustc --version
cargo --version
```

2. 依存関係をインストール:
```bash
cargo build
```

## 実行方法

```bash
cargo run
```

## 必要な依存関係

- `reqwest`: HTTPクライアント
- `scraper`: HTMLパース用ライブラリ
- `chrono`: 日時処理
- `anyhow`: エラーハンドリング
- `urlencoding`: URLエンコーディング

## 出力例

```
🚇 Yahoo乗換案内からルート情報を取得中...
Fetching transit routes from: https://transit.yahoo.co.jp/search/result?...

📋 取得したルート情報 (3件):

=== ルート 1 ===
発車時刻: 12:30
到着時刻: 13:45
ルート: 新富町 → 渋谷 → 上福岡

=== ルート 2 ===
発車時刻: 12:45
到着時刻: 14:00
ルート: 新富町 → 新宿 → 上福岡

=== ルート 3 ===
発車時刻: 13:00
到着時刻: 14:15
ルート: 新富町 → 池袋 → 上福岡
```

## ビルド

リリースビルドを作成する場合:
```bash
cargo build --release
```

実行ファイルは `target/release/kaeru` に生成されます。 