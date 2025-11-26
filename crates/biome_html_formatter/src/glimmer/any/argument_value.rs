//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyGlimmerArgumentValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGlimmerArgumentValue;
impl FormatRule<AnyGlimmerArgumentValue> for FormatAnyGlimmerArgumentValue {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyGlimmerArgumentValue, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyGlimmerArgumentValue::GlimmerLiteral(node) => node.format().fmt(f),
            AnyGlimmerArgumentValue::GlimmerPath(node) => node.format().fmt(f),
            AnyGlimmerArgumentValue::GlimmerStringLiteral(node) => node.format().fmt(f),
        }
    }
}
