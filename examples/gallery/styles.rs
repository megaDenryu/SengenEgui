//! 見本帳の装飾。テーマと名前付きのスタイルをここへ集める。

use eframe::egui::Color32;
use sengen_egui::{
    スタイル, テーマ, 余白画素, 明暗, 画素, 画素の組, 角丸画素
};

pub const 画面のテーマ: テーマ = テーマ {
    基調: 明暗::濃色,
    強調色: Some(Color32::from_rgb(80, 150, 240)),
    地の色: Some(Color32::from_rgb(22, 25, 31)),
    文字色: Some(Color32::from_rgb(226, 230, 237)),
    部品の面の色: Some(Color32::from_rgb(52, 60, 76)),
    表示倍率: None,
    部品の間隔: Some(画素の組(8.0, 6.0)),
    ボタンの内余白: Some(画素の組(8.0, 3.0)),
    部品の角丸: Some(角丸画素(4)),
};

pub const 見出し: スタイル = スタイル {
    文字サイズ: Some(画素(16.0)),
    太字: Some(true),
    ..スタイル::無指定
};

pub const カード: スタイル = スタイル {
    背景色: Some(Color32::from_rgb(31, 35, 43)),
    内余白: Some(余白画素(10)),
    角丸: Some(角丸画素(6)),
    枠線色: Some(Color32::from_rgb(52, 59, 72)),
    ..スタイル::無指定
};

pub const 斜体と下線: スタイル = スタイル {
    斜体: Some(true),
    下線: Some(true),
    ..スタイル::無指定
};

pub const 取り消し線と弱い: スタイル = スタイル {
    取り消し線: Some(true),
    弱い: Some(true),
    ..スタイル::無指定
};

pub const 補足: スタイル = スタイル {
    弱い: Some(true),
    ..スタイル::無指定
};

pub const 札: スタイル = スタイル {
    背景色: Some(Color32::from_rgb(38, 43, 53)),
    内余白: Some(余白画素(8)),
    角丸: Some(角丸画素(6)),
    枠線色: Some(Color32::from_rgb(52, 59, 72)),
    ..スタイル::無指定
};

pub const 選ばれた札: スタイル = スタイル {
    枠線色: Some(Color32::from_rgb(245, 158, 11)),
    枠線太さ: Some(画素(2.0)),
    ..札
};

pub const 映像の上の文字: スタイル = スタイル {
    文字色: Some(Color32::WHITE),
    背景色: Some(Color32::from_black_alpha(160)),
    内余白: Some(余白画素(4)),
    ..スタイル::無指定
};

pub const 映像の欄: スタイル = スタイル {
    背景色: Some(Color32::BLACK),
    ..スタイル::無指定
};

pub const 停止中の覆い: スタイル = スタイル {
    背景色: Some(Color32::from_black_alpha(140)),
    ..スタイル::無指定
};
