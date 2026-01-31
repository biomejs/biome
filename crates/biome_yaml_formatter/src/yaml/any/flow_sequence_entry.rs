//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_yaml_syntax::AnyYamlFlowSequenceEntry;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyYamlFlowSequenceEntry;
impl FormatRule<AnyYamlFlowSequenceEntry> for FormatAnyYamlFlowSequenceEntry {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &AnyYamlFlowSequenceEntry, f: &mut YamlFormatter) -> FormatResult<()> {
        match node {
            AnyYamlFlowSequenceEntry::AnyYamlFlowMapEntry(node) => node.format().fmt(f),
            AnyYamlFlowSequenceEntry::AnyYamlFlowNode(node) => node.format().fmt(f),
        }
    }
}
