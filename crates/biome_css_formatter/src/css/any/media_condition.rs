//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssMediaCondition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssMediaCondition;
impl FormatRule<AnyCssMediaCondition> for FormatAnyCssMediaCondition {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssMediaCondition, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssMediaCondition::AnyCssMediaInParens(node) => node.format().fmt(f),
            AnyCssMediaCondition::CssMediaAndCondition(node) => node.format().fmt(f),
            AnyCssMediaCondition::CssMediaNotCondition(node) => node.format().fmt(f),
            AnyCssMediaCondition::CssMediaOrCondition(node) => node.format().fmt(f),
        }
    }
}
