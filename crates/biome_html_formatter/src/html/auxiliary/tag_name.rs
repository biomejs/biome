use crate::{
    prelude::*,
    utils::{formatters::FormatTokenAsLowercase, metadata::is_canonical_html_tag},
};
use biome_formatter::write;
use biome_html_syntax::{HtmlTagName, HtmlTagNameFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlTagName;
impl FormatNodeRule<HtmlTagName> for FormatHtmlTagName {
    fn fmt_fields(&self, node: &HtmlTagName, f: &mut HtmlFormatter) -> FormatResult<()> {
        // TODO: maybe move this check to a parent node so we aren't checking this twice per tag?
        let is_canonical_html_tag = is_canonical_html_tag(node);
        let HtmlTagNameFields { value_token } = node.as_fields();

        if is_canonical_html_tag {
            write!(f, [FormatTokenAsLowercase::from(value_token?)])
        } else {
            write![f, [value_token.format()]]
        }
    }
}
