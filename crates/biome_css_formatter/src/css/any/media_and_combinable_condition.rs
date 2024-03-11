//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssMediaAndCombinableCondition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssMediaAndCombinableCondition;
impl FormatRule<AnyCssMediaAndCombinableCondition> for FormatAnyCssMediaAndCombinableCondition {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyCssMediaAndCombinableCondition,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyCssMediaAndCombinableCondition::AnyCssMediaInParens(node) => node.format().fmt(f),
            AnyCssMediaAndCombinableCondition::CssMediaAndCondition(node) => node.format().fmt(f),
        }
    }
}
