//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyGlimmerExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGlimmerExpression;
impl FormatRule<AnyGlimmerExpression> for FormatAnyGlimmerExpression {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyGlimmerExpression, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyGlimmerExpression::GlimmerBlockHelper(node) => node.format().fmt(f),
            AnyGlimmerExpression::GlimmerBogusExpression(node) => node.format().fmt(f),
            AnyGlimmerExpression::GlimmerMustacheComment(node) => node.format().fmt(f),
            AnyGlimmerExpression::GlimmerMustacheExpression(node) => node.format().fmt(f),
            AnyGlimmerExpression::GlimmerNamedBlock(node) => node.format().fmt(f),
            AnyGlimmerExpression::GlimmerTripleStashExpression(node) => node.format().fmt(f),
        }
    }
}
