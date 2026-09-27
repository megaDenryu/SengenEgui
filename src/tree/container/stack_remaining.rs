//! 横並びの残りの幅の配分。指定の子に、その後ろの子を置いた残りの幅を渡し、子は左から右へ置く順に描く。
//! 後ろの子の幅は描くまで分からないため、前の回に測った幅を egui の一時記憶に置いて使う。測った幅が変わった回は、
//! egui にその回の描画を捨てさせて描き直すため、利用者に見える回で幅が食い違うことは無い。
//! 右寄せの横並び(右から左へ詰める)で残りを渡す形と違い、描く順が見えている並びと同じなので、Tab キーでフォーカスが移る順も
//! 左から右になる。

use crate::tree::ノード;

/// 後ろの子の幅がこれより変わったら描き直す(小数の誤差で描き直し続けないため)。
const 描き直す幅の差: f32 = 0.5;

/// 子を左から右へ描き、番号の子(0から数える)には、後ろの子を置いた残りの幅を渡す。
pub(super) fn 残りの幅を渡して描く<M: Clone>(
    子一覧: &[ノード<M>],
    渡す子の番号: usize,
    ui: &mut egui::Ui,
    発行した応答: &mut Vec<M>,
) {
    let 記憶の鍵 = ui.id().with("後ろの子の幅");
    let 前の回の幅 = ui
        .ctx()
        .data(|記憶| 記憶.get_temp::<f32>(記憶の鍵))
        .unwrap_or(0.0);
    let mut 渡した子の右端 = None;
    for (番号, 子) in 子一覧.iter().enumerate() {
        if 番号 != 渡す子の番号 {
            子.描画する(ui, 発行した応答);
            continue;
        }
        let 大きさ = egui::vec2(
            (ui.available_size_before_wrap().x - 前の回の幅).max(0.0),
            ui.spacing().interact_size.y,
        );
        let 配置 = *ui.layout();
        let 反応 = ui.allocate_ui_with_layout(大きさ, 配置, |内側| {
            子.描画する(内側, 発行した応答)
        });
        渡した子の右端 = Some(反応.response.rect.max.x);
    }
    let Some(右端) = 渡した子の右端 else {
        return;
    };
    let 測った幅 = ui.min_rect().max.x - 右端;
    if (測った幅 - 前の回の幅).abs() > 描き直す幅の差 {
        ui.ctx()
            .data_mut(|記憶| 記憶.insert_temp(記憶の鍵, 測った幅));
        ui.ctx().request_discard("横並びの後ろの子の幅が変わった");
    }
}
