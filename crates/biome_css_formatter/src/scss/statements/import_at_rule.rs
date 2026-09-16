use crate::prelude::*;
use crate::utils::statement_at_rule_ending::FormatStatementAtRuleEnding;
use biome_css_syntax::{ScssImportAtRule, ScssImportAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssImportAtRule;

impl FormatNodeRule<ScssImportAtRule> for FormatScssImportAtRule {
    fn fmt_fields(&self, node: &ScssImportAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssImportAtRuleFields {
            import_token,
            imports,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                import_token.format()?.with_text_case(CssCase::Lowercase),
                space(),
                group(&indent(&imports.format())),
                FormatStatementAtRuleEnding::new(node.syntax(), semicolon_token)
            ]
        )
    }

    fn fmt_dangling_comments(
        &self,
        _node: &ScssImportAtRule,
        _f: &mut CssFormatter,
    ) -> FormatResult<()> {
        Ok(())
    }
}
