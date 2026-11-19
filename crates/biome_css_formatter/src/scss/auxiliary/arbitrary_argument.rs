use crate::prelude::*;
use biome_css_syntax::{ScssArbitraryArgument, ScssArbitraryArgumentFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssArbitraryArgument;
impl FormatNodeRule<ScssArbitraryArgument> for FormatScssArbitraryArgument {
    fn fmt_fields(&self, node: &ScssArbitraryArgument, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssArbitraryArgumentFields {
            value,
            dotdotdot_token,
        } = node.as_fields();

        let value = value?;

        if let Some(identifier) = value
            .as_any_css_value()
            .and_then(|value| value.as_css_identifier())
        {
            write!(
                f,
                [
                    identifier.format().with_text_case(CssCase::Preserve),
                    dotdotdot_token.format()
                ]
            )
        } else {
            write!(f, [value.format(), dotdotdot_token.format()])
        }
    }
}
