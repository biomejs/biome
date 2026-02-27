//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_yaml_syntax::AnyYamlBlockHeader;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyYamlBlockHeader;
impl FormatRule<AnyYamlBlockHeader> for FormatAnyYamlBlockHeader {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &AnyYamlBlockHeader, f: &mut YamlFormatter) -> FormatResult<()> {
        match node {
            AnyYamlBlockHeader::YamlBlockKeepIndicator(node) => node.format().fmt(f),
            AnyYamlBlockHeader::YamlBlockStripIndicator(node) => node.format().fmt(f),
            AnyYamlBlockHeader::YamlBogusBlockHeader(node) => node.format().fmt(f),
            AnyYamlBlockHeader::YamlIndentationIndicator(node) => node.format().fmt(f),
        }
    }
}
