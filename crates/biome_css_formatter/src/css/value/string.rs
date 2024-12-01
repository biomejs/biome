use crate::{
    prelude::*,
    utils::string_utils::{FormatLiteralStringToken, StringLiteralParentKind},
};
use biome_css_syntax::{CssString, CssStringFields, CssSyntaxKind};
use biome_formatter::write;
use biome_rowan::SyntaxNodeOptionExt;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssString;
impl FormatNodeRule<CssString> for FormatCssString {
    fn fmt_fields(&self, node: &CssString, f: &mut CssFormatter) -> FormatResult<()> {
        let CssStringFields { value_token } = node.as_fields();
        if matches!(
            node.syntax().parent().kind(),
            Some(CssSyntaxKind::CSS_CHARSET_AT_RULE)
        ) {
            write!(
                f,
                [FormatLiteralStringToken::new(
                    &value_token?,
                    StringLiteralParentKind::CharsetAtRule
                )]
            )
        } else {
            write!(
                f,
                [FormatLiteralStringToken::new(
                    &value_token?,
                    StringLiteralParentKind::Others
                )]
            )
        }
    }
}
