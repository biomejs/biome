use crate::context::trailing_commas::FormatTrailingCommas;
use crate::prelude::*;
use biome_js_syntax::JsObjectMemberList;
use biome_rowan::{AstNode, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsObjectMemberList;

impl FormatRule<JsObjectMemberList> for FormatJsObjectMemberList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsObjectMemberList, f: &mut JsFormatter) -> FormatResult<()> {
        let trailing_separator = FormatTrailingCommas::ES5.trailing_separator(f.options());

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
