//! 見本帳。全部品を1画面に並べ、利用側と同じ書き方（糖衣ファクトリだけ）で組む。
//! 素の egui を使うのは、この起動の部分（窓の作成・フォント・テクスチャの登録）だけである。
//! 起動: リポジトリルートで `cargo run -p sengen_egui --example gallery`
//! （GameScriptingTheory では `cargo xtask ui-gallery`）。

#![forbid(unsafe_code)]

mod attach_page;
mod container_page;
mod display_page;
mod font;
mod input_page;
mod sample_frame;
mod sample_range_frame;
mod screen;
mod state;
mod styles;

use eframe::egui;

struct 見本帳アプリ {
    状態: state::状態,
}

impl eframe::App for 見本帳アプリ {
    fn update(&mut self, 文脈: &egui::Context, _枠: &mut eframe::Frame) {
        let mut 応答一覧 = Vec::new();
        egui::CentralPanel::default().show(文脈, |ui| {
            応答一覧 = screen::画面(&self.状態).描画して集める(ui);
        });
        for 応答 in 応答一覧 {
            self.状態.適用する(応答);
        }
    }
}

/// 読み込み器なしで画像を出すため、画素から作った見本のテクスチャを egui へ登録する。
fn 見本の画像を登録する(文脈: &egui::Context) -> egui::TextureHandle {
    let 一辺 = 64usize;
    let 画素一覧: Vec<egui::Color32> = (0..一辺 * 一辺)
        .map(|番号| {
            let (横, 縦) = (番号 % 一辺, 番号 / 一辺);
            let 赤 = u8::try_from(横 * 4).unwrap_or(u8::MAX);
            let 緑 = u8::try_from(縦 * 4).unwrap_or(u8::MAX);
            egui::Color32::from_rgb(赤, 緑, 160)
        })
        .collect();
    let 画像 = egui::ColorImage {
        size: [一辺, 一辺],
        source_size: egui::vec2(64.0, 64.0),
        pixels: 画素一覧,
    };
    文脈.load_texture("見本の画像", 画像, egui::TextureOptions::NEAREST)
}

fn main() -> std::process::ExitCode {
    let 選択肢 = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(960.0, 720.0)),
        ..Default::default()
    };
    let 結果 = eframe::run_native(
        "SengenEgui 見本帳",
        選択肢,
        Box::new(|作成文脈| {
            styles::画面のテーマ.適用する(&作成文脈.egui_ctx);
            font::日本語フォントを設定する(&作成文脈.egui_ctx);
            let 画像 = 見本の画像を登録する(&作成文脈.egui_ctx);
            Ok(Box::new(見本帳アプリ {
                状態: state::状態::新規(画像),
            }))
        }),
    );
    match 結果 {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(原因) => {
            eprintln!("見本帳の起動に失敗した: {原因}");
            std::process::ExitCode::FAILURE
        }
    }
}
