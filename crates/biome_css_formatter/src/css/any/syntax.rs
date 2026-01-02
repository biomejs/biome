//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSyntax;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSyntax;
impl FormatRule<AnyCssSyntax> for FormatAnyCssSyntax {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssSyntax, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssSyntax::AnyCssSyntaxComponent(node) => node.format().fmt(f),
            AnyCssSyntax::CssBogusSyntax(node) => node.format().fmt(f),
            AnyCssSyntax::CssString(node) => node.format().fmt(f),
            AnyCssSyntax::CssSyntaxComponentList(node) => node.format().fmt(f),
            AnyCssSyntax::CssUniversalSyntax(node) => node.format().fmt(f),
        }
    }
}
