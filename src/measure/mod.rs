//! 寸法と割合の単位付きの数値。裸の f32 を寸法として公開APIへ出さないための層である。
//! 生の f32 へ戻すのは egui へ渡す境界だけであり、その口は `eguiへ渡す値` と名前で明示する。

mod band_value;
mod integer_pixel;
mod logical_pixel;
mod pair;
mod ratio;
mod ratio_difference;
mod ratio_rect;
mod zoom;

pub use band_value::{
    帯の値, 帯の値の差, 帯の全体の範囲, 帯の全体の範囲の不正
};
pub use integer_pixel::{
    余白の画素, 余白画素, 整数画素の範囲外, 角丸の画素, 角丸画素
};
pub use logical_pixel::{画素, 論理画素};
pub use pair::{画素の組, 縦横の論理画素};
pub use ratio::{割合, 割合の範囲外};
pub use ratio_difference::{割合の差, 縦横の割合の差};
pub use ratio_rect::{割合で表した矩形, 割合で表した矩形の不正};
pub use zoom::{拡大率, 拡大率の範囲外};

/// 倍精度の小数（f64）を、egui が使う単精度の小数（f32）の最も近い値へ丸める。単精度の範囲を超える値は無限大になる。
/// lint が `as` による変換を禁じているため、egui の数の変換を使う。
fn 単精度の小数へ丸める(値: f64) -> f32 {
    <f32 as egui::emath::Numeric>::from_f64(値)
}
