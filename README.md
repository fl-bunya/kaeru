# Kaeru - 乗換案内ルート取得プロジェクト

このリポジトリは、Yahoo乗換案内から「新富町(東京都) → 上福岡」のルート情報を取得するサンプルを、TypeScript (Bun)・TypeScript (Node.js)・Go・Rustの4言語で実装したものです。

---

## 各言語の実行方法

### TypeScript (Bun)
```sh
cd bun
bun run index.ts
```

### TypeScript (Node.js)
```sh
cd node
npm install
npm run dev
```

### Go
```sh
cd go
go run main.go
```

### Rust
```sh
cd rust
cargo run
```

## ビルド済みバイナリの実行

### TypeScript (Bun) - バンドル版
```sh
cd bun
bun build index.ts --outdir dist --target node
node dist/index.js
```

### TypeScript (Node.js) - コンパイル版
```sh
cd node
npm run build
node dist/index.js
```

### Go - バイナリ版
```sh
cd go
go build -o kaeru main.go
./kaeru
```

### Rust - リリース版
```sh
cd rust
cargo build --release
./target/release/kaeru
```

---

## 実行結果の比較

### 出力例（2025年7月17日 21:28頃）

```
🚇 Yahoo乗換案内からルート情報を取得中...
Fetching transit routes from: https://transit.yahoo.co.jp/search/result?...

📋 取得したルート情報 (3件):

=== ルート 1 ===
発車時刻: 21:42
到着時刻: 22:54
ルート: 東京メトロ有楽町線川越市行 → 東武東上線川越市行

=== ルート 2 ===
発車時刻: 22:03
到着時刻: 23:11
ルート: 東京メトロ有楽町線森林公園行 → 東武東上線森林公園行

=== ルート 3 ===
発車時刻: 22:25
到着時刻: 23:37
ルート: 東京メトロ有楽町線森林公園行 → 東武東上線森林公園行
```

### 開発モードでの実行時間比較

| 言語 | 実行方法 | 実行時間 | 特徴 |
|------|----------|----------|------|
| **TypeScript (Bun)** | `bun run index.ts` | ~2秒 | 軽量、開発しやすい、依存関係管理が簡単 |
| **TypeScript (Node.js)** | `npm run dev` | ~3秒 | 標準的なNode.js環境、TypeScriptコンパイルが必要 |
| **Go** | `go run main.go` | ~1秒 | 高速、ランタイム不要、シンプルな実行 |
| **Rust** | `cargo run` | ~4秒 | 最速実行、メモリ安全、コンパイル時間が長い |

### ビルド済みバイナリでの実行時間比較

| 言語 | ファイルサイズ | 実行時間 | 特徴 |
|------|---------------|----------|------|
| **TypeScript (Bun)** | 1.6MB | ~1.11秒 | バンドル版、依存関係含む単一ファイル |
| **TypeScript (Node.js)** | 5.9KB | ~1.23秒 | コンパイル版、軽量だがNode.jsランタイム必要 |
| **Go** | 8.3MB | ~0.63秒 | 単一バイナリ、ランタイム不要、高速 |
| **Rust** | 4.4MB | ~0.74秒 | 最適化済みバイナリ、メモリ安全、高速 |

### 詳細比較

#### 開発モード
- **TypeScript (Bun)**: 最も開発しやすく、依存関係の管理が簡単。Bunの最適化で高速実行
- **TypeScript (Node.js)**: 標準的なNode.js環境、TypeScriptコンパイルが必要だが、より広く使われている
- **Go**: バランスの取れた性能、ランタイム不要、シンプルな実行方法
- **Rust**: 最高の実行速度、メモリ安全、コンパイル時間は長いが実行は高速

#### ビルド済みバイナリ
- **TypeScript (Bun)**: バンドル版は依存関係を含む単一ファイル、配布しやすい
- **TypeScript (Node.js)**: コンパイル版は軽量だがNode.jsランタイムが必要
- **Go**: 単一バイナリで最も高速、ランタイム不要で真のポータビリティ
- **Rust**: 最適化済みバイナリで高速、メモリ安全で本番環境に最適

### 注意事項
- **TypeScript**: 実行にはNode.js/bunランタイムが必要
- **Go/Rust**: ランタイム不要、真の意味での単一バイナリ配布が可能
- すべての言語で同じルート情報が取得でき、結果は一致します
- ビルド済みバイナリは配布用に最適化されており、実行速度が向上します

---

## ディレクトリ構成

```
kaeru/
├── bun/     # TypeScript (Bun) 実装
├── node/    # TypeScript (Node.js) 実装
├── go/      # Go実装
└── rust/    # Rust実装
```

---

## 備考
- 各言語の詳細なREADMEはそれぞれのディレクトリにあります。
- Yahoo乗換案内のHTML構造が変わると、パース部分の修正が必要になる場合があります。
- すべての実装で同じ機能を提供し、結果は一致します。 