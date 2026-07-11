use crate::prelude::*;
use crate::utils::import::FormatImportClause;
use biome_css_syntax::{CssImportAtRule, CssImportAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssImportAtRule;
impl FormatNodeRule<CssImportAtRule> for FormatCssImportAtRule {
    fn fmt_fields(&self, node: &CssImportAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssImportAtRuleFields {
            import_token,
            url,
            layer,
            supports,
            media,
            semicolon_token,
        } = node.as_fields();

        write!(f, [import_token.format(), space()])?;
        let import_clause = FormatImportClause::new(url, layer, supports, media);
        write!(f, [group(&indent(&import_clause))])?;

        write!(f, [semicolon_token.format()])
    }
}
