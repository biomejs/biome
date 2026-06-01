use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlFlowInBlockNode, YamlFlowInBlockNodeFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowInBlockNode;
impl FormatNodeRule<YamlFlowInBlockNode> for FormatYamlFlowInBlockNode {
    fn fmt_fields(&self, node: &YamlFlowInBlockNode, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlFlowInBlockNodeFields {
            flow_start_token: _,
            flow,
            flow_end_token: _,
        } = node.as_fields();

        write!(f, [flow.format()])
    }
}
