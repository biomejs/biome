//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssRule;
impl FormatRule<AnyCssRule> for FormatAnyCssRule {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssRule, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssRule::CssAtRule(node) => node.format().fmt(f),
            AnyCssRule::CssBogusRule(node) => node.format().fmt(f),
            AnyCssRule::CssNestedQualifiedRule(node) => node.format().fmt(f),
            AnyCssRule::CssQualifiedRule(node) => node.format().fmt(f),
        }
    }
}
