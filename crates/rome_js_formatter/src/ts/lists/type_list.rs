use crate::prelude::*;
use biome_js_syntax::TsTypeList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeList;

impl FormatRule<TsTypeList> for FormatTsTypeList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsTypeList, f: &mut JsFormatter) -> FormatResult<()> {
        // the grouping will be applied by the parent
        f.join_with(&soft_line_break_or_space())
            .entries(
                node.format_separated(",")
                    .with_trailing_separator(TrailingSeparator::Disallowed),
            )
            .finish()
    }
}
