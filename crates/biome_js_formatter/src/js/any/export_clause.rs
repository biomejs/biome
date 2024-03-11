//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsExportClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsExportClause;
impl FormatRule<AnyJsExportClause> for FormatAnyJsExportClause {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsExportClause, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsExportClause::AnyJsDeclarationClause(node) => node.format().fmt(f),
            AnyJsExportClause::JsExportDefaultDeclarationClause(node) => node.format().fmt(f),
            AnyJsExportClause::JsExportDefaultExpressionClause(node) => node.format().fmt(f),
            AnyJsExportClause::JsExportFromClause(node) => node.format().fmt(f),
            AnyJsExportClause::JsExportNamedClause(node) => node.format().fmt(f),
            AnyJsExportClause::JsExportNamedFromClause(node) => node.format().fmt(f),
            AnyJsExportClause::TsExportAsNamespaceClause(node) => node.format().fmt(f),
            AnyJsExportClause::TsExportAssignmentClause(node) => node.format().fmt(f),
            AnyJsExportClause::TsExportDeclareClause(node) => node.format().fmt(f),
        }
    }
}
