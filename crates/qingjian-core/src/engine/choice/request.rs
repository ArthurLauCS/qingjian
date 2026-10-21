//! 选词请求。

/// 一次选词请求：前文、敲的键的输入单元、要比的候选文本。三样都相同才算同一页，缓存按它对上。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ChoiceRequest {
    pub context: String,
    pub typed: Vec<u32>,
    pub texts: Vec<String>,
}
