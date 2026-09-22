//! 寸法と割合の単位付きの数値。裸の f32 を寸法として公開APIへ出さないための層である。
//! 生の f32 へ戻すのは egui へ渡す境界だけであり、その口は `eguiへ渡す値` と名前で明示する。

mod logical_pixel;
mod pair;
mod ratio;

pub use logical_pixel::{画素, 論理画素};
pub use pair::{画素の組, 縦横の論理画素};
pub use ratio::{割合, 割合の範囲外};
