//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssRelativeSelector;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssRelativeSelector;
impl FormatRule<AnyCssRelativeSelector> for FormatAnyCssRelativeSelector {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssRelativeSelector, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssRelativeSelector::CssBogusSelector(node) => node.format().fmt(f),
            AnyCssRelativeSelector::CssRelativeSelector(node) => node.format().fmt(f),
        }
    }
}
