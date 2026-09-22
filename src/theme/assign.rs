//! 配色の各色を egui の見た目のどの項目へ書くかの対応表。色をどう導くかは palette.rs が担い、
//! ここは導いた色の置き場だけを担う。None の色は書かず、その項目は egui の既定のまま残る。

use super::palette::配色;
use egui::Color32;

fn 指定があれば書く(項目: &mut Color32, 色: Option<Color32>) {
    if let Some(色) = 色 {
        *項目 = 色;
    }
}

impl 配色 {
    /// 指定された色と導いた色を、egui の見た目の全体へ書く。
    pub(super) fn 見た目へ割り当てる(&self, 見た目: &mut egui::Visuals) {
        self.指定された色を書く(見た目);
        self.部品の状態ごとの色を書く(&mut 見た目.widgets);
        let 線 = self.線の色();
        指定があれば書く(&mut 見た目.window_stroke.color, 線);
        指定があれば書く(&mut 見た目.extreme_bg_color, self.窪んだ面());
        if let Some(色) = self.窪んだ面() {
            見た目.text_edit_bg_color = Some(色);
        }
        指定があれば書く(&mut 見た目.faint_bg_color, self.格子の縞());
        指定があれば書く(&mut 見た目.code_bg_color, self.部品の面の色);
        指定があれば書く(&mut 見た目.selection.stroke.color, self.選択の縁());
        指定があれば書く(&mut 見た目.text_cursor.stroke.color, self.選択の縁());
    }

    fn 指定された色を書く(&self, 見た目: &mut egui::Visuals) {
        指定があれば書く(&mut 見た目.selection.bg_fill, self.強調色);
        指定があれば書く(&mut 見た目.hyperlink_color, self.強調色);
        指定があれば書く(&mut 見た目.panel_fill, self.地の色);
        指定があれば書く(&mut 見た目.window_fill, self.地の色);
    }

    fn 部品の状態ごとの色を書く(&self, 部品: &mut egui::style::Widgets) {
        let 静止 = &mut 部品.noninteractive;
        指定があれば書く(&mut 静止.bg_fill, self.地の色);
        指定があれば書く(&mut 静止.weak_bg_fill, self.地の色);
        指定があれば書く(&mut 静止.bg_stroke.color, self.線の色());
        指定があれば書く(&mut 静止.fg_stroke.color, self.文字色);

        let 触っていない = &mut 部品.inactive;
        指定があれば書く(&mut 触っていない.bg_fill, self.部品の面の色);
        指定があれば書く(&mut 触っていない.weak_bg_fill, self.部品の面の色);
        指定があれば書く(&mut 触っていない.fg_stroke.color, self.文字色);

        let 乗せた = &mut 部品.hovered;
        指定があれば書く(&mut 乗せた.bg_fill, self.マウスを乗せた面());
        指定があれば書く(&mut 乗せた.weak_bg_fill, self.マウスを乗せた面());
        指定があれば書く(&mut 乗せた.bg_stroke.color, self.マウスを乗せた枠());
        指定があれば書く(&mut 乗せた.fg_stroke.color, self.マウスを乗せた文字());

        let 押した = &mut 部品.active;
        指定があれば書く(&mut 押した.bg_fill, self.押した面());
        指定があれば書く(&mut 押した.weak_bg_fill, self.押した面());
        指定があれば書く(&mut 押した.bg_stroke.color, self.押した文字());
        指定があれば書く(&mut 押した.fg_stroke.color, self.押した文字());

        let 開いた = &mut 部品.open;
        指定があれば書く(&mut 開いた.bg_fill, self.地の色);
        指定があれば書く(&mut 開いた.weak_bg_fill, self.マウスを乗せた面());
        指定があれば書く(&mut 開いた.bg_stroke.color, self.線の色());
        指定があれば書く(&mut 開いた.fg_stroke.color, self.文字色);
    }
}
