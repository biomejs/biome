use crate::prelude::*;
use biome_css_syntax::{CssNamespaceAtRule, CssNamespaceAtRuleFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNamespaceAtRule;
impl FormatNodeRule<CssNamespaceAtRule> for FormatCssNamespaceAtRule {
    fn fmt_fields(&self, node: &CssNamespaceAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssNamespaceAtRuleFields {
            namespace_token,
            prefix,
            url,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                namespace_token.format(),
                space(),
                prefix.format(),
                space(),
                url.format(),
                semicolon_token.format(),
            ]
        )
    }
}
