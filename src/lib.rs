//! 宣言的UIラッパー `sengen_egui`。C#版 SengenImgui・TypeScript版 SengenUI と同じ思想を
//! egui の上で実現する。このリポジトリの他クレートへ依存しない葉のライブラリである。
//!
//! 中核の方針は SengenImgui と同じ3つである。
//!
//! 1. **ノード木は構築時に一度だけ組み、毎フレーム [`ノード::描画する`] で再生する。**
//!    描画のたびに木を作り直さない
//! 2. **動的な値はノードに持たせず、クロージャで描画のたびに取りに行く。**
//!    購読・通知の仕組みなしで「値が変われば表示も追従」が成立する
//! 3. **装飾（[`スタイル`]）は構造から分離し、名前付き定数として集約する**
//!
//! ```
//! use std::{cell::Cell, rc::Rc};
//! use sengen_egui::{子, ボタン, 文章, 文章動, 縦積み};
//!
//! let 数 = Rc::new(Cell::new(0));
//! let 表示用 = Rc::clone(&数);
//! let 加算用 = Rc::clone(&数);
//! let mut 根 = 縦積み().子(子![
//!     文章("カウンタ"),
//!     文章動(move || 表示用.get().to_string()),
//!     ボタン("+1").押されたら(move || 加算用.set(加算用.get() + 1)),
//! ]);
//! // 毎フレーム、eguiの描画閉包の中で 根.描画する(ui) を呼ぶ
//! # let _ = &mut 根;
//! ```
#![forbid(unsafe_code)]

mod component;
mod containers;
mod node;
mod primitives;
mod source;
mod style;
mod ui;

pub use component::{部品, 部品箱};
pub use containers::{
    条件ノード, 横並びノード, 縦スクロールノード, 縦積みノード
};
pub use node::{ノード, ノード箱, 空ノード, 箱};
pub use primitives::{
    ボタンノード, 余白ノード, 入力欄ノード, 切り替えノード, 区切り線ノード, 文章ノード,
};
pub use source::文字列源;
pub use style::スタイル;
pub use ui::{
    ボタン, ボタン動, 余白, 入力欄, 切り替え, 区切り線, 文章, 文章動, 条件, 条件二択, 横並び, 無し,
    縦スクロール, 縦積み, 部品化,
};

/// 子ノード列を組む糖衣。各要素を [`箱`] で包んだ `Vec<ノード箱>` を返す。
#[macro_export]
macro_rules! 子 {
    ($($節:expr),* $(,)?) => { vec![$($crate::箱($節)),*] };
}
