//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsCombinedSpecifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsCombinedSpecifier;
impl FormatRule<AnyJsCombinedSpecifier> for FormatAnyJsCombinedSpecifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsCombinedSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsCombinedSpecifier::JsNamedImportSpecifiers(node) => node.format().fmt(f),
            AnyJsCombinedSpecifier::JsNamespaceImportSpecifier(node) => node.format().fmt(f),
        }
    }
}
