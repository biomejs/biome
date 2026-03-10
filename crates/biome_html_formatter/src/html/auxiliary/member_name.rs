use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlMemberName, HtmlMemberNameFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlMemberName;
impl FormatNodeRule<HtmlMemberName> for FormatHtmlMemberName {
    fn fmt_fields(&self, node: &HtmlMemberName, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlMemberNameFields {
            object,
            member,
            dot_token,
        } = node.as_fields();

        write!(f, [object.format(), dot_token.format(), member.format()])
    }
}
