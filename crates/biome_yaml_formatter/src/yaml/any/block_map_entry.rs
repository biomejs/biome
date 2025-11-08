//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_yaml_syntax::AnyYamlBlockMapEntry;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyYamlBlockMapEntry;
impl FormatRule<AnyYamlBlockMapEntry> for FormatAnyYamlBlockMapEntry {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &AnyYamlBlockMapEntry, f: &mut YamlFormatter) -> FormatResult<()> {
        match node {
            AnyYamlBlockMapEntry::YamlBlockMapExplicitEntry(node) => node.format().fmt(f),
            AnyYamlBlockMapEntry::YamlBlockMapImplicitEntry(node) => node.format().fmt(f),
            AnyYamlBlockMapEntry::YamlBogusBlockMapEntry(node) => node.format().fmt(f),
        }
    }
}
