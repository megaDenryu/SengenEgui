//! テーマ。画面全体の見た目（明暗・色・表示倍率・部品の間隔・ボタンの内余白・部品の角丸）を
//! 1箇所で決める。適用しなければ egui の既定（OSの明暗設定への追従）のまま動き、
//! 適用すると明暗は基調へ固定され、OSの明暗設定に追従しなくなる。
//! 色は4つ（地の色・部品の面の色・文字色・強調色）だけを指定させ、egui の見た目が要る残りの色
//! （マウスを乗せた面・押した面・線・入力欄の窪んだ面・格子の縞・選択の縁・文字カーソル等）は
//! 指定された色どうしを混ぜて導く。指定の色と egui の既定の無彩色が画面の中で混ざらないためである。
//! 導出の元になる色が未指定の項目は egui の既定色のまま残す。導出の規則は palette.rs にある。

mod assign;
mod contrast;
mod mix;
mod on_accent;
mod palette;
mod tone;

use crate::measure::{拡大率, 縦横の論理画素, 角丸の画素};
pub(crate) use on_accent::強調色まわりの色;
use palette::配色;
pub use tone::明暗;

/// テーマとは、画面全体へ一括で適用する見た目の指定のことである。
/// ノード単位の装飾はスタイルが担い、テーマは全体の基調だけを担う。
#[derive(Clone, Copy)]
pub struct テーマ {
    /// 画面全体の明暗の基調。
    pub 基調: 明暗,
    /// 選択・リンク等の強調に使う色。選択の縁と文字カーソルの色もここから導く。
    pub 強調色: Option<egui::Color32>,
    /// 画面と浮きウィンドウの背景の色。線・入力欄の窪んだ面・格子の縞もここから導く。
    pub 地の色: Option<egui::Color32>,
    /// 装飾で色を指定していない文字と部品の文字の色。他の色を明るくする（淡色では暗くする）方向でもある。
    pub 文字色: Option<egui::Color32>,
    /// ボタン等の触っていないときの面の色。マウスを乗せた面と押した面もここから導く。
    pub 部品の面の色: Option<egui::Color32>,
    /// 強調色の地の上に載せる文字の色。選択ボタンと選択欄の一覧の選ばれた項目の文字に使う。未指定なら、地の色と文字色のうち
    /// 強調色と見分けやすい方を使う（強調色・地の色・文字色のどれかが未指定なら egui の既定の選択の縁の色のまま）。
    pub 強調色の上の文字色: Option<egui::Color32>,
    /// 画面全体の拡大率。未指定なら等倍。
    pub 表示倍率: Option<拡大率>,
    /// 隣り合う部品の間に空ける横と縦の距離。
    pub 部品の間隔: Option<縦横の論理画素>,
    /// ボタンの文字と縁の間の横と縦の余白。
    pub ボタンの内余白: Option<縦横の論理画素>,
    /// ボタン等の部品の角の丸み。
    pub 部品の角丸: Option<角丸の画素>,
}

impl テーマ {
    /// 基調だけを決め、ほかの項目を全部未指定にしたテーマ。構造体の式で `..テーマ::基調から作る(明暗::濃色)` の形の基底に使い、
    /// 書かなかった項目を未指定にする。テーマに項目が増えても、この形で書いた利用する側はそのままコンパイルできる。
    /// 基調には既定の値を置かない。明暗は画面全体を決めるため、利用する側が必ず選ぶ。
    pub const fn 基調から作る(基調: 明暗) -> テーマ {
        テーマ {
            基調,
            強調色: None,
            地の色: None,
            文字色: None,
            部品の面の色: None,
            強調色の上の文字色: None,
            表示倍率: None,
            部品の間隔: None,
            ボタンの内余白: None,
            部品の角丸: None,
        }
    }

    /// テーマを egui の文脈へ適用する。起動時に1回呼ぶ。
    /// egui の明暗の選択を基調へ固定し、OSの明暗設定に追従させない。
    /// 指定された色から導いた色も含めて、固定した側の見た目の全体へ書く。
    pub fn 適用する(&self, 文脈: &egui::Context) {
        let eguiの明暗 = self.基調.eguiの明暗へ変換する();
        let mut 見た目 = match self.基調 {
            明暗::濃色 => egui::Visuals::dark(),
            明暗::淡色 => egui::Visuals::light(),
        };
        let 色の組 = self.配色();
        色の組.見た目へ割り当てる(&mut 見た目);
        色の組.強調色まわりの色().置く(文脈);
        self.角丸を見た目へ反映する(&mut 見た目);
        // 注意: set_visuals はその時点の明暗の側にだけ書く。起動時はOSの明暗が未着で濃色側になり、
        // 最初の描画でOSが淡色なら書いていない淡色側へ切り替わるため、明暗を固定してから明示の側へ書く。
        文脈.set_theme(eguiの明暗);
        文脈.set_visuals_of(eguiの明暗, 見た目);
        文脈.style_mut_of(eguiの明暗, |様式| {
            self.間隔を様式へ反映する(様式)
        });
        if let Some(倍率) = self.表示倍率 {
            文脈.set_zoom_factor(倍率.eguiへ渡す値());
        }
    }

    fn 配色(&self) -> 配色 {
        配色 {
            基調: self.基調,
            強調色: self.強調色,
            地の色: self.地の色,
            文字色: self.文字色,
            部品の面の色: self.部品の面の色,
            強調色の上の文字色: self.強調色の上の文字色,
        }
    }

    fn 角丸を見た目へ反映する(&self, 見た目: &mut egui::Visuals) {
        let Some(丸み) = self.部品の角丸 else {
            return;
        };
        let 丸み = egui::CornerRadius::same(丸み.eguiへ渡す値());
        for 状態 in [
            &mut 見た目.widgets.noninteractive,
            &mut 見た目.widgets.inactive,
            &mut 見た目.widgets.hovered,
            &mut 見た目.widgets.active,
            &mut 見た目.widgets.open,
        ] {
            状態.corner_radius = 丸み;
        }
    }

    fn 間隔を様式へ反映する(&self, 様式: &mut egui::Style) {
        if let Some(間隔) = self.部品の間隔 {
            様式.spacing.item_spacing = 間隔.eguiへ渡す値();
        }
        if let Some(余白) = self.ボタンの内余白 {
            様式.spacing.button_padding = 余白.eguiへ渡す値();
        }
    }
}
