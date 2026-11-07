use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{JsSvelteLegacyConstRoot, JsSvelteLegacyConstRootFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsSvelteLegacyConstRoot;
impl FormatNodeRule<JsSvelteLegacyConstRoot> for FormatJsSvelteLegacyConstRoot {
    fn fmt_fields(&self, node: &JsSvelteLegacyConstRoot, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSvelteLegacyConstRootFields {
            declarator,
            eof_token,
        } = node.as_fields();

        write!(f, [declarator.format(), format_removed(&eof_token?)])
    }
}
