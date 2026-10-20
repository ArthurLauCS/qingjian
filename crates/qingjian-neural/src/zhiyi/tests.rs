//! 对拍模型包附带的测试向量（fp32 ONNX 的输出）。要设 `QINGJIAN_ZHIYI_DIR` 指向解压的模型目录，没设就跳过。

use std::collections::HashMap;
use std::path::PathBuf;

use candle_core::{Device, Tensor};

use super::ChoiceScorer;

#[test]
fn matches_the_reference_vectors() {
    let Some(dir) = std::env::var_os("QINGJIAN_ZHIYI_DIR").map(PathBuf::from) else {
        eprintln!("没设 QINGJIAN_ZHIYI_DIR，跳过");
        return;
    };
    let scorer = ChoiceScorer::load(&dir).unwrap();
    let vectors: HashMap<String, Tensor> =
        candle_core::safetensors::load(dir.join("test_vectors.safetensors"), &Device::Cpu).unwrap();
    let ints = |name: String| -> Vec<u32> {
        vectors[&name]
            .to_vec1::<i64>()
            .unwrap()
            .into_iter()
            .map(|v| v as u32)
            .collect()
    };
    for case in 0..5 {
        let expected: Vec<f32> = vectors[&format!("{case}.logits")].to_vec1().unwrap();
        let got = scorer
            .score_ids(
                &ints(format!("{case}.input_ids")),
                &ints(format!("{case}.unit_ids")),
                &ints(format!("{case}.marker_pos")),
            )
            .unwrap();
        eprintln!("case {case}: expected {expected:?} got {got:?}");
        let best = |v: &[f32]| {
            v.iter()
                .enumerate()
                .max_by(|a, b| a.1.total_cmp(b.1))
                .map(|(i, _)| i)
        };
        assert_eq!(best(&expected), best(&got), "case {case}");
        for (e, g) in expected.iter().zip(&got) {
            assert!((e - g).abs() < 0.5, "case {case}: {expected:?} vs {got:?}");
        }
    }
}
