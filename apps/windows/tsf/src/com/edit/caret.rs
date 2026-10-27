//! 重量候选窗口锚点的只读会话：应用报「排版变了」（见 `composition/layout.rs`）时量一次组句位置报给 Server。

use std::rc::Rc;

use windows::Win32::UI::TextServices::{
    ITfContext, ITfEditSession, ITfEditSession_Impl, TF_ES_READ,
};
use windows::core::{Result, implement};

use crate::com::composition::{Shared, report_caret};
use crate::com::service::SharedClient;

#[implement(ITfEditSession)]
pub(crate) struct CaretSession {
    /// 目标文档上下文。
    context: ITfContext,

    /// 引擎层：量到的锚点报给 Server。
    engine: SharedClient,

    /// 组句状态：组句已收就不报。
    shared: Rc<Shared>,
}

impl ITfEditSession_Impl for CaretSession_Impl {
    fn DoEditSession(&self, ec: u32) -> Result<()> {
        report_caret(&self.shared, &self.engine, &self.context, ec);
        Ok(())
    }
}

/// 请求一个异步只读会话重报锚点。`Ok` 只说明已受理。
pub(crate) fn request_caret(
    context: &ITfContext,
    client_id: u32,
    engine: SharedClient,
    shared: Rc<Shared>,
) -> Result<()> {
    let session = CaretSession {
        context: context.clone(),
        engine,
        shared,
    };
    super::update::request(context, client_id, session.into(), TF_ES_READ)
}
