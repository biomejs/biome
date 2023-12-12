//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsExtraImportSpecifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsExtraImportSpecifier;
impl FormatRule<AnyJsExtraImportSpecifier> for FormatAnyJsExtraImportSpecifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsExtraImportSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsExtraImportSpecifier::JsNamedImportSpecifiers(node) => node.format().fmt(f),
            AnyJsExtraImportSpecifier::JsNamespaceImportSpecifier(node) => node.format().fmt(f),
        }
    }
}
