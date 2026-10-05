use crate::{AnyYamlFlowNode, AnyYamlJsonContent, AnyYamlMappingImplicitKey};

impl AnyYamlFlowNode {
    /// Whether this node is a flow collection (`[...]` or `{...}`)
    pub fn is_flow_collection(&self) -> bool {
        matches!(
            self,
            Self::YamlFlowJsonNode(node) if matches!(
                node.content(),
                Ok(AnyYamlJsonContent::YamlFlowMapping(_)
                    | AnyYamlJsonContent::YamlFlowSequence(_))
            )
        )
    }
}

impl AnyYamlMappingImplicitKey {
    /// Whether this key is a flow collection (`[...]` or `{...}`), which may
    /// be printed across multiple lines and is then only a valid mapping key
    /// in the explicit `? key : value` form
    pub fn is_flow_collection(&self) -> bool {
        matches!(
            self,
            Self::YamlFlowJsonNode(node) if matches!(
                node.content(),
                Ok(AnyYamlJsonContent::YamlFlowMapping(_)
                    | AnyYamlJsonContent::YamlFlowSequence(_))
            )
        )
    }
}
