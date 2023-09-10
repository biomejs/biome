use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};

use biome_js_syntax::JsModuleSource;
use biome_js_syntax::JsModuleSourceFields;
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsModuleSource;

impl FormatNodeRule<JsModuleSource> for FormatJsModuleSource {
    fn fmt_fields(&self, node: &JsModuleSource, f: &mut JsFormatter) -> FormatResult<()> {
        let JsModuleSourceFields { value_token } = node.as_fields();

        write!(
            f,
            [FormatLiteralStringToken::new(
                &value_token?,
                StringLiteralParentKind::Expression
            )]
        )
    }
}
