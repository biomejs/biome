//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSimpleSelector;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSimpleSelector;
impl FormatRule<AnyCssSimpleSelector> for FormatAnyCssSimpleSelector {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssSimpleSelector, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssSimpleSelector::CssTypeSelector(node) => node.format().fmt(f),
            AnyCssSimpleSelector::CssUniversalSelector(node) => node.format().fmt(f),
        }
    }
}
