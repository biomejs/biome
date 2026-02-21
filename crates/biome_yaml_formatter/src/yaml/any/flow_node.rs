//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_yaml_syntax::AnyYamlFlowNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyYamlFlowNode;
impl FormatRule<AnyYamlFlowNode> for FormatAnyYamlFlowNode {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &AnyYamlFlowNode, f: &mut YamlFormatter) -> FormatResult<()> {
        match node {
            AnyYamlFlowNode::YamlAliasNode(node) => node.format().fmt(f),
            AnyYamlFlowNode::YamlBogusFlowNode(node) => node.format().fmt(f),
            AnyYamlFlowNode::YamlFlowJsonNode(node) => node.format().fmt(f),
            AnyYamlFlowNode::YamlFlowYamlNode(node) => node.format().fmt(f),
        }
    }
}
