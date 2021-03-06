<h1 align="center">lets</h1>

<h3 align="center">General purpose meta-build system</h3>

## 動機

Make や関数型プログラミング言語を参考に、

- 挙動を静的に検証できる
- ビルドタスクの定義を配布・再利用できる

ビルドシステムを作りたいと考えたことがきっかけです。

### Make の欠点

- タスクの再利用が難しい
- 変数に依存するタスクのデバッグ効率が悪い
- コマンドの単純な記述ミスでさえ、実際に動かしてみなければ気づけない
- clean タスクの定義が冗長になりがち

### lets でのアプローチ

ビルド設定を記述するためのドメイン特化言語(以下、DSL)を定義し、**ninja-build のフロントエンド**としてのコンパイラを提供します。

- ビルドタスクは「ファイルシステムの状態を遷移させる関数」として抽象化され、そのタスクが依存するものはすべて引数として宣言されていなければならない
- タスク同士を粗結合にするために、グローバル変数やシェルの環境変数といった機能を排除する
- コンパイル中に「想定しているディレクトリ構造」や「各タスクの引数・生成物」、「タスク同士の依存関係」を解析し、実際にビルドした際に不整合がないか確認する

## 実行方法

```bash
cd compiler
cargo run
```
