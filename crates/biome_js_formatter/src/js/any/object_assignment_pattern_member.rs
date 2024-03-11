//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsObjectAssignmentPatternMember;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsObjectAssignmentPatternMember;
impl FormatRule<AnyJsObjectAssignmentPatternMember> for FormatAnyJsObjectAssignmentPatternMember {
    type Context = JsFormatContext;
    fn fmt(
        &self,
        node: &AnyJsObjectAssignmentPatternMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyJsObjectAssignmentPatternMember::JsBogusAssignment(node) => node.format().fmt(f),
            AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(node) => {
                node.format().fmt(f)
            }
            AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(node) => {
                node.format().fmt(f)
            }
            AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
                node,
            ) => node.format().fmt(f),
        }
    }
}
