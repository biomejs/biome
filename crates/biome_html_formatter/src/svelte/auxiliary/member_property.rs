use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteMemberProperty, SvelteMemberPropertyFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteMemberProperty;
impl FormatNodeRule<SvelteMemberProperty> for FormatSvelteMemberProperty {
    fn fmt_fields(&self, node: &SvelteMemberProperty, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteMemberPropertyFields {
            object,
            dot_token,
            member,
        } = node.as_fields();

        write!(f, [object.format(), dot_token.format(), member.format()])
    }
}
