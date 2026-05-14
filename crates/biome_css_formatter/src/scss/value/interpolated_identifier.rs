use crate::prelude::*;
use biome_css_syntax::{ScssInterpolatedIdentifier, ScssInterpolatedIdentifierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedIdentifier;
impl FormatNodeRule<ScssInterpolatedIdentifier> for FormatScssInterpolatedIdentifier {
    fn fmt_fields(
        &self,
        node: &ScssInterpolatedIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssInterpolatedIdentifierFields { items } = node.as_fields();

        write!(f, [items.format()])
    }
}
