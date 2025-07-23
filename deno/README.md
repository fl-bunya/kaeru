# Kaeru - Deno実装

Yahoo乗換案内から「新富町(東京都) → 上福岡」のルート情報を取得するDeno実装です。

## セットアップ

```sh
# 依存関係は自動的にダウンロードされます
```

## 開発

```sh
deno run --allow-net src/index.ts
```

## ビルド

### コンパイル版バイナリ
```sh
deno compile --allow-net --output dist/kaeru src/index.ts
```
Denoランタイムを含む単一の実行可能バイナリを作成します。

### バンドル版（外部依存関係）
```sh
deno bundle --minify --output dist/index.js --external cheerio src/index.ts
```
依存関係を除外してbundleとminifyを行います。

### バンドル版（完全版）
```sh
deno bundle --minify --output dist/index-full.js src/index-esm-cheerio.ts
```
cheerioを含む完全なbundleとminifyを行います。

## 実行

### TypeScriptファイルの実行
```sh
deno run --allow-net src/index.ts
```

### コンパイル版バイナリの実行
```sh
./dist/kaeru
```

### 外部依存関係版JavaScriptの実行
```sh
deno run --allow-net dist/index.js
```

### 完全版JavaScriptの実行
```sh
deno run --allow-net dist/index-full.js
```

## ファイルサイズ比較

- **TypeScriptソース**: 4.5KB
- **バンドル版（外部依存関係）**: 2.2KB
- **バンドル版（完全版）**: 413KB
- **コンパイル版バイナリ**: 76MB

## 注意事項

- Denoはセキュリティ重視の設計のため、ネットワークアクセスに明示的な許可が必要です
- **外部依存関係版**: cheerioを外部依存関係として除外しているため、実行時にcheerioが必要です
- **完全版**: cheerioを含むため、単体で動作しますが、ファイルサイズが大きくなります
- コンパイル版バイナリはDenoランタイムを含むため、ファイルサイズが大きくなります
- すべてのバージョンで同じ機能を提供し、結果は一致します 