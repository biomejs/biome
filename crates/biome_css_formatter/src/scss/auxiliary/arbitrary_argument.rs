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

        write!(f, [value.format(), dotdotdot_token.format()])
    }
}
