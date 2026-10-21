//! int8 ONNX 版，ONNX Runtime 跑（Windows 用）：运行库在运行时按路径加载，有 DX12 显卡走 DirectML，没有退回 CPU。

use std::path::Path;
use std::sync::{Mutex, OnceLock};

use ort::ep::DirectML;
use ort::session::Session;
use ort::session::builder::GraphOptimizationLevel;
use ort::value::Tensor;

use super::forward::Forward;
use super::input::Sequence;
use crate::NeuralError;

const MODEL_FILE: &str = "laya.int8g.onnx";

pub(super) struct OnnxForward {
    /// `Session::run` 要 `&mut`；打分本来就一次一条。
    session: Mutex<Session>,
    gpu: bool,
}

impl OnnxForward {
    pub fn load(dir: &Path, runtime: &Path, cpu_threads: usize) -> Result<Self, NeuralError> {
        load_runtime(runtime)?;
        let model = dir.join(MODEL_FILE);
        // DirectML 不支持内存复用模式与并行执行，按 ONNX Runtime 文档关掉
        let gpu = Session::builder()
            .and_then(|builder| builder.with_memory_pattern(false).map_err(Into::into))
            .and_then(|builder| builder.with_parallel_execution(false).map_err(Into::into))
            .and_then(|builder| {
                builder
                    .with_execution_providers([DirectML::default().build().error_on_failure()])
                    .map_err(Into::into)
            })
            .and_then(|mut builder| builder.commit_from_file(&model));
        let (session, gpu) = match gpu {
            Ok(session) => (session, true),
            Err(error) => {
                tracing::info!(%error, "知意用不了 DirectML，改用 CPU");
                let session = Session::builder()
                    .and_then(|builder| {
                        builder
                            .with_optimization_level(GraphOptimizationLevel::Level3)
                            .map_err(Into::into)
                    })
                    .and_then(|builder| builder.with_intra_threads(cpu_threads).map_err(Into::into))
                    .and_then(|mut builder| builder.commit_from_file(&model))
                    .map_err(onnx_error)?;
                (session, false)
            }
        };
        Ok(Self {
            session: Mutex::new(session),
            gpu,
        })
    }
}

impl Forward for OnnxForward {
    fn logits(&self, sequence: &Sequence) -> Result<Vec<f32>, NeuralError> {
        let row = |values: Vec<i64>| Tensor::from_array(([1, values.len()], values));
        let ints = |values: &[u32]| values.iter().map(|&v| i64::from(v)).collect::<Vec<_>>();
        let slots: Vec<i64> = (0..sequence.units.len() as i64)
            .filter(|&i| sequence.units[i as usize] != 0)
            .collect();
        let markers = sequence.markers.len();
        let inputs = ort::inputs![
            "input_ids" => row(ints(&sequence.ids)).map_err(onnx_error)?,
            "attention_mask" => row(vec![1; sequence.ids.len()]).map_err(onnx_error)?,
            "unit_ids" => row(ints(&sequence.units)).map_err(onnx_error)?,
            "marker_pos" => row(ints(&sequence.markers)).map_err(onnx_error)?,
            "marker_mask" => Tensor::from_array(([1, markers], vec![true; markers])).map_err(onnx_error)?,
            "slot_pos" => row(slots).map_err(onnx_error)?,
        ];
        let mut session = self
            .session
            .lock()
            .map_err(|_| NeuralError::Corrupt("ONNX session poisoned"))?;
        let outputs = session.run(inputs).map_err(onnx_error)?;
        let (_, logits) = outputs["logits"]
            .try_extract_tensor::<f32>()
            .map_err(onnx_error)?;
        Ok(logits.to_vec())
    }

    fn on_gpu(&self) -> bool {
        self.gpu
    }
}

/// 整个进程只加载一次运行库；第二次起用第一次的结果（换路径也不重载）。
fn load_runtime(path: &Path) -> Result<(), NeuralError> {
    static LOADED: OnceLock<Result<(), String>> = OnceLock::new();
    LOADED
        .get_or_init(|| {
            // 先按路径载入同目录的 DirectML.dll：onnxruntime.dll 再按名字找时就用它，不会找到系统目录里那份旧的
            if let Some(directml) = path.parent().map(|dir| dir.join("DirectML.dll"))
                && directml.is_file()
                && let Err(error) = ort::util::preload_dylib(&directml)
            {
                tracing::warn!(%error, "载入 DirectML.dll 失败");
            }
            ort::init_from(path)
                .map(|builder| {
                    // ONNX Runtime 的 Windows 版缺省开着遥测（ETW），关掉
                    builder.with_name("qingjian").with_telemetry(false).commit();
                })
                .map_err(|error| format!("{}: {error}", path.display()))
        })
        .clone()
        .map_err(NeuralError::Onnx)
}

fn onnx_error(error: impl std::fmt::Display) -> NeuralError {
    NeuralError::Onnx(error.to_string())
}
