use crate::prelude::*;
use biome_css_syntax::{ScssInterpolatedIdentifier, ScssInterpolatedIdentifierFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedIdentifier;
impl FormatNodeRule<ScssInterpolatedIdentifier> for FormatScssInterpolatedIdentifier {
    fn fmt_fields(
        &self,
        node: &ScssInterpolatedIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssInterpolatedIdentifierFields { items } = node.as_fields();
        items.format().fmt(f)
    }
}
