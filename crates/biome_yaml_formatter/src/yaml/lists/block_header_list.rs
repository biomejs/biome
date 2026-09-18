use crate::prelude::*;
use biome_yaml_syntax::{AnyYamlBlockHeader, YamlBlockHeaderList};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockHeaderList;
impl FormatRule<YamlBlockHeaderList> for FormatYamlBlockHeaderList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlBlockHeaderList, f: &mut YamlFormatter) -> FormatResult<()> {
        // The spec allows the indentation indicator and the chomping indicator
        // in either order, but the indentation indicator is always printed
        // first (`|-2` becomes `|2-`)
        let mut join = f.join();
        for header in node
            .iter()
            .filter(|header| matches!(header, AnyYamlBlockHeader::YamlIndentationIndicator(_)))
        {
            join.entry(&header.format());
        }
        for header in node
            .iter()
            .filter(|header| !matches!(header, AnyYamlBlockHeader::YamlIndentationIndicator(_)))
        {
            join.entry(&header.format());
        }
        join.finish()
    }
}
