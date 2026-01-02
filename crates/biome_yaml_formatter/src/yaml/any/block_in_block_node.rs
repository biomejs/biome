//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_yaml_syntax::AnyYamlBlockInBlockNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyYamlBlockInBlockNode;
impl FormatRule<AnyYamlBlockInBlockNode> for FormatAnyYamlBlockInBlockNode {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &AnyYamlBlockInBlockNode, f: &mut YamlFormatter) -> FormatResult<()> {
        match node {
            AnyYamlBlockInBlockNode::YamlBlockMapping(node) => node.format().fmt(f),
            AnyYamlBlockInBlockNode::YamlBlockSequence(node) => node.format().fmt(f),
            AnyYamlBlockInBlockNode::YamlFoldedScalar(node) => node.format().fmt(f),
            AnyYamlBlockInBlockNode::YamlLiteralScalar(node) => node.format().fmt(f),
        }
    }
}
