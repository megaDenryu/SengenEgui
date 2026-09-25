//! 見本帳。全部品を1画面に並べ、利用側と同じ書き方（糖衣ファクトリだけ）で組む。
//! 素の egui を使うのは、この起動の部分（窓の作成・egui の本体をフォントの設定とテクスチャの登録へ渡すこと）だけである。
//! 起動: リポジトリルートで `cargo run -p sengen_egui --example gallery`
//! （GameScriptingTheory では `cargo xtask ui-gallery`）。

#![forbid(unsafe_code)]

mod attach_page;
mod container_page;
mod display_page;
mod input_page;
mod panel_section;
mod player_page;
mod player_video;
mod sample_frame;
mod sample_player;
mod sample_range_frame;
mod sample_texture;
mod screen;
mod state;
mod styles;

use eframe::egui;
use sengen_egui::{拡大縮小の仕方, 日本語フォントの候補};

struct 見本帳アプリ {
    状態: state::状態,
}

impl eframe::App for 見本帳アプリ {
    fn update(&mut self, eguiの本体: &egui::Context, _枠: &mut eframe::Frame) {
        if let Err(不正) = self.状態.再生.進める(std::time::Instant::now()) {
            eprintln!("見本の映像を差し替えられない: {不正}");
        }
        let mut 応答一覧 = Vec::new();
        egui::CentralPanel::default().show(eguiの本体, |ui| {
            応答一覧 = screen::画面(&self.状態).描画して集める(ui);
        });
        for 応答 in 応答一覧 {
            if let Err(不正) = self.状態.適用する(応答) {
                eprintln!("見本の画像を差し替えられない: {不正}");
            }
        }
    }
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
            let 候補 = 日本語フォントの候補::標準で入っている候補();
            if let Err(失敗) = 候補.最初に読めたものを設定する(&作成文脈.egui_ctx)
            {
                eprintln!("{失敗}。表示が崩れる場合は OS へ日本語フォントを導入する");
            }
            let eguiの本体 = &作成文脈.egui_ctx;
            let 見本の画素 = sample_texture::見本の画素のバイト列::ずらして作る(0);
            let 画像 = sengen_egui::差し替えられるテクスチャ::登録して作る(
                eguiの本体,
                "見本の画像",
                見本の画素.画素の並び()?,
                拡大縮小の仕方::画素をそのまま使う,
            );
            let 再生 = sample_player::見本の再生::登録して作る(eguiの本体)?;
            Ok(Box::new(見本帳アプリ {
                状態: state::状態::新規(画像, 再生),
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
