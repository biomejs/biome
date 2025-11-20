use crate::prelude::*;
use biome_css_syntax::{
    CssViewTransitionAtRuleDeclarator, CssViewTransitionAtRuleDeclaratorFields,
};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssViewTransitionAtRuleDeclarator;

impl FormatNodeRule<CssViewTransitionAtRuleDeclarator> for FormatCssViewTransitionAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssViewTransitionAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssViewTransitionAtRuleDeclaratorFields {
            view_transition_token,
        } = node.as_fields();

        write!(f, [view_transition_token.format()])
    }
}
