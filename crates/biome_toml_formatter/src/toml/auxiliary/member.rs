use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlMember, TomlMemberFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlMember;

impl FormatNodeRule<TomlMember> for FormatTomlMember {
    fn fmt_fields(&self, node: &TomlMember, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlMemberFields {
            key,
            eq_token,
            value,
        } = node.as_fields();

        write!(
            f,
            [
                key.format(),
                space(),
                eq_token.format(),
                space(),
                value.format()
            ]
        )
    }
}
