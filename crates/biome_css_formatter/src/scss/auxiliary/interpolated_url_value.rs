use crate::prelude::*;
use crate::verbatim::{CssVerbatimTokenFormat, format_css_verbatim_range};
use biome_css_syntax::ScssInterpolatedUrlValue;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedUrlValue;

impl FormatNodeRule<ScssInterpolatedUrlValue> for FormatScssInterpolatedUrlValue {
    fn fmt_fields(
        &self,
        node: &ScssInterpolatedUrlValue,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_css_verbatim_range(
            node.syntax(),
            node.syntax().text_trimmed_range(),
            |_, _, _| Ok(CssVerbatimTokenFormat::Source),
        )
        .fmt(f)
    }
}
