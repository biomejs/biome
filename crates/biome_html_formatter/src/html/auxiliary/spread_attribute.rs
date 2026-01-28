use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlSpreadAttribute, HtmlSpreadAttributeFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlSpreadAttribute;
impl FormatNodeRule<HtmlSpreadAttribute> for FormatHtmlSpreadAttribute {
    fn fmt_fields(&self, node: &HtmlSpreadAttribute, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlSpreadAttributeFields {
            l_curly_token,
            dotdotdot_token,
            argument,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_curly_token.format(),
                dotdotdot_token.format(),
                argument.format(),
                r_curly_token.format()
            ]
        )
    }
}
