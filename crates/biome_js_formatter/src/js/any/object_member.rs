//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsObjectMember;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsObjectMember;
impl FormatRule<AnyJsObjectMember> for FormatAnyJsObjectMember {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsObjectMember::JsBogusMember(node) => node.format().fmt(f),
            AnyJsObjectMember::JsGetterObjectMember(node) => node.format().fmt(f),
            AnyJsObjectMember::JsMethodObjectMember(node) => node.format().fmt(f),
            AnyJsObjectMember::JsPropertyObjectMember(node) => node.format().fmt(f),
            AnyJsObjectMember::JsSetterObjectMember(node) => node.format().fmt(f),
            AnyJsObjectMember::JsShorthandPropertyObjectMember(node) => node.format().fmt(f),
            AnyJsObjectMember::JsSpread(node) => node.format().fmt(f),
        }
    }
}
