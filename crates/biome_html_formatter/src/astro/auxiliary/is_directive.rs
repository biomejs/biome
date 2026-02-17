use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::AstroIsDirective;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroIsDirective;
impl FormatNodeRule<AstroIsDirective> for FormatAstroIsDirective {
    fn fmt_fields(&self, node: &AstroIsDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        let fields = node.as_fields();
        write!(f, [fields.is_token.format(), fields.value.format()])
    }
}
