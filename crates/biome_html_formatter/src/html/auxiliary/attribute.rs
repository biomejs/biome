use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlAttribute, HtmlAttributeFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttribute;
impl FormatNodeRule<HtmlAttribute> for FormatHtmlAttribute {
    fn fmt_fields(&self, node: &HtmlAttribute, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlAttributeFields { name, initializer } = node.as_fields();

        write![f, [name.format(), initializer.format()]]
    }
}
