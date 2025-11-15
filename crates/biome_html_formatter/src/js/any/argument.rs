//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyGlimmerArgument;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGlimmerArgument;
impl FormatRule<AnyGlimmerArgument> for FormatAnyGlimmerArgument {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyGlimmerArgument, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyGlimmerArgument::GlimmerNamedArgument(node) => node.format().fmt(f),
            AnyGlimmerArgument::GlimmerPositionalArgument(node) => node.format().fmt(f),
        }
    }
}
