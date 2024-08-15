//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSelector;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSelector;
impl FormatRule<AnyCssSelector> for FormatAnyCssSelector {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssSelector, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssSelector::CssBogusSelector(node) => node.format().fmt(f),
            AnyCssSelector::CssComplexSelector(node) => node.format().fmt(f),
            AnyCssSelector::CssCompoundSelector(node) => node.format().fmt(f),
            AnyCssSelector::CssMetavariable(node) => node.format().fmt(f),
        }
    }
}
