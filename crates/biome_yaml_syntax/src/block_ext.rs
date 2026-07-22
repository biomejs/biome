use crate::{AnyYamlBlockInBlockContent, AnyYamlBlockNode, AnyYamlFlowNode, AnyYamlJsonContent};
use biome_rowan::AstNodeList;

impl AnyYamlBlockNode {
    pub fn is_nested_block_collection(&self) -> bool {
        matches!(
            self,
            Self::YamlBlockInBlockNode(node)
                if matches!(
                    node.content(),
                    Ok(
                        AnyYamlBlockInBlockContent::YamlBlockMapping(_)
                            | AnyYamlBlockInBlockContent::YamlBlockSequence(_)
                    )
                )
        )
    }

    /// Whether this node is a literal (`|`) or folded (`>`) block scalar
    pub fn is_block_scalar(&self) -> bool {
        matches!(
            self,
            Self::YamlBlockInBlockNode(node)
                if matches!(
                    node.content(),
                    Ok(
                        AnyYamlBlockInBlockContent::YamlLiteralScalar(_)
                            | AnyYamlBlockInBlockContent::YamlFoldedScalar(_)
                    )
                )
        )
    }

    /// Whether this node has any tag or anchor properties attached
    pub fn has_properties(&self) -> bool {
        match self {
            Self::YamlBlockInBlockNode(node) => !node.properties().is_empty(),
            Self::YamlFlowInBlockNode(node) => match node.flow() {
                Ok(AnyYamlFlowNode::YamlFlowJsonNode(node)) => !node.properties().is_empty(),
                Ok(AnyYamlFlowNode::YamlFlowYamlNode(node)) => !node.properties().is_empty(),
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_flow_collection(&self) -> bool {
        let Self::YamlFlowInBlockNode(flow_in_block) = self else {
            return false;
        };
        match flow_in_block.flow() {
            Ok(AnyYamlFlowNode::YamlFlowJsonNode(json)) => matches!(
                json.content(),
                Ok(AnyYamlJsonContent::YamlFlowSequence(_)
                    | AnyYamlJsonContent::YamlFlowMapping(_))
            ),
            _ => false,
        }
    }
}
