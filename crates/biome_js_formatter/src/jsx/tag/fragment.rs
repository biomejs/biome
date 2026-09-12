use crate::prelude::*;

use crate::jsx::tag::element::AnyJsxTagWithChildren;
use crate::utils::jsx::is_jsx_suppressed;
use biome_formatter::write;
use biome_js_syntax::JsxFragment;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxFragment;

impl FormatNodeRule<JsxFragment> for FormatJsxFragment {
    fn fmt_fields(&self, node: &JsxFragment, f: &mut JsFormatter) -> FormatResult<()> {
        write!(f, [AnyJsxTagWithChildren::from(node.clone())])
    }

    fn is_suppressed(&self, node: &JsxFragment, f: &JsFormatter) -> bool {
        is_jsx_suppressed(&node.clone().into(), f.comments())
    }

    /// [`AnyJsxTagWithChildren`] prints them between the tags instead of after `</>`.
    fn fmt_dangling_comments(&self, _: &JsxFragment, _: &mut JsFormatter) -> FormatResult<()> {
        Ok(())
    }
}
