use crate::{
    AnyYamlBlockInBlockContent, AnyYamlBlockNode, AnyYamlFlowNode, AnyYamlJsonContent,
    AnyYamlMappingImplicitKey, AnyYamlProperty, YamlBlockHeaderList, YamlBlockInBlockNode,
    YamlBlockMapExplicitEntry, YamlFoldedScalar, YamlLiteralScalar, YamlPropertyList,
    YamlSyntaxKind, YamlSyntaxNode,
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

declare_node_union! {
    /// The value of a mapping entry: a flow node in a flow mapping, or the
    /// block node that wraps one in a block mapping
    pub AnyYamlEntryValue = AnyYamlFlowNode | AnyYamlBlockNode
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

    /// Whether this node has any tag or anchor properties attached,
    /// including the ones the parser left on the first key of a nested
    /// block mapping
    pub fn has_properties(&self) -> bool {
        match self {
            Self::YamlBlockInBlockNode(node) => {
                !node.properties().is_empty() || node.properties_on_first_key().is_some()
            }
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

impl YamlBlockInBlockNode {
    /// The properties of this node's block mapping that the parser attached
    /// to the mapping's first key.
    ///
    /// Returns that key's property list together with the number of its
    /// leading properties that belong to the mapping, or `None` when the
    /// content is not a block mapping or every property is the key's own.
    pub fn properties_on_first_key(&self) -> Option<(YamlPropertyList, usize)> {
        let content = self.content().ok()?;
        let mapping = content.as_yaml_block_mapping()?;
        let first_entry = mapping.entries().first()?;
        let key = first_entry.as_yaml_block_map_implicit_entry()?.key()?;
        let count = key.enclosing_mapping_property_count();
        if count == 0 {
            return None;
        }
        let properties = match key {
            AnyYamlMappingImplicitKey::YamlFlowYamlNode(node) => node.properties(),
            AnyYamlMappingImplicitKey::YamlFlowJsonNode(node) => node.properties(),
            AnyYamlMappingImplicitKey::YamlAliasNode(_) => return None,
        };
        Some((properties, count))
    }
}

impl AnyYamlMappingImplicitKey {
    /// The number of leading properties in this key's property list that
    /// belong to the enclosing block mapping rather than to the key.
    ///
    /// The parser flattens the properties of a block mapping and those of
    /// its first key into a single list on that key. The key's own
    /// properties are exactly the ones on the key's line; every property on
    /// an earlier line belongs to the mapping:
    ///
    /// ```yaml
    /// top: &mapping
    ///   &key key1: one
    /// ```
    pub fn enclosing_mapping_property_count(&self) -> usize {
        // Only the first key of a block mapping can carry the mapping's
        // properties
        let is_first_key = self.syntax().parent().is_some_and(|entry| {
            entry.kind() == YamlSyntaxKind::YAML_BLOCK_MAP_IMPLICIT_ENTRY
                && entry.prev_sibling().is_none()
                && entry
                    .parent()
                    .is_some_and(|list| list.kind() == YamlSyntaxKind::YAML_BLOCK_MAP_ENTRY_LIST)
        });
        if !is_first_key {
            return 0;
        }

        let (properties, content) = match self {
            Self::YamlAliasNode(_) => return 0,
            Self::YamlFlowYamlNode(node) => (
                node.properties(),
                node.content().map(|content| content.into_syntax()),
            ),
            Self::YamlFlowJsonNode(node) => (
                node.properties(),
                node.content().ok().map(|content| content.into_syntax()),
            ),
        };

        // A key that starts on its own line has no properties of its own
        if content.as_ref().is_some_and(starts_own_line) {
            return properties.len();
        }

        // The key's line opens at the last property that starts a line
        properties
            .iter()
            .enumerate()
            .rfind(|(_, property)| starts_own_line(property.syntax()))
            .map_or(0, |(index, _)| index)
    }
}

/// Whether the node's first token begins a new line, i.e. has a line break
/// in its leading trivia
#[inline]
fn starts_own_line(node: &YamlSyntaxNode) -> bool {
    node.first_token().is_some_and(|token| {
        token
            .leading_trivia()
            .pieces()
            .any(|piece| piece.is_newline())
    })
}
