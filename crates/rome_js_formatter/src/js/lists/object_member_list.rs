use crate::context::trailing_comma::FormatTrailingComma;
use crate::prelude::*;
use biome_rowan::{AstNode, AstSeparatedList};
use rome_js_syntax::JsObjectMemberList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsObjectMemberList;

impl FormatRule<JsObjectMemberList> for FormatJsObjectMemberList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsObjectMemberList, f: &mut JsFormatter) -> FormatResult<()> {
        let trailing_separator = FormatTrailingComma::ES5.trailing_separator(f.options());

        let mut join = f.join_nodes_with_soft_line();

        for (element, formatted) in node.elements().zip(
            node.format_separated(",")
                .with_trailing_separator(trailing_separator),
        ) {
            join.entry(element.node()?.syntax(), &formatted);
        }

        join.finish()
    }
}
