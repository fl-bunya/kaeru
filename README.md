# Kaeru - 乗換案内ルート取得プロジェクト

このリポジトリは、Yahoo乗換案内から「新富町(東京都) → 上福岡」のルート情報を取得するサンプルを、TypeScript (Bun)・TypeScript (Node.js)・Go・Rust・Denoの5言語で実装したものです。

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

### Deno
```sh
cd deno
deno run --allow-net src/index.ts
```

## ビルド済みバイナリの実行

### TypeScript (Bun) - バンドル版
```sh
cd bun
bun run build
bun dist/index.js
```

### TypeScript (Bun) - コンパイル版バイナリ
```sh
cd bun
bun run compile
./dist/kaeru
```

### TypeScript (Node.js) - Viteビルド版
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

### Deno - コンパイル版
```sh
cd deno
deno compile --allow-net --output dist/kaeru src/index.ts
./dist/kaeru
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

### ビルド済みバイナリでの性能比較（10回測定の平均値）

| 言語 | ファイルサイズ | 実行時間 | メモリ使用量 | 特徴 |
|------|---------------|----------|-------------|------|
| **Go** | 8.3MB | 0.55秒 | 11.8MB | 単一バイナリ、ランタイム不要、高速・軽量 |
| **Rust** | 4.4MB | 0.58秒 | 10.1MB | 最適化済みバイナリ、メモリ安全、最高性能 |
| **Node.js** | 279KB | 0.82秒 | 91.6MB | Viteビルド版、依存関係含む単一ファイル |
| **Bun** | 326KB | 0.87秒 | 68.9MB | Bun向けビルド版、依存関係含む単一ファイル |
| **Bun (Compile)** | 53MB | 0.91秒 | 69.8MB | コンパイル版バイナリ、Bunランタイム不要 |
| **Deno** | 76.6MB | 1.12秒 | 80.3MB | コンパイル版、セキュリティ重視 |

### 詳細比較

#### 処理速度
- **Go**: 最も高速（0.55秒）、軽量なメモリ使用量
- **Rust**: ほぼ同等の高速性（0.58秒）、最小メモリ使用量
- **Node.js**: 高速（0.82秒）、Viteによる最適化で効率的
- **Bun**: 高速（0.87秒）、Bunネイティブの最適化
- **Bun (Compile)**: 高速（0.91秒）、コンパイル版でも良好な性能
- **Deno**: 最も遅い（1.12秒）、セキュリティ機能のオーバーヘッド

#### メモリ使用量
- **Rust**: 最小（10.1MB）、メモリ効率が最高
- **Go**: 軽量（11.8MB）、バランスの取れた性能
- **Bun**: 中程度（68.9MB）、TypeScriptランタイム中では効率的
- **Bun (Compile)**: 中程度（69.8MB）、コンパイル版でも効率的
- **Deno**: 中程度（80.3MB）、セキュリティ機能による増加
- **Node.js**: 最大（91.6MB）、Viteビルドでも標準的なNode.jsのメモリ使用量

#### ファイルサイズ
- **Node.js**: 小（279KB）、Viteによる効率的なbundle
- **Bun**: 小（326KB）、Bun向け最適化されたbundle
- **Rust**: 中（4.4MB）、最適化済みバイナリ
- **Go**: 大（8.3MB）、単一バイナリでランタイム含む
- **Bun (Compile)**: 大（53MB）、Bunランタイムを含む単一バイナリ
- **Deno**: 最大（76.6MB）、Denoランタイムを含む

### 開発モードでの実行時間比較

| 言語 | 実行方法 | 実行時間 | 特徴 |
|------|----------|----------|------|
| **TypeScript (Bun)** | `bun run index.ts` | ~2秒 | 軽量、開発しやすい、依存関係管理が簡単 |
| **TypeScript (Node.js)** | `npm run dev` | ~3秒 | 標準的なNode.js環境、TypeScriptコンパイルが必要 |
| **Go** | `go run main.go` | ~1秒 | 高速、ランタイム不要、シンプルな実行 |
| **Rust** | `cargo run` | ~4秒 | 最速実行、メモリ安全、コンパイル時間が長い |
| **Deno** | `deno run src/index.ts` | ~2秒 | セキュリティ重視、依存関係管理が簡単 |

### 注意事項
- **TypeScript (Bun/Node.js)**: 実行にはNode.js/bunランタイムが必要
- **Go/Rust**: ランタイム不要、真の意味での単一バイナリ配布が可能
- **Deno**: セキュリティ機能により、ネットワークアクセスに明示的な許可が必要
- すべての言語で同じルート情報が取得でき、結果は一致します
- ビルド済みバイナリは配布用に最適化されており、実行速度が向上します
- **Bun**: Bun向けに最適化されたビルドにより、効率的な実行が可能
- **Node.js**: Viteによるbundleとminifyにより、依存関係を含む単一ファイルで配布可能
- **Bun (Compile)**: 単一の実行可能バイナリで、Bunランタイム不要。配布用に最適化されています

---

## ディレクトリ構成

```
kaeru/
├── bun/     # TypeScript (Bun) 実装
├── node/    # TypeScript (Node.js) 実装
├── go/      # Go実装
├── rust/    # Rust実装
└── deno/    # Deno実装
```

---

## 備考
- 各言語の詳細なREADMEはそれぞれのディレクトリにあります。
- Yahoo乗換案内のHTML構造が変わると、パース部分の修正が必要になる場合があります。
- すべての実装で同じ機能を提供し、結果は一致します。
- 性能測定はネットワーク状況により変動する可能性があります。
- **測定日時**: 2025年1月23日