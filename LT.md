# 帰宅の電車発射時間を取得してみる

## モチベ
電車が20-30分に1本程度しかないので、毎回退社前に電車の発射時刻を確認している。
毎回yahoo乗り換え案内を開いて確認するのが面倒だったので、スクリプトを書いてみる。

## スクリプトを作る
### 方法
いい感じの乗り換えAPIがなかったので、yahoo乗り換え案内ページをスクレイピングする。

### yahoo乗り換え案内ページのクエリパラメータ

### 実行結果

## 他の言語でも作ってみる

### 対象言語
Rust, Go, Node.js, Bun, Deno

それぞれ下記で比較
- 本番ビルドでの実行結果（JSはbundle, minify。Bun、Denoはコンパイル
- 開発モードでの実行結果


### 結果
ネットワーク状況や各言語でのスクレイピングライブラリの実装の差があるので、ガバガバの比較。
JSのファイルサイズは、ランタイムの事も考慮しなければならない。

README.mdに記載した。

### 気になった点
- Rust
  - 実行ファイルサイズ、メモリ使用量が少ない。
  - 開発モードの実行はそんなに早くない。（コンパイル遅い）
  - 人間が書くとコンパイルエラーとの格闘になる記憶が。
- Go
  - 実行結果が最速。
  - コンパイルも速い。
  - フォルダ構成がシンプル。
  - 書き方迷わない。
- Node.js
  - Viteのおかげか、bundleファイルはJSの中では小さい。
  - 開発モードの実行が一番遅い。
  - メモリ使用量がJSでは一番多い。（V8だから？）
- Bun
  - 開発モードの実行がJSでは一番遅い。
  - メモリ使用量がJSでは一番少ない。（JavaScript Coreだから？）
- Deno
  - セキュリティが厳しい、オーバーヘッドで色々遅め。

## 日常使いする
エイリアスを設定する。
一番速いgoで。

.zprofile
```
alias kaerujs='bun /Users/bunya/my/kaeru/ts/dist/index.js'
alias kaerurust='/Users/bunya/my/kaeru/rust/target/release/kaeru'
alias kaerugo='/Users/bunya/my/kaeru/go/kaeru'
alias kaeru='kaerugo'
```

らくまっちに言わせてみる
```
alias kaerucchi='kaeru | grep 発車時刻 | head -n 1 | xargs -I {} npx rakumacchisay "{}"'
```

おわり