# Kaeru - Node.js実装

Yahoo乗換案内から「新富町(東京都) → 上福岡」のルート情報を取得するNode.js実装です。

## セットアップ

```sh
npm install
```

## 開発

```sh
npm run dev
```

## ビルド

### 本番用ビルド（bundle + minify）
```sh
npm run build
```
Viteを使用してTypeScriptをJavaScriptにトランスパイルし、bundleとminifyを行います。

### 開発用ビルド（bundleのみ）
```sh
npm run build:dev
```
Viteを使用してTypeScriptをJavaScriptにトランスパイルし、bundleを行います（minifyなし）。

## 実行

```sh
npm run start
```

## ファイルサイズ比較

- **TypeScriptコンパイル版**: 5.9KB
- **Viteビルド版（bundle + minify）**: 2.35KB

## 注意事項

- Viteを使用することで、より効率的なbundleとminifyが可能になります
- 外部依存関係（cheerio）は除外され、Node.jsのrequireで読み込まれます
- ビルド済みファイルはNode.js環境での実行が必要です
- sourcemapが生成されるため、デバッグが容易です 