use crate::prelude::*;
use biome_toml_syntax::TomlRootItemList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlRootItemList;

impl FormatRule<TomlRootItemList> for FormatTomlRootItemList {
    type Context = TomlFormatContext;

    fn fmt(&self, node: &TomlRootItemList, f: &mut TomlFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();
        for item in node {
            join.entry(item.syntax(), &format_or_verbatim(item.format()));
        }
        join.finish()
    }
}
