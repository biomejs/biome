//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssKeyframesSelector;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssKeyframesSelector;
impl FormatRule<AnyCssKeyframesSelector> for FormatAnyCssKeyframesSelector {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssKeyframesSelector, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssKeyframesSelector::CssBogusSelector(node) => node.format().fmt(f),
            AnyCssKeyframesSelector::CssKeyframesIdentSelector(node) => node.format().fmt(f),
            AnyCssKeyframesSelector::CssKeyframesPercentageSelector(node) => node.format().fmt(f),
        }
    }
}
