//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsAssignmentPattern;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsAssignmentPattern;
impl FormatRule<AnyJsAssignmentPattern> for FormatAnyJsAssignmentPattern {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsAssignmentPattern, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsAssignmentPattern::AnyJsAssignment(node) => node.format().fmt(f),
            AnyJsAssignmentPattern::JsArrayAssignmentPattern(node) => node.format().fmt(f),
            AnyJsAssignmentPattern::JsObjectAssignmentPattern(node) => node.format().fmt(f),
        }
    }
}
