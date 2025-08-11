//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyTwUtilityName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTwUtilityName;
impl FormatRule<AnyTwUtilityName> for FormatAnyTwUtilityName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyTwUtilityName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyTwUtilityName::CssIdentifier(node) => node.format().fmt(f),
            AnyTwUtilityName::TwFunctionalUtilityName(node) => node.format().fmt(f),
        }
    }
}
