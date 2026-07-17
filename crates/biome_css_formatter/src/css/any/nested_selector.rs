//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssNestedSelector;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssNestedSelector;
impl FormatRule<AnyCssNestedSelector> for FormatAnyCssNestedSelector {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssNestedSelector, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssNestedSelector::CssNestedSelector(node) => node.format().fmt(f),
            AnyCssNestedSelector::ScssParentSelector(node) => node.format().fmt(f),
        }
    }
}
