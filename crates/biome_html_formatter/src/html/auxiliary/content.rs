use crate::prelude::*;
use crate::verbatim::format_html_verbatim_node;
use biome_deserialize::TextRange;
use biome_html_syntax::{HtmlContent, HtmlElement};
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlContent;
impl FormatNodeRule<HtmlContent> for FormatHtmlContent {
    fn fmt_fields(&self, node: &HtmlContent, f: &mut HtmlFormatter) -> FormatResult<()> {
        dbg!("here???");
        format_html_verbatim_node(node.syntax()).fmt(f)
    }

    fn embedded_node_range(&self, node: &HtmlContent) -> Option<TextRange> {
        node.syntax()
            .grand_parent()
            .and_then(HtmlElement::cast)
            .and_then(|node| node.opening_element().ok())
            .and_then(|opening_element| opening_element.name().ok())
            .and_then(|name| name.value_token().ok())
            .and_then(|value_token| {
                if value_token.text_trimmed() == "script" || value_token.text_trimmed() == "style" {
                    // Need to return the range of the entire node
                    Some(node.range())
                } else {
                    None
                }
            })
    }
}
