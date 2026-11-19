use crate::prelude::*;

use crate::jsx::tag::opening_element::AnyJsxOpeningElement;

use crate::utils::jsx::is_jsx_suppressed;
use biome_js_syntax::JsxSelfClosingElement;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxSelfClosingElement;

impl FormatNodeRule<JsxSelfClosingElement> for FormatJsxSelfClosingElement {
    fn fmt_fields(&self, node: &JsxSelfClosingElement, f: &mut JsFormatter) -> FormatResult<()> {
        AnyJsxOpeningElement::from(node.clone()).fmt(f)
    }

    fn is_suppressed(&self, node: &JsxSelfClosingElement, f: &JsFormatter) -> bool {
        is_jsx_suppressed(&node.clone().into(), f.comments())
    }
}
