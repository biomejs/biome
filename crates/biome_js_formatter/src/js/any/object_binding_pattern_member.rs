//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsObjectBindingPatternMember;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsObjectBindingPatternMember;
impl FormatRule<AnyJsObjectBindingPatternMember> for FormatAnyJsObjectBindingPatternMember {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsObjectBindingPatternMember, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsObjectBindingPatternMember::JsBogusBinding(node) => node.format().fmt(f),
            AnyJsObjectBindingPatternMember::JsMetavariable(node) => node.format().fmt(f),
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(node) => {
                node.format().fmt(f)
            }
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(node) => {
                node.format().fmt(f)
            }
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(node) => {
                node.format().fmt(f)
            }
        }
    }
}
