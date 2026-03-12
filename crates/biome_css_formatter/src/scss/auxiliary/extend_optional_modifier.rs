use crate::prelude::*;
use biome_css_syntax::{ScssExtendOptionalModifier, ScssExtendOptionalModifierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssExtendOptionalModifier;
impl FormatNodeRule<ScssExtendOptionalModifier> for FormatScssExtendOptionalModifier {
    fn fmt_fields(
        &self,
        node: &ScssExtendOptionalModifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssExtendOptionalModifierFields {
            excl_token,
            optional_token,
        } = node.as_fields();

        write!(f, [excl_token.format(), optional_token.format()])
    }
}
