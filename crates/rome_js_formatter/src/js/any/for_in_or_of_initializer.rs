//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsForInOrOfInitializer;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsForInOrOfInitializer;
impl FormatRule<AnyJsForInOrOfInitializer> for FormatAnyJsForInOrOfInitializer {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsForInOrOfInitializer, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsForInOrOfInitializer::AnyJsAssignmentPattern(node) => node.format().fmt(f),
            AnyJsForInOrOfInitializer::JsForVariableDeclaration(node) => node.format().fmt(f),
        }
    }
}
