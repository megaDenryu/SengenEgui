//! テキスト入力の欄の幅。既定・指定の幅・使える幅いっぱいの3つと、幅の下限を持ち、egui の部品へ写す。
//! 幅と下限は、どちらも欄の枠の内側（文字を並べる部分）の幅である。
//! 注意: 下限は egui の `min_size`（枠を含めた大きさ）で守らせるため、枠の内側の余白をここで決めて部品へ渡す。
//! 余白の値は egui の既定と同じにしてあり、見た目は変えない。

use crate::measure::論理画素;

/// 入力欄の枠の内側の余白（左右4論理画素・上下2論理画素）。egui の `TextEdit` の既定と同じ値である。
const 枠の内側の余白: egui::Margin = egui::Margin::symmetric(4, 2);

/// 入力欄の幅の決め方とは、テキスト入力の欄の幅をどう決めるかの区別のことである。
#[derive(Clone, Copy, Default)]
enum 幅の決め方 {
    /// egui の既定の幅。
    #[default]
    既定,
    /// 指定の幅。
    指定(論理画素),
    /// 使える幅の全部。
    使える幅いっぱい,
}

/// 入力欄の幅とは、テキスト入力の欄の幅の決め方と幅の下限の組のことである。
#[derive(Clone, Copy, Default)]
pub(super) struct 入力欄の幅 {
    決め方: 幅の決め方,
    下限: Option<論理画素>,
}

impl 入力欄の幅 {
    pub(super) fn 幅を指定する(&mut self, 幅: 論理画素) {
        self.決め方 = 幅の決め方::指定(幅);
    }

    pub(super) fn 使える幅いっぱいにする(&mut self) {
        self.決め方 = 幅の決め方::使える幅いっぱい;
    }

    pub(super) fn 下限を指定する(&mut self, 下限: 論理画素) {
        self.下限 = Some(下限);
    }

    /// 幅の決め方と下限を egui の部品へ写す。使える幅が下限より狭いときは、下限の幅で描いて使える幅からはみ出す。
    pub(super) fn 部品へ写す<'a>(self, 部品: egui::TextEdit<'a>) -> egui::TextEdit<'a> {
        let 部品 = 部品.margin(枠の内側の余白);
        let 部品 = match self.決め方 {
            幅の決め方::既定 => 部品,
            幅の決め方::指定(幅) => 部品.desired_width(幅.eguiへ渡す値()),
            幅の決め方::使える幅いっぱい => 部品.desired_width(f32::INFINITY),
        };
        match self.下限 {
            Some(下限) => 部品.min_size(egui::vec2(
                下限.eguiへ渡す値() + 枠の内側の余白.sum().x,
                0.0,
            )),
            None => 部品,
        }
    }
}
