//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssMediaTypeCondition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssMediaTypeCondition;
impl FormatRule<AnyCssMediaTypeCondition> for FormatAnyCssMediaTypeCondition {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssMediaTypeCondition, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssMediaTypeCondition::AnyCssMediaInParens(node) => node.format().fmt(f),
            AnyCssMediaTypeCondition::CssMediaAndCondition(node) => node.format().fmt(f),
            AnyCssMediaTypeCondition::CssMediaNotCondition(node) => node.format().fmt(f),
        }
    }
}
