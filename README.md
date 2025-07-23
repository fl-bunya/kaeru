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



### Deno - バンドル版
```sh
cd deno
deno bundle --minify --output dist/index.js src/index.ts
deno run --allow-net dist/index.js
```

### Deno - コンパイル版バイナリ
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

#### 計測コマンド
```sh
# Go
for i in {1..10}; do /usr/bin/time -l ./go/kaeru 2>&1 | grep -E "(real|maximum resident set size)"; done

# Rust
for i in {1..10}; do /usr/bin/time -l ./rust/target/release/kaeru 2>&1 | grep -E "(real|maximum resident set size)"; done

# Node.js
for i in {1..10}; do /usr/bin/time -l node ./node/dist/index.js 2>&1 | grep -E "(real|maximum resident set size)"; done

# Bun
for i in {1..10}; do /usr/bin/time -l bun ./bun/dist/index.js 2>&1 | grep -E "(real|maximum resident set size)"; done

# Bun (Compile)
for i in {1..10}; do /usr/bin/time -l ./bun/dist/kaeru 2>&1 | grep -E "(real|maximum resident set size)"; done

# Deno
for i in {1..10}; do /usr/bin/time -l deno run --allow-net ./deno/dist/index.js 2>&1 | grep -E "(real|maximum resident set size)"; done

# Deno (Compile)
for i in {1..10}; do /usr/bin/time -l ./deno/dist/kaeru 2>&1 | grep -E "(real|maximum resident set size)"; done
```

| 言語 | ファイルサイズ | 実行時間 | メモリ使用量 | 特徴 |
|------|---------------|----------|-------------|------|
| **Rust** | 4.4MB | 0.49秒 | 9.9MB | 最適化済みバイナリ、メモリ安全、最高性能 |
| **Go** | 8.3MB | 0.51秒 | 12.0MB | 単一バイナリ、ランタイム不要、高速・軽量 |
| **Bun (Compile)** | 53MB | 1.09秒 | 69.5MB | コンパイル版バイナリ、Bunランタイム不要 |
| **Bun** | 326KB | 1.07秒 | 67.8MB | Bun向けビルド版、依存関係含む単一ファイル |
| **Deno (Compile)** | 76MB | 1.30秒 | 80.7MB | コンパイル版バイナリ、Denoランタイム不要 |
| **Deno** | 413KB | 1.61秒 | 73.6MB | バンドル版、依存関係含む単一ファイル |
| **Node.js** | 279KB | 1.57秒 | 91.2MB | Viteビルド版、依存関係含む単一ファイル |

### 詳細比較

#### 処理速度
- **Rust**: 最速（0.49秒）、最小メモリ使用量
- **Go**: 高速（0.51秒）、軽量なメモリ使用量
- **Bun (Compile)**: 高速（1.09秒）、コンパイル版で効率的
- **Bun**: 高速（1.07秒）、Bunネイティブの最適化
- **Deno (Compile)**: 中程度（1.30秒）、コンパイル版で効率的
- **Deno**: 中程度（1.61秒）、バンドル版で効率的
- **Node.js**: 中程度（1.57秒）、Viteによる最適化で効率的

#### メモリ使用量
- **Rust**: 最小（9.9MB）、メモリ効率が最高
- **Go**: 軽量（12.0MB）、バランスの取れた性能
- **Bun**: 中程度（67.8MB）、TypeScriptランタイム中では効率的
- **Bun (Compile)**: 中程度（69.5MB）、コンパイル版でも効率的
- **Deno**: 中程度（73.6MB）、バンドル版で効率的
- **Deno (Compile)**: 中程度（80.7MB）、セキュリティ機能による増加
- **Node.js**: 最大（91.2MB）、Viteビルドでも標準的なNode.jsのメモリ使用量

#### ファイルサイズ
- **Node.js**: 小（279KB）、Viteによる効率的なbundle
- **Bun**: 小（326KB）、Bun向け最適化されたbundle
- **Deno**: 中（413KB）、バンドル版、依存関係含む単一ファイル
- **Rust**: 中（4.4MB）、最適化済みバイナリ
- **Go**: 大（8.3MB）、単一バイナリでランタイム含む
- **Bun (Compile)**: 大（53MB）、Bunランタイムを含む単一バイナリ
- **Deno (Compile)**: 最大（76MB）、Denoランタイムを含む

### 開発モードでの実行時間比較（5回測定の平均値）

| 言語 | 実行方法 | 実行時間 | 特徴 |
|------|----------|----------|------|
| **Go** | `cd go && go run main.go` | 0.85秒 | 最速、ランタイム不要、シンプルな実行 |
| **TypeScript (Bun)** | `bun run index.ts` | 1.14秒 | 軽量、開発しやすい、依存関係管理が簡単 |
| **Rust** | `cargo run` | 1.37秒 | 高速実行、メモリ安全、初回はコンパイル時間含む |
| **Deno** | `deno run src/index.ts` | 1.76秒 | セキュリティ重視、依存関係管理が簡単 |
| **TypeScript (Node.js)** | `npm run dev` | 3.48秒 | 標準的なNode.js環境、TypeScriptコンパイルが必要 |

### 注意事項
- **TypeScript (Bun/Node.js)**: 実行にはNode.js/bunランタイムが必要
- **Go/Rust**: ランタイム不要、真の意味での単一バイナリ配布が可能
- **Deno**: セキュリティ機能により、ネットワークアクセスに明示的な許可が必要
- すべての言語で同じルート情報が取得でき、結果は一致します
- ビルド済みバイナリは配布用に最適化されており、実行速度が向上します
- **Bun**: Bun向けに最適化されたビルドにより、効率的な実行が可能
- **Node.js**: Viteによるbundleとminifyにより、依存関係を含む単一ファイルで配布可能
- **Bun (Compile)**: 単一の実行可能バイナリで、Bunランタイム不要。配布用に最適化されています
- **Deno**: バンドル版、依存関係含む単一ファイルで配布可能
- **Deno (Compile)**: 単一の実行可能バイナリで、Denoランタイム不要。配布用に最適化されています

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
- **測定日時**: 2025年7月23日