//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssAtRuleDashedName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAtRuleDashedName;
impl FormatRule<AnyCssAtRuleDashedName> for FormatAnyCssAtRuleDashedName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssAtRuleDashedName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssAtRuleDashedName::CssBogusCustomIdentifier(node) => node.format().fmt(f),
            AnyCssAtRuleDashedName::CssDashedIdentifier(node) => node.format().fmt(f),
            AnyCssAtRuleDashedName::ScssInterpolatedDashedIdentifier(node) => node.format().fmt(f),
            AnyCssAtRuleDashedName::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
        }
    }
}
