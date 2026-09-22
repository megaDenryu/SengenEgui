//! 積みの向きと寄せから egui の配置を決める。積み型の描画が使う純粋な対応表である。

/// 積む向きとは、縦積み・横並び・折り返す横並びの区別のことである。
#[derive(Clone, Copy)]
pub enum 積む向き {
    /// 上から下へ積む。
    縦,
    /// 左から右へ並べる。幅が足りなくても折り返さない。
    横,
    /// 左から右へ並べ、幅が足りなければ次の行へ折り返す。
    折り返す横,
}

/// 寄せとは、並びの中で子をどこへ寄せるかの区別のことである。
#[derive(Clone, Copy)]
pub enum 寄せ {
    /// egui の既定（縦積みは左、横並びは上）。
    既定,
    /// 縦積みでは左右中央、横並びでは上下中央。
    中央,
    /// 横並びで右から左へ詰める。
    右,
}

impl 積む向き {
    /// 向きと寄せから egui の配置を作る。
    pub(super) fn 配置を決める(self, 寄せ: 寄せ) -> egui::Layout {
        match (self, 寄せ) {
            (Self::縦, 寄せ::既定) => egui::Layout::top_down(egui::Align::Min),
            (Self::縦, 寄せ::中央) => egui::Layout::top_down(egui::Align::Center),
            (Self::縦, 寄せ::右) => egui::Layout::top_down(egui::Align::Max),
            (Self::横 | Self::折り返す横, 寄せ::既定) => {
                egui::Layout::left_to_right(egui::Align::Min).with_main_wrap(self.折り返すか())
            }
            (Self::横 | Self::折り返す横, 寄せ::中央) => {
                egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(self.折り返すか())
            }
            (Self::横 | Self::折り返す横, 寄せ::右) => {
                egui::Layout::right_to_left(egui::Align::Center).with_main_wrap(self.折り返すか())
            }
        }
    }

    /// 横の並びで最初に確保する行の大きさ。egui の `horizontal` と同じく、幅は使える幅、
    /// 高さは操作部品の高さとする。縦積みは確保せず、親の配置に従う（None）。
    pub(super) fn 行の初期の大きさ(self, ui: &egui::Ui) -> Option<egui::Vec2> {
        match self {
            Self::縦 => None,
            Self::横 | Self::折り返す横 => Some(egui::vec2(
                ui.available_size_before_wrap().x,
                ui.spacing().interact_size.y,
            )),
        }
    }

    fn 折り返すか(self) -> bool {
        matches!(self, Self::折り返す横)
    }
}
