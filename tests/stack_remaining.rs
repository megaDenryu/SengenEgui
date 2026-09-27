//! 横並びの残りの幅を渡す指定を確かめる。渡した子は後ろの子を置いた残りの幅いっぱいに広がり、並びの右端は使える幅の右端に揃う。
//! 子は左から右へ置く順に描くため、egui が部品を記録する順(Tab キーでフォーカスが移る順)も左から右になる。

use sengen_egui::{ノード, ボタン, 一行テキスト入力, 子, 横並び};

fn 名前の行() -> ノード<()> {
    横並び(子![
        ボタン("左", ()),
        一行テキスト入力("名前", |_| ()).幅いっぱい(),
        ボタン("右の一つ目", ()),
        ボタン("右の二つ目", ()),
    ])
    .残りの幅を渡す(1)
    .into()
}

/// 1回描き、使える幅の右端と、押せる部品の矩形を egui が記録した順に返す。
fn 描いて測る() -> (f32, Vec<egui::Rect>) {
    let eguiの本体 = egui::Context::default();
    let mut 右端 = 0.0;
    let _ = eguiの本体.run(egui::RawInput::default(), |eguiの本体| {
        egui::CentralPanel::default().show(eguiの本体, |ui| {
            右端 = ui.max_rect().max.x;
            let _ = 名前の行().描画する(ui, &mut Vec::new());
        });
    });
    let 部品 = eguiの本体.viewport(|ビューポート| {
        ビューポート
            .prev_pass
            .widgets
            .layers()
            .flat_map(|(_, 部品)| 部品.to_vec())
            .filter(|部品| 部品.sense.senses_click() || 部品.sense.senses_drag())
            .map(|部品| 部品.rect)
            .collect()
    });
    (右端, 部品)
}

#[test]
fn 渡した子は残りの幅いっぱいに広がり並びの右端は使える幅の右端に揃う() {
    let (右端, 部品) = 描いて測る();
    assert_eq!(部品.len(), 4, "{部品:?}");
    let 最後 = 部品.iter().map(|矩形| 矩形.max.x).fold(f32::MIN, f32::max);
    assert!(
        (最後 - 右端).abs() < 0.5,
        "並びの右端: {最後} 使える幅の右端: {右端}"
    );
}

#[test]
fn 部品は左から右の順に記録される() {
    let (_, 部品) = 描いて測る();
    for 隣 in 部品.windows(2) {
        assert!(隣[0].max.x <= 隣[1].min.x, "{隣:?}");
    }
}
