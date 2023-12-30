use std::borrow::Cow;

use crate::prelude::*;
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

                // Unlike almost all other usages of regular identifiers,
                // attribute values are case-sensitive, so the identifier here
                // does not get converted to lowercase. Once it's quoted, it
                // will be parsed as a CssString on the next pass, at which
                // point casing is preserved no matter what.
                let value = ident.value_token()?;
                let quoted = std::format!("\"{}\"", value.text_trimmed());

                write!(
                    f,
                    [
                        format_leading_comments(ident.syntax()),
                        format_replaced(
                            &value,
                            &syntax_token_cow_slice(
                                Cow::Owned(quoted),
                                &value,
                                value.text_trimmed_range().start()
                            )
                        ),
                        format_trailing_comments(ident.syntax()),
                        format_dangling_comments(ident.syntax())
                    ]
                )
            }
        }
    }
}
