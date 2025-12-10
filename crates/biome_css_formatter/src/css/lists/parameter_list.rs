use biome_css_syntax::CssParameterList;
use biome_formatter::separated::TrailingSeparator;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssParameterList;

impl FormatRule<CssParameterList> for FormatCssParameterList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssParameterList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(
                node.format_separated(",")
                    .with_trailing_separator(TrailingSeparator::Disallowed),
            )
            .finish()
    }
}
