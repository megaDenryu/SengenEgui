//! 次元の無い比。同じ次元の2つの量（論理画素どうし、帯の値の差どうし）を割った値であり、0〜1 に限らず負の値も
//! 1を超える値も取る。0〜1 に限る `割合` とは別の量である。部品の中で、ある長さや値の差を別の基準で測り直すときに使う。
//! 割り算を量の型のメソッド（`論理画素::に対する比` 等）に置き、次元の合わない割り算を書けないようにする。

// 次元の無い比とは、同じ次元の2つの量を割った、単位を持たない実数のことである。
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[repr(transparent)]
pub(crate) struct 次元の無い比(f64);

impl 次元の無い比 {
    pub(crate) const ゼロ: 次元の無い比 = 次元の無い比(0.0);

    // 割られる量と基準の量を倍精度の小数へ直した値から作る。基準が0か、商が有限でなければ比が決まらないため None を返す。
    pub(crate) fn 二つの量から求める(割られる量: f64, 基準: f64) -> Option<Self> {
        let 商 = 割られる量 / 基準;
        (基準 != 0.0 && 商.is_finite()).then_some(Self(商))
    }

    pub(crate) const fn 倍精度の小数にする(self) -> f64 {
        self.0
    }

    // 最も近い整数。ちょうど半分は0から遠い方へ丸める（0.5 は 1、-0.5 は -1）。i32 の範囲を超えるなら端の値にする。
    pub(crate) fn 最も近い整数へ丸める(self) -> i32 {
        <i32 as egui::emath::Numeric>::from_f64(self.0.round())
    }
}

#[cfg(test)]
#[path = "dimensionless_ratio_tests.rs"]
mod dimensionless_ratio_tests;
