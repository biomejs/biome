use biome_css_syntax::CssFunctionParameterList;
use biome_formatter::separated::TrailingSeparator;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFunctionParameterList;

impl FormatRule<CssFunctionParameterList> for FormatCssFunctionParameterList {
    type Context = CssFormatContext;

    fn fmt(&self, node: &CssFunctionParameterList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(
                node.format_separated(",")
                    .with_trailing_separator(TrailingSeparator::Allowed),
            )
            .finish()

        // let separator = soft_line_break_or_space();
        // let mut joiner = f.join_with(&separator);
        //
        // for formatted in node.format_separated(",") {
        //     joiner.entry(&group(&indent(&formatted)));
        // }
        //
        // joiner.finish()
    }
}
