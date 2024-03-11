//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsForInitializer;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsForInitializer;
impl FormatRule<AnyJsForInitializer> for FormatAnyJsForInitializer {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsForInitializer, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsForInitializer::AnyJsExpression(node) => node.format().fmt(f),
            AnyJsForInitializer::JsVariableDeclaration(node) => node.format().fmt(f),
        }
    }
}
