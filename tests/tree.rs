//! 公開APIだけで、木の構築・スタイル合成・描画と応答収集を確かめる。
//! クリック操作の再現はeguiの入力合成が要るため対象外とする。

use sengen_egui::{スタイル, ノード, ボタン, 子, 文章, 条件, 縦積み};

#[derive(Clone, PartialEq, Debug)]
enum 応答 {
    加算,
}

#[test]
fn スタイルは上書き側を優先して重なる() {
    let 基本 = スタイル {
        文字サイズ: Some(12.0),
        太字: Some(true),
        ..スタイル::無指定
    };
    let 合成 = 基本.重ねる(スタイル {
        文字サイズ: Some(16.0),
        ..スタイル::無指定
    });
    assert_eq!(合成.文字サイズ, Some(16.0));
    assert_eq!(合成.太字, Some(true));
}

#[test]
fn 条件は不成立なら無しになる() {
    let 節: ノード<応答> = 条件(false, || 文章("出ない"));
    assert!(matches!(節, ノード::無し));
    let 節: ノード<応答> = 条件(true, || 文章("出る"));
    assert!(matches!(節, ノード::文章(_)));
}

#[test]
fn 描画できて操作が無ければ応答は空になる() {
    let 文脈 = egui::Context::default();
    let _ = 文脈.run(egui::RawInput::default(), |文脈| {
        egui::CentralPanel::default().show(文脈, |ui| {
            let 木: ノード<応答> = 縦積み(子![
                文章("見出し"),
                ボタン("実行", 応答::加算),
                条件(true, || 文章("展開中")),
            ])
            .into();
            assert!(木.描画して集める(ui).is_empty());
        });
    });
}
