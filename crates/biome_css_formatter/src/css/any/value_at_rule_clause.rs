//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssValueAtRuleClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssValueAtRuleClause;
impl FormatRule<AnyCssValueAtRuleClause> for FormatAnyCssValueAtRuleClause {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssValueAtRuleClause, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssValueAtRuleClause::CssValueAtRuleDeclarationClause(node) => node.format().fmt(f),
            AnyCssValueAtRuleClause::CssValueAtRuleImportClause(node) => node.format().fmt(f),
        }
    }
}
