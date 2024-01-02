use crate::prelude::*;
use biome_css_syntax::{CssScopeEdge, CssScopeEdgeFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssScopeEdge;
impl FormatNodeRule<CssScopeEdge> for FormatCssScopeEdge {
    fn fmt_fields(&self, node: &CssScopeEdge, f: &mut CssFormatter) -> FormatResult<()> {
        let CssScopeEdgeFields {
            l_paren_token,
            selectors,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent(&selectors.format()),
                r_paren_token.format()
            ])]
        )
    }
}
