//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsClassMember;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsClassMember;
impl FormatRule<AnyJsClassMember> for FormatAnyJsClassMember {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsClassMember::JsBogusMember(node) => node.format().fmt(f),
            AnyJsClassMember::JsConstructorClassMember(node) => node.format().fmt(f),
            AnyJsClassMember::JsEmptyClassMember(node) => node.format().fmt(f),
            AnyJsClassMember::JsGetterClassMember(node) => node.format().fmt(f),
            AnyJsClassMember::JsMetavariable(node) => node.format().fmt(f),
            AnyJsClassMember::JsMethodClassMember(node) => node.format().fmt(f),
            AnyJsClassMember::JsPropertyClassMember(node) => node.format().fmt(f),
            AnyJsClassMember::JsSetterClassMember(node) => node.format().fmt(f),
            AnyJsClassMember::JsStaticInitializationBlockClassMember(node) => node.format().fmt(f),
            AnyJsClassMember::TsConstructorSignatureClassMember(node) => node.format().fmt(f),
            AnyJsClassMember::TsGetterSignatureClassMember(node) => node.format().fmt(f),
            AnyJsClassMember::TsIndexSignatureClassMember(node) => node.format().fmt(f),
            AnyJsClassMember::TsInitializedPropertySignatureClassMember(node) => {
                node.format().fmt(f)
            }
            AnyJsClassMember::TsMethodSignatureClassMember(node) => node.format().fmt(f),
            AnyJsClassMember::TsPropertySignatureClassMember(node) => node.format().fmt(f),
            AnyJsClassMember::TsSetterSignatureClassMember(node) => node.format().fmt(f),
        }
    }
}
