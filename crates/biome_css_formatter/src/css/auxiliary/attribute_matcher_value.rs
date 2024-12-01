use crate::{
    prelude::*,
    utils::string_utils::{FormatLiteralStringToken, StringLiteralParentKind},
};
use biome_css_syntax::{
    AnyCssAttributeMatcherValue, CssAttributeMatcherValue, CssAttributeMatcherValueFields,
};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttributeMatcherValue;
impl FormatNodeRule<CssAttributeMatcherValue> for FormatCssAttributeMatcherValue {
    fn fmt_fields(
        &self,
        node: &CssAttributeMatcherValue,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssAttributeMatcherValueFields { name } = node.as_fields();

        // All attribute values get quoted, no matter what. Strings already
        // have the quotes around them, but identifiers need to have quotes
        // added.
        match name? {
            AnyCssAttributeMatcherValue::CssString(string) => {
                write!(f, [string.format()])
            }
            AnyCssAttributeMatcherValue::CssIdentifier(ident) => {
                if f.comments().is_suppressed(ident.syntax()) {
                    return write!(f, [ident.format()]);
                }

                write!(
                    f,
                    [
                        format_leading_comments(ident.syntax()),
                        // Unlike almost all other usages of regular identifiers,
                        // attribute values are case-sensitive, so the identifier here
                        // does not get converted to lowercase. Once it's quoted, it
                        // will be parsed as a CssString on the next pass, at which
                        // point casing is preserved no matter what.
                        FormatLiteralStringToken::new(
                            &ident.value_token()?,
                            StringLiteralParentKind::Others
                        ),
                        format_trailing_comments(ident.syntax()),
                        format_dangling_comments(ident.syntax())
                    ]
                )
            }
        }
    }
}
