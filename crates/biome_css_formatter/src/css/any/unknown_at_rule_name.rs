//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssUnknownAtRuleName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssUnknownAtRuleName;
impl FormatRule<AnyCssUnknownAtRuleName> for FormatAnyCssUnknownAtRuleName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssUnknownAtRuleName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssUnknownAtRuleName::CssIdentifier(node) => node.format().fmt(f),
            AnyCssUnknownAtRuleName::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
            AnyCssUnknownAtRuleName::ScssInterpolation(node) => node.format().fmt(f),
        }
    }
}
