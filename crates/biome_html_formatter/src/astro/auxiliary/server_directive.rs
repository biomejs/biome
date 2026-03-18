use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::AstroServerDirective;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroServerDirective;
impl FormatNodeRule<AstroServerDirective> for FormatAstroServerDirective {
    fn fmt_fields(&self, node: &AstroServerDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        let fields = node.as_fields();
        write!(f, [fields.server_token.format(), fields.value.format()])
    }
}
