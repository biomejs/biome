use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_json_syntax::{JsonMember, JsonMemberFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonMember;

impl FormatNodeRule<JsonMember> for FormatJsonMember {
    fn fmt_fields(&self, node: &JsonMember, f: &mut JsonFormatter) -> FormatResult<()> {
        let JsonMemberFields {
            name,
            colon_token,
            value,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                &name.format(),
                colon_token.format(),
                space(),
                format_or_verbatim(value?.format())
            ])]
        )
    }
}
