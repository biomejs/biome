use crate::prelude::*;
use biome_yaml_syntax::YamlBlockMapEntryList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockMapEntryList;
impl FormatRule<YamlBlockMapEntryList> for FormatYamlBlockMapEntryList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlBlockMapEntryList, f: &mut YamlFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for entry in node {
            join.entry(entry.syntax(), &entry.format());
        }

        join.finish()
    }
}
