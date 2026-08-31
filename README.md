# SengenEgui

egui の上で宣言的にUIを組む薄いラッパー。C#（Unity IMGUI）版の SengenImgui、
TypeScript（DOM）版の SengenUI と同じ思想の Rust（egui）版である。

## 中核の方針

1. **ノード木は構築時に一度だけ組み、毎フレーム `描画する` で再生する。** 描画のたびに木を作り直さない
2. **動的な値はノードに持たせず、クロージャで描画のたびに取りに行く。** 購読・通知の仕組みなしで「値が変われば表示も追従」が成立する
3. **装飾（`スタイル`）は構造から分離し、名前付き定数として集約する**

## 最小例

```rust
use std::{cell::Cell, rc::Rc};
use sengen_egui::{子, ボタン, 文章, 文章動, 縦積み};

let 数 = Rc::new(Cell::new(0));
let 表示用 = Rc::clone(&数);
let 加算用 = Rc::clone(&数);
let mut 根 = 縦積み().子(子![
    文章("カウンタ"),
    文章動(move || 表示用.get().to_string()),
    ボタン("+1").押されたら(move || 加算用.set(加算用.get() + 1)),
]);
// 毎フレーム、eguiの描画閉包の中で 根.描画する(ui) を呼ぶ
```

## リファレンス

APIの正本は rustdoc である。`cargo doc --open` で読む。

- 糖衣ファクトリ（`ui` モジュール）が利用側の唯一の入口。利用側は egui のAPIを直接呼ばない。口が足りないときはこのリポジトリへ足す
- 複合UI単位は `部品` トレイトで表し、`部品化` でノードとして子に渡す
- 実行中に変わる条件の表示切り替えは `条件` / `条件二択`、構築時に決まる分岐は `無し()` を使う

## 利用形態

Gitサブモジュールとして利用元リポジトリの Cargo ワークスペースへ埋め、メンバーとして登録する。
単独クローンでも `cargo test` が通る自己完結の定義を保つ。

利用例: [GameScriptingTheory](https://github.com/megaDenryu/GameScriptingTheory) の `crates/devtool`（開発ツールGUI）。
