//! テーマが、指定した4つの色から残りの色を導いて egui の見た目の全体へ書くことと、
//! 導出の元が未指定の項目を egui の既定のまま残すことを確かめる。

use egui::Color32;
use sengen_egui::{テーマ, 明暗};

const 地の色: Color32 = Color32::from_rgb(22, 25, 31);
const 部品の面の色: Color32 = Color32::from_rgb(52, 60, 76);
const 文字色: Color32 = Color32::from_rgb(226, 230, 237);
const 強調色: Color32 = Color32::from_rgb(80, 150, 240);

fn 色だけのテーマ(基調: 明暗, 色: [Option<Color32>; 4]) -> テーマ {
    let [指定の強調色, 指定の地の色, 指定の文字色, 指定の部品の面の色] = 色;
    テーマ {
        強調色: 指定の強調色,
        地の色: 指定の地の色,
        文字色: 指定の文字色,
        部品の面の色: 指定の部品の面の色,
        ..テーマ::基調から作る(基調)
    }
}

fn 適用した見た目(テーマ: テーマ) -> egui::Visuals {
    let 文脈 = egui::Context::default();
    テーマ.適用する(&文脈);
    文脈.style().visuals.clone()
}

/// 色の各成分が甲と乙の間にあり、しかも両端のどちらとも違うか。
fn 両端を除いて間にあるか(色: Color32, 甲: Color32, 乙: Color32) -> bool {
    let 成分ごとに間にある = 色
        .to_array()
        .into_iter()
        .zip(甲.to_array().into_iter().zip(乙.to_array()))
        .all(|(値, (左, 右))| 左.min(右) <= 値 && 値 <= 左.max(右));
    成分ごとに間にある && 色 != 甲 && 色 != 乙
}

#[test]
fn 四つの色を指定すると残りの色が指定の色どうしの間から導かれる() {
    let 全部の色 = [Some(強調色), Some(地の色), Some(文字色), Some(部品の面の色)];
    let 見た目 = 適用した見た目(色だけのテーマ(明暗::濃色, 全部の色));
    let 既定 = egui::Visuals::dark();
    let 部品 = &見た目.widgets;

    assert_ne!(部品.hovered.bg_fill, 既定.widgets.hovered.bg_fill);
    assert!(両端を除いて間にあるか(
        部品.hovered.bg_fill,
        部品の面の色,
        文字色
    ));
    assert_ne!(部品.active.bg_fill, 既定.widgets.active.bg_fill);
    assert!(両端を除いて間にあるか(
        部品.active.bg_fill,
        部品の面の色,
        文字色
    ));
    assert_ne!(見た目.extreme_bg_color, 既定.extreme_bg_color);
    assert!(両端を除いて間にあるか(
        見た目.extreme_bg_color,
        地の色,
        Color32::BLACK
    ));
    assert_eq!(見た目.text_edit_bg_color, Some(見た目.extreme_bg_color));
    let 線 = 部品.noninteractive.bg_stroke.color;
    assert_ne!(線, 既定.widgets.noninteractive.bg_stroke.color);
    assert!(両端を除いて間にあるか(線, 地の色, 文字色));
    assert_eq!(見た目.window_stroke.color, 線);
    assert!(両端を除いて間にあるか(
        見た目.selection.stroke.color,
        強調色,
        文字色
    ));
    assert_eq!(見た目.code_bg_color, 部品の面の色);
}

#[test]
fn 導出の元が未指定の項目はeguiの既定のまま残る() {
    let 文字色だけ = [None, None, Some(文字色), None];
    let 見た目 = 適用した見た目(色だけのテーマ(明暗::濃色, 文字色だけ));
    let 既定 = egui::Visuals::dark();

    assert_eq!(見た目.widgets.hovered.bg_fill, 既定.widgets.hovered.bg_fill);
    assert_eq!(見た目.widgets.active.bg_fill, 既定.widgets.active.bg_fill);
    assert_eq!(見た目.extreme_bg_color, 既定.extreme_bg_color);
    assert_eq!(見た目.text_edit_bg_color, 既定.text_edit_bg_color);
    assert_eq!(見た目.window_stroke, 既定.window_stroke);
    assert_eq!(見た目.selection.stroke, 既定.selection.stroke);
    assert_eq!(見た目.widgets.inactive.fg_stroke.color, 文字色);
}

#[test]
fn 淡色の基調では窪んだ面を白の方へ寄せ乗せた文字を黒の方へ寄せる() {
    let 淡い地 = Color32::from_rgb(236, 238, 242);
    let 暗い文字 = Color32::from_rgb(40, 44, 52);
    let 色 = [Some(強調色), Some(淡い地), Some(暗い文字), Some(淡い地)];
    let 見た目 = 適用した見た目(色だけのテーマ(明暗::淡色, 色));

    assert!(両端を除いて間にあるか(
        見た目.extreme_bg_color,
        淡い地,
        Color32::WHITE
    ));
    let 乗せた文字 = 見た目.widgets.hovered.fg_stroke.color;
    assert!(両端を除いて間にあるか(
        乗せた文字,
        暗い文字,
        Color32::BLACK
    ));
}
