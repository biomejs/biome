use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::AstroDirectiveValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroDirectiveValue;
impl FormatNodeRule<AstroDirectiveValue> for FormatAstroDirectiveValue {
    fn fmt_fields(&self, node: &AstroDirectiveValue, f: &mut HtmlFormatter) -> FormatResult<()> {
        let fields = node.as_fields();
        write!(
            f,
            [
                fields.colon_token_token.format(),
                fields.name.format(),
                fields.initializer.format()
            ]
        )
    }
}
