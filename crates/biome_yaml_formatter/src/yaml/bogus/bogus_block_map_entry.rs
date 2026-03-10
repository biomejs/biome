use crate::FormatBogusNodeRule;
use biome_yaml_syntax::YamlBogusBlockMapEntry;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBogusBlockMapEntry;
impl FormatBogusNodeRule<YamlBogusBlockMapEntry> for FormatYamlBogusBlockMapEntry {}
