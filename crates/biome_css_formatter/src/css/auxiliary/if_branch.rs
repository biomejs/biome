use crate::prelude::*;
use biome_css_syntax::{CssIfBranch, CssIfBranchFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfBranch;

impl FormatNodeRule<CssIfBranch> for FormatCssIfBranch {
    fn fmt_fields(&self, node: &CssIfBranch, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIfBranchFields {
            condition,
            colon_token,
            value,
        } = node.as_fields();
        let value = value?;

        write!(f, [condition.format(), colon_token.format()])?;

        if f.comments()
            .leading_comments(value.syntax())
            .iter()
            .any(|comment| comment.kind().is_line())
        {
            // Keep line comments before the value so they retain their suppression target.
            write!(
                f,
                [indent(&format_args![hard_line_break(), value.format()])]
            )
        } else {
            write!(f, [space(), value.format()])
        }
    }
}
