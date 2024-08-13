//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyTsModuleName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsModuleName;
impl FormatRule<AnyTsModuleName> for FormatAnyTsModuleName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyTsModuleName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyTsModuleName::AnyTsIdentifierBinding(node) => node.format().fmt(f),
            AnyTsModuleName::TsQualifiedModuleName(node) => node.format().fmt(f),
        }
    }
}
