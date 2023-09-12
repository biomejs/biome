//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsFunctionBody;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsFunctionBody;
impl FormatRule<AnyJsFunctionBody> for FormatAnyJsFunctionBody {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsFunctionBody, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsFunctionBody::AnyJsExpression(node) => node.format().fmt(f),
            AnyJsFunctionBody::JsFunctionBody(node) => node.format().fmt(f),
        }
    }
}
