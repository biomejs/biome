//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssPseudoClassNthSelector;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssPseudoClassNthSelector;
impl FormatRule<AnyCssPseudoClassNthSelector> for FormatAnyCssPseudoClassNthSelector {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssPseudoClassNthSelector, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssPseudoClassNthSelector::CssBogusSelector(node) => node.format().fmt(f),
            AnyCssPseudoClassNthSelector::CssPseudoClassNthSelector(node) => node.format().fmt(f),
        }
    }
}
