use crate::prelude::*;
use biome_yaml_syntax::YamlPropertyList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlPropertyList;
impl FormatRule<YamlPropertyList> for FormatYamlPropertyList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlPropertyList, f: &mut YamlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
