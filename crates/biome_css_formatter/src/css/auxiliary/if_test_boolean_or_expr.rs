use crate::prelude::*;
use biome_css_syntax::{CssIfTestBooleanOrExpr, CssIfTestBooleanOrExprFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfTestBooleanOrExpr;

impl FormatNodeRule<CssIfTestBooleanOrExpr> for FormatCssIfTestBooleanOrExpr {
    fn fmt_fields(&self, node: &CssIfTestBooleanOrExpr, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIfTestBooleanOrExprFields {
            left,
            or_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                or_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
