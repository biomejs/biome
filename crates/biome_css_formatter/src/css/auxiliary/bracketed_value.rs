use crate::prelude::*;
use biome_css_syntax::{CssBracketedValue, CssBracketedValueFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBracketedValue;
impl FormatNodeRule<CssBracketedValue> for FormatCssBracketedValue {
    fn fmt_fields(&self, node: &CssBracketedValue, f: &mut CssFormatter) -> FormatResult<()> {
        let CssBracketedValueFields {
            l_brack_token,
            items,
            r_brack_token,
        } = node.as_fields();

        let maybe_space = format_with(|f: &mut CssFormatter| {
            if f.options().delimiter_spacing().value() && !items.is_empty() {
                write!(f, [space()])?;
            }
            Ok(())
        });

        write!(
            f,
            [
                l_brack_token.format(),
                maybe_space,
                items.format(),
                maybe_space,
                r_brack_token.format()
            ]
        )
    }
}
