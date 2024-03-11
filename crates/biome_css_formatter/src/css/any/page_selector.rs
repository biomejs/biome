//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssPageSelector;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssPageSelector;
impl FormatRule<AnyCssPageSelector> for FormatAnyCssPageSelector {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssPageSelector, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssPageSelector::CssBogusSelector(node) => node.format().fmt(f),
            AnyCssPageSelector::CssPageSelector(node) => node.format().fmt(f),
        }
    }
}
