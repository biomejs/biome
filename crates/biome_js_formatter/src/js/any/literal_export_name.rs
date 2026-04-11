//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsLiteralExportName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsLiteralExportName;
impl FormatRule<AnyJsLiteralExportName> for FormatAnyJsLiteralExportName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsLiteralExportName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsLiteralExportName::JsLiteralExportName(node) => node.format().fmt(f),
            AnyJsLiteralExportName::JsMetavariable(node) => node.format().fmt(f),
        }
    }
}
