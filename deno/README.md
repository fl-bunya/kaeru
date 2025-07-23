# Kaeru Deno Version

Yahoo乗換案内からルート情報を取得するDeno版のスクレイピングツールです。

## 機能

- Yahoo乗換案内からリアルタイムでルート情報を取得
- Cheerioを使用したHTML解析
- 発車時刻、到着時刻、ルート詳細の抽出
- 現在時刻に基づくクエリ対応

## セットアップ

### 前提条件
- Deno (v1.40以上推奨)

### インストール
Denoは自動的に依存関係を管理するため、特別なインストール手順は不要です。

## 使用方法

### 開発モード:
```bash
deno run --allow-net src/index.ts
```

### ビルド済みバイナリ:
```bash
deno compile --allow-net --output dist/kaeru src/index.ts
./dist/kaeru
```

## プロジェクト構造

```
deno/
├── src/
│   └── index.ts          # メインTypeScriptソースファイル
├── dist/                 # コンパイル済みバイナリ出力 (ビルド後に生成)
├── deno.json            # Deno設定ファイル
└── README.md            # このファイル
```

## スクリプト

- `deno run --allow-net src/index.ts` - 開発モードで実行
- `deno compile --allow-net --output dist/kaeru src/index.ts` - バイナリにコンパイル
- `./dist/kaeru` - コンパイル済みバイナリを実行

## 依存関係

- `cheerio` - HTML解析と操作（npmパッケージとして自動インポート）

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

## Denoの特徴

- **依存関係管理**: `deno.json`で一元管理、自動ダウンロード
- **セキュリティ**: 明示的な権限が必要（`--allow-net`）
- **単一バイナリ**: `deno compile`でランタイム不要のバイナリ生成
- **TypeScript**: 標準でTypeScriptサポート
- **標準ライブラリ**: 豊富な標準ライブラリを提供

## 注意事項

- Yahoo乗換案内のHTML構造が変更された場合、セレクタの調整が必要になる可能性があります
- 過度なリクエストは避けてください
- 取得したデータの使用は、Yahoo乗換案内の利用規約に従ってください
- ネットワークアクセスには`--allow-net`権限が必要です 