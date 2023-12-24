use crate::prelude::*;
use biome_css_syntax::{CssCharsetAtRule, CssCharsetAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCharsetAtRule;
impl FormatNodeRule<CssCharsetAtRule> for FormatCssCharsetAtRule {
    fn fmt_fields(&self, node: &CssCharsetAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssCharsetAtRuleFields {
            charset_token,
            encoding,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                charset_token.format(),
                space(),
                encoding.format(),
                semicolon_token.format()
            ]
        )
    }
}
