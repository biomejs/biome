//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssDeclarationOrAtRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssDeclarationOrAtRule;
impl FormatRule<AnyCssDeclarationOrAtRule> for FormatAnyCssDeclarationOrAtRule {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssDeclarationOrAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssDeclarationOrAtRule::CssAtRule(node) => node.format().fmt(f),
            AnyCssDeclarationOrAtRule::CssDeclarationWithSemicolon(node) => node.format().fmt(f),
        }
    }
}
