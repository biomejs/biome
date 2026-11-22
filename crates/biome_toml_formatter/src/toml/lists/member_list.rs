use crate::prelude::*;
use biome_toml_syntax::TomlMemberList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlMemberList;

impl FormatRule<TomlMemberList> for FormatTomlMemberList {
    type Context = TomlFormatContext;

    fn fmt(&self, node: &TomlMemberList, f: &mut TomlFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();
        for member in node {
            join.entry(member.syntax(), &format_or_verbatim(member.format()));
        }
        join.finish()
    }
}
