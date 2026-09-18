//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssMediaConditionOperand;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssMediaConditionOperand;
impl FormatRule<AnyCssMediaConditionOperand> for FormatAnyCssMediaConditionOperand {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssMediaConditionOperand, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssMediaConditionOperand::AnyCssMediaInParens(node) => node.format().fmt(f),
            AnyCssMediaConditionOperand::ScssMediaQuery(node) => node.format().fmt(f),
        }
    }
}
