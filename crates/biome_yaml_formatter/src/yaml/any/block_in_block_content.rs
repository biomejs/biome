//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_yaml_syntax::AnyYamlBlockInBlockContent;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyYamlBlockInBlockContent;
impl FormatRule<AnyYamlBlockInBlockContent> for FormatAnyYamlBlockInBlockContent {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &AnyYamlBlockInBlockContent, f: &mut YamlFormatter) -> FormatResult<()> {
        match node {
            AnyYamlBlockInBlockContent::YamlBlockMapping(node) => node.format().fmt(f),
            AnyYamlBlockInBlockContent::YamlBlockSequence(node) => node.format().fmt(f),
            AnyYamlBlockInBlockContent::YamlFoldedScalar(node) => node.format().fmt(f),
            AnyYamlBlockInBlockContent::YamlLiteralScalar(node) => node.format().fmt(f),
        }
    }
}
