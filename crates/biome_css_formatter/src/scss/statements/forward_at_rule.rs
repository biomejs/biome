use crate::prelude::*;
use biome_css_syntax::{ScssForwardAtRule, ScssForwardAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssForwardAtRule;

impl FormatNodeRule<ScssForwardAtRule> for FormatScssForwardAtRule {
    fn fmt_fields(&self, node: &ScssForwardAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssForwardAtRuleFields {
            forward_token,
            url,
            as_clause,
            visibility_clause,
            with_clause,
            semicolon_token,
        } = node.as_fields();

        write!(f, [forward_token.format(), space(), url.format()])?;

        if let Some(as_clause) = as_clause {
            write!(f, [space(), as_clause.format()])?;
        }

        if let Some(visibility_clause) = visibility_clause {
            write!(f, [space(), visibility_clause.format()])?;
        }

        if let Some(with_clause) = with_clause {
            write!(f, [space(), with_clause.format()])?;
        }

        write!(f, [semicolon_token.format()])
    }
}
