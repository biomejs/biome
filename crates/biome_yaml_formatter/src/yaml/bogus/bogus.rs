use crate::FormatBogusNodeRule;
use biome_yaml_syntax::YamlBogus;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBogus;
impl FormatBogusNodeRule<YamlBogus> for FormatYamlBogus {}
