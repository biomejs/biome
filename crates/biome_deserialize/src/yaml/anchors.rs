//! The resolution of [aliases](https://yaml.org/spec/1.2.2/#71-alias-nodes) to the nodes of
//! their anchors.
use super::value::AnyYamlValueNode;
use crate::DeserializationDiagnostic;
use biome_console::markup;
use biome_rowan::{AstNode, AstNodeList, TextSize, TokenText};
use biome_yaml_syntax::{
    AnyYamlBlockNode, AnyYamlMappingImplicitKey, YamlAliasNode, YamlAnchorProperty,
    YamlBlockInBlockNode, YamlDocument, YamlPropertyList,
};

/// Bounds the source text that aliases expand to, as a factor of the size of the document,
/// so that documents such as the [billion laughs](https://en.wikipedia.org/wiki/Billion_laughs_attack)
/// can't expand to an unbounded size.
const MAX_ALIAS_EXPANSION_FACTOR: usize = 100;

/// The anchors and alias expansion budget shared by the values of a document.
pub(super) struct Anchors {
    /// `None` when the stream has no document, and so no anchors.
    document: Option<YamlDocument>,
    /// The anchors in source order, collected when resolving the first alias.
    anchors: Option<Box<[Anchor]>>,
    /// The size of the source text that aliases can still expand to, or `None` once exceeded.
    expansion_budget: Option<usize>,
}

struct Anchor {
    /// The name of the anchor, without `&`.
    name: TokenText,
    /// The offset of the anchor in the source. Only the aliases after it can refer to it.
    offset: TextSize,
    /// The node that the anchor is attached to.
    node: AnyYamlValueNode,
}

impl Anchors {
    pub(super) fn new(document: Option<YamlDocument>) -> Self {
        let document_len = document.as_ref().map_or(0, |document| {
            usize::from(document.syntax().text_range_with_trivia().len())
        });
        Self {
            document,
            anchors: None,
            expansion_budget: Some(document_len.saturating_mul(MAX_ALIAS_EXPANSION_FACTOR)),
        }
    }

    fn anchors(&mut self) -> &[Anchor] {
        let document = &self.document;
        self.anchors.get_or_insert_with(|| {
            document
                .iter()
                .flat_map(|document| document.syntax().descendants())
                .filter_map(YamlAnchorProperty::cast)
                .filter_map(|anchor| {
                    Some(Anchor {
                        name: anchor.name().ok()?,
                        offset: anchor.range().start(),
                        node: anchored_node(&anchor)?,
                    })
                })
                .collect()
        })
    }

    /// Returns the node that `alias` refers to: the node of the last anchor that precedes it,
    /// with the same name.
    ///
    /// Returns `Err(None)` when the alias has no name, which the parser reports.
    pub(super) fn resolve(
        &mut self,
        alias: &YamlAliasNode,
    ) -> Result<AnyYamlValueNode, Option<DeserializationDiagnostic>> {
        let name = alias.name().map_err(|_| None)?;
        let range = alias.range();
        let Some(anchor) = self
            .anchors()
            .iter()
            .rev()
            .find(|anchor| anchor.offset < range.start() && anchor.name == name)
        else {
            return Err(Some(
                DeserializationDiagnostic::new(markup! {
                    "The anchor "<Emphasis>{name.text()}</Emphasis>" isn't defined."
                })
                .with_range(range)
                .with_note("An alias can only refer to an anchor that precedes it."),
            ));
        };
        if anchor.node.range().contains(range.start()) {
            return Err(Some(
                DeserializationDiagnostic::new(markup! {
                    "The alias "<Emphasis>"*"{name.text()}</Emphasis>" refers to a node that contains it."
                })
                .with_range(range)
                .with_note("Recursive structures can't be deserialized."),
            ));
        }
        Ok(anchor.node.clone())
    }

    /// Returns the node that `alias` refers to, and charges its expansion to the budget of the
    /// document.
    ///
    /// Returns the diagnostic of the exceeded budget the first time, and `Err(None)` afterwards.
    pub(super) fn expand(
        &mut self,
        alias: &YamlAliasNode,
    ) -> Result<AnyYamlValueNode, Option<DeserializationDiagnostic>> {
        let node = self.resolve(alias)?;
        let Some(budget) = self.expansion_budget else {
            return Err(None);
        };
        self.expansion_budget = budget.checked_sub(usize::from(node.range().len()));
        if self.expansion_budget.is_none() {
            return Err(Some(
                DeserializationDiagnostic::new("Aliases expand to too much content.")
                    .with_range(alias.range())
                    .with_note(markup! {
                        "Aliases can expand to at most "{MAX_ALIAS_EXPANSION_FACTOR}" times the size of the document."
                    }),
            ));
        }
        Ok(node)
    }
}

fn anchored_node(anchor: &YamlAnchorProperty) -> Option<AnyYamlValueNode> {
    let properties = anchor.parent::<YamlPropertyList>()?;
    if let Some(key) = properties.parent::<AnyYamlMappingImplicitKey>() {
        // In `a: &anchor\n  b: c`, `&anchor` belongs to the mapping that `b` starts, but the
        // parser puts it in the property list of `b`, before the properties of `b` itself
        let index = properties
            .iter()
            .position(|property| property.as_yaml_anchor_property() == Some(anchor))?;
        if index < key.enclosing_mapping_property_count() {
            let mapping = key
                .syntax()
                .ancestors()
                .find_map(YamlBlockInBlockNode::cast)?;
            return Some(AnyYamlBlockNode::from(mapping).into());
        }
    }
    properties.parent::<AnyYamlValueNode>()
}
