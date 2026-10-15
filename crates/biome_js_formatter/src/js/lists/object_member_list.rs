use crate::context::trailing_commas::FormatTrailingCommas;
use crate::context::{ConsistentQuoteProperties, QuoteProperties};
use crate::prelude::*;
use crate::utils::object::object_member_list_requires_quotes;
use biome_js_syntax::JsObjectMemberList;
use biome_rowan::{AstNode, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsObjectMemberList;

impl FormatRule<JsObjectMemberList> for FormatJsObjectMemberList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsObjectMemberList, f: &mut JsFormatter) -> FormatResult<()> {
        let trailing_separator = FormatTrailingCommas::ES5.trailing_separator(f.options());

        let needs_quotes = f.options().quote_properties() == QuoteProperties::Consistent
            && object_member_list_requires_quotes(node, f.options().source_type().into());

        let previous = f.context().consistent_quote_properties();
        f.context_mut()
            .set_consistent_quote_properties(Some(ConsistentQuoteProperties::new(
                node.syntax().clone(),
                needs_quotes,
            )));

        let mut join = f.join_nodes_with_soft_line();

        for (element, formatted) in node.elements().zip(
            node.format_separated(",")
                .with_trailing_separator(trailing_separator),
        ) {
            join.entry(element.node()?.syntax(), &formatted);
        }

        let result = join.finish();

        f.context_mut().set_consistent_quote_properties(previous);

        result
    }
}
