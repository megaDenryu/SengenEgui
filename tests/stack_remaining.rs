//! 横並びの残りの幅を渡す指定を確かめる。渡した子は後ろの子を置いた残りの幅いっぱいに広がり、並びの右端は使える幅の右端に揃う。
//! 子は左から右へ置く順に描くため、egui が操作できる部品を記録する順(Tab キーでフォーカスが移る順)も左から右になる。
//! 後ろの子の幅は同じ回の中で測るため、最初の回から使える幅を超えず、egui に描き直しを頼まない。

use sengen_egui::{
    ノード, ボタン, 一行テキスト入力, 子, 横並び, 画素, 縦積み
};

fn 名前の行(右の文字: &'static str) -> ノード<()> {
    横並び(子![
        ボタン("左", ()),
        一行テキスト入力("名前", |_| ()).幅いっぱい(),
        ボタン(右の文字, ()),
        ボタン("右の二つ目", ()),
    ])
    .残りの幅を渡す(1)
    .into()
}

/// 後ろの子の幅が違う行を2つ縦に並べる。
fn 二つの行() -> ノード<()> {
    縦積み(子![名前の行("短い"), 名前の行("とても長い右の一つ目")]).into()
}

/// 描いた結果。使える幅の右端、操作できる部品の矩形(egui が記録した順)、測るために見えない Ui へ描いた部品の数、egui の出力の組である。
struct 描いた結果 {
    右端: f32,
    部品: Vec<egui::Rect>,
    測るために描いた部品の数: usize,
    出力: egui::FullOutput,
}

/// 同じ egui の本体で1回描く。
fn 描く(eguiの本体: &egui::Context, 木: &dyn Fn() -> ノード<()>) -> 描いた結果 {
    let mut 右端 = 0.0;
    let 出力 = eguiの本体.run(egui::RawInput::default(), |eguiの本体| {
        egui::CentralPanel::default().show(eguiの本体, |ui| {
            右端 = ui.max_rect().max.x;
            let _ = 木().描画する(ui, &mut Vec::new());
        });
    });
    let 押せる部品: Vec<egui::WidgetRect> = eguiの本体.viewport(|ビューポート| {
        ビューポート
            .prev_pass
            .widgets
            .layers()
            .flat_map(|(_, 部品)| 部品.to_vec())
            .filter(|部品| 部品.sense.senses_click() || 部品.sense.senses_drag())
            .collect()
    });
    let 部品 = 押せる部品
        .iter()
        .filter(|部品| 部品.enabled)
        .map(|部品| 部品.rect)
        .collect();
    let 測るために描いた部品の数 = 押せる部品.iter().filter(|部品| !部品.enabled).count();
    描いた結果 {
        右端,
        部品,
        測るために描いた部品の数,
        出力,
    }
}

#[test]
fn 最初の回から渡した子は残りの幅いっぱいに広がり並びの右端は使える幅の右端に揃う() {
    let 描いた = 描く(&egui::Context::default(), &|| 名前の行("右の一つ目"));
    assert_eq!(描いた.部品.len(), 4, "{:?}", 描いた.部品);
    let 最後 = 描いた
        .部品
        .iter()
        .map(|矩形| 矩形.max.x)
        .fold(f32::MIN, f32::max);
    assert!(
        (最後 - 描いた.右端).abs() < 0.5,
        "並びの右端: {最後} 使える幅の右端: {}",
        描いた.右端
    );
}

#[test]
fn 操作できる部品は左から右の順に記録される() {
    let 描いた = 描く(&egui::Context::default(), &|| 名前の行("右の一つ目"));
    for 隣 in 描いた.部品.windows(2) {
        assert!(隣[0].max.x <= 隣[1].min.x, "{隣:?}");
    }
}

#[test]
fn 後ろの子の幅が違う行を並べてもどの行も使える幅に収まり描き直しを頼まない() {
    let eguiの本体 = egui::Context::default();
    for 回 in 0..2 {
        let 描いた = 描く(&eguiの本体, &二つの行);
        assert_eq!(描いた.部品.len(), 8, "{:?}", 描いた.部品);
        for 矩形 in &描いた.部品 {
            assert!(
                矩形.max.x <= 描いた.右端 + 0.5,
                "{回}回目 {矩形:?} 右端 {}",
                描いた.右端
            );
        }
        let 出力 = &描いた.出力.platform_output;
        assert!(
            出力.request_discard_reasons.is_empty(),
            "{回}回目: {:?}",
            出力.request_discard_reasons
        );
        assert_eq!(出力.num_completed_passes, 1, "{回}回目");
    }
}

#[test]
fn 右寄せの横並びでは残りの幅を渡さない() {
    let 木 = || -> ノード<()> {
        横並び(子![
            一行テキスト入力("名前", |_| ()).幅(画素(50.0)),
            ボタン("甲", ()),
        ])
        .右寄せ()
        .残りの幅を渡す(0)
        .into()
    };
    let 描いた = 描く(&egui::Context::default(), &木);
    assert_eq!(描いた.部品.len(), 2, "{:?}", 描いた.部品);
    assert_eq!(描いた.測るために描いた部品の数, 0);
}
