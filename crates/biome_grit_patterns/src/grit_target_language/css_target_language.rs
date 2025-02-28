mod constants;

use super::{
    normalize_quoted_string, DisregardedSlotCondition, GritTargetLanguageImpl,
    LeafEquivalenceClass, LeafNormalizer,
};
use crate::{
    grit_target_node::{GritTargetNode, GritTargetSyntaxKind},
    CompileError,
};
use biome_css_syntax::{CssLanguage, CssSyntaxKind};
use biome_rowan::{RawSyntaxKind, SyntaxKindSet};
use constants::DISREGARDED_SNIPPET_SLOTS;

const COMMENT_KINDS: SyntaxKindSet<CssLanguage> =
    SyntaxKindSet::from_raw(RawSyntaxKind(CssSyntaxKind::COMMENT as u16)).union(
        SyntaxKindSet::from_raw(RawSyntaxKind(CssSyntaxKind::MULTILINE_COMMENT as u16)),
    );

const EQUIVALENT_LEAF_NODES: &[&[LeafNormalizer]] = &[&[LeafNormalizer::new(
    GritTargetSyntaxKind::CssSyntaxKind(CssSyntaxKind::CSS_STRING_LITERAL),
    normalize_quoted_string,
)]];

#[derive(Clone, Debug)]
pub struct CssTargetLanguage;

impl GritTargetLanguageImpl for CssTargetLanguage {
    type Kind = CssSyntaxKind;

    /// Returns the syntax kind for a node by name.
    ///
    /// For compatibility with existing Grit snippets (as well as the online
    /// Grit playground), node names should be aligned with TreeSitter's
    /// `ts_language_symbol_for_name()`.
    fn kind_by_name(&self, _node_name: &str) -> Option<CssSyntaxKind> {
        // TODO: See [super::JsTargetLanguage::kind_by_name()].
        None
    }

    /// Returns the node name for a given syntax kind.
    ///
    /// For compatibility with existing Grit snippets (as well as the online
    /// Grit playground), node names should be aligned with TreeSitter's
    /// `ts_language_symbol_name()`.
    fn name_for_kind(&self, _kind: GritTargetSyntaxKind) -> &'static str {
        // TODO: See [super::JsTargetLanguage::name_for_kind()].
        "(unknown node)"
    }

    /// Returns the slots with their names for the given node kind.
    ///
    /// For compatibility with existing Grit snippets (as well as the online
    /// Grit playground), node names should be aligned with TreeSitter's
    /// `ts_language_field_name_for_id()`.
    fn named_slots_for_kind(&self, _kind: GritTargetSyntaxKind) -> &'static [(&'static str, u32)] {
        // TODO: See [super::JsTargetLanguage::named_slots_for_kind()].
        &[]
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[
            ("", ""),
            ("GRIT_BLOCK { ", " }"),
            ("GRIT_BLOCK { GRIT_PROPERTY: ", " }"),
        ]
    }

    fn is_comment_kind(kind: GritTargetSyntaxKind) -> bool {
        kind.as_css_kind()
            .is_some_and(|kind| COMMENT_KINDS.matches(kind))
    }

    fn metavariable_kind() -> Self::Kind {
        CssSyntaxKind::CSS_METAVARIABLE
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
