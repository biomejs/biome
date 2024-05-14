use crate::prelude::*;

use crate::context::trailing_commas::FormatTrailingCommas;
use biome_js_syntax::TsEnumMemberList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsEnumMemberList;

impl FormatRule<TsEnumMemberList> for FormatTsEnumMemberList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsEnumMemberList, f: &mut JsFormatter) -> FormatResult<()> {
        let trailing_separator = FormatTrailingCommas::ES5.trailing_separator(f.options());
        let mut joiner = f.join_nodes_with_soft_line();

        for variant in node
            .format_separated(",")
            .with_trailing_separator(trailing_separator)
            .nodes_grouped()
        {
            joiner.entry(variant.node()?.syntax(), &variant)
        }

        joiner.finish()
    }
}
