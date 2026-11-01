use crate::prelude::*;
use biome_toml_syntax::TomlKeyValueList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlKeyValueList;

impl FormatRule<TomlKeyValueList> for FormatTomlKeyValueList {
    type Context = TomlFormatContext;

    fn fmt(&self, node: &TomlKeyValueList, f: &mut TomlFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();
        for entry in node {
            join.entry(entry.syntax(), &format_or_verbatim(entry.format()));
        }
        join.finish()
    }
}
