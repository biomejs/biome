//! The resolution of [aliases](https://yaml.org/spec/1.2.2/#71-alias-nodes) to the nodes of
//! their anchors.
use super::value::YamlNode;
use crate::{DeserializationContext, DeserializationDiagnostic};
use biome_console::markup;
use biome_rowan::{AstNode, AstNodeList, TextRange, TextSize, TokenText};
use biome_yaml_syntax::{
    AnyYamlBlockNode, AnyYamlFlowNode, AnyYamlMappingImplicitKey, YamlAnchorProperty,
    YamlBlockInBlockNode, YamlPropertyList, YamlSyntaxNode, YamlSyntaxToken,
};
use std::cell::{Cell, OnceCell};

/// Bounds the source text that aliases expand to, as a factor of the size of the document,
/// so that documents such as the [billion laughs](https://en.wikipedia.org/wiki/Billion_laughs_attack)
/// can't expand to an unbounded size.
const MAX_ALIAS_EXPANSION_FACTOR: usize = 100;

/// The anchors of a document, which its aliases refer to.
pub(super) struct Anchors {
    /// The document, or the stream when it has no document.
    scope: YamlSyntaxNode,
    /// The anchors in source order, collected when resolving the first alias.
    anchors: OnceCell<Box<[Anchor]>>,
    /// The size of the source text that aliases can still expand to, or `None` once exceeded.
    expansion_budget: Cell<Option<usize>>,
}

struct Anchor {
    /// The name of the anchor, without `&`.
    name: TokenText,
    /// The offset of the anchor in the source.
    offset: TextSize,
    /// The node that the anchor is attached to.
    node: YamlNode,
}

impl Anchors {
    pub(super) fn new(scope: YamlSyntaxNode) -> Self {
        let budget = usize::from(scope.text_range_with_trivia().len())
            .saturating_mul(MAX_ALIAS_EXPANSION_FACTOR);
        Self {
            scope,
            anchors: OnceCell::new(),
            expansion_budget: Cell::new(Some(budget)),
        }
    }

    fn anchors(&self) -> &[Anchor] {
        self.anchors.get_or_init(|| {
            self.scope
                .descendants()
                .filter_map(YamlAnchorProperty::cast)
                .filter_map(|anchor| {
                    let token = anchor.value_token().ok()?;
                    let name = token.token_text_trimmed();
                    let name_range = TextRange::new(TextSize::from(1), name.len());
                    Some(Anchor {
                        // Removes the `&`
                        name: name.slice(name_range),
                        offset: token.text_trimmed_range().start(),
                        node: anchored_node(&anchor)?,
                    })
                })
                .collect()
        })
    }

    /// Returns the node that `alias` refers to: the node of the last anchor that precedes it,
    /// with the same name.
    pub(super) fn resolve(
        &self,
        alias: &YamlSyntaxToken,
    ) -> Result<&YamlNode, DeserializationDiagnostic> {
        let range = alias.text_trimmed_range();
        let name = alias.text_trimmed().trim_start_matches('*');
        let Some(anchor) = self
            .anchors()
            .iter()
            .rev()
            .find(|anchor| anchor.offset < range.start() && anchor.name.text() == name)
        else {
            return Err(DeserializationDiagnostic::new(markup! {
                "The anchor "<Emphasis>{name}</Emphasis>" isn't defined."
            })
            .with_range(range)
            .with_note("An alias can only refer to an anchor that precedes it."));
        };
        if anchor.node.range().contains(range.start()) {
            return Err(DeserializationDiagnostic::new(markup! {
                "The alias "<Emphasis>"*"{name}</Emphasis>" refers to a node that contains it."
            })
            .with_range(range)
            .with_note("Recursive structures can't be deserialized."));
        }
        Ok(&anchor.node)
    }

    /// Charges the expansion of an alias to `node` to the budget of the document.
    ///
    /// Returns `false` if the budget is exceeded, and reports it the first time.
    pub(super) fn expand(
        &self,
        ctx: &mut dyn DeserializationContext,
        node: &YamlNode,
        alias_range: TextRange,
    ) -> bool {
        let Some(budget) = self.expansion_budget.get() else {
            return false;
        };
        let budget = budget.checked_sub(usize::from(node.range().len()));
        self.expansion_budget.set(budget);
        if budget.is_none() {
            ctx.report(
                DeserializationDiagnostic::new("Aliases expand to too much content.")
                    .with_range(alias_range)
                    .with_note(markup! {
                        "Aliases can expand to at most "{MAX_ALIAS_EXPANSION_FACTOR}" times the size of the document."
                    }),
            );
        }
        budget.is_some()
    }
}

/// Returns the node that `anchor` is attached to.
fn anchored_node(anchor: &YamlAnchorProperty) -> Option<YamlNode> {
    let properties = YamlPropertyList::cast(anchor.syntax().parent()?)?;
    let owner = properties.syntax().parent()?;
    if let Some(key) = AnyYamlMappingImplicitKey::cast_ref(&owner) {
        // The parser attaches the properties of a block mapping to its first key
        let index = properties
            .iter()
            .position(|property| property.syntax() == anchor.syntax())?;
        if index < key.enclosing_mapping_property_count() {
            let mapping = owner.ancestors().find_map(YamlBlockInBlockNode::cast)?;
            return Some(YamlNode::Block(mapping.into()));
        }
    }
    match AnyYamlFlowNode::cast_ref(&owner) {
        Some(node) => Some(YamlNode::Flow(node)),
        None => AnyYamlBlockNode::cast(owner).map(YamlNode::Block),
    }
}
