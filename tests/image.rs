//! 画像の表示寸法が縦横比を保って指定の枠へ収まることと、押せる画像が押されると応答を発することを確かめる。

mod common;

use sengen_egui::{ノード, 子, 画像, 画素の組, 縦積み};

#[derive(Clone, PartialEq, Debug)]
enum 応答 {
    画像を押した,
}

/// 幅64・高さ32の単色のテクスチャを登録する。
fn 横長のテクスチャを登録する(文脈: &egui::Context) -> egui::TextureHandle {
    let 画像 = egui::ColorImage::filled([64, 32], egui::Color32::GRAY);
    文脈.load_texture("横長", 画像, egui::TextureOptions::NEAREST)
}

/// 木を1フレーム描画し、木の描画した範囲の矩形を返す。
fn 描画した矩形(文脈: &egui::Context, 木: ノード<応答>) -> egui::Rect {
    let mut 矩形 = egui::Rect::NOTHING;
    let _ = 文脈.run(egui::RawInput::default(), |文脈| {
        egui::CentralPanel::default().show(文脈, |ui| {
            矩形 = 木.描画する(ui, &mut Vec::new()).rect;
        });
    });
    矩形
}

#[test]
fn 表示寸法の縦横比が画像と一致すれば指定の幅と高さにちょうど描く() {
    let 文脈 = egui::Context::default();
    let テクスチャ = 横長のテクスチャを登録する(&文脈);
    let 木 = 画像(テクスチャ).表示寸法(画素の組(160.0, 80.0)).into();
    assert_eq!(描画した矩形(&文脈, 木).size(), egui::vec2(160.0, 80.0));
}

#[test]
fn 表示寸法の縦横比が画像と違えば縦横比を保って枠に収まる大きさで描く() {
    let 文脈 = egui::Context::default();
    let テクスチャ = 横長のテクスチャを登録する(&文脈);
    let 木 = 画像(テクスチャ).表示寸法(画素の組(200.0, 50.0)).into();
    assert_eq!(描画した矩形(&文脈, 木).size(), egui::vec2(100.0, 50.0));
}

#[test]
fn 押せる画像を押すと応答が出る() {
    let 文脈 = egui::Context::default();
    let テクスチャ = 横長のテクスチャを登録する(&文脈);
    let 木を組む = || {
        縦積み(子![
            画像(テクスチャ.clone())
                .表示寸法(画素の組(64.0, 32.0))
                .押されたら発する(応答::画像を押した)
        ])
        .into()
    };
    let 集まり = common::左クリックする(&文脈, egui::pos2(30.0, 20.0), &木を組む);
    assert_eq!(集まり, vec![応答::画像を押した]);
}

#[test]
fn 押せる画像は写しても押すと写した応答が出る() {
    let 文脈 = egui::Context::default();
    let テクスチャ = 横長のテクスチャを登録する(&文脈);
    let 木を組む = || {
        ノード::from(画像(テクスチャ.clone()).押されたら発する(応答::画像を押した))
            .写す(|応答| format!("{応答:?}"))
    };
    let 集まり = common::左クリックする(&文脈, egui::pos2(30.0, 20.0), &木を組む);
    assert_eq!(集まり, vec!["画像を押した".to_string()]);
}
