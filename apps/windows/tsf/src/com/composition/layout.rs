//! 组句期间挂在文档上的排版变化通知（`ITfTextLayoutSink`）。Chrome 系应用的组句位置由渲染进程异步报回，
//! 新一段组句第一键量到的常是上一段的旧光标（#809）；应用排好版会发这条通知，这时再量一次报给 Server。

use std::rc::Rc;

use windows::Win32::UI::TextServices::{
    ITfContext, ITfContextView, ITfSource, ITfTextLayoutSink, ITfTextLayoutSink_Impl, TF_LC_CHANGE,
    TfLayoutCode,
};
use windows::core::{Interface, Ref, Result, implement};

use super::Shared;
use crate::com::edit::request_caret;
use crate::com::log::log;
use crate::com::service::SharedClient;

#[implement(ITfTextLayoutSink)]
struct LayoutSink {
    /// 组句状态：重量时组句已收就不报。
    shared: Rc<Shared>,

    /// 引擎层：重量的锚点报给 Server。
    engine: SharedClient,

    /// 本服务的 TSF client id：量位置要申请只读会话。
    client_id: u32,
}

impl ITfTextLayoutSink_Impl for LayoutSink_Impl {
    fn OnLayoutChange(
        &self,
        pic: Ref<ITfContext>,
        lcode: TfLayoutCode,
        _pview: Ref<ITfContextView>,
    ) -> Result<()> {
        if lcode == TF_LC_CHANGE
            && let Err(error) = request_caret(
                pic.ok()?,
                self.client_id,
                self.engine.clone(),
                self.shared.clone(),
            )
        {
            log(&format!("排版变了，重量锚点的会话没被受理: {error}"));
        }
        Ok(())
    }
}

/// 在 `context` 上挂通知，返回 `(source, cookie)` 交给 [`Shared::set_layout_sink`] 保管、组句收掉时撤。
pub(super) fn advise(
    context: &ITfContext,
    shared: &Rc<Shared>,
    engine: &SharedClient,
    client_id: u32,
) -> Result<(ITfSource, u32)> {
    let source: ITfSource = context.cast()?;
    let sink: ITfTextLayoutSink = LayoutSink {
        shared: shared.clone(),
        engine: engine.clone(),
        client_id,
    }
    .into();
    let cookie = unsafe { source.AdviseSink(&ITfTextLayoutSink::IID, &sink)? };
    Ok((source, cookie))
}
