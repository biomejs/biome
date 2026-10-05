//! Syntax shared by collection and the flow-candidate index.

use biome_js_syntax::{
    JsCallExpression, JsConditionalExpression, JsDoWhileStatement, JsForStatement, JsIfStatement,
    JsLogicalExpression, JsSyntaxKind, JsWhileStatement,
};
use biome_rowan::{AstNode, SyntaxKind, declare_node_union};

declare_node_union! {
    pub(crate) FlowConditionSource = JsIfStatement
        | JsWhileStatement
        | JsDoWhileStatement
        | JsForStatement
        | JsLogicalExpression
        | JsConditionalExpression
        | JsCallExpression
}

// A supported statement contributes at most seven predecessor edges, a logical
// or conditional expression four, and a standalone call one. Stay below the
// 16,384 steps in `narrow_binding_at_flow`, which can return Unknown before
// checking relevance. Larger roots must retain reads absent from every condition.
pub(crate) const MAX_FLOW_CONSTRUCTS: usize = 1_024;

pub(crate) fn is_flow_construct(kind: JsSyntaxKind) -> bool {
    // Blocks, jumps and ordinary calls consume the flow-work budget even when
    // they do not supply conditions for narrowing.
    FlowConditionSource::can_cast(kind)
        || matches!(
            kind,
            JsSyntaxKind::JS_BLOCK_STATEMENT
                | JsSyntaxKind::JS_BREAK_STATEMENT
                | JsSyntaxKind::JS_CONTINUE_STATEMENT
        )
}

/// Detects sources that need no raw signature information.
///
/// The collector separately flags assertion signatures and counts calls for the
/// flow-work fallback, so ordinary calls alone do not enable candidate queries.
pub(crate) fn may_affect_flow_candidates(kind: JsSyntaxKind) -> bool {
    (is_flow_construct(kind) && kind != JsSyntaxKind::JS_CALL_EXPRESSION)
        || kind.is_bogus()
        || kind.is_metavariable()
}
