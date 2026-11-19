use crate::prelude::*;
use biome_css_syntax::{TwThemePrefixOption, TwThemePrefixOptionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwThemePrefixOption;
impl FormatNodeRule<TwThemePrefixOption> for FormatTwThemePrefixOption {
    fn fmt_fields(&self, node: &TwThemePrefixOption, f: &mut CssFormatter) -> FormatResult<()> {
        let TwThemePrefixOptionFields {
            prefix_token,
            l_paren_token,
            name,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                prefix_token.format()?.with_text_case(CssCase::Preserve),
                l_paren_token.format(),
                name?.format().with_text_case(CssCase::Preserve),
                r_paren_token.format()
            ]
        )
    }
}
