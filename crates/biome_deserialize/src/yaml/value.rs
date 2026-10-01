//! The traversal of YAML documents: the [DeserializableValue] of their nodes.
use super::anchors::Anchors;
use super::block_scalar::block_scalar_value;
use super::flow_scalar::{FlowStyle, flow_scalar_text};
use super::schema::{Scalar, resolve_flow_scalar};
use crate::{
    DeserializableValue, DeserializationContext, DeserializationDiagnostic,
    ErasedDeserializationVisitor, diagnostics::DeserializableType,
};
use biome_console::markup;
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, TextRange};
use biome_yaml_syntax::{
    AnyYamlBlockInBlockContent, AnyYamlBlockMapEntry, AnyYamlBlockNode, AnyYamlBlockScalar,
    AnyYamlBlockSequenceEntry, AnyYamlDocument, AnyYamlFlowMapEntry, AnyYamlFlowNode,
    AnyYamlFlowSequenceEntry, AnyYamlJsonContent, AnyYamlMappingImplicitKey, YamlBlockMapping,
    YamlBlockSequence, YamlFlowMapping, YamlFlowSequence, YamlPropertyList, YamlRoot,
    YamlSyntaxToken,
};
use std::rc::Rc;

/// A YAML node to deserialize.
#[derive(Clone)]
pub(super) enum YamlNode {
    Block(AnyYamlBlockNode),
    Flow(AnyYamlFlowNode),
    /// A mapping of a single key-value pair in a flow sequence, such as `b: c` in `[a, b: c]`.
    FlowPair(AnyYamlFlowMapEntry),
    /// A missing node, which is null, such as the value of `a:`.
    /// The range is the one of the entry or the document of the node.
    Empty(TextRange),
}

impl YamlNode {
    pub(super) fn range(&self) -> TextRange {
        match self {
            Self::Block(node) => node.range(),
            Self::Flow(node) => node.range(),
            Self::FlowPair(entry) => entry.range(),
            Self::Empty(range) => *range,
        }
    }

    /// Returns the content of this node, or `None` if the node is bogus.
    fn content(&self) -> Option<Content> {
        let content = match self {
            Self::Block(AnyYamlBlockNode::YamlBlockInBlockNode(node)) => match node.content() {
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
            },
            Self::Block(AnyYamlBlockNode::YamlFlowInBlockNode(node)) => {
                return Self::Flow(node.flow().ok()?).content();
            }
            Self::Flow(AnyYamlFlowNode::YamlFlowJsonNode(node)) => {
                let (token, style) = match node.content().ok()? {
                    AnyYamlJsonContent::YamlFlowMapping(mapping) => {
                        return Some(Content::FlowMapping(mapping));
                    }
                    AnyYamlJsonContent::YamlFlowSequence(sequence) => {
                        return Some(Content::FlowSequence(sequence));
                    }
                    AnyYamlJsonContent::YamlSingleQuotedScalar(scalar) => {
                        (scalar.value_token().ok()?, FlowStyle::SingleQuoted)
                    }
                    AnyYamlJsonContent::YamlDoubleQuotedScalar(scalar) => {
                        (scalar.value_token().ok()?, FlowStyle::DoubleQuoted)
                    }
                };
                Content::Scalar {
                    token,
                    style,
                    properties: node.properties(),
                }
            }
            Self::Flow(AnyYamlFlowNode::YamlFlowYamlNode(node)) => match node.content() {
                Some(scalar) => Content::Scalar {
                    token: scalar.value_token().ok()?,
                    style: FlowStyle::Plain,
                    properties: node.properties(),
                },
                // A node with only properties, such as `&anchor`, is empty
                None => Content::Null,
            },
            Self::Flow(AnyYamlFlowNode::YamlAliasNode(alias)) => {
                Content::Alias(alias.value_token().ok()?)
            }
            Self::FlowPair(entry) => Content::FlowPair(entry.clone()),
            Self::Empty(_) => Content::Null,
            Self::Block(AnyYamlBlockNode::YamlBogusBlockNode(_))
            | Self::Flow(AnyYamlFlowNode::YamlBogusFlowNode(_)) => return None,
        };
        Some(content)
    }
}

/// The content of a [YamlNode], without the nodes that wrap it.
enum Content {
    Null,
    /// A plain, single-quoted, or double-quoted scalar, with the properties of its node.
    Scalar {
        token: YamlSyntaxToken,
        style: FlowStyle,
        properties: YamlPropertyList,
    },
    BlockScalar(AnyYamlBlockScalar),
    BlockMapping(YamlBlockMapping),
    BlockSequence(YamlBlockSequence),
    FlowMapping(YamlFlowMapping),
    FlowSequence(YamlFlowSequence),
    FlowPair(AnyYamlFlowMapEntry),
    /// The token of an alias, such as `*anchor`.
    Alias(YamlSyntaxToken),
}

type Member = (Box<dyn DeserializableValue>, Box<dyn DeserializableValue>);

/// A [YamlNode], together with the anchors of its document.
pub(super) struct YamlValue {
    node: YamlNode,
    /// The range of the node, or of the alias that refers to it.
    range: TextRange,
    /// Whether the node is a mapping key, whose scalars are always strings.
    is_key: bool,
    anchors: Rc<Anchors>,
}

impl YamlValue {
    /// Returns the value of the first document of `root`, or `None` if the document is bogus.
    pub(super) fn root(root: &YamlRoot) -> Option<Self> {
        let (scope, node) = match root.documents().first() {
            Some(AnyYamlDocument::YamlDocument(document)) => {
                let node = document
                    .node()
                    .map_or_else(|| YamlNode::Empty(document.range()), YamlNode::Block);
                (document.into_syntax(), node)
            }
            // The parser already reported the bogus document
            Some(AnyYamlDocument::YamlBogus(_)) => return None,
            None => (root.syntax().clone(), YamlNode::Empty(root.range())),
        };
        Some(Self {
            range: node.range(),
            node,
            is_key: false,
            anchors: Rc::new(Anchors::new(scope)),
        })
    }

    fn child(&self, node: YamlNode) -> Self {
        Self {
            range: node.range(),
            node,
            is_key: false,
            anchors: Rc::clone(&self.anchors),
        }
    }

    /// Returns the value of `node`, which an alias in place of this value refers to.
    fn aliased(&self, node: YamlNode) -> Self {
        Self {
            node,
            range: self.range,
            is_key: self.is_key,
            anchors: Rc::clone(&self.anchors),
        }
    }

    /// Returns a mapping member. A missing key or value is null, at the `range` of the entry.
    fn member(&self, key: Option<YamlNode>, value: Option<YamlNode>, range: TextRange) -> Member {
        let key = Self {
            is_key: true,
            ..self.child(key.unwrap_or(YamlNode::Empty(range)))
        };
        let value = self.child(value.unwrap_or(YamlNode::Empty(range)));
        (Box::new(key), Box::new(value))
    }

    fn block_map_member(&self, entry: AnyYamlBlockMapEntry) -> Option<Member> {
        let range = entry.range();
        let (key, value) = match entry {
            AnyYamlBlockMapEntry::YamlBlockMapImplicitEntry(entry) => (
                entry
                    .key()
                    .map(|key| YamlNode::Flow(implicit_key_node(key))),
                entry.value(),
            ),
            AnyYamlBlockMapEntry::YamlBlockMapExplicitEntry(entry) => {
                (entry.key().map(YamlNode::Block), entry.value())
            }
            AnyYamlBlockMapEntry::YamlBogusBlockMapEntry(_) => return None,
        };
        Some(self.member(key, value.map(YamlNode::Block), range))
    }

    fn flow_map_member(&self, entry: &AnyYamlFlowMapEntry) -> Member {
        let (key, value) = match entry {
            AnyYamlFlowMapEntry::YamlFlowMapExplicitEntry(entry) => (entry.key(), entry.value()),
            AnyYamlFlowMapEntry::YamlFlowMapImplicitEntry(entry) => (entry.key(), entry.value()),
        };
        self.member(
            key.map(|key| YamlNode::Flow(implicit_key_node(key))),
            value.map(YamlNode::Flow),
            entry.range(),
        )
    }

    /// Returns the value of a flow scalar.
    fn scalar(
        &self,
        token: &YamlSyntaxToken,
        style: FlowStyle,
        properties: &YamlPropertyList,
    ) -> Scalar {
        let text = flow_scalar_text(token, style);
        if self.is_key {
            Scalar::Str(text.into())
        } else {
            resolve_flow_scalar(text, style, properties)
        }
    }
}

impl DeserializableValue for YamlValue {
    fn range(&self) -> TextRange {
        self.range
    }

    fn deserialize_erased(
        &self,
        ctx: &mut dyn DeserializationContext,
        visitor: &mut dyn ErasedDeserializationVisitor,
        name: &str,
    ) {
        // The parser already reported the bogus node
        let Some(content) = self.node.content() else {
            return;
        };
        let range = self.range;
        match content {
            Content::Null => visitor.visit_null(ctx, range, name),
            Content::Scalar {
                token,
                style,
                properties,
            } => match self.scalar(&token, style, &properties) {
                Scalar::Null => visitor.visit_null(ctx, range, name),
                Scalar::Bool(value) => visitor.visit_bool(ctx, value, range, name),
                Scalar::Int(value) | Scalar::Float(value) => {
                    visitor.visit_number(ctx, value, range, name)
                }
                Scalar::Str(value) => visitor.visit_str(ctx, value, range, name),
                Scalar::Invalid(tag) => ctx.report(
                    DeserializationDiagnostic::new(markup! {
                        "The value isn't a valid "<Emphasis>{tag.name()}</Emphasis>"."
                    })
                    .with_range(range),
                ),
            },
            Content::BlockScalar(scalar) => {
                if let Some(value) = block_scalar_value(&scalar) {
                    visitor.visit_str(ctx, value.into(), range, name)
                }
            }
            Content::BlockMapping(mapping) => {
                let mut members = mapping
                    .entries()
                    .iter()
                    .map(|entry| self.block_map_member(entry));
                visitor.visit_map(ctx, &mut members, range, name)
            }
            Content::BlockSequence(sequence) => {
                let mut items = sequence.entries().iter().map(|entry| {
                    let AnyYamlBlockSequenceEntry::YamlBlockSequenceEntry(entry) = entry else {
                        return None;
                    };
                    let node = entry
                        .value()
                        .map_or_else(|| YamlNode::Empty(entry.range()), YamlNode::Block);
                    Some(Box::new(self.child(node)) as Box<dyn DeserializableValue>)
                });
                visitor.visit_array(ctx, &mut items, range, name)
            }
            Content::FlowMapping(mapping) => {
                let mut members = mapping
                    .entries()
                    .iter()
                    .map(|entry| Some(self.flow_map_member(&entry.ok()?)));
                visitor.visit_map(ctx, &mut members, range, name)
            }
            Content::FlowSequence(sequence) => {
                let mut items = sequence.entries().iter().map(|entry| {
                    let node = match entry.ok()? {
                        AnyYamlFlowSequenceEntry::AnyYamlFlowNode(node) => YamlNode::Flow(node),
                        AnyYamlFlowSequenceEntry::AnyYamlFlowMapEntry(entry) => {
                            YamlNode::FlowPair(entry)
                        }
                    };
                    Some(Box::new(self.child(node)) as Box<dyn DeserializableValue>)
                });
                visitor.visit_array(ctx, &mut items, range, name)
            }
            Content::FlowPair(entry) => {
                let mut members = std::iter::once(Some(self.flow_map_member(&entry)));
                visitor.visit_map(ctx, &mut members, range, name)
            }
            Content::Alias(alias) => match self.anchors.resolve(&alias) {
                Ok(node) => {
                    if self.anchors.expand(ctx, node, range) {
                        self.aliased(node.clone())
                            .deserialize_erased(ctx, visitor, name);
                    }
                }
                Err(diagnostic) => ctx.report(diagnostic),
            },
        }
    }

    fn visitable_type(&self) -> Option<DeserializableType> {
        let visitable_type = match self.node.content()? {
            Content::Null => DeserializableType::Null,
            Content::Scalar {
                token,
                style,
                properties,
            } => match self.scalar(&token, style, &properties) {
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
                let node = self.anchors.resolve(&alias).ok()?;
                return self.aliased(node.clone()).visitable_type();
            }
        };
        Some(visitable_type)
    }
}

/// Returns the flow node of an implicit mapping key.
fn implicit_key_node(key: AnyYamlMappingImplicitKey) -> AnyYamlFlowNode {
    match key {
        AnyYamlMappingImplicitKey::YamlAliasNode(node) => node.into(),
        AnyYamlMappingImplicitKey::YamlFlowJsonNode(node) => node.into(),
        AnyYamlMappingImplicitKey::YamlFlowYamlNode(node) => node.into(),
    }
}
