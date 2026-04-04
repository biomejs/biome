use crate::prelude::*;
use biome_yaml_syntax::YamlBlockHeaderList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockHeaderList;
impl FormatRule<YamlBlockHeaderList> for FormatYamlBlockHeaderList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlBlockHeaderList, f: &mut YamlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
