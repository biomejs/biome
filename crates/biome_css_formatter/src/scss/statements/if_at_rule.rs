use crate::prelude::*;
use biome_css_syntax::{ScssIfAtRule, ScssIfAtRuleFields};
use biome_formatter::{format_args, write};

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

        let header_group_id = f.group_id("scss_if_header");

        write!(
            f,
            [
                if_token.format(),
                space(),
                group(&format_args![
                    indent_if_group_breaks(&condition.format(), header_group_id),
                    soft_line_break_or_space()
                ])
                .with_group_id(Some(header_group_id)),
                block.format()
            ]
        )?;

        if let Some(else_clause) = else_clause {
            if f.comments().has_leading_comments(else_clause.syntax()) {
                write!(f, [hard_line_break(), else_clause.format()])?;
            } else {
                write!(f, [space(), else_clause.format()])?;
            }
        }

        Ok(())
    }
}
