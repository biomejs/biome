use crate::context::trailing_commas::FormatTrailingCommas;
use crate::prelude::*;
use biome_js_syntax::JsExportNamedSpecifierList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsExportNamedSpecifierList;

impl FormatRule<JsExportNamedSpecifierList> for FormatJsExportNamedSpecifierList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsExportNamedSpecifierList, f: &mut JsFormatter) -> FormatResult<()> {
        let trailing_separator = FormatTrailingCommas::ES5.trailing_separator(f.options());

        f.join_with(&soft_line_break_or_space())
            .entries(
                node.format_separated(",")
                    .with_trailing_separator(trailing_separator),
            )
            .finish()
    }
}
