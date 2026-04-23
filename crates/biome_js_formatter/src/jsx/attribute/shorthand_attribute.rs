use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsxShorthandAttribute, JsxShorthandAttributeFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsxShorthandAttribute;

impl FormatNodeRule<JsxShorthandAttribute> for FormatJsxShorthandAttribute {
    fn fmt_fields(&self, node: &JsxShorthandAttribute, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxShorthandAttributeFields {
            l_curly_token,
            name,
            r_curly_token,
        } = node.as_fields();

        write![
            f,
            [
                l_curly_token.format(),
                name.format(),
                r_curly_token.format()
            ]
        ]
    }
}
