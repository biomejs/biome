use crate::FormatBogusNodeRule;
use biome_yaml_syntax::YamlBogusFlowNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBogusFlowNode;
impl FormatBogusNodeRule<YamlBogusFlowNode> for FormatYamlBogusFlowNode {}
