//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsAssignment;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsAssignment;
impl FormatRule<AnyJsAssignment> for FormatAnyJsAssignment {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsAssignment::JsBogusAssignment(node) => node.format().fmt(f),
            AnyJsAssignment::JsComputedMemberAssignment(node) => node.format().fmt(f),
            AnyJsAssignment::JsIdentifierAssignment(node) => node.format().fmt(f),
            AnyJsAssignment::JsParenthesizedAssignment(node) => node.format().fmt(f),
            AnyJsAssignment::JsStaticMemberAssignment(node) => node.format().fmt(f),
            AnyJsAssignment::TsAsAssignment(node) => node.format().fmt(f),
            AnyJsAssignment::TsNonNullAssertionAssignment(node) => node.format().fmt(f),
            AnyJsAssignment::TsSatisfiesAssignment(node) => node.format().fmt(f),
            AnyJsAssignment::TsTypeAssertionAssignment(node) => node.format().fmt(f),
        }
    }
}
