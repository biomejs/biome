use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritBubbleScope, GritBubbleScopeFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBubbleScope;
impl FormatNodeRule<GritBubbleScope> for FormatGritBubbleScope {
    fn fmt_fields(&self, node: &GritBubbleScope, f: &mut GritFormatter) -> FormatResult<()> {
        let GritBubbleScopeFields {
            l_paren_token,
            variables,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_paren_token.format(),
                variables.format(),
                r_paren_token.format()
            ]
        )
    }
}
