//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_yaml_syntax::AnyYamlBlockSequenceEntry;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyYamlBlockSequenceEntry;
impl FormatRule<AnyYamlBlockSequenceEntry> for FormatAnyYamlBlockSequenceEntry {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &AnyYamlBlockSequenceEntry, f: &mut YamlFormatter) -> FormatResult<()> {
        match node {
            AnyYamlBlockSequenceEntry::YamlBlockSequenceEntry(node) => node.format().fmt(f),
            AnyYamlBlockSequenceEntry::YamlBogus(node) => node.format().fmt(f),
        }
    }
}
