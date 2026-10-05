//! Rejects occurrence lookups that cannot depend on runtime conditions.
//!
//! The index stores source ranges, not inferred types or flow states. Semantic
//! references connect conditions to reads of the same binding. Expression
//! ancestors remain candidates even when their own evaluation is unsupported;
//! the occurrence query decides whether an operand actually changes their type.

use crate::JsModuleInfo;
use biome_js_control_flow::AnyJsControlFlowRoot;
use biome_js_semantic::JsDeclarationKind;
use biome_js_syntax::{
    AnyJsExpression, AnyJsRoot, AnyTsType, JsConditionalExpression, JsDoWhileStatement,
    JsForStatement, JsIdentifierExpression, JsIfStatement, JsLogicalExpression, JsSyntaxKind,
    JsWhileStatement,
};
use biome_rowan::{AstNode, SyntaxKind, TextRange, WalkEvent, declare_node_union};
use rustc_hash::FxHashSet;

const MAX_INDEX_STEPS: usize = 1_048_576;
// A supported statement contributes at most seven predecessor edges, and a
// logical or conditional expression contributes four. Stay below the 16,384
// steps in `narrow_binding_at_flow`: that walk can return Unknown before checking
// relevance. Larger roots must retain even reads absent from every condition.
const MAX_FLOW_CONSTRUCTS: usize = 1_024;

/// A conservative set of expressions that may have a flow override.
///
/// Incomplete indexing permits every lookup. A missing candidate only rules out
/// flow evaluation, not the expression's ordinary raw type lookup.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::db) struct FlowCandidates {
    expressions: Option<FxHashSet<TextRange>>,
}

impl FlowCandidates {
    pub(in crate::db) fn collect(info: &JsModuleInfo) -> Self {
        Self {
            expressions: collect_candidates(info),
        }
    }

    pub(in crate::db) fn contains(&self, expression: TextRange) -> bool {
        self.expressions
            .as_ref()
            .is_none_or(|expressions| expressions.contains(&expression))
    }
}

declare_node_union! {
    FlowConditionSource = JsIfStatement
        | JsWhileStatement
        | JsDoWhileStatement
        | JsForStatement
        | JsLogicalExpression
        | JsConditionalExpression
}

fn collect_candidates(info: &JsModuleInfo) -> Option<FxHashSet<TextRange>> {
    let mut remaining = MAX_INDEX_STEPS;
    let conditions = condition_ranges(&info.semantic_model.root(), &mut remaining)?;
    let mut candidates = FxHashSet::default();
    if conditions.is_empty() {
        return Some(candidates);
    }

    let mut expression_ancestors = Vec::new();
    for binding in info.semantic_model.all_bindings() {
        remaining = remaining.checked_sub(1)?;
        if binding.is_imported() || binding.declaration_kind() == JsDeclarationKind::HoistedValue {
            continue;
        }

        let mut mentioned = false;
        let mut written = false;
        for reference in binding.all_references() {
            remaining = remaining.checked_sub(1)?;
            if reference.is_write() {
                written = true;
                break;
            }
            let start = reference.range_start();
            let index = conditions.partition_point(|range| range.end() <= start);
            mentioned |= conditions
                .get(index)
                .is_some_and(|range| range.contains(start));
        }
        if written || !mentioned {
            continue;
        }

        let mut declaration_root = None;
        for ancestor in binding.syntax().ancestors().skip(1) {
            remaining = remaining.checked_sub(1)?;
            if let Some(root) = AnyJsControlFlowRoot::cast(ancestor) {
                declaration_root = Some(root);
                break;
            }
        }
        let Some(declaration_root) = declaration_root else {
            continue;
        };

        for reference in binding.all_reads() {
            remaining = remaining.checked_sub(1)?;
            let Some(identifier) = reference
                .syntax()
                .parent()
                .and_then(JsIdentifierExpression::cast)
            else {
                continue;
            };
            expression_ancestors.clear();
            for ancestor in identifier.syntax().ancestors() {
                remaining = remaining.checked_sub(1)?;
                if AnyJsControlFlowRoot::can_cast(ancestor.kind()) {
                    if &ancestor == declaration_root.syntax() {
                        candidates.extend(expression_ancestors.iter().copied());
                    }
                    break;
                }
                if AnyJsExpression::can_cast(ancestor.kind()) {
                    expression_ancestors.push(ancestor.text_trimmed_range());
                }
            }
        }
    }
    Some(candidates)
}

/// Collects condition subtrees and roots whose flow walk may exceed its budget.
///
/// Source order does not restrict candidates: a later loop condition can affect
/// an earlier read through a backedge. Overlapping ranges are merged only for
/// reference membership checks, not to represent control-flow relationships.
fn condition_ranges(root: &AnyJsRoot, remaining: &mut usize) -> Option<Vec<TextRange>> {
    let mut ranges = Vec::new();
    let mut root_constructs = Vec::<usize>::new();
    let mut traversal = root.syntax().preorder();
    while let Some(event) = traversal.next() {
        *remaining = remaining.checked_sub(1)?;
        match event {
            WalkEvent::Enter(node) => {
                // Type syntax cannot supply runtime conditions, even when it contains expressions.
                if AnyTsType::can_cast(node.kind())
                    || matches!(
                        node.kind(),
                        JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION
                            | JsSyntaxKind::TS_INTERFACE_DECLARATION
                            | JsSyntaxKind::TS_DECLARE_FUNCTION_DECLARATION
                    )
                {
                    traversal.skip_subtree();
                    traversal.next()?;
                    continue;
                }
                if node.kind().is_bogus() || node.kind().is_metavariable() {
                    return None;
                }
                if AnyJsControlFlowRoot::can_cast(node.kind()) {
                    root_constructs.push(0);
                }
                // Blocks and jumps also contribute to the flow-work budget, even
                // though they do not supply conditions for narrowing.
                if FlowConditionSource::can_cast(node.kind())
                    || matches!(
                        node.kind(),
                        JsSyntaxKind::JS_BLOCK_STATEMENT
                            | JsSyntaxKind::JS_BREAK_STATEMENT
                            | JsSyntaxKind::JS_CONTINUE_STATEMENT
                    )
                {
                    *root_constructs.last_mut()? += 1;
                }
                if let Some(source) = FlowConditionSource::cast(node) {
                    let condition = match source {
                        FlowConditionSource::JsIfStatement(statement) => {
                            Some(statement.test().ok()?)
                        }
                        FlowConditionSource::JsWhileStatement(statement) => {
                            Some(statement.test().ok()?)
                        }
                        FlowConditionSource::JsDoWhileStatement(statement) => {
                            Some(statement.test().ok()?)
                        }
                        FlowConditionSource::JsForStatement(statement) => statement.test(),
                        FlowConditionSource::JsLogicalExpression(expression) => {
                            Some(expression.left().ok()?)
                        }
                        FlowConditionSource::JsConditionalExpression(expression) => {
                            Some(expression.test().ok()?)
                        }
                    };
                    if let Some(condition) = condition {
                        ranges.push(condition.range());
                    }
                }
            }
            WalkEvent::Leave(node) => {
                if AnyJsControlFlowRoot::can_cast(node.kind())
                    && root_constructs.pop()? > MAX_FLOW_CONSTRUCTS
                {
                    ranges.push(node.text_trimmed_range());
                }
            }
        }
    }

    ranges.sort_unstable_by_key(|range| (range.start(), range.end()));
    let mut disjoint = Vec::<TextRange>::new();
    for range in ranges {
        if let Some(previous) = disjoint.last_mut()
            && range.start() <= previous.end()
        {
            *previous = TextRange::new(previous.start(), previous.end().max(range.end()));
        } else {
            disjoint.push(range);
        }
    }
    Some(disjoint)
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_languages::JsFileSource;

    #[test]
    fn incomplete_condition_index_keeps_all_expressions_eligible() {
        for (source, mut remaining) in [
            ("function f(x) { if (x) x; }", 1),
            ("function f(x) { if () x; }", MAX_INDEX_STEPS),
        ] {
            let root = parse(source, JsFileSource::ts(), JsParserOptions::default()).tree();
            assert!(condition_ranges(&root, &mut remaining).is_none());
            let candidates = FlowCandidates { expressions: None };
            assert!(candidates.contains(root.range()));
        }
    }

    #[test]
    fn type_only_subtrees_do_not_consume_the_condition_index_budget() {
        let variants = (0..256)
            .map(|index| format!("\"value{index}\""))
            .collect::<Vec<_>>()
            .join(" | ");
        let source = format!(
            "type Alias = {variants}; interface Shape {{ field: {variants} }}\n\
             declare function read(value: {variants}): {variants};\n\
             const value: {variants} = 'value0';"
        );
        let parsed = parse(&source, JsFileSource::ts(), JsParserOptions::default());
        assert!(!parsed.has_errors());
        let mut remaining = 100;
        assert_eq!(
            condition_ranges(&parsed.tree(), &mut remaining),
            Some(Vec::new())
        );
    }

    #[test]
    fn large_roots_keep_reads_even_without_conditions() {
        for count in [MAX_FLOW_CONSTRUCTS, MAX_FLOW_CONSTRUCTS + 1] {
            let source = format!("function f(value) {{ {} value; }}", "{}".repeat(count));
            let parsed = parse(&source, JsFileSource::ts(), JsParserOptions::default());
            assert!(!parsed.has_errors());
            let mut remaining = MAX_INDEX_STEPS;
            let ranges = condition_ranges(&parsed.tree(), &mut remaining).unwrap();
            assert_eq!(ranges.is_empty(), count == MAX_FLOW_CONSTRUCTS);
        }
    }
}
