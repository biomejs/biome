use crate::prelude::*;
use biome_css_syntax::ScssMapExpressionPairList;
use biome_formatter::separated::TrailingSeparator;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssMapExpressionPairList;
impl FormatRule<ScssMapExpressionPairList> for FormatScssMapExpressionPairList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &ScssMapExpressionPairList, f: &mut CssFormatter) -> FormatResult<()> {
        let separator = hard_line_break();

        let formatted = format_with(|formatter: &mut CssFormatter| {
            let mut joiner = formatter.join_with(&separator);

            let mut separated = node
                .format_separated(",")
                .with_trailing_separator(TrailingSeparator::Mandatory);

            for formatted in &mut separated {
                joiner.entry(&formatted);
            }

            joiner.finish()
        });

        write!(f, [group(&formatted)])
    }
}
