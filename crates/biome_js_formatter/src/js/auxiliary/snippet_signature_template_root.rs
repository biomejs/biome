use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsSnippetSignatureTemplateRoot, JsSnippetSignatureTemplateRootFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsSnippetSignatureTemplateRoot;
impl FormatNodeRule<JsSnippetSignatureTemplateRoot> for FormatJsSnippetSignatureTemplateRoot {
    fn fmt_fields(
        &self,
        node: &JsSnippetSignatureTemplateRoot,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsSnippetSignatureTemplateRootFields {
            name,
            eof_token,
            parameters,
        } = node.as_fields();

        write!(f, [name.format(), parameters.format(), eof_token.format()])
    }
}
