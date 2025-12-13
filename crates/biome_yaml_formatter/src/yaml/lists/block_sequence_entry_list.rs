use crate::prelude::*;
use biome_yaml_syntax::YamlBlockSequenceEntryList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockSequenceEntryList;
impl FormatRule<YamlBlockSequenceEntryList> for FormatYamlBlockSequenceEntryList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlBlockSequenceEntryList, f: &mut YamlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
