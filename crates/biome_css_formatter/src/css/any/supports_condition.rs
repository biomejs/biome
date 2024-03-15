//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSupportsCondition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSupportsCondition;
impl FormatRule<AnyCssSupportsCondition> for FormatAnyCssSupportsCondition {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssSupportsCondition, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssSupportsCondition::AnyCssSupportsInParens(node) => node.format().fmt(f),
            AnyCssSupportsCondition::CssSupportsAndCondition(node) => node.format().fmt(f),
            AnyCssSupportsCondition::CssSupportsNotCondition(node) => node.format().fmt(f),
            AnyCssSupportsCondition::CssSupportsOrCondition(node) => node.format().fmt(f),
        }
    }
}
