use crate::{
    AnyYamlBlockInBlockContent, AnyYamlBlockNode, AnyYamlFlowNode, AnyYamlJsonContent,
    AnyYamlProperty, YamlBlockHeaderList, YamlBlockInBlockNode, YamlBlockMapExplicitEntry,
    YamlFoldedScalar, YamlLiteralScalar,
};
use biome_rowan::{AstNode, AstNodeList, declare_node_union};

declare_node_union! {
    /// A block scalar, the node a `YamlBlockContent` sits in
    pub AnyYamlBlockScalar = YamlLiteralScalar | YamlFoldedScalar
}

impl AnyYamlBlockScalar {
    pub fn headers(&self) -> YamlBlockHeaderList {
        match self {
            Self::YamlLiteralScalar(scalar) => scalar.headers(),
            Self::YamlFoldedScalar(scalar) => scalar.headers(),
        }
    }
}

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

/// The tag of a [set](https://yaml.org/type/set.html), as a
/// [shorthand](https://yaml.org/spec/1.2.2/#691-node-tags) and in verbatim
/// form
const SET_TAGS: [&str; 2] = ["!!set", "!<tag:yaml.org,2002:set>"];

impl YamlBlockMapExplicitEntry {
    /// Whether the mapping this entry belongs to is tagged as a set, whose
    /// entries are keys without values:
    ///
    /// ```yaml
    /// !!set
    /// ? a
    /// ? b
    /// ```
    pub fn is_in_set_mapping(&self) -> bool {
        self.syntax()
            .ancestors()
            .find_map(YamlBlockInBlockNode::cast)
            .is_some_and(|block| {
                block.properties().iter().any(|property| {
                    matches!(
                        &property,
                        AnyYamlProperty::YamlTagProperty(tag)
                            if tag.value_token().is_ok_and(|token| {
                                SET_TAGS.contains(&token.text_trimmed())
                            })
                    )
                })
            })
    }
}
