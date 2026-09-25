//! 利用する側が自分の日本語フォントの字形の縦の補正量を探す道具。通常の試験では走らせない。
//! 使い方（SengenEgui の複製のルートで実行する）:
//! `SENGEN_EGUI_FONT_TO_MEASURE=<フォントの置き場所> cargo test --test font_correction_search -- --ignored --nocapture`
//! 補正の比を -0.10 から 0.40 まで 0.01 刻みで試し、最大の差が最も小さい比を表示する。測り方は `src/font/correction.rs` の説明と同じである。
//! 表示された比を `字形の縦の補正::字の大きさに対する比から作る` へ渡し、`日本語フォントの候補::補正付きの一覧から作る` で使う。

mod common;

use common::font_measure::{最大の差, 補正量を探す};
use sengen_egui::字形の縦の補正;

/// 利用する側が自分のフォントの補正量を探す道具。通常の試験では走らせない。
#[test]
#[ignore = "環境変数 SENGEN_EGUI_FONT_TO_MEASURE にフォントの置き場所を入れて --ignored で走らせる道具"]
fn 補正量を探す道具() {
    let Some(置き場所) = std::env::var_os("SENGEN_EGUI_FONT_TO_MEASURE") else {
        panic!("環境変数 SENGEN_EGUI_FONT_TO_MEASURE にフォントの置き場所を入れる");
    };
    let 置き場所 = std::path::PathBuf::from(置き場所);
    let (補正, 差) = 補正量を探す(&置き場所);
    let 補正しない差 = 最大の差(&置き場所, 字形の縦の補正::無し);
    println!(
        "{}: 補正の比 {} で最大の差 {差} 論理画素（補正しないと {補正しない差}）",
        置き場所.display(),
        補正.字の大きさに対する比()
    );
}
