use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsGlimmerTemplate, JsGlimmerTemplateFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsGlimmerTemplate;

impl FormatNodeRule<JsGlimmerTemplate> for FormatJsGlimmerTemplate {
    /// Formats the fields of a `JsGlimmerTemplate` into the provided formatter.
    ///
    /// This writes the template token field of the node to `f`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use biome_js_formatter::js::auxiliary::FormatJsGlimmerTemplate;
    /// use biome_js_syntax::JsGlimmerTemplate;
    /// use biome_js_formatter::JsFormatter;
    ///
    /// let formatter = &mut JsFormatter::default();
    /// let node: JsGlimmerTemplate = /* obtain or parse a Glimmer template node */;
    /// let rule = FormatJsGlimmerTemplate::default();
    /// rule.fmt_fields(&node, formatter).unwrap();
    /// ```
    fn fmt_fields(&self, node: &JsGlimmerTemplate, f: &mut JsFormatter) -> FormatResult<()> {
        let JsGlimmerTemplateFields {
            template_token_token,
        } = node.as_fields();

        write![f, [template_token_token.format()]]
    }
}