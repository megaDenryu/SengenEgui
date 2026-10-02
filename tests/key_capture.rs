//! 次に押されたキーの組の受け取りを確かめる。押していた修飾キーを含む組を1つだけ受け取り、Ctrl を Command にそろえること、
//! 受け取った押下を後に描画する部品へ渡さないこと(Escape も含む)、繰り返しの押下を受け取らないこと、応答を写せることを確かめる。

mod common;

use sengen_egui::{
    キー, キーの組, キー操作, ノード, 修飾キー, 子, 次に押されたキーの組を受け取る, 縦積み,
};

#[derive(Clone, PartialEq, Debug)]
enum 応答 {
    受け取った(キーの組),
    キー操作が発した,
    包んだ(Box<応答>),
}

fn 受け取りだけの木() -> ノード<応答> {
    次に押されたキーの組を受け取る(応答::受け取った).into()
}

/// 受け取りを先に、同じキーのキー操作を後に描画する木。
fn 受け取りとエスケープのキー操作の木() -> ノード<応答> {
    縦積み(子![
        次に押されたキーの組を受け取る(応答::受け取った),
        キー操作(キーの組::単独(キー::Escape), 応答::キー操作が発した),
    ])
    .into()
}

#[test]
fn 押していた修飾キーを含む組を受け取り_コントロールをコマンドにそろえる() {
    let eguiの本体 = egui::Context::default();
    let windowsのctrlとshift = egui::Modifiers {
        ctrl: true,
        command: true,
        shift: true,
        ..egui::Modifiers::NONE
    };
    let 集まり = common::描画する(
        &eguiの本体,
        vec![common::キー押下(egui::Key::Z, windowsのctrlとshift)],
        &受け取りだけの木,
    );
    let コマンドとシフト = 修飾キー {
        shift: true,
        ..修飾キー::COMMAND
    };
    assert_eq!(
        集まり,
        vec![応答::受け取った(キーの組::生成する(
            コマンドとシフト,
            キー::Z
        ))]
    );
}

#[test]
fn 同じフレームの押下は最初の1つだけを受け取る() {
    let eguiの本体 = egui::Context::default();
    let 集まり = common::描画する(
        &eguiの本体,
        vec![
            common::キー押下(egui::Key::A, egui::Modifiers::NONE),
            common::キー押下(egui::Key::B, egui::Modifiers::NONE),
        ],
        &受け取りだけの木,
    );
    assert_eq!(集まり, vec![応答::受け取った(キーの組::単独(キー::A))]);
}

#[test]
fn 受け取った押下は後に描画するキー操作へ渡さずエスケープも受け取る() {
    let eguiの本体 = egui::Context::default();
    let 集まり = common::描画する(
        &eguiの本体,
        vec![common::キー押下(
            egui::Key::Escape,
            egui::Modifiers::NONE,
        )],
        &受け取りとエスケープのキー操作の木,
    );
    assert_eq!(集まり, vec![応答::受け取った(キーの組::単独(キー::Escape))]);
}

#[test]
fn 押し続けによる繰り返しの押下は受け取らない() {
    let eguiの本体 = egui::Context::default();
    let 押下 = || vec![common::キー押下(egui::Key::A, egui::Modifiers::NONE)];
    assert_eq!(
        common::描画する(&eguiの本体, 押下(), &受け取りだけの木),
        vec![応答::受け取った(キーの組::単独(キー::A))]
    );
    // 離さずにもう一度押した事象は、egui が繰り返しの印を付ける。
    assert!(common::描画する(&eguiの本体, 押下(), &受け取りだけの木).is_empty());
}

#[test]
fn 写した受け取りは写した応答を発する() {
    let eguiの本体 = egui::Context::default();
    let 写した木 = || 受け取りだけの木().写す(|応答| 応答::包んだ(Box::new(応答)));
    let 集まり = common::描画する(
        &eguiの本体,
        vec![common::キー押下(egui::Key::T, egui::Modifiers::NONE)],
        &写した木,
    );
    assert_eq!(
        集まり,
        vec![応答::包んだ(Box::new(応答::受け取った(
            キーの組::単独(キー::T)
        )))]
    );
}
