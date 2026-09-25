//! スライダーの軌道の幅の決め方。既定・指定の幅・使える幅いっぱいの3つを区別し、egui へ渡す幅を求める。

use crate::measure::論理画素;

/// 軌道の幅とは、スライダーのつまみが動く軌道の幅をどう決めるかの区別のことである。
#[derive(Clone, Copy)]
pub(super) enum 軌道の幅 {
    /// egui の既定の幅。
    既定,
    /// 指定の幅。
    指定(論理画素),
    /// 使える幅から、数値の欄と添える文字の分を差し引いた残りの全部。
    使える幅いっぱい,
}

/// 軌道の横に並ぶもの。使える幅いっぱいのときに差し引く幅を求めるために使う。
pub(super) struct 軌道の横に並ぶもの<'文字> {
    pub(super) 数値を表示する: bool,
    pub(super) 添える文字: Option<&'文字 str>,
}

impl 軌道の幅 {
    /// egui の間隔の設定へ軌道の幅を書く。既定なら何も書かない。
    pub(super) fn 間隔の設定へ書く(
        self,
        ui: &mut egui::Ui,
        横に並ぶもの: 軌道の横に並ぶもの<'_>,
    ) {
        let 幅 = match self {
            Self::既定 => return,
            Self::指定(幅) => 幅.eguiへ渡す値(),
            Self::使える幅いっぱい => ui.available_width() - 横に並ぶもの.占める幅(ui),
        };
        ui.spacing_mut().slider_width = 幅.max(0.0);
    }
}

impl 軌道の横に並ぶもの<'_> {
    /// 数値の欄は egui の操作部品の標準の幅で、添える文字は描いたときの幅で見積もり、それぞれ部品の間隔を足す。
    fn 占める幅(&self, ui: &egui::Ui) -> f32 {
        let 間隔 = ui.spacing().item_spacing.x;
        let 数値の欄 = if self.数値を表示する {
            ui.spacing().interact_size.x + 間隔
        } else {
            0.0
        };
        let 文字 = self.添える文字.map_or(0.0, |文字| {
            let 描いた文字 = egui::WidgetText::from(文字).into_galley(
                ui,
                Some(egui::TextWrapMode::Extend),
                f32::INFINITY,
                egui::TextStyle::Button,
            );
            描いた文字.size().x + 間隔
        });
        数値の欄 + 文字
    }
}
