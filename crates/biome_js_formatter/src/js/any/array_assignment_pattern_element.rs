//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsArrayAssignmentPatternElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsArrayAssignmentPatternElement;
impl FormatRule<AnyJsArrayAssignmentPatternElement> for FormatAnyJsArrayAssignmentPatternElement {
    type Context = JsFormatContext;
    fn fmt(
        &self,
        node: &AnyJsArrayAssignmentPatternElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyJsArrayAssignmentPatternElement::JsArrayAssignmentPatternElement(node) => {
                node.format().fmt(f)
            }
            AnyJsArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(node) => {
                node.format().fmt(f)
            }
            AnyJsArrayAssignmentPatternElement::JsArrayHole(node) => node.format().fmt(f),
        }
    }
}
