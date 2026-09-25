//! 描画の出力の読み取り。時刻や落とされたファイルを指定した入力で1フレーム描画し、出力と発した応答を返す。
//! 出力からは、描いた文字と、egui が求めた次の描き直しまでの時間を読み取る。

use std::time::Duration;

use sengen_egui::ノード;

/// 入力を指定して1フレーム描画し、発した応答と egui の出力を返す。
pub fn 入力を指定して描画する<M: Clone>(
    eguiの本体: &egui::Context,
    入力: egui::RawInput,
    木を組む: &dyn Fn() -> ノード<M>,
) -> (Vec<M>, egui::FullOutput) {
    let mut 集まり = Vec::new();
    let 出力 = eguiの本体.run(入力, |eguiの本体| {
        egui::CentralPanel::default().show(eguiの本体, |ui| {
            集まり.extend(木を組む().描画して集める(ui));
        });
    });
    (集まり, 出力)
}

/// 時刻（起動からの秒）を指定して1フレーム描画し、egui の出力を返す。
pub fn 時刻を指定して描画する<M: Clone>(
    eguiの本体: &egui::Context,
    秒: f64,
    木を組む: &dyn Fn() -> ノード<M>,
) -> egui::FullOutput {
    let 入力 = egui::RawInput {
        time: Some(秒),
        ..Default::default()
    };
    入力を指定して描画する(eguiの本体, 入力, 木を組む).1
}

/// 出力の図形の中に、指定の文字列をそのまま描いた文字があるか。
pub fn 文字を描いたか(出力: &egui::FullOutput, 文字列: &str) -> bool {
    出力
        .shapes
        .iter()
        .any(|切り抜いた図形| 図形に文字があるか(&切り抜いた図形.shape, 文字列))
}

fn 図形に文字があるか(図形: &egui::Shape, 文字列: &str) -> bool {
    match 図形 {
        egui::Shape::Text(文字の図形) => 文字の図形.galley.text() == 文字列,
        egui::Shape::Vec(図形一覧) => {
            図形一覧.iter().any(|中| 図形に文字があるか(中, 文字列))
        }
        _ => false,
    }
}

/// 出力が求めた、次の描き直しまでの時間。
pub fn 描き直しまでの時間(出力: &egui::FullOutput) -> Duration {
    出力
        .viewport_output
        .get(&egui::ViewportId::ROOT)
        .map_or(Duration::MAX, |窓| 窓.repaint_delay)
}
