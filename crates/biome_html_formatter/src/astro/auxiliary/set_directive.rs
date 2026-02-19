use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::AstroSetDirective;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroSetDirective;
impl FormatNodeRule<AstroSetDirective> for FormatAstroSetDirective {
    fn fmt_fields(&self, node: &AstroSetDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        let fields = node.as_fields();
        write!(f, [fields.set_token.format(), fields.value.format()])
    }
}
