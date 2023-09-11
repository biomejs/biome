use crate::format_string::format_string_token;
use crate::prelude::*;
use biome_json_syntax::JsonMemberName;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonMemberName;

impl FormatNodeRule<JsonMemberName> for FormatJsonMemberName {
    fn fmt_fields(&self, node: &JsonMemberName, f: &mut JsonFormatter) -> FormatResult<()> {
        format_string_token(&node.value_token()?).fmt(f)
    }
}
