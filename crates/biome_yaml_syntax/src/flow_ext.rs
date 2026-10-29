use crate::{
    AnyYamlFlowNode, AnyYamlJsonContent, AnyYamlMappingImplicitKey, YamlDoubleQuotedScalar,
    YamlPlainScalar, YamlSingleQuotedScalar, YamlSyntaxToken, inner_string_text,
};
use biome_rowan::{SyntaxResult, TokenText, declare_node_union};

declare_node_union! {
    /// A scalar in the flow style: plain, single-quoted, or double-quoted
    pub AnyYamlFlowScalar = YamlPlainScalar | YamlSingleQuotedScalar | YamlDoubleQuotedScalar
}

impl AnyYamlFlowScalar {
    pub fn value_token(&self) -> SyntaxResult<YamlSyntaxToken> {
        match self {
            Self::YamlPlainScalar(scalar) => scalar.value_token(),
            Self::YamlSingleQuotedScalar(scalar) => scalar.value_token(),
            Self::YamlDoubleQuotedScalar(scalar) => scalar.value_token(),
        }
    }

    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}

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
