//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsImportClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsImportClause;
impl FormatRule<AnyJsImportClause> for FormatAnyJsImportClause {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsImportClause, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsImportClause::JsImportBareClause(node) => node.format().fmt(f),
            AnyJsImportClause::JsImportCombinedClause(node) => node.format().fmt(f),
            AnyJsImportClause::JsImportDefaultClause(node) => node.format().fmt(f),
            AnyJsImportClause::JsImportNamedClause(node) => node.format().fmt(f),
            AnyJsImportClause::JsImportNamespaceClause(node) => node.format().fmt(f),
        }
    }
}
