use crate::prelude::*;
use biome_js_syntax::TsTypeArgumentList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeArgumentList;

impl FormatRule<TsTypeArgumentList> for FormatTsTypeArgumentList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsTypeArgumentList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(
                node.format_separated(",")
                    .with_trailing_separator(TrailingSeparator::Disallowed),
            )
            .finish()
    }
}
