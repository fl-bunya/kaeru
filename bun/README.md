# Kaeru - Bun実装

Yahoo乗換案内から「新富町(東京都) → 上福岡」のルート情報を取得するBun実装です。

## セットアップ

```sh
bun install
```

## 開発

```sh
bun run dev
```

## ビルド

### 本番用ビルド（minify済み）
```sh
bun run build
```
Bun向けにトランスパイルし、minifyして出力します。

### 開発用ビルド（minifyなし）
```sh
bun run build:dev
```
Bun向けにトランスパイルして出力します（デバッグ用）。

## 実行

```sh
bun run start
```

## ファイルサイズ比較

- **開発用ビルド**: 519KB
- **本番用ビルド（minify）**: 326KB

