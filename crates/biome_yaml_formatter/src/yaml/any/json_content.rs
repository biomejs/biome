//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_yaml_syntax::AnyYamlJsonContent;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyYamlJsonContent;
impl FormatRule<AnyYamlJsonContent> for FormatAnyYamlJsonContent {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &AnyYamlJsonContent, f: &mut YamlFormatter) -> FormatResult<()> {
        match node {
            AnyYamlJsonContent::YamlDoubleQuotedScalar(node) => node.format().fmt(f),
            AnyYamlJsonContent::YamlFlowMapping(node) => node.format().fmt(f),
            AnyYamlJsonContent::YamlFlowSequence(node) => node.format().fmt(f),
            AnyYamlJsonContent::YamlSingleQuotedScalar(node) => node.format().fmt(f),
        }
    }
}
