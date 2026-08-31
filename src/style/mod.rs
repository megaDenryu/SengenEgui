//! 装飾。構造（ノード木）から分離し、名前付き定数として集約する（lib.rs 方針3）。
//! C#版の StyleResolver（GUIStyle 変換のキャッシュ）は、egui に相当する変換工程が
//! 無いため持たない。

mod apply;
mod definition;

pub use definition::スタイル;
