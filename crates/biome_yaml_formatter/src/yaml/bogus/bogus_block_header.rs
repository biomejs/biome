use crate::FormatBogusNodeRule;
use biome_yaml_syntax::YamlBogusBlockHeader;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBogusBlockHeader;
impl FormatBogusNodeRule<YamlBogusBlockHeader> for FormatYamlBogusBlockHeader {}
