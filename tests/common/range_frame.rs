//! 範囲枠付き画像の試験の共通部。見本の木の組み立てと、画像の左上の画面上の位置の読み取り。

use sengen_egui::{
    スタイル, ノード, 割合, 割合で表した矩形, 画素, 画素の組, 範囲枠の操作, 範囲枠付き画像,
    範囲枠付き画像の周りの余白,
};

const 赤い枠線: スタイル = スタイル {
    枠線色: Some(egui::Color32::RED),
    枠線太さ: Some(画素(1.0)),
    ..スタイル::無指定
};

/// 幅100・高さ50のテクスチャを、幅200・高さ100で描き、中央の半分の大きさの枠を重ねた木を組む。
pub fn 範囲枠の木(テクスチャ: &egui::TextureHandle) -> ノード<範囲枠の操作> {
    let 四分の一 = 割合::範囲へ丸めて生成する(0.25);
    let 四分の三 = 割合::範囲へ丸めて生成する(0.75);
    match 割合で表した矩形::生成する(四分の一, 四分の一, 四分の三, 四分の三)
    {
        Ok(枠) => 枠を重ねた木(テクスチャ, 枠),
        Err(不正) => panic!("中央の枠は正しい矩形である: {不正}"),
    }
}

pub fn 枠を重ねた木(
    テクスチャ: &egui::TextureHandle,
    枠: 割合で表した矩形,
) -> ノード<範囲枠の操作> {
    範囲枠付き画像(テクスチャ.clone(), 画素の組(200.0, 100.0), 枠, |操作| 操作)
        .装飾(赤い枠線)
        .into()
}

pub fn テクスチャを登録する(文脈: &egui::Context) -> egui::TextureHandle {
    let 画像 = egui::ColorImage::filled([100, 50], egui::Color32::GRAY);
    文脈.load_texture("範囲枠の下地", 画像, egui::TextureOptions::NEAREST)
}

/// 画像の左上の画面上の位置。中央パネルの内余白は最初の配置から読み取り、部品が画像の周りに確保する
/// 余白を足す。
pub fn 画像の左上(文脈: &egui::Context) -> egui::Pos2 {
    let mut 左上 = egui::Pos2::ZERO;
    let _ = 文脈.run(egui::RawInput::default(), |文脈| {
        egui::CentralPanel::default().show(文脈, |ui| 左上 = ui.cursor().min);
    });
    左上 + egui::Vec2::splat(範囲枠付き画像の周りの余白.eguiへ渡す値())
}
