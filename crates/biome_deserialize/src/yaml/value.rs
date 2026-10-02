//! The traversal of YAML documents: the [DeserializableValue] of their nodes.
use super::YamlDeserializationContext;
use super::block_scalar::block_scalar_value;
use super::flow_scalar::flow_scalar_text;
use super::schema::{Scalar, resolve_flow_scalar};
use crate::{
    DeserializableValue, DeserializationContext, DeserializationDiagnostic,
    ErasedDeserializationVisitor, diagnostics::DeserializableType,
};
use biome_console::markup;
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, TextRange, declare_node_union};
use biome_yaml_syntax::{
    AnyYamlBlockInBlockContent, AnyYamlBlockMapEntry, AnyYamlBlockNode, AnyYamlBlockScalar,
    AnyYamlBlockSequenceEntry, AnyYamlDocument, AnyYamlFlowMapEntry, AnyYamlFlowNode,
    AnyYamlFlowScalar, AnyYamlFlowSequenceEntry, AnyYamlJsonContent, AnyYamlMappingImplicitKey,
    YamlAliasNode, YamlBlockMapExplicitEntry, YamlBlockMapImplicitEntry, YamlBlockMapping,
    YamlBlockSequence, YamlDocument, YamlFlowMapExplicitEntry, YamlFlowMapImplicitEntry,
    YamlFlowMapping, YamlFlowSequence, YamlPropertyList, YamlRoot,
};

declare_node_union! {
    /// A node that is deserialized as a value. A flow map entry is a value in a flow sequence,
    /// such as `b: c` in `[a, b: c]`, where it's a mapping of a single key-value pair.
    pub(super) AnyYamlValueNode = AnyYamlBlockNode | AnyYamlFlowNode | AnyYamlFlowMapEntry
}

impl AnyYamlValueNode {
    /// Returns the content of this node, or `None` if the node is bogus.
    fn content(&self) -> Option<Content> {
        let content = match self {
            Self::AnyYamlBlockNode(AnyYamlBlockNode::YamlBlockInBlockNode(node)) => {
                match node.content() {
                    Ok(AnyYamlBlockInBlockContent::YamlBlockMapping(mapping)) => {
                        Content::BlockMapping(mapping)
                    }
                    Ok(AnyYamlBlockInBlockContent::YamlBlockSequence(sequence)) => {
                        Content::BlockSequence(sequence)
                    }
                    Ok(AnyYamlBlockInBlockContent::YamlLiteralScalar(scalar)) => {
                        Content::BlockScalar(scalar.into())
                    }
                    Ok(AnyYamlBlockInBlockContent::YamlFoldedScalar(scalar)) => {
                        Content::BlockScalar(scalar.into())
                    }
                    // A node with only properties, such as `&anchor`, is empty
                    Err(_) if !node.properties().is_empty() => Content::Null,
                    Err(_) => return None,
                }
            }
            Self::AnyYamlBlockNode(AnyYamlBlockNode::YamlFlowInBlockNode(node)) => {
                return Self::AnyYamlFlowNode(node.flow().ok()?).content();
            }
            Self::AnyYamlFlowNode(AnyYamlFlowNode::YamlFlowJsonNode(node)) => {
                let scalar: AnyYamlFlowScalar = match node.content().ok()? {
                    AnyYamlJsonContent::YamlFlowMapping(mapping) => {
                        return Some(Content::FlowMapping(mapping));
                    }
                    AnyYamlJsonContent::YamlFlowSequence(sequence) => {
                        return Some(Content::FlowSequence(sequence));
                    }
                    AnyYamlJsonContent::YamlSingleQuotedScalar(scalar) => scalar.into(),
                    AnyYamlJsonContent::YamlDoubleQuotedScalar(scalar) => scalar.into(),
                };
                Content::Scalar {
                    scalar,
                    properties: node.properties(),
                }
            }
            Self::AnyYamlFlowNode(AnyYamlFlowNode::YamlFlowYamlNode(node)) => {
                match node.content() {
                    Some(scalar) => Content::Scalar {
                        scalar: scalar.into(),
                        properties: node.properties(),
                    },
                    // A node with only properties, such as `&anchor`, is empty
                    None => Content::Null,
                }
            }
            Self::AnyYamlFlowNode(AnyYamlFlowNode::YamlAliasNode(alias)) => {
                Content::Alias(alias.clone())
            }
            Self::AnyYamlFlowMapEntry(entry) => Content::FlowPair(entry.clone()),
            Self::AnyYamlBlockNode(AnyYamlBlockNode::YamlBogusBlockNode(_))
            | Self::AnyYamlFlowNode(AnyYamlFlowNode::YamlBogusFlowNode(_)) => return None,
        };
        Some(content)
    }
}

/// The content of an [AnyYamlValueNode], without the nodes that wrap it.
enum Content {
    Null,
    /// A flow scalar, with the properties of its node.
    Scalar {
        scalar: AnyYamlFlowScalar,
        properties: YamlPropertyList,
    },
    BlockScalar(AnyYamlBlockScalar),
    BlockMapping(YamlBlockMapping),
    BlockSequence(YamlBlockSequence),
    FlowMapping(YamlFlowMapping),
    FlowSequence(YamlFlowSequence),
    FlowPair(AnyYamlFlowMapEntry),
    Alias(YamlAliasNode),
}

type Member = (
    Box<dyn DeserializableValue<Context = YamlDeserializationContext>>,
    Box<dyn DeserializableValue<Context = YamlDeserializationContext>>,
);

/// A node to deserialize. Its aliases are resolved with the anchors of the context.
pub(super) struct YamlValue {
    /// `None` when the node is missing, which is null, such as the value of `a:`.
    node: Option<AnyYamlValueNode>,
    /// The range of the node, of the alias that refers to it, or of the entry or document of a
    /// missing node.
    range: TextRange,
    /// Whether the node is a mapping key, whose scalars are always strings.
    is_key: bool,
}

impl YamlValue {
    /// Returns the value of the first document of `root`, with the document. The value of a
    /// stream without documents is null.
    ///
    /// Returns `None` if the document is bogus.
    pub(super) fn root(root: &YamlRoot) -> Option<(Self, Option<YamlDocument>)> {
        match root.documents().first() {
            Some(AnyYamlDocument::YamlDocument(document)) => {
                let value = document
                    .node()
                    .map_or_else(|| Self::missing(document.range()), Self::new);
                Some((value, Some(document)))
            }
            // The parser already reported the bogus document
            Some(AnyYamlDocument::YamlBogus(_)) => None,
            None => Some((Self::missing(root.range()), None)),
        }
    }

    fn new(node: impl Into<AnyYamlValueNode>) -> Self {
        let node = node.into();
        Self {
            range: node.range(),
            node: Some(node),
            is_key: false,
        }
    }

    /// Returns the value of a missing node, which is null, at the `range` of its entry or
    /// document.
    fn missing(range: TextRange) -> Self {
        Self {
            node: None,
            range,
            is_key: false,
        }
    }

    /// Returns the value of `node`, which an alias in place of this value refers to.
    fn aliased(&self, node: AnyYamlValueNode) -> Self {
        Self {
            node: Some(node),
            range: self.range,
            is_key: self.is_key,
        }
    }

    /// Returns the content of the node, or `None` if the node is bogus.
    fn content(&self) -> Option<Content> {
        match &self.node {
            Some(node) => node.content(),
            None => Some(Content::Null),
        }
    }

    /// Returns the key and the value of a mapping `entry`. A missing key or value is null, at
    /// the range of the entry.
    fn member(entry: AnyYamlMappingEntry) -> Member {
        let range = entry.range();
        let (key, value): (Option<AnyYamlValueNode>, Option<AnyYamlValueNode>) = match entry {
            AnyYamlMappingEntry::YamlBlockMapImplicitEntry(entry) => (
                entry.key().map(implicit_key_node),
                entry.value().map(Into::into),
            ),
            AnyYamlMappingEntry::YamlBlockMapExplicitEntry(entry) => {
                (entry.key().map(Into::into), entry.value().map(Into::into))
            }
            AnyYamlMappingEntry::YamlFlowMapImplicitEntry(entry) => (
                entry.key().map(implicit_key_node),
                entry.value().map(Into::into),
            ),
            AnyYamlMappingEntry::YamlFlowMapExplicitEntry(entry) => (
                entry.key().map(implicit_key_node),
                entry.value().map(Into::into),
            ),
        };
        let value_or_missing =
            |node: Option<AnyYamlValueNode>| node.map_or_else(|| Self::missing(range), Self::new);
        let key = Self {
            is_key: true,
            ..value_or_missing(key)
        };
        (Box::new(key), Box::new(value_or_missing(value)))
    }

    /// Returns the value of `scalar`, which is always a string when this value is a mapping
    /// key. Returns `None` if the scalar has no token.
    fn scalar(&self, scalar: &AnyYamlFlowScalar, properties: &YamlPropertyList) -> Option<Scalar> {
        let text = flow_scalar_text(scalar)?;
        Some(if self.is_key {
            Scalar::Str(text.into())
        } else {
            resolve_flow_scalar(scalar, text, properties)
        })
    }
}

impl DeserializableValue for YamlValue {
    type Context = YamlDeserializationContext;

    fn range(&self) -> TextRange {
        self.range
    }

    fn deserialize_erased(
        &self,
        ctx: &mut YamlDeserializationContext,
        visitor: &mut dyn ErasedDeserializationVisitor<YamlDeserializationContext>,
        name: &str,
    ) {
        // The parser already reported the bogus node
        let Some(content) = self.content() else {
            return;
        };
        let range = self.range;
        match content {
            Content::Null => visitor.visit_null(ctx, range, name),
            Content::Scalar { scalar, properties } => match self.scalar(&scalar, &properties) {
                Some(Scalar::Null) => visitor.visit_null(ctx, range, name),
                Some(Scalar::Bool(value)) => visitor.visit_bool(ctx, value, range, name),
                Some(Scalar::Int(value) | Scalar::Float(value)) => {
                    visitor.visit_number(ctx, value, range, name)
                }
                Some(Scalar::Str(value)) => visitor.visit_str(ctx, value, range, name),
                Some(Scalar::Invalid(tag)) => ctx.report(
                    DeserializationDiagnostic::new(markup! {
                        "The value isn't a valid "<Emphasis>{tag.name()}</Emphasis>"."
                    })
                    .with_range(range),
                ),
                // The parser already reported the scalar without a token
                None => {}
            },
            Content::BlockScalar(scalar) => {
                if let Some(value) = block_scalar_value(&scalar) {
                    visitor.visit_str(ctx, value.into(), range, name)
                }
            }
            Content::BlockMapping(mapping) => {
                let mut members = mapping.entries().iter().map(|entry| {
                    let entry: AnyYamlMappingEntry = match entry {
                        AnyYamlBlockMapEntry::YamlBlockMapImplicitEntry(entry) => entry.into(),
                        AnyYamlBlockMapEntry::YamlBlockMapExplicitEntry(entry) => entry.into(),
                        // The parser already reported the bogus entry
                        AnyYamlBlockMapEntry::YamlBogusBlockMapEntry(_) => return None,
                    };
                    Some(Self::member(entry))
                });
                visitor.visit_map(ctx, &mut members, range, name)
            }
            Content::BlockSequence(sequence) => {
                let mut items = sequence.entries().iter().map(|entry| {
                    let AnyYamlBlockSequenceEntry::YamlBlockSequenceEntry(entry) = entry else {
                        return None;
                    };
                    let value = entry
                        .value()
                        .map_or_else(|| Self::missing(entry.range()), Self::new);
                    Some(Box::new(value)
                        as Box<
                            dyn DeserializableValue<Context = YamlDeserializationContext>,
                        >)
                });
                visitor.visit_array(ctx, &mut items, range, name)
            }
            Content::FlowMapping(mapping) => {
                let mut members = mapping
                    .entries()
                    .iter()
                    .map(|entry| Some(Self::member(entry.ok()?.into())));
                visitor.visit_map(ctx, &mut members, range, name)
            }
            Content::FlowSequence(sequence) => {
                let mut items = sequence.entries().iter().map(|entry| {
                    let value = match entry.ok()? {
                        AnyYamlFlowSequenceEntry::AnyYamlFlowNode(node) => Self::new(node),
                        AnyYamlFlowSequenceEntry::AnyYamlFlowMapEntry(entry) => Self::new(entry),
                    };
                    Some(Box::new(value)
                        as Box<
                            dyn DeserializableValue<Context = YamlDeserializationContext>,
                        >)
                });
                visitor.visit_array(ctx, &mut items, range, name)
            }
            Content::FlowPair(entry) => {
                let mut members = std::iter::once(Some(Self::member(entry.into())));
                visitor.visit_map(ctx, &mut members, range, name)
            }
            Content::Alias(alias) => {
                match ctx.anchors.expand(&alias) {
                    Ok(node) => self.aliased(node).deserialize_erased(ctx, visitor, name),
                    Err(Some(diagnostic)) => ctx.report(diagnostic),
                    // The exceeded budget was already reported
                    Err(None) => {}
                }
            }
        }
    }

    fn visitable_type(&self, ctx: &mut YamlDeserializationContext) -> Option<DeserializableType> {
        let visitable_type = match self.content()? {
            Content::Null => DeserializableType::Null,
            Content::Scalar { scalar, properties } => match self.scalar(&scalar, &properties)? {
                Scalar::Null => DeserializableType::Null,
                Scalar::Bool(_) => DeserializableType::Bool,
                Scalar::Int(_) | Scalar::Float(_) => DeserializableType::Number,
                Scalar::Str(_) => DeserializableType::Str,
                Scalar::Invalid(_) => return None,
            },
            Content::BlockScalar(_) => DeserializableType::Str,
            Content::BlockMapping(_) | Content::FlowMapping(_) | Content::FlowPair(_) => {
                DeserializableType::Map
            }
            Content::BlockSequence(_) | Content::FlowSequence(_) => DeserializableType::Array,
            Content::Alias(alias) => {
                let node = ctx.anchors.resolve(&alias).ok()?;
                return self.aliased(node).visitable_type(ctx);
            }
        };
        Some(visitable_type)
    }
}

fn implicit_key_node(key: AnyYamlMappingImplicitKey) -> AnyYamlValueNode {
    AnyYamlValueNode::AnyYamlFlowNode(match key {
        AnyYamlMappingImplicitKey::YamlAliasNode(node) => node.into(),
        AnyYamlMappingImplicitKey::YamlFlowJsonNode(node) => node.into(),
        AnyYamlMappingImplicitKey::YamlFlowYamlNode(node) => node.into(),
    })
}

declare_node_union! {
    AnyYamlMappingEntry = YamlBlockMapImplicitEntry
        | YamlBlockMapExplicitEntry
        | YamlFlowMapImplicitEntry
        | YamlFlowMapExplicitEntry
}

impl From<AnyYamlFlowMapEntry> for AnyYamlMappingEntry {
    fn from(entry: AnyYamlFlowMapEntry) -> Self {
        match entry {
            AnyYamlFlowMapEntry::YamlFlowMapImplicitEntry(entry) => entry.into(),
            AnyYamlFlowMapEntry::YamlFlowMapExplicitEntry(entry) => entry.into(),
        }
    }
}
