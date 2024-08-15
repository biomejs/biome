use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};
use biome_formatter::write;
use biome_js_syntax::{JsSyntaxKind, TsLiteralEnumMemberName, TsLiteralEnumMemberNameFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsLiteralEnumMemberName;

impl FormatNodeRule<TsLiteralEnumMemberName> for FormatTsLiteralEnumMemberName {
    fn fmt_fields(&self, node: &TsLiteralEnumMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        let TsLiteralEnumMemberNameFields { value } = node.as_fields();
        let value = value?;
        match value.kind() {
            JsSyntaxKind::JS_STRING_LITERAL => {
                write![
                    f,
                    [FormatLiteralStringToken::new(
                        &value,
                        StringLiteralParentKind::Member
                    )]
                ]
            }
            _ => write![f, [value.format()]],
        }
    }
}
