//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsCallArgument;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsCallArgument;
impl FormatRule<AnyJsCallArgument> for FormatAnyJsCallArgument {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsCallArgument, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsCallArgument::AnyJsExpression(node) => node.format().fmt(f),
            AnyJsCallArgument::JsSpread(node) => node.format().fmt(f),
        }
    }
}
