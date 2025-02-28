mod constants;

use super::{
    normalize_quoted_string, DisregardedSlotCondition, GritTargetLanguageImpl,
    LeafEquivalenceClass, LeafNormalizer,
};
use crate::{
    grit_target_node::{GritTargetNode, GritTargetSyntaxKind},
    CompileError,
};
use biome_js_syntax::{JsLanguage, JsSyntaxKind};
use biome_rowan::{RawSyntaxKind, SyntaxKindSet};
use constants::DISREGARDED_SNIPPET_SLOTS;

const COMMENT_KINDS: SyntaxKindSet<JsLanguage> =
    SyntaxKindSet::from_raw(RawSyntaxKind(JsSyntaxKind::COMMENT as u16)).union(
        SyntaxKindSet::from_raw(RawSyntaxKind(JsSyntaxKind::MULTILINE_COMMENT as u16)),
    );

const EQUIVALENT_LEAF_NODES: &[&[LeafNormalizer]] = &[&[
    LeafNormalizer::new(
        GritTargetSyntaxKind::JsSyntaxKind(JsSyntaxKind::JS_STRING_LITERAL),
        normalize_quoted_string,
    ),
    LeafNormalizer::new(
        GritTargetSyntaxKind::JsSyntaxKind(JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION),
        normalize_quoted_string,
    ),
]];

#[derive(Clone, Debug)]
pub struct JsTargetLanguage;

impl GritTargetLanguageImpl for JsTargetLanguage {
    type Kind = JsSyntaxKind;

    /// Returns the syntax kind for a node by name.
    ///
    /// For compatibility with existing Grit snippets (as well as the online
    /// Grit playground), node names should be aligned with TreeSitter's
    /// `ts_language_symbol_for_name()`.
    fn kind_by_name(&self, node_name: &str) -> Option<JsSyntaxKind> {
        use JsSyntaxKind::*;
        let kind = match node_name {
            "assignment_expression" => JS_ASSIGNMENT_EXPRESSION,
            "call_expression" => JS_CALL_EXPRESSION,
            "new_expression" => JS_NEW_EXPRESSION,
            // TODO: Many more of these. We should probably find a way to
            // generate these impls from TS `grammar.js` files, combined with
            // our `js.ungram`.
            _ => return None,
        };

        Some(kind)
    }

    /// Returns the node name for a given syntax kind.
    ///
    /// For compatibility with existing Grit snippets (as well as the online
    /// Grit playground), node names should be aligned with TreeSitter's
    /// `ts_language_symbol_name()`.
    fn name_for_kind(&self, kind: GritTargetSyntaxKind) -> &'static str {
        let Some(kind) = kind.as_js_kind() else {
            return "(unexpected language)";
        };

        use JsSyntaxKind::*;
        match kind {
            JS_ASSIGNMENT_EXPRESSION => "assignment_expression",
            JS_CALL_EXPRESSION => "call_expression",
            JS_NEW_EXPRESSION => "new_expression",
            // TODO: Many more of these. We should probably find a way to
            // generate these impls from TS `grammar.js` files, combined with
            // our `js.ungram`.
            _ => "(unknown node)",
        }
    }

    /// Returns the slots with their names for the given node kind.
    ///
    /// For compatibility with existing Grit snippets (as well as the online
    /// Grit playground), node names should be aligned with TreeSitter's
    /// `ts_language_field_name_for_id()`.
    fn named_slots_for_kind(&self, kind: GritTargetSyntaxKind) -> &'static [(&'static str, u32)] {
        let Some(kind) = kind.as_js_kind() else {
            return &[];
        };

        use JsSyntaxKind::*;
        match kind {
            JS_ASSIGNMENT_EXPRESSION => &[],
            JS_CALL_EXPRESSION => &[("function", 0), ("type_arguments", 2), ("arguments", 3)],
            JS_NEW_EXPRESSION => &[],
            // TODO: Many more of these. We should probably find a way to
            // generate these impls from TS `grammar.js` files, combined with
            // our `js.ungram`.
            _ => &[],
        }
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[
            ("", ""),
            ("import ", " from 'GRIT_PACKAGE';"),
            ("GRIT_OBJECT.", ""),
            ("GRIT_VALUE ", " GRIT_VALUE"),
            ("class GRIT_CLASS ", " {}"),
            ("class GRIT_CLASS { ", " GRIT_PROP = 'GRIT_VALUE'; }"),
            ("", "  function GRIT_FUNCTION() {}"),
            ("GRIT_OBJ = { ", " }"),
            ("class GRIT_CLASS { ", " }"),
            ("GRIT_VAR = ", ""),
            ("<f>", "</f>"),
            ("<f ", " />"),
            ("function GRIT_FN(", ") {}"),
            ("function GRIT_FN() {", "}"),
            ("var ", ";"),
            ("", " class GRIT_CLASS {}"),
            ("function GRIT_FN(GRIT_ARG:", ") { }"),
            ("import { ", " } from 'GRIT_PACKAGE'"),
            ("function GRIT_FN(GRIT_ARG", ") { }"),
            ("GRIT_FN<{ ", " }>();"),
        ]
    }

    fn is_comment_kind(kind: GritTargetSyntaxKind) -> bool {
        kind.as_js_kind()
            .is_some_and(|kind| COMMENT_KINDS.matches(kind))
    }

    fn metavariable_kind() -> Self::Kind {
        JsSyntaxKind::JS_METAVARIABLE
    }

    fn is_alternative_metavariable_kind(kind: GritTargetSyntaxKind) -> bool {
        kind.as_js_kind().is_some_and(|kind| {
            kind == JsSyntaxKind::JS_TEMPLATE_ELEMENT_LIST
                || kind == JsSyntaxKind::TS_TEMPLATE_ELEMENT_LIST
        })
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
