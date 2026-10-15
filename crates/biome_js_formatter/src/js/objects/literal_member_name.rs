use crate::prelude::*;
use crate::utils::object::{literal_member_name_needs_quotes, write_literal_member_name_value};

use biome_js_syntax::JsLiteralMemberName;
use biome_js_syntax::JsLiteralMemberNameFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsLiteralMemberName;

impl FormatNodeRule<JsLiteralMemberName> for FormatJsLiteralMemberName {
    fn fmt_fields(&self, node: &JsLiteralMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsLiteralMemberNameFields { value } = node.as_fields();

        let value = value?;
        let needs_quotes = literal_member_name_needs_quotes(node, f);
        write_literal_member_name_value(&value, needs_quotes, f)?;

        Ok(())
    }
}
