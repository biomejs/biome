//! Syntax shared by collection and the flow-candidate index.

use biome_js_syntax::{
    JsConditionalExpression, JsDoWhileStatement, JsForStatement, JsIfStatement,
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
}

pub(crate) fn is_flow_construct(kind: JsSyntaxKind) -> bool {
    // Blocks and jumps also consume the flow-work budget, although they do not
    // supply conditions for narrowing.
    FlowConditionSource::can_cast(kind)
        || matches!(
            kind,
            JsSyntaxKind::JS_BLOCK_STATEMENT
                | JsSyntaxKind::JS_BREAK_STATEMENT
                | JsSyntaxKind::JS_CONTINUE_STATEMENT
        )
}

/// Returns false only for syntax that cannot require flow analysis or its fallback.
pub(crate) fn may_affect_flow_candidates(kind: JsSyntaxKind) -> bool {
    is_flow_construct(kind) || kind.is_bogus() || kind.is_metavariable()
}
