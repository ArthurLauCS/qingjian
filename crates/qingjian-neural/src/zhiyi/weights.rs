use std::collections::HashMap;
use std::path::Path;

use candle_core::quantized::{GgmlDType, QMatMul, QTensor};
use candle_core::{DType, Device, Module, Result, Tensor};

/// safetensors 里的全部张量，按名取；线性层取的时候量化成 Q8_0（模型说明要求这么跑才是它量过的准确率）。
pub(super) struct Weights {
    tensors: HashMap<String, Tensor>,
}

impl Weights {
    pub fn load(path: &Path, device: &Device) -> Result<Self> {
        Ok(Self {
            tensors: candle_core::safetensors::load(path, device)?,
        })
    }

    /// 按 f32 取（嵌入、范数、偏置这些不量化的）。
    pub fn get(&self, name: &str) -> Result<Tensor> {
        self.tensors
            .get(name)
            .ok_or_else(|| candle_core::Error::Msg(format!("missing tensor {name}")))?
            .to_dtype(DType::F32)
    }

    /// 原样取（`vocab_remap` 是 I32）。
    pub fn raw(&self, name: &str) -> Result<Tensor> {
        self.tensors
            .get(name)
            .cloned()
            .ok_or_else(|| candle_core::Error::Msg(format!("missing tensor {name}")))
    }

    /// 线性层，权重量化成 Q8_0；`bias` 是偏置名，没有偏置给 `None`。
    pub fn linear(&self, weight: &str, bias: Option<&str>) -> Result<QLinear> {
        let w = self.get(weight)?;
        Ok(QLinear {
            weight: QMatMul::from_qtensor(QTensor::quantize(&w, GgmlDType::Q8_0)?)?,
            bias: bias.map(|name| self.get(name)).transpose()?,
        })
    }
}

/// `x · wᵀ (+ b)`，`w` 是 Q8_0。
pub(super) struct QLinear {
    weight: QMatMul,
    bias: Option<Tensor>,
}

impl QLinear {
    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let y = self.weight.forward(x)?;
        match &self.bias {
            Some(bias) => y.broadcast_add(bias),
            None => Ok(y),
        }
    }
}

/// LayerNorm（最后一维），`bias` 可以没有（编码器里的都没有）。
pub(super) fn layer_norm(x: &Tensor, weight: &Tensor, bias: Option<&Tensor>) -> Result<Tensor> {
    let mean = x.mean_keepdim(candle_core::D::Minus1)?;
    let centered = x.broadcast_sub(&mean)?;
    let var = centered.sqr()?.mean_keepdim(candle_core::D::Minus1)?;
    let y = centered
        .broadcast_div(&(var + 1e-5)?.sqrt()?)?
        .broadcast_mul(weight)?;
    match bias {
        Some(bias) => y.broadcast_add(bias),
        None => Ok(y),
    }
}
