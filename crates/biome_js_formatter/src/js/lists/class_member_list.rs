use crate::context::{ConsistentQuoteProperties, QuoteProperties};
use crate::prelude::*;
use crate::utils::object::class_member_list_requires_quotes;
use biome_js_syntax::JsClassMemberList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsClassMemberList;

impl FormatRule<JsClassMemberList> for FormatJsClassMemberList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsClassMemberList, f: &mut JsFormatter) -> FormatResult<()> {
        let needs_quotes = f.options().quote_properties() == QuoteProperties::Consistent
            && class_member_list_requires_quotes(node, f.options().source_type().into());

        let previous = f.context().consistent_quote_properties();
        f.context_mut()
            .set_consistent_quote_properties(Some(ConsistentQuoteProperties::new(
                node.syntax().clone(),
                needs_quotes,
            )));

        let mut join = f.join_nodes_with_hardline();

        for member in node {
            join.entry(member.syntax(), &format_or_verbatim(member.format()));
        }

        let result = join.finish();

        f.context_mut().set_consistent_quote_properties(previous);

        result
    }
}
