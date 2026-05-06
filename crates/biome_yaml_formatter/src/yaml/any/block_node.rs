//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_yaml_syntax::AnyYamlBlockNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyYamlBlockNode;
impl FormatRule<AnyYamlBlockNode> for FormatAnyYamlBlockNode {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &AnyYamlBlockNode, f: &mut YamlFormatter) -> FormatResult<()> {
        match node {
            AnyYamlBlockNode::YamlBlockInBlockNode(node) => node.format().fmt(f),
            AnyYamlBlockNode::YamlBogusBlockNode(node) => node.format().fmt(f),
            AnyYamlBlockNode::YamlFlowInBlockNode(node) => node.format().fmt(f),
        }
    }
}
