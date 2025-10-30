use crate::prelude::*;
use biome_css_syntax::CssIfBranchList;
use biome_formatter::separated::TrailingSeparator;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfBranchList;

impl FormatRule<CssIfBranchList> for FormatCssIfBranchList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssIfBranchList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(
                node.format_separated(";")
                    .with_trailing_separator(TrailingSeparator::Allowed),
            )
            .finish()
    }
}
