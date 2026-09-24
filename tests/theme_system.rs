//! テーマの基調が、最初の描画でOSの明暗設定が届いた後も保たれることを確かめる。
//! 起動時はOSの明暗が未着のため、書く側を明示しないと、OSの設定の側へ切り替わった時点で
//! テーマを書いていない既定の見た目が出る（OSが淡色の環境で濃色のテーマが消えた不具合）。

use egui::Color32;
use sengen_egui::{テーマ, 明暗};

const 地の色: Color32 = Color32::from_rgb(22, 25, 31);
const 文字色: Color32 = Color32::from_rgb(226, 230, 237);

fn 地と文字だけのテーマ(基調: 明暗) -> テーマ {
    テーマ {
        基調,
        強調色: None,
        地の色: Some(地の色),
        文字色: Some(文字色),
        部品の面の色: None,
        表示倍率: None,
        部品の間隔: None,
        ボタンの内余白: None,
        部品の角丸: None,
    }
}

fn 動作環境の明暗が届いた後の見た目(
    テーマ: テーマ,
    動作環境の明暗: egui::Theme,
) -> egui::Visuals {
    let 文脈 = egui::Context::default();
    テーマ.適用する(&文脈);
    let 入力 = egui::RawInput {
        system_theme: Some(動作環境の明暗),
        ..Default::default()
    };
    let _ = 文脈.run(入力, |_| {});
    文脈.style().visuals.clone()
}

#[test]
fn 動作環境が淡色でも濃色のテーマの色が出る() {
    let 見た目 = 動作環境の明暗が届いた後の見た目(
        地と文字だけのテーマ(明暗::濃色),
        egui::Theme::Light,
    );
    assert!(見た目.dark_mode);
    assert_eq!(見た目.panel_fill, 地の色);
    assert_eq!(見た目.widgets.noninteractive.fg_stroke.color, 文字色);
}

#[test]
fn 動作環境が濃色でも淡色のテーマの基調が保たれる() {
    let 見た目 = 動作環境の明暗が届いた後の見た目(
        地と文字だけのテーマ(明暗::淡色),
        egui::Theme::Dark,
    );
    assert!(!見た目.dark_mode);
    assert_eq!(見た目.panel_fill, 地の色);
}
