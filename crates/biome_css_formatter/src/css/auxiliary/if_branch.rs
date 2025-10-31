use crate::prelude::*;
use biome_css_syntax::{CssIfBranch, CssIfBranchFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfBranch;

impl FormatNodeRule<CssIfBranch> for FormatCssIfBranch {
    fn fmt_fields(&self, node: &CssIfBranch, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIfBranchFields {
            condition,
            colon_token,
            value,
        } = node.as_fields();

        write!(
            f,
            [
                condition.format(),
                colon_token.format(),
                space(),
                value.format()
            ]
        )
    }
}
