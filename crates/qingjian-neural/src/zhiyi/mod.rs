//! 试验：知意输入法的选词模型（知意 1.4V2-Long-L12，safetensors 版）给候选打分，只供 CLI 回放对比。
//! 结构与输入格式按模型包里的 README 实现：mmBERT-small 留 12 层的编码器 + 两层选词头，线性层加载时量化成 Q8_0。
//! 权重是 GPL-3.0-only，不随包、不进壳；目录里要有 `model.safetensors`、`config.json`、`tokenizer.json`、`unified_vocab.json`。

mod encoder;
mod head;
mod input;
mod weights;

use std::collections::HashMap;
use std::path::Path;

use candle_core::Device;
use serde::Deserialize;
use tokenizers::Tokenizer;

use crate::NeuralError;
use encoder::Encoder;
use head::ChoiceHead;
use weights::Weights;

/// `config.json` 里用得上的部分。
#[derive(Deserialize)]
struct Config {
    encoder: EncoderConfig,
}

#[derive(Deserialize)]
struct EncoderConfig {
    layer_types: Vec<String>,
}

/// `unified_vocab.json` 里用得上的部分。
#[derive(Deserialize)]
struct UnifiedVocab {
    units: Vec<String>,
    letters: String,
}

/// 加载好的选词模型。
pub struct ChoiceScorer {
    encoder: Encoder,
    head: ChoiceHead,
    tokenizer: Tokenizer,
    /// 拼音单元名（`f:zhong` / `p:g`）→ 单元号。
    units: HashMap<String, u32>,
    /// 英文字母表，单元号是 `units.len()` + 下标。
    letters: Vec<char>,
}

impl ChoiceScorer {
    pub fn load(dir: &Path) -> Result<Self, NeuralError> {
        let device = Device::Cpu;
        let config: Config = read_json(&dir.join("config.json"))?;
        let vocab: UnifiedVocab = read_json(&dir.join("unified_vocab.json"))?;
        let sliding: Vec<bool> = config
            .encoder
            .layer_types
            .iter()
            .map(|kind| kind == "sliding_attention")
            .collect();
        let weights = Weights::load(&dir.join("model.safetensors"), &device)?;
        let tokenizer = Tokenizer::from_file(dir.join("tokenizer.json"))
            .map_err(|_| NeuralError::Corrupt("tokenizer.json"))?;
        Ok(Self {
            encoder: Encoder::load(&weights, &sliding, &device)?,
            head: ChoiceHead::load(&weights)?,
            tokenizer,
            units: vocab
                .units
                .iter()
                .enumerate()
                .map(|(i, name)| (name.clone(), i as u32))
                .collect(),
            letters: vocab.letters.chars().collect(),
        })
    }

    /// 给 `candidates` 各打一个分（越大越合适）：`context` 是光标前的文字，`typed` 是敲的拼音或字母的单元
    /// （[`Self::align`] / [`Self::letter_units`]）。
    pub fn score(
        &self,
        context: &str,
        typed: &[u32],
        candidates: &[&str],
    ) -> Result<Vec<f32>, NeuralError> {
        let sequence = self.sequence(context, typed, candidates)?;
        self.score_ids(&sequence.ids, &sequence.units, &sequence.markers)
    }

    /// 直接按 token 序列打分（测试向量用）。
    pub fn score_ids(
        &self,
        ids: &[u32],
        units: &[u32],
        markers: &[u32],
    ) -> Result<Vec<f32>, NeuralError> {
        let encoded = self.encoder.forward(ids, units)?;
        Ok(self.head.scores(&encoded, markers)?)
    }

    fn encode(&self, text: &str) -> Result<Vec<u32>, NeuralError> {
        self.tokenizer
            .encode(text, false)
            .map(|encoding| encoding.get_ids().to_vec())
            .map_err(|_| NeuralError::Corrupt("tokenizer failed"))
    }
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, NeuralError> {
    let text = std::fs::read_to_string(path).map_err(|source| NeuralError::Io {
        path: path.to_owned(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| NeuralError::Json {
        path: path.to_owned(),
        source,
    })
}

#[cfg(test)]
mod tests;
