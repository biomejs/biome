use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::AstroDefineDirective;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroDefineDirective;
impl FormatNodeRule<AstroDefineDirective> for FormatAstroDefineDirective {
    fn fmt_fields(&self, node: &AstroDefineDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        let fields = node.as_fields();
        write!(f, [fields.define_token.format(), fields.value.format()])
    }
}
