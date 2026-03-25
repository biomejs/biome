use crate::prelude::*;
use crate::utils::import::write_import_payload;
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
        write_import_payload(f, &url, layer.as_ref(), supports.as_ref(), &media)?;

        write!(f, [semicolon_token.format()])
    }
}
