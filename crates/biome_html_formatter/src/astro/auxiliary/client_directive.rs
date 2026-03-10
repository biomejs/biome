use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::AstroClientDirective;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroClientDirective;
impl FormatNodeRule<AstroClientDirective> for FormatAstroClientDirective {
    fn fmt_fields(&self, node: &AstroClientDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        let fields = node.as_fields();
        write!(f, [fields.client_token.format(), fields.value.format()])
    }
}
