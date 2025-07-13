//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssDeclarationOrRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssDeclarationOrRule;
impl FormatRule<AnyCssDeclarationOrRule> for FormatAnyCssDeclarationOrRule {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssDeclarationOrRule, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssDeclarationOrRule::AnyCssDeclarationWithSemicolon(node) => node.format().fmt(f),
            AnyCssDeclarationOrRule::AnyCssRule(node) => node.format().fmt(f),
            AnyCssDeclarationOrRule::CssBogus(node) => node.format().fmt(f),
            AnyCssDeclarationOrRule::CssMetavariable(node) => node.format().fmt(f),
        }
    }
}
