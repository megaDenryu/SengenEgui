//! 画像の試験の共通部。見本のテクスチャの登録と、描く部分の矩形の作成と、描画した範囲の読み取り。

use sengen_egui::{ノード, 割合, 割合で表した矩形};

/// 幅64・高さ32の単色のテクスチャを登録する。
pub fn 横長のテクスチャを登録する(文脈: &egui::Context) -> egui::TextureHandle {
    let 画像 = egui::ColorImage::filled([64, 32], egui::Color32::GRAY);
    文脈.load_texture("横長", 画像, egui::TextureOptions::NEAREST)
}

/// 木を1フレーム描画し、木の描画した範囲の矩形を返す。
pub fn 描画した矩形<M: Clone>(文脈: &egui::Context, 木: ノード<M>) -> egui::Rect {
    let mut 矩形 = egui::Rect::NOTHING;
    let _ = 文脈.run(egui::RawInput::default(), |文脈| {
        egui::CentralPanel::default().show(文脈, |ui| {
            矩形 = 木.描画する(ui, &mut Vec::new()).rect;
        });
    });
    矩形
}

/// 左端から右端までの横の範囲と、縦の全体を指す矩形を作る。
pub fn 横の範囲(左端: f32, 右端: f32) -> 割合で表した矩形 {
    let 割合にする = 割合::範囲へ丸めて生成する;
    match 割合で表した矩形::生成する(
        割合にする(左端),
        割合::ゼロ,
        割合にする(右端),
        割合::全部,
    ) {
        Ok(矩形) => 矩形,
        Err(不正) => panic!("試験の矩形が不正である: {不正}"),
    }
}
