use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsReferenceIdentifier;
use biome_js_syntax::JsReferenceIdentifierFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsReferenceIdentifier;

impl FormatNodeRule<JsReferenceIdentifier> for FormatJsReferenceIdentifier {
    fn fmt_fields(&self, node: &JsReferenceIdentifier, f: &mut JsFormatter) -> FormatResult<()> {
        let JsReferenceIdentifierFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
