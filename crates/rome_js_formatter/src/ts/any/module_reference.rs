//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyTsModuleReference;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsModuleReference;
impl FormatRule<AnyTsModuleReference> for FormatAnyTsModuleReference {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyTsModuleReference, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyTsModuleReference::AnyTsName(node) => node.format().fmt(f),
            AnyTsModuleReference::TsExternalModuleReference(node) => node.format().fmt(f),
        }
    }
}
