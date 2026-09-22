//! 配色。テーマで指定された4つの色（地の色・部品の面の色・文字色・強調色）を保持し、
//! egui の見た目が要る残りの色を、指定された色どうし（と明暗の極の白黒）を混ぜて導く。
//! 導出の元になる色が1つでも未指定なら、その導出は None を返し、呼び出し側は egui の既定を残す。
//! 混ぜる方向は明暗で切り替える。濃色では文字を白へ、窪みを黒へ寄せ、淡色ではその逆へ寄せる。

use super::mix::二色を混ぜる;
use super::明暗;
use crate::measure::割合;
use egui::Color32;

const マウスを乗せた面へ寄せる割合: 割合 = 割合::範囲へ丸めて生成する(0.15);
const 押した面へ寄せる割合: 割合 = 割合::範囲へ丸めて生成する(0.25);
const マウスを乗せた文字へ寄せる割合: 割合 = 割合::範囲へ丸めて生成する(0.5);
const 押した文字へ寄せる割合: 割合 = 割合::範囲へ丸めて生成する(0.8);
const 線へ寄せる割合: 割合 = 割合::範囲へ丸めて生成する(0.18);
const マウスを乗せた枠へ寄せる割合: 割合 = 割合::範囲へ丸めて生成する(0.45);
const 窪んだ面へ寄せる割合: 割合 = 割合::範囲へ丸めて生成する(0.35);
const 格子の縞へ寄せる割合: 割合 = 割合::範囲へ丸めて生成する(0.04);
const 選択の縁へ寄せる割合: 割合 = 割合::範囲へ丸めて生成する(0.5);

/// 配色とは、テーマで指定された色の組と、そこから導く色の規則のことである。
pub(super) struct 配色 {
    pub(super) 基調: 明暗,
    pub(super) 強調色: Option<Color32>,
    pub(super) 地の色: Option<Color32>,
    pub(super) 文字色: Option<Color32>,
    pub(super) 部品の面の色: Option<Color32>,
}

impl 明暗 {
    fn 文字を際立たせる極(self) -> Color32 {
        match self {
            明暗::濃色 => Color32::WHITE,
            明暗::淡色 => Color32::BLACK,
        }
    }

    fn 窪ませる極(self) -> Color32 {
        match self {
            明暗::濃色 => Color32::BLACK,
            明暗::淡色 => Color32::WHITE,
        }
    }
}

fn 寄せた色(
    元: Option<Color32>, 先: Option<Color32>, 寄せる割合: 割合
) -> Option<Color32> {
    Some(二色を混ぜる(元?, 先?, 寄せる割合))
}

impl 配色 {
    pub(super) fn マウスを乗せた面(&self) -> Option<Color32> {
        寄せた色(self.部品の面の色, self.文字色, マウスを乗せた面へ寄せる割合)
    }

    pub(super) fn 押した面(&self) -> Option<Color32> {
        寄せた色(self.部品の面の色, self.文字色, 押した面へ寄せる割合)
    }

    pub(super) fn マウスを乗せた文字(&self) -> Option<Color32> {
        let 極 = Some(self.基調.文字を際立たせる極());
        寄せた色(self.文字色, 極, マウスを乗せた文字へ寄せる割合)
    }

    pub(super) fn 押した文字(&self) -> Option<Color32> {
        let 極 = Some(self.基調.文字を際立たせる極());
        寄せた色(self.文字色, 極, 押した文字へ寄せる割合)
    }

    pub(super) fn 線の色(&self) -> Option<Color32> {
        寄せた色(self.地の色, self.文字色, 線へ寄せる割合)
    }

    pub(super) fn マウスを乗せた枠(&self) -> Option<Color32> {
        寄せた色(self.地の色, self.文字色, マウスを乗せた枠へ寄せる割合)
    }

    pub(super) fn 窪んだ面(&self) -> Option<Color32> {
        let 極 = Some(self.基調.窪ませる極());
        寄せた色(self.地の色, 極, 窪んだ面へ寄せる割合)
    }

    pub(super) fn 格子の縞(&self) -> Option<Color32> {
        寄せた色(self.地の色, self.文字色, 格子の縞へ寄せる割合)
    }

    pub(super) fn 選択の縁(&self) -> Option<Color32> {
        寄せた色(self.強調色, self.文字色, 選択の縁へ寄せる割合)
    }
}
