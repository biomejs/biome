use crate::prelude::*;
use biome_css_syntax::{ScssInterpolatedDashedIdentifier, ScssInterpolatedDashedIdentifierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedDashedIdentifier;
impl FormatNodeRule<ScssInterpolatedDashedIdentifier> for FormatScssInterpolatedDashedIdentifier {
    fn fmt_fields(
        &self,
        node: &ScssInterpolatedDashedIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssInterpolatedDashedIdentifierFields { items } = node.as_fields();

        write!(f, [items.format()])
    }
}
