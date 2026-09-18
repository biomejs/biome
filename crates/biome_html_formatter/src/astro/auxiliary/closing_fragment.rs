use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{AstroClosingFragment, AstroClosingFragmentFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroClosingFragment;
impl FormatNodeRule<AstroClosingFragment> for FormatAstroClosingFragment {
    fn fmt_fields(&self, node: &AstroClosingFragment, f: &mut HtmlFormatter) -> FormatResult<()> {
        let AstroClosingFragmentFields {
            l_angle_token,
            slash_token,
            r_angle_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_angle_token.format(),
                slash_token.format(),
                r_angle_token.format()
            ]
        )
    }
}
