//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssMediaInParens;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssMediaInParens;
impl FormatRule<AnyCssMediaInParens> for FormatAnyCssMediaInParens {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssMediaInParens, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssMediaInParens::CssMediaConditionInParens(node) => node.format().fmt(f),
            AnyCssMediaInParens::CssMediaFeatureInParens(node) => node.format().fmt(f),
        }
    }
}
