//! 繁体输出：候选与上屏文本的简繁转换。
//!
//! 转换属于文本变换，按架构约束整个放在 Core：壳只是把候选原样画出来、把选中的候选原样送回来。
//! 词库与学习数据一律是简体，转换只发生在候选出 Core 的那一步，见 [`Traditional`]。
//! 中文词展开成几种繁体写法（台湾用语、逐字字形、单字的各个繁体字），用户选过的写法排第一。

mod alternatives;
mod converter;
mod variant;

pub use converter::Traditional;
pub use variant::TraditionalVariant;
