//! 画像の表示寸法・描く部分・表示寸法へ引き伸ばす指定が、描く大きさを約束どおりに決めることを確かめる。

mod common;

use common::image::{描画した矩形, 横の範囲, 横長のテクスチャを登録する};
use sengen_egui::{ノード, 画像, 画素の組};

#[test]
fn 表示寸法の縦横比が画像と一致すれば指定の幅と高さにちょうど描く() {
    let eguiの本体 = egui::Context::default();
    let テクスチャ = 横長のテクスチャを登録する(&eguiの本体);
    let 木: ノード<()> = 画像(テクスチャ).表示寸法(画素の組(160.0, 80.0)).into();
    assert_eq!(
        描画した矩形(&eguiの本体, 木).size(),
        egui::vec2(160.0, 80.0)
    );
}

#[test]
fn 表示寸法の縦横比が画像と違えば縦横比を保って枠に収まる大きさで描く() {
    let eguiの本体 = egui::Context::default();
    let テクスチャ = 横長のテクスチャを登録する(&eguiの本体);
    let 木: ノード<()> = 画像(テクスチャ).表示寸法(画素の組(200.0, 50.0)).into();
    assert_eq!(
        描画した矩形(&eguiの本体, 木).size(),
        egui::vec2(100.0, 50.0)
    );
}

#[test]
fn 描く部分を指定すると縦横比は描く部分の縦横比で保つ() {
    let eguiの本体 = egui::Context::default();
    let テクスチャ = 横長のテクスチャを登録する(&eguiの本体);
    let 木: ノード<()> = 画像(テクスチャ)
        .描く部分(横の範囲(0.0, 0.5))
        .表示寸法(画素の組(100.0, 50.0))
        .into();
    assert_eq!(描画した矩形(&eguiの本体, 木).size(), egui::vec2(50.0, 50.0));
}

#[test]
fn 表示寸法へ引き伸ばす画像は指定の幅と高さちょうどに描く() {
    let eguiの本体 = egui::Context::default();
    let テクスチャ = 横長のテクスチャを登録する(&eguiの本体);
    let 木: ノード<()> = 画像(テクスチャ)
        .描く部分(横の範囲(0.25, 0.5))
        .表示寸法へ引き伸ばす(画素の組(200.0, 60.0))
        .into();
    assert_eq!(
        描画した矩形(&eguiの本体, 木).size(),
        egui::vec2(200.0, 60.0)
    );
}

/// 木を1フレーム描画し、テクスチャを貼った矩形の uv（テクスチャのうち描いた範囲）を返す。
fn 描いた画像のuv(eguiの本体: &egui::Context, 木: ノード<()>) -> egui::Rect {
    let 出力 = eguiの本体.run(egui::RawInput::default(), |eguiの本体| {
        egui::CentralPanel::default().show(eguiの本体, |ui| {
            let _ = 木.描画する(ui, &mut Vec::new());
        });
    });
    出力
        .shapes
        .iter()
        .find_map(|切り抜いた形| match &切り抜いた形.shape {
            egui::Shape::Rect(矩形) => 矩形.brush.as_ref().map(|塗り方| 塗り方.uv),
            _ => None,
        })
        .unwrap_or_else(|| panic!("テクスチャを貼った矩形が無い"))
}

#[test]
fn 左右を反転すると描く部分の中でuvの左端と右端を入れ替え_描く大きさは変わらない() {
    let eguiの本体 = egui::Context::default();
    let テクスチャ = 横長のテクスチャを登録する(&eguiの本体);
    let 組む = |反転するか: bool| -> ノード<()> {
        let 画像 = 画像(テクスチャ.clone())
            .描く部分(横の範囲(0.25, 0.5))
            .表示寸法へ引き伸ばす(画素の組(200.0, 60.0));
        if 反転するか {
            画像.左右を反転する()
        } else {
            画像
        }
        .into()
    };
    let そのまま = 描いた画像のuv(&eguiの本体, 組む(false));
    assert_eq!((そのまま.min.x, そのまま.max.x), (0.25, 0.5));
    let 反転 = 描いた画像のuv(&eguiの本体, 組む(true));
    assert_eq!((反転.min.x, 反転.max.x), (0.5, 0.25));
    assert_eq!((反転.min.y, 反転.max.y), (そのまま.min.y, そのまま.max.y));
    assert_eq!(
        描画した矩形(&eguiの本体, 組む(true)).size(),
        egui::vec2(200.0, 60.0)
    );
}
