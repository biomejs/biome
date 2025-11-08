use crate::FormatBogusNodeRule;
use biome_yaml_syntax::YamlBogusBlockNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBogusBlockNode;
impl FormatBogusNodeRule<YamlBogusBlockNode> for FormatYamlBogusBlockNode {}
