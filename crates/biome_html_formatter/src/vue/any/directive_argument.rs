//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyVueDirectiveArgument;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyVueDirectiveArgument;
impl FormatRule<AnyVueDirectiveArgument> for FormatAnyVueDirectiveArgument {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyVueDirectiveArgument, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyVueDirectiveArgument::VueBogusDirectiveArgument(node) => node.format().fmt(f),
            AnyVueDirectiveArgument::VueDynamicArgument(node) => node.format().fmt(f),
            AnyVueDirectiveArgument::VueStaticArgument(node) => node.format().fmt(f),
        }
    }
}
