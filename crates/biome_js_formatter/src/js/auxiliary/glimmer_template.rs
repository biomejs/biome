use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsGlimmerTemplate, JsGlimmerTemplateFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsGlimmerTemplate;

impl FormatNodeRule<JsGlimmerTemplate> for FormatJsGlimmerTemplate {
    /// Formats a Glimmer template by writing its template token verbatim.
    ///
    /// This is a pass-through formatter that preserves the original template content.
    fn fmt_fields(&self, node: &JsGlimmerTemplate, f: &mut JsFormatter) -> FormatResult<()> {
        let JsGlimmerTemplateFields {
            template_token_token,
        } = node.as_fields();

        write![f, [template_token_token.format()]]
    }
}
