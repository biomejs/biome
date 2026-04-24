use crate::prelude::*;
use biome_css_syntax::{ScssUseAtRule, ScssUseAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssUseAtRule;

impl FormatNodeRule<ScssUseAtRule> for FormatScssUseAtRule {
    fn fmt_fields(&self, node: &ScssUseAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssUseAtRuleFields {
            use_token,
            url,
            as_clause,
            with_clause,
            semicolon_token,
        } = node.as_fields();

        write!(f, [use_token.format(), space(), url.format()])?;

        if let Some(as_clause) = as_clause {
            write!(f, [space(), as_clause.format()])?;
        }

        if let Some(with_clause) = with_clause {
            write!(f, [space(), with_clause.format()])?;
        }

        write!(f, [semicolon_token.format()])
    }
}
