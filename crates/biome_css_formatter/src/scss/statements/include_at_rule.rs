use crate::prelude::*;
use biome_css_syntax::{ScssIncludeAtRule, ScssIncludeAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssIncludeAtRule;

impl FormatNodeRule<ScssIncludeAtRule> for FormatScssIncludeAtRule {
    fn fmt_fields(&self, node: &ScssIncludeAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssIncludeAtRuleFields {
            include_token,
            name,
            arguments,
            block,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                include_token.format(),
                space(),
                name.format(),
                arguments.format()
            ]
        )?;

        if let Some(block) = block {
            write!(f, [space(), block.format()])?;

            if let Some(semicolon_token) = semicolon_token {
                write!(f, [format_removed(&semicolon_token)])?;
            }
        } else {
            write!(f, [semicolon_token.format()])?;
        }

        Ok(())
    }
}
