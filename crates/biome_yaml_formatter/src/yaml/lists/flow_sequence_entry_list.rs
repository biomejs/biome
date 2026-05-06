use crate::prelude::*;
use biome_yaml_syntax::YamlFlowSequenceEntryList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowSequenceEntryList;
impl FormatRule<YamlFlowSequenceEntryList> for FormatYamlFlowSequenceEntryList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlFlowSequenceEntryList, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
