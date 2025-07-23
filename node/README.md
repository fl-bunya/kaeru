# Kaeru Node.js Version

Yahoo乗換案内からルート情報を取得するNode.js版のスクレイピングツールです。

## 機能

- Yahoo乗換案内からリアルタイムでルート情報を取得
- Cheerioを使用したHTML解析
- 発車時刻、到着時刻、ルート詳細の抽出
- 現在時刻に基づくクエリ対応

## セットアップ

### 前提条件
- Node.js (v16以上推奨)
- npm

### インストール

1. 依存関係をインストール:
```bash
npm install
```

2. TypeScriptコードをビルド:
```bash
npm run build
```

## 使用方法

### 開発モード (ts-node使用):
```bash
npm run dev
```

### 本番モード (コンパイル済みJavaScript):
```bash
npm start
```

## プロジェクト構造

```
node/
├── src/
│   └── index.ts          # メインTypeScriptソースファイル
├── dist/                 # コンパイル済みJavaScript出力 (ビルド後に生成)
├── package.json          # プロジェクト依存関係とスクリプト
├── tsconfig.json         # TypeScript設定
├── .gitignore           # Git除外設定
└── README.md            # このファイル
```

## スクリプト

- `npm install` - 依存関係をインストール
- `npm run build` - TypeScriptコードをコンパイル
- `npm run dev` - 開発モードで実行 (ts-node使用)
- `npm start` - 本番モードで実行 (コンパイル済みJS使用)

## 依存関係

### 本番依存関係
- `cheerio` - HTML解析と操作

### 開発依存関係
- `typescript` - TypeScriptコンパイラ
- `ts-node` - 開発用TypeScript実行環境
- `@types/node` - Node.js型定義

## 出力例

```
🚇 Yahoo乗換案内からルート情報を取得中...
Fetching transit routes from: https://transit.yahoo.co.jp/search/result?...

📋 取得したルート情報 (3件):

=== ルート 1 ===
発車時刻: 14:30
到着時刻: 15:45
ルート: 新富町駅 → 渋谷駅 → 上福岡駅

=== ルート 2 ===
発車時刻: 14:35
到着時刻: 15:50
ルート: 新富町駅 → 新宿駅 → 上福岡駅
```

## 注意事項

- Yahoo乗換案内のHTML構造が変更された場合、セレクタの調整が必要になる可能性があります
- 過度なリクエストは避けてください
- 取得したデータの使用は、Yahoo乗換案内の利用規約に従ってください 