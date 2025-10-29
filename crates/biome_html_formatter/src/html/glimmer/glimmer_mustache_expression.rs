use crate::prelude::*;
use biome_html_syntax::GlimmerMustacheExpression;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerMustacheExpression;

impl FormatNodeRule<GlimmerMustacheExpression> for FormatGlimmerMustacheExpression {
    fn fmt_fields(&self, node: &GlimmerMustacheExpression, f: &mut HtmlFormatter) -> FormatResult<()> {
        // Stub: Just preserve the original text for now
        write!(f, [format_verbatim(node.syntax())])
    }
}
