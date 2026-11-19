//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssAtRuleCustomName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAtRuleCustomName;
impl FormatRule<AnyCssAtRuleCustomName> for FormatAnyCssAtRuleCustomName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssAtRuleCustomName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssAtRuleCustomName::CssBogusCustomIdentifier(node) => node.format().fmt(f),
            AnyCssAtRuleCustomName::CssCustomIdentifier(node) => node.format().fmt(f),
            AnyCssAtRuleCustomName::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
        }
    }
}
