//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsBindingPattern;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsBindingPattern;
impl FormatRule<AnyJsBindingPattern> for FormatAnyJsBindingPattern {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsBindingPattern, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsBindingPattern::AnyJsBinding(node) => node.format().fmt(f),
            AnyJsBindingPattern::JsArrayBindingPattern(node) => node.format().fmt(f),
            AnyJsBindingPattern::JsObjectBindingPattern(node) => node.format().fmt(f),
        }
    }
}
