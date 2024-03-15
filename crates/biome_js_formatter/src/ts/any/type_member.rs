//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyTsTypeMember;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsTypeMember;
impl FormatRule<AnyTsTypeMember> for FormatAnyTsTypeMember {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyTsTypeMember, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyTsTypeMember::JsBogusMember(node) => node.format().fmt(f),
            AnyTsTypeMember::TsCallSignatureTypeMember(node) => node.format().fmt(f),
            AnyTsTypeMember::TsConstructSignatureTypeMember(node) => node.format().fmt(f),
            AnyTsTypeMember::TsGetterSignatureTypeMember(node) => node.format().fmt(f),
            AnyTsTypeMember::TsIndexSignatureTypeMember(node) => node.format().fmt(f),
            AnyTsTypeMember::TsMethodSignatureTypeMember(node) => node.format().fmt(f),
            AnyTsTypeMember::TsPropertySignatureTypeMember(node) => node.format().fmt(f),
            AnyTsTypeMember::TsSetterSignatureTypeMember(node) => node.format().fmt(f),
        }
    }
}
