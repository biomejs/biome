use std::borrow::Cow;

use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{TsBigintLiteralType, TsBigintLiteralTypeFields};
use biome_string_case::StrLikeExtension;

#[derive(Debug, Clone, Default)]
pub struct FormatTsBigintLiteralType;

impl FormatNodeRule<TsBigintLiteralType> for FormatTsBigintLiteralType {
    fn fmt_fields(&self, node: &TsBigintLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsBigintLiteralTypeFields {
            minus_token,
            literal_token,
        } = node.as_fields();
        write![f, [minus_token.format()]]?;
        let literal_token = literal_token?;

        let original = literal_token.text_trimmed();
        match original.to_ascii_lowercase_cow() {
            Cow::Borrowed(_) => write![f, [literal_token.format()]],
            Cow::Owned(lowercase) => {
                write!(
                    f,
                    [format_replaced(
                        &literal_token,
                        &dynamic_text(&lowercase, literal_token.text_trimmed_range().start())
                    )]
                )
            }
        }
    }
}
