use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::JsxSingleTagRoot;
use biome_js_syntax::JsxSingleTagRootFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsxSingleTagRoot;
impl FormatNodeRule<JsxSingleTagRoot> for FormatJsxSingleTagRoot {
    fn fmt_fields(&self, node: &JsxSingleTagRoot, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxSingleTagRootFields { tag, eof_token } = node.as_fields();

        write!(f, [tag.format(), eof_token.format()])
    }
}
