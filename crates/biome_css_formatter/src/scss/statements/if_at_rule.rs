use crate::prelude::*;
use biome_css_syntax::{ScssIfAtRule, ScssIfAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssIfAtRule;

impl FormatNodeRule<ScssIfAtRule> for FormatScssIfAtRule {
    fn fmt_fields(&self, node: &ScssIfAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssIfAtRuleFields {
            if_token,
            condition,
            block,
            else_clause,
        } = node.as_fields();

        write!(
            f,
            [
                if_token.format(),
                space(),
                condition.format(),
                space(),
                block.format()
            ]
        )?;

        if let Some(else_clause) = else_clause {
            write!(f, [space(), else_clause.format()])?;
        }

        Ok(())
    }
}
