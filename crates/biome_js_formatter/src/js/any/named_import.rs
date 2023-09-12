//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsNamedImport;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsNamedImport;
impl FormatRule<AnyJsNamedImport> for FormatAnyJsNamedImport {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsNamedImport, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsNamedImport::JsNamedImportSpecifiers(node) => node.format().fmt(f),
            AnyJsNamedImport::JsNamespaceImportSpecifier(node) => node.format().fmt(f),
        }
    }
}
