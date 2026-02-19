//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyAstroDirective;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyAstroDirective;
impl FormatRule<AnyAstroDirective> for FormatAnyAstroDirective {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyAstroDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyAstroDirective::AstroClassDirective(node) => node.format().fmt(f),
            AnyAstroDirective::AstroClientDirective(node) => node.format().fmt(f),
            AnyAstroDirective::AstroDefineDirective(node) => node.format().fmt(f),
            AnyAstroDirective::AstroIsDirective(node) => node.format().fmt(f),
            AnyAstroDirective::AstroServerDirective(node) => node.format().fmt(f),
            AnyAstroDirective::AstroSetDirective(node) => node.format().fmt(f),
        }
    }
}
