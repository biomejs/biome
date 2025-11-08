//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_yaml_syntax::AnyYamlFlowMapEntry;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyYamlFlowMapEntry;
impl FormatRule<AnyYamlFlowMapEntry> for FormatAnyYamlFlowMapEntry {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &AnyYamlFlowMapEntry, f: &mut YamlFormatter) -> FormatResult<()> {
        match node {
            AnyYamlFlowMapEntry::YamlFlowMapExplicitEntry(node) => node.format().fmt(f),
            AnyYamlFlowMapEntry::YamlFlowMapImplicitEntry(node) => node.format().fmt(f),
        }
    }
}
