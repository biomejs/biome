//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssCompoundSelector;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssCompoundSelector;
impl FormatRule<AnyCssCompoundSelector> for FormatAnyCssCompoundSelector {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssCompoundSelector, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssCompoundSelector::CssBogusSelector(node) => node.format().fmt(f),
            AnyCssCompoundSelector::CssCompoundSelector(node) => node.format().fmt(f),
        }
    }
}
