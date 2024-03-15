//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsNamedImportSpecifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsNamedImportSpecifier;
impl FormatRule<AnyJsNamedImportSpecifier> for FormatAnyJsNamedImportSpecifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsNamedImportSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(node) => node.format().fmt(f),
            AnyJsNamedImportSpecifier::JsNamedImportSpecifier(node) => node.format().fmt(f),
            AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(node) => {
                node.format().fmt(f)
            }
        }
    }
}
