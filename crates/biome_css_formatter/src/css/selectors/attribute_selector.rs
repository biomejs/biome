use crate::prelude::*;
use biome_css_syntax::{CssAttributeSelector, CssAttributeSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttributeSelector;
impl FormatNodeRule<CssAttributeSelector> for FormatCssAttributeSelector {
    fn fmt_fields(&self, node: &CssAttributeSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let CssAttributeSelectorFields {
            l_brack_token,
            name,
            matcher,
            r_brack_token,
        } = node.as_fields();

        let maybe_space = format_with(|f: &mut CssFormatter| {
            if f.options().delimiter_spacing().value() {
                write!(f, [space()])?;
            }
            Ok(())
        });

        write!(
            f,
            [
                l_brack_token.format(),
                maybe_space,
                name.format(),
                matcher.format(),
                maybe_space,
                r_brack_token.format()
            ]
        )
    }
}
