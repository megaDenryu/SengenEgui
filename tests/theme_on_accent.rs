//! テーマの強調色の上の文字色が、選択ボタン・選択欄の一覧・タブ列の選ばれた項目の文字にだけ当たることを、描いた文字の書式の色で確かめる。

mod common;

use common::accent::{
    地の色, 指定の文字色, 文字の色, 文字の色一覧, 琥珀のテーマ
};
use egui::Color32;
use sengen_egui::{タブ列, ノード, 子, 横並び, 選択ボタン, 選択欄, 選択肢};

fn 選択ボタンの木() -> ノード<()> {
    横並び(子![
        選択ボタン("甲", true, ()),
        選択ボタン("乙", false, ())
    ])
    .into()
}

#[test]
fn 選ばれた選択ボタンの文字だけが指定した強調色の上の文字色で描かれる() {
    let eguiの本体 = egui::Context::default();
    琥珀のテーマ(Some(指定の文字色)).適用する(&eguiの本体);
    assert_eq!(
        文字の色(&eguiの本体, "甲", &選択ボタンの木),
        Some(指定の文字色)
    );
    assert_eq!(
        文字の色(&eguiの本体, "乙", &選択ボタンの木),
        Some(Color32::PLACEHOLDER)
    );
}

#[test]
fn 強調色の上の文字色が未指定なら地の色と文字色のうち強調色と見分けやすい方を使う() {
    let eguiの本体 = egui::Context::default();
    琥珀のテーマ(None).適用する(&eguiの本体);
    assert_eq!(文字の色(&eguiの本体, "甲", &選択ボタンの木), Some(地の色));
}

#[test]
fn テーマを適用しなければ選ばれた項目の文字は既定の色のまま描かれる() {
    let eguiの本体 = egui::Context::default();
    assert_eq!(
        文字の色(&eguiの本体, "甲", &選択ボタンの木),
        Some(Color32::PLACEHOLDER)
    );
}

#[test]
fn 選択欄の一覧の選ばれた項目も強調色の上の文字色で描かれる() {
    let eguiの本体 = egui::Context::default();
    琥珀のテーマ(Some(指定の文字色)).適用する(&eguiの本体);
    let 木を組む = || -> ノード<()> {
        選択欄("欄", "乙", vec![選択肢("甲", ()), 選択肢("乙", ())]).into()
    };
    let _ = common::左クリックする(&eguiの本体, egui::pos2(20.0, 18.0), &木を組む);
    let [甲の色一覧, 乙の色一覧] =
        ["甲", "乙"].map(|文字列| 文字の色一覧(&eguiの本体, 文字列, &木を組む));
    assert_eq!(甲の色一覧, vec![Color32::PLACEHOLDER]);
    assert!(乙の色一覧.contains(&指定の文字色), "{乙の色一覧:?}");
}

#[test]
fn タブ列の選ばれた見出しも強調色の上の文字色で描かれる() {
    let eguiの本体 = egui::Context::default();
    琥珀のテーマ(Some(指定の文字色)).適用する(&eguiの本体);
    let 木を組む = || -> ノード<()> {
        タブ列(vec!["甲".to_string(), "乙".to_string()], 1, |_| ()).into()
    };
    assert_eq!(文字の色(&eguiの本体, "乙", &木を組む), Some(指定の文字色));
    assert_eq!(
        文字の色(&eguiの本体, "甲", &木を組む),
        Some(Color32::PLACEHOLDER)
    );
}
