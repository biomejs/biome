mod constants;
pub mod generated_mappings;

use super::{
    DisregardedSlotCondition, GritTargetLanguageImpl, LeafEquivalenceClass, LeafNormalizer,
    normalize_quoted_string,
};
use crate::{
    CompileError,
    grit_target_node::{GritTargetNode, GritTargetSyntaxKind},
};
use biome_json_syntax::{JsonLanguage, JsonSyntaxKind};
use biome_rowan::{RawSyntaxKind, SyntaxKindSet};
use constants::DISREGARDED_SNIPPET_SLOTS;
use generated_mappings::{
    kind_by_name, legacy_treesitter_name_for_kind, legacy_treesitter_slots_for_kind,
};

const COMMENT_KINDS: SyntaxKindSet<JsonLanguage> =
    SyntaxKindSet::from_raw(RawSyntaxKind(JsonSyntaxKind::COMMENT as u16)).union(
        SyntaxKindSet::from_raw(RawSyntaxKind(JsonSyntaxKind::MULTILINE_COMMENT as u16)),
    );

const EQUIVALENT_LEAF_NODES: &[&[LeafNormalizer]] = &[&[LeafNormalizer::new(
    GritTargetSyntaxKind::JsonSyntaxKind(JsonSyntaxKind::JSON_STRING_LITERAL),
    normalize_quoted_string,
)]];

#[derive(Clone, Debug)]
pub struct JsonTargetLanguage;

impl GritTargetLanguageImpl for JsonTargetLanguage {
    type Kind = JsonSyntaxKind;

    /// Returns the syntax kind for a node by name.
    ///
    /// Supports native Biome AST patterns for full language coverage.
    fn kind_by_name(&self, node_name: &str) -> Option<JsonSyntaxKind> {
        kind_by_name(node_name)
    }

    /// Returns the node name for a given syntax kind.
    ///
    /// For compatibility with existing Grit snippets (as well as the online
    /// Grit playground), node names should be aligned with TreeSitter's
    /// `ts_language_symbol_name()`.
    fn name_for_kind(&self, kind: GritTargetSyntaxKind) -> &'static str {
        let Some(kind) = kind.as_json_kind() else {
            return "(unexpected language)";
        };
        legacy_treesitter_name_for_kind(kind).unwrap_or("(unknown node)")
    }

    /// Returns the slots with their names for the given node kind.
    ///
    /// For compatibility with existing Grit snippets (as well as the online
    /// Grit playground), node names should be aligned with TreeSitter's
    /// `ts_language_field_name_for_id()`.
    fn named_slots_for_kind(&self, kind: GritTargetSyntaxKind) -> &'static [(&'static str, u32)] {
        let Some(kind) = kind.as_json_kind() else {
            return &[];
        };
        legacy_treesitter_slots_for_kind(kind)
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[("", ""), ("{\"GRIT_KEY\": ", "}"), ("[", "]")]
    }

    fn is_comment_kind(kind: GritTargetSyntaxKind) -> bool {
        kind.as_json_kind()
            .is_some_and(|kind| COMMENT_KINDS.matches(kind))
    }

    fn metavariable_kind() -> Self::Kind {
        JsonSyntaxKind::JSON_METAVARIABLE
    }

    fn is_disregarded_snippet_field(
        &self,
        kind: GritTargetSyntaxKind,
        slot_index: u32,
        node: Option<GritTargetNode<'_>>,
    ) -> bool {
        DISREGARDED_SNIPPET_SLOTS.iter().any(
            |(disregarded_kind, disregarded_slot_index, condition)| {
                if GritTargetSyntaxKind::from(*disregarded_kind) != kind
                    || *disregarded_slot_index != slot_index
                {
                    return false;
                }

                match condition {
                    DisregardedSlotCondition::Always => true,
                    DisregardedSlotCondition::OnlyIf(node_texts) => node_texts.iter().any(|text| {
                        *text == node.as_ref().map(|node| node.text()).unwrap_or_default()
                    }),
                }
            },
        )
    }

    fn get_equivalence_class(
        &self,
        kind: GritTargetSyntaxKind,
        text: &str,
    ) -> Result<Option<LeafEquivalenceClass>, CompileError> {
        if let Some(class) = EQUIVALENT_LEAF_NODES
            .iter()
            .find(|v| v.iter().any(|normalizer| normalizer.kind() == kind))
        {
            LeafEquivalenceClass::new(text, kind, class)
        } else {
            Ok(None)
        }
    }
}
