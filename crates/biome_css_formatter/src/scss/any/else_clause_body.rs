//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssElseClauseBody;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssElseClauseBody;
impl FormatRule<AnyScssElseClauseBody> for FormatAnyScssElseClauseBody {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssElseClauseBody, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssElseClauseBody::CssDeclarationOrRuleBlock(node) => node.format().fmt(f),
            AnyScssElseClauseBody::ScssIfAtRule(node) => node.format().fmt(f),
        }
    }
}
