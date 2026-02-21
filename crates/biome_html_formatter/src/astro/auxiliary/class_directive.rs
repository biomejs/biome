use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::AstroClassDirective;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroClassDirective;
impl FormatNodeRule<AstroClassDirective> for FormatAstroClassDirective {
    fn fmt_fields(&self, node: &AstroClassDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        let fields = node.as_fields();
        write!(f, [fields.class_token.format(), fields.value.format()])
    }
}
