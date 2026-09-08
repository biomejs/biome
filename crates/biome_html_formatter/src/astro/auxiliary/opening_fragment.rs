use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{AstroOpeningFragment, AstroOpeningFragmentFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroOpeningFragment;
impl FormatNodeRule<AstroOpeningFragment> for FormatAstroOpeningFragment {
    fn fmt_fields(&self, node: &AstroOpeningFragment, f: &mut HtmlFormatter) -> FormatResult<()> {
        let AstroOpeningFragmentFields {
            l_angle_token,
            r_angle_token,
        } = node.as_fields();

        write!(f, [l_angle_token.format(), r_angle_token.format()])
    }
}
