//! Occurrence-local conditions layered over the statement control-flow graph.

use crate::{AnyJsControlFlowRoot, JsControlFlowGraph};
use biome_control_flow::InstructionKind;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsOptionalChainExpression, AnyJsStatement, AnyTsType,
    JsArrowFunctionExpression, JsDoWhileStatement, JsExpressionStatement, JsForStatement,
    JsIfStatement, JsSyntaxKind, JsSyntaxNode, JsWhileStatement, T,
};
use biome_rowan::{AstNode, SyntaxKind, TextRange};
use rustc_hash::{FxHashMap, FxHashSet};

pub type FlowNodeId = usize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum FlowOutcome {
    Truthy,
    Falsy,
    Nullish,
    NonNullish,
}

/// A predecessor relation, not an instruction to execute.
///
/// Joins retain every reachable predecessor. Loop backedges can make this graph
/// cyclic, so consumers must bound their traversal and handle cycles conservatively.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FlowNode {
    Start,
    Join(Vec<FlowNodeId>),
    Condition {
        antecedent: FlowNodeId,
        expression: TextRange,
        outcome: FlowOutcome,
    },
    /// Normal completion of a standalone call, after evaluating its callee and arguments.
    /// The range identifies the call itself, excluding surrounding parentheses.
    CallContinuation {
        antecedent: FlowNodeId,
        expression: TextRange,
    },
}

/// Syntax-only flow for one execution root and one source snapshot.
///
/// Ranges are trimmed, source-local ranges from the input CFG. Each indexed
/// expression points to its incoming flow, before evaluating its children or
/// applying a condition or call continuation. Unreachable expressions and nested
/// execution-root bodies are not indexed. Reference identifiers are also indexed,
/// including object shorthand reads that have no expression wrapper.
///
/// This graph does not invalidate facts on writes or calls. Consumers must limit
/// narrowing to bindings proven unwritten and free of capture hazards.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NarrowingFlowGraph {
    pub nodes: Vec<FlowNode>,
    pub expression_flows: FxHashMap<TextRange, FlowNodeId>,
}

const MAX_SYNTAX_NODES: usize = 16_384;
const MAX_DEPTH: usize = 128;
const MAX_BLOCKS: usize = 4_096;
const MAX_INSTRUCTIONS: usize = 16_384;
const MAX_FLOW_NODES: usize = 65_536;

/// Builds occurrence-local flow using the CFG's ordered, reachable instructions.
///
/// Supports truthiness tests in `if`, `while`, `do` and ordinary `for` statements,
/// and expression-level `&&`, `||`, `??` and `?:`, including expression-bodied
/// arrows. Non-optional calls that form an entire expression statement, allowing
/// surrounding parentheses, add a normal-completion event. Calls with optional
/// callee chains or nested in other expressions do not. Parameter initializers
/// are not indexed.
///
/// Returns `None` for unsupported control flow or evaluation order: exception
/// handlers, switch, for-in/of, with, classes, destructuring and logical
/// assignments. Optional chains retain incoming facts without adding a receiver
/// condition. Bogus syntax, missing branch operands, foreign-snapshot instructions
/// and inputs exceeding construction limits also return `None`. Nested execution
/// roots are isolated, so unsupported syntax inside them does not reject this root.
pub fn narrowing_flow_graph(graph: &JsControlFlowGraph) -> Option<NarrowingFlowGraph> {
    AnyJsControlFlowRoot::cast_ref(&graph.node)?;
    if graph.blocks.is_empty() || graph.blocks.len() > MAX_BLOCKS {
        return None;
    }
    let mut instructions = 0usize;
    for block in &graph.blocks {
        instructions = instructions.checked_add(block.instructions.len())?;
        if instructions > MAX_INSTRUCTIONS
            || !block.exception_handlers.is_empty()
            || !block.cleanup_handlers.is_empty()
        {
            return None;
        }
    }

    let syntax = validate_syntax(&graph.node)?;
    let reachable = reachable_blocks(graph)?;
    let mut builder = FlowBuilder {
        graph: NarrowingFlowGraph {
            nodes: vec![FlowNode::Start],
            expression_flows: FxHashMap::default(),
        },
        remaining_visits: MAX_SYNTAX_NODES,
    };
    let entries: Vec<_> = reachable
        .iter()
        .enumerate()
        .map(|(index, reachable)| {
            if *reachable {
                builder
                    .push(FlowNode::Join(if index == 0 { vec![0] } else { vec![] }))
                    .map(Some)
            } else {
                Some(None)
            }
        })
        .collect::<Option<_>>()?;

    let arrow_body = match JsArrowFunctionExpression::cast_ref(&graph.node) {
        Some(arrow) => match arrow.body().ok()? {
            AnyJsFunctionBody::AnyJsExpression(expression) => Some(expression),
            AnyJsFunctionBody::JsFunctionBody(_) => None,
        },
        None => None,
    };
    if arrow_body.is_some()
        && (graph.blocks.len() != 1
            || graph.blocks[0].instructions.len() != 1
            || graph.blocks[0].instructions[0].kind != InstructionKind::Return
            || graph.blocks[0].instructions[0].node.is_some())
    {
        return None;
    }

    for (index, block) in graph.blocks.iter().enumerate() {
        let Some(mut flow) = entries[index] else {
            continue;
        };
        if index == 0
            && let Some(body) = &arrow_body
        {
            flow = builder.evaluate(body.syntax(), flow, 0)?;
        }
        for instruction in &block.instructions {
            let node = match &instruction.node {
                Some(element) => {
                    let node = element.as_node()?;
                    if !syntax.contains(node) {
                        return None;
                    }
                    Some(node)
                }
                None => None,
            };
            match instruction.kind {
                InstructionKind::Statement => {
                    let node = node?;
                    if !AnyJsExpression::can_cast(node.kind())
                        && !matches!(
                            node.kind(),
                            JsSyntaxKind::JS_EXPRESSION_STATEMENT
                                | JsSyntaxKind::JS_VARIABLE_DECLARATION
                                | JsSyntaxKind::JS_DEBUGGER_STATEMENT
                                | JsSyntaxKind::JS_EMPTY_STATEMENT
                        )
                    {
                        return None;
                    }
                    flow = builder.evaluate(node, flow, 0)?;
                    if let Some(expression) = JsExpressionStatement::cast_ref(node)
                        .as_ref()
                        .and_then(standalone_call_range)
                    {
                        flow = builder.push(FlowNode::CallContinuation {
                            antecedent: flow,
                            expression,
                        })?;
                    }
                }
                InstructionKind::Jump {
                    conditional,
                    block,
                    finally_fallthrough,
                } => {
                    if finally_fallthrough {
                        return None;
                    }
                    let target = entries.get(block.index() as usize).copied().flatten()?;
                    if conditional {
                        let expression = AnyJsExpression::cast_ref(node?)?;
                        if !is_truthiness_test(&expression) {
                            return None;
                        }
                        flow = builder.evaluate(expression.syntax(), flow, 0)?;
                        let positive =
                            builder.condition(flow, expression.range(), FlowOutcome::Truthy)?;
                        builder.predecessor(target, positive)?;
                        flow = builder.condition(flow, expression.range(), FlowOutcome::Falsy)?;
                    } else {
                        if let Some(node) = node {
                            match node.kind() {
                                JsSyntaxKind::JS_BREAK_STATEMENT
                                | JsSyntaxKind::JS_CONTINUE_STATEMENT => {}
                                _ => {
                                    let expression = AnyJsExpression::cast_ref(node)?;
                                    if !is_truthiness_test(&expression) {
                                        return None;
                                    }
                                    flow = builder.evaluate(node, flow, 0)?;
                                }
                            }
                        }
                        builder.predecessor(target, flow)?;
                        break;
                    }
                }
                InstructionKind::Return => {
                    if let Some(node) = node {
                        if !matches!(
                            node.kind(),
                            JsSyntaxKind::JS_RETURN_STATEMENT | JsSyntaxKind::JS_THROW_STATEMENT
                        ) {
                            return None;
                        }
                        builder.evaluate(node, flow, 0)?;
                    }
                    break;
                }
            }
        }
    }
    Some(builder.graph)
}

fn validate_syntax(root: &JsSyntaxNode) -> Option<FxHashSet<JsSyntaxNode>> {
    let mut nodes = FxHashSet::default();
    let mut pending = vec![(root.clone(), 0)];
    let mut remaining = MAX_SYNTAX_NODES - 1;
    while let Some((node, depth)) = pending.pop() {
        if depth > MAX_DEPTH {
            return None;
        }
        nodes.insert(node.clone());
        if &node != root && AnyJsControlFlowRoot::can_cast(node.kind()) {
            continue;
        }
        if node.kind().is_bogus() || node.kind().is_metavariable() {
            return None;
        }
        if let Some(statement) = AnyJsStatement::cast_ref(&node) {
            match statement {
                AnyJsStatement::JsBlockStatement(_)
                | AnyJsStatement::JsBreakStatement(_)
                | AnyJsStatement::JsContinueStatement(_)
                | AnyJsStatement::JsDebuggerStatement(_)
                | AnyJsStatement::JsDoWhileStatement(_)
                | AnyJsStatement::JsEmptyStatement(_)
                | AnyJsStatement::JsExpressionStatement(_)
                | AnyJsStatement::JsForStatement(_)
                | AnyJsStatement::JsFunctionDeclaration(_)
                | AnyJsStatement::JsIfStatement(_)
                | AnyJsStatement::JsLabeledStatement(_)
                | AnyJsStatement::JsReturnStatement(_)
                | AnyJsStatement::JsThrowStatement(_)
                | AnyJsStatement::JsVariableStatement(_)
                | AnyJsStatement::JsWhileStatement(_) => {}
                AnyJsStatement::TsTypeAliasDeclaration(_)
                | AnyJsStatement::TsInterfaceDeclaration(_)
                | AnyJsStatement::TsDeclareFunctionDeclaration(_) => continue,
                _ => return None,
            }
        }
        if matches!(
            node.kind(),
            JsSyntaxKind::JS_CLASS_EXPRESSION
                | JsSyntaxKind::JS_CLASS_EXPORT_DEFAULT_DECLARATION
                | JsSyntaxKind::JS_ARRAY_BINDING_PATTERN
                | JsSyntaxKind::JS_OBJECT_BINDING_PATTERN
                | JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN
                | JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN
                | JsSyntaxKind::TS_PROPERTY_SIGNATURE_TYPE_MEMBER
        ) {
            return None;
        }
        for child in node.children_with_tokens() {
            if let Some(token) = child.as_token()
                && matches!(token.kind(), T![&&=] | T![||=] | T![??=])
            {
                return None;
            }
            if let Some(child) = child.into_node() {
                remaining = remaining.checked_sub(1)?;
                pending.push((child, depth + 1));
            }
        }
    }
    Some(nodes)
}

fn reachable_blocks(graph: &JsControlFlowGraph) -> Option<Vec<bool>> {
    let mut reachable = vec![false; graph.blocks.len()];
    reachable[0] = true;
    let mut pending = vec![0];
    while let Some(index) = pending.pop() {
        let mut terminated = false;
        for instruction in &graph.blocks[index].instructions {
            match instruction.kind {
                InstructionKind::Statement => {}
                InstructionKind::Jump {
                    conditional,
                    block,
                    finally_fallthrough,
                } => {
                    if finally_fallthrough {
                        return None;
                    }
                    let index = block.index() as usize;
                    let visited = reachable.get_mut(index)?;
                    if !*visited {
                        *visited = true;
                        pending.push(index);
                    }
                    if !conditional {
                        terminated = true;
                        break;
                    }
                }
                InstructionKind::Return => {
                    terminated = true;
                    break;
                }
            }
        }
        if !terminated {
            return None;
        }
    }
    Some(reachable)
}

fn is_truthiness_test(expression: &AnyJsExpression) -> bool {
    let Some(parent) = expression.syntax().parent() else {
        return false;
    };
    let test = match parent.kind() {
        JsSyntaxKind::JS_IF_STATEMENT => JsIfStatement::cast(parent).and_then(|s| s.test().ok()),
        JsSyntaxKind::JS_WHILE_STATEMENT => {
            JsWhileStatement::cast(parent).and_then(|s| s.test().ok())
        }
        JsSyntaxKind::JS_DO_WHILE_STATEMENT => {
            JsDoWhileStatement::cast(parent).and_then(|s| s.test().ok())
        }
        JsSyntaxKind::JS_FOR_STATEMENT => JsForStatement::cast(parent).and_then(|s| s.test()),
        _ => None,
    };
    test.as_ref() == Some(expression)
}

fn standalone_call_range(statement: &JsExpressionStatement) -> Option<TextRange> {
    let AnyJsExpression::JsCallExpression(call) = statement.expression().ok()?.omit_parentheses()
    else {
        return None;
    };
    let range = call.range();
    let mut callee = AnyJsOptionalChainExpression::from(call);
    loop {
        if callee.is_optional() {
            return None;
        }
        let object = callee.object().ok()?.inner_expression()?;
        let Some(expression) = AnyJsOptionalChainExpression::cast(object.into_syntax()) else {
            return Some(range);
        };
        callee = expression;
    }
}

struct FlowBuilder {
    graph: NarrowingFlowGraph,
    remaining_visits: usize,
}

impl FlowBuilder {
    fn push(&mut self, node: FlowNode) -> Option<FlowNodeId> {
        if self.graph.nodes.len() >= MAX_FLOW_NODES {
            return None;
        }
        let id = self.graph.nodes.len();
        self.graph.nodes.push(node);
        Some(id)
    }

    fn predecessor(&mut self, join: FlowNodeId, flow: FlowNodeId) -> Option<()> {
        let FlowNode::Join(predecessors) = self.graph.nodes.get_mut(join)? else {
            return None;
        };
        predecessors.push(flow);
        Some(())
    }

    fn condition(
        &mut self,
        antecedent: FlowNodeId,
        expression: TextRange,
        outcome: FlowOutcome,
    ) -> Option<FlowNodeId> {
        self.push(FlowNode::Condition {
            antecedent,
            expression,
            outcome,
        })
    }

    fn evaluate(
        &mut self,
        node: &JsSyntaxNode,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Option<FlowNodeId> {
        self.remaining_visits = self.remaining_visits.checked_sub(1)?;
        if depth > MAX_DEPTH {
            return None;
        }
        let expression = AnyJsExpression::cast_ref(node);
        if expression.is_some() || node.kind() == JsSyntaxKind::JS_REFERENCE_IDENTIFIER {
            let previous = self
                .graph
                .expression_flows
                .insert(node.text_trimmed_range(), incoming);
            if previous.is_some_and(|previous| previous != incoming) {
                return None;
            }
        }
        if AnyJsControlFlowRoot::can_cast(node.kind()) || AnyTsType::can_cast(node.kind()) {
            return Some(incoming);
        }
        match expression {
            Some(AnyJsExpression::JsLogicalExpression(logical)) => {
                let left = logical.left().ok()?;
                let right = logical.right().ok()?;
                let (evaluate_right, skip_right) = match logical.operator_token().ok()?.kind() {
                    T![&&] => (FlowOutcome::Truthy, FlowOutcome::Falsy),
                    T![||] => (FlowOutcome::Falsy, FlowOutcome::Truthy),
                    T![??] => (FlowOutcome::Nullish, FlowOutcome::NonNullish),
                    _ => return None,
                };
                let left_flow = self.evaluate(left.syntax(), incoming, depth + 1)?;
                let right_flow = self.condition(left_flow, left.range(), evaluate_right)?;
                let right_flow = self.evaluate(right.syntax(), right_flow, depth + 1)?;
                let skip_flow = self.condition(left_flow, left.range(), skip_right)?;
                self.push(FlowNode::Join(vec![skip_flow, right_flow]))
            }
            Some(AnyJsExpression::JsConditionalExpression(conditional)) => {
                let test = conditional.test().ok()?;
                let consequent = conditional.consequent().ok()?;
                let alternate = conditional.alternate().ok()?;
                let test_flow = self.evaluate(test.syntax(), incoming, depth + 1)?;
                let positive = self.condition(test_flow, test.range(), FlowOutcome::Truthy)?;
                let positive = self.evaluate(consequent.syntax(), positive, depth + 1)?;
                let negative = self.condition(test_flow, test.range(), FlowOutcome::Falsy)?;
                let negative = self.evaluate(alternate.syntax(), negative, depth + 1)?;
                self.push(FlowNode::Join(vec![positive, negative]))
            }
            _ => {
                let mut flow = incoming;
                for child in node.children() {
                    flow = self.evaluate(&child, flow, depth + 1)?;
                }
                Some(flow)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FunctionBuilder, control_flow_model};
    use biome_js_parser::{JsParserOptions, parse};
    use biome_languages::JsFileSource;
    use biome_rowan::TextSize;

    static_assertions::assert_impl_all!(NarrowingFlowGraph: Send, Sync, Eq);

    fn cfg(source: &str, kind: JsSyntaxKind) -> JsControlFlowGraph {
        let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
        assert!(!parsed.has_errors(), "{source}");
        control_flow_model(&parsed.tree())
            .graphs()
            .filter(|graph| graph.node.kind() == kind)
            .min_by_key(|graph| graph.node.text_trimmed_range().start())
            .unwrap()
    }

    fn flows(source: &str) -> NarrowingFlowGraph {
        narrowing_flow_graph(&cfg(source, JsSyntaxKind::JS_FUNCTION_DECLARATION)).unwrap()
    }

    fn range(source: &str, text: &str, occurrence: usize) -> TextRange {
        let start = source.match_indices(text).nth(occurrence).unwrap().0;
        TextRange::at(
            TextSize::try_from(start).unwrap(),
            TextSize::try_from(text.len()).unwrap(),
        )
    }

    fn paths(
        graph: &NarrowingFlowGraph,
        occurrence: TextRange,
    ) -> Vec<Vec<(TextRange, FlowOutcome)>> {
        let mut result = Vec::new();
        let mut pending = vec![(graph.expression_flows[&occurrence], Vec::new())];
        let mut budget = 1_000usize;
        while let Some((flow, mut facts)) = pending.pop() {
            budget = budget
                .checked_sub(1)
                .expect("cyclic or excessive test graph");
            match &graph.nodes[flow] {
                FlowNode::Start => result.push(facts),
                FlowNode::Join(predecessors) => {
                    for predecessor in predecessors {
                        pending.push((*predecessor, facts.clone()));
                    }
                }
                FlowNode::Condition {
                    antecedent,
                    expression,
                    outcome,
                } => {
                    facts.push((*expression, *outcome));
                    pending.push((*antecedent, facts));
                }
                FlowNode::CallContinuation { antecedent, .. } => {
                    pending.push((*antecedent, facts));
                }
            }
        }
        assert!(!result.is_empty());
        result
    }

    fn call_continuations(graph: &NarrowingFlowGraph) -> Vec<TextRange> {
        graph
            .nodes
            .iter()
            .filter_map(|node| match node {
                FlowNode::CallContinuation { expression, .. } => Some(*expression),
                _ => None,
            })
            .collect()
    }

    fn assert_fact(
        graph: &NarrowingFlowGraph,
        occurrence: TextRange,
        condition: TextRange,
        outcome: FlowOutcome,
    ) {
        assert!(
            paths(graph, occurrence)
                .iter()
                .all(|path| path.contains(&(condition, outcome)))
        );
    }

    #[test]
    fn if_else_and_join() {
        let source = "function f(x) { if (x) yes(x); else no(x); after(x); }";
        let graph = flows(source);
        let guard = range(source, "x", 1);
        assert_fact(
            &graph,
            range(source, "yes(x)", 0),
            guard,
            FlowOutcome::Truthy,
        );
        assert_fact(&graph, range(source, "no(x)", 0), guard, FlowOutcome::Falsy);
        let joined = paths(&graph, range(source, "after(x)", 0));
        assert_eq!(joined.len(), 2);
        assert!(
            joined
                .iter()
                .any(|path| path.contains(&(guard, FlowOutcome::Truthy)))
        );
        assert!(
            joined
                .iter()
                .any(|path| path.contains(&(guard, FlowOutcome::Falsy)))
        );
        assert_eq!(paths(&graph, guard), vec![vec![]]);
        let FlowNode::Join(predecessors) =
            &graph.nodes[graph.expression_flows[&range(source, "after(x)", 0)]]
        else {
            panic!("expected a join after the branches");
        };
        assert_eq!(predecessors.len(), 2);
        for call in ["yes(x)", "no(x)"] {
            let expression = range(source, call, 0);
            assert!(predecessors.iter().any(|predecessor| {
                graph.nodes[*predecessor]
                    == FlowNode::CallContinuation {
                        antecedent: graph.expression_flows[&expression],
                        expression,
                    }
            }));
        }
    }

    #[test]
    fn early_return_discards_later_instructions_and_edges() {
        let source = "function f(x) { if (x) { check(x); return result(x); dead(x); } after(x); }";
        let graph = flows(source);
        let guard = range(source, "x", 1);
        assert_fact(
            &graph,
            range(source, "result(x)", 0),
            guard,
            FlowOutcome::Truthy,
        );
        let check = range(source, "check(x)", 0);
        assert_eq!(
            graph.nodes[graph.expression_flows[&range(source, "result(x)", 0)]],
            FlowNode::CallContinuation {
                antecedent: graph.expression_flows[&check],
                expression: check,
            }
        );
        assert_eq!(
            call_continuations(&graph),
            vec![check, range(source, "after(x)", 0)]
        );
        assert_fact(
            &graph,
            range(source, "after(x)", 0),
            guard,
            FlowOutcome::Falsy,
        );
        assert!(
            !graph
                .expression_flows
                .contains_key(&range(source, "dead(x)", 0))
        );
        assert_eq!(paths(&graph, range(source, "after(x)", 0)).len(), 1);
    }

    #[test]
    fn standalone_calls_keep_incoming_facts_and_continue_after_the_statement() {
        for (statement, call) in [
            ("assert(x)", "assert(x)"),
            ("(assert(x))", "assert(x)"),
            ("((assert(x)))", "assert(x)"),
            ("(assert)(x)", "(assert)(x)"),
            ("ordinary(x?.value)", "ordinary(x?.value)"),
        ] {
            let source = format!("function f(x) {{ if (x) {{ {statement}; after; }} }}");
            let graph = flows(&source);
            let expression = range(&source, call, 0);
            let incoming = graph.expression_flows[&expression];
            assert_fact(
                &graph,
                expression,
                range(&source, "x", 1),
                FlowOutcome::Truthy,
            );
            assert_eq!(call_continuations(&graph), vec![expression]);
            assert_eq!(
                graph.nodes[graph.expression_flows[&range(&source, "after", 0)]],
                FlowNode::CallContinuation {
                    antecedent: incoming,
                    expression,
                }
            );
            let statement = range(&source, statement, 0);
            for (occurrence, flow) in &graph.expression_flows {
                if statement.contains_range(*occurrence) {
                    assert_eq!(*flow, incoming);
                }
            }
        }
    }

    #[test]
    fn call_continuations_follow_the_callee_and_all_arguments() {
        let call = "(flag ? left : right)(x || first(x), y ? second(y) : third(y), last(x))";
        let source = format!("function f(flag, x, y) {{ (({call})); after; }}");
        let graph = flows(&source);
        let expression = range(&source, call, 0);
        let continuation = graph.expression_flows[&range(&source, "after", 0)];
        let last_argument = range(&source, "last(x)", 0);
        assert_eq!(call_continuations(&graph), vec![expression]);
        assert_eq!(paths(&graph, last_argument).len(), 8);
        assert_eq!(
            graph.nodes[continuation],
            FlowNode::CallContinuation {
                antecedent: graph.expression_flows[&last_argument],
                expression,
            }
        );
        for (occurrence, flow) in &graph.expression_flows {
            if expression.contains_range(*occurrence) {
                assert!(*flow < continuation);
            }
        }
    }

    #[test]
    fn calls_in_other_expression_contexts_do_not_continue() {
        for body in [
            "const value = assert(x);",
            "x = assert(x);",
            "x && assert(x);",
            "x || assert(x);",
            "x ?? assert(x);",
            "x ? assert(x) : other(x);",
            "(assert(x), other(x));",
            "void assert(x);",
            "!assert(x);",
            "assert(x).member;",
            "new Factory(assert(x));",
            "if (assert(x)) {}",
            "while (assert(x)) {}",
            "for (assert(x); x; assert(x)) {}",
            "return assert(x);",
            "throw assert(x);",
        ] {
            let source = format!("function f(x) {{ {body} }}");
            let graph = flows(&source);
            assert!(call_continuations(&graph).is_empty(), "{body}");
            assert!(
                graph
                    .expression_flows
                    .contains_key(&range(&source, "assert(x)", 0)),
                "{body}"
            );
        }
    }

    #[test]
    fn guard_operands_use_incoming_not_resulting_facts() {
        let source = "function f(x) { if (x) { if (x === null) read(x); } }";
        let graph = flows(source);
        let outer = range(source, "x", 1);
        let inner = range(source, "x === null", 0);
        for read in [inner, range(source, "x", 2), range(source, "null", 0)] {
            assert_fact(&graph, read, outer, FlowOutcome::Truthy);
            assert!(
                paths(&graph, read)
                    .iter()
                    .all(|path| { !path.iter().any(|(expression, _)| *expression == inner) })
            );
        }
        assert_fact(
            &graph,
            range(source, "read(x)", 0),
            inner,
            FlowOutcome::Truthy,
        );
    }

    #[test]
    fn logical_operators_index_nested_operand_reads_and_join() {
        for (operator, outcome) in [
            ("&&", FlowOutcome::Truthy),
            ("||", FlowOutcome::Falsy),
            ("??", FlowOutcome::Nullish),
        ] {
            let source = format!("function f(x) {{ x {operator} read(x); after(x); }}");
            let graph = flows(&source);
            let guard = range(&source, "x", 1);
            for read in [range(&source, "read(x)", 0), range(&source, "x", 2)] {
                assert_fact(&graph, read, guard, outcome);
            }
            assert_eq!(paths(&graph, guard), vec![vec![]]);
            let joined = paths(&graph, range(&source, "after(x)", 0));
            assert_eq!(joined.len(), 2);
            assert!(joined.iter().any(|path| !path.contains(&(guard, outcome))));
            if operator == "??" {
                assert!(
                    joined
                        .iter()
                        .any(|path| path.contains(&(guard, FlowOutcome::NonNullish)))
                );
            }
        }
        let source = "function f(x, y) { x && (y || read(x, y)); }";
        let graph = flows(source);
        let read = range(source, "read(x, y)", 0);
        assert_fact(&graph, read, range(source, "x", 1), FlowOutcome::Truthy);
        assert_fact(&graph, read, range(source, "y", 1), FlowOutcome::Falsy);
        assert_fact(
            &graph,
            range(source, "x", 2),
            range(source, "y", 1),
            FlowOutcome::Falsy,
        );
    }

    #[test]
    fn conditional_expressions_branch_and_rejoin_before_sibling_reads() {
        let source = "function f(x, y) { use(x ? left(x) : (y && no(x)), after(x)); }";
        let graph = flows(source);
        let guard = range(source, "x", 1);
        assert_fact(
            &graph,
            range(source, "left(x)", 0),
            guard,
            FlowOutcome::Truthy,
        );
        assert_fact(&graph, range(source, "no(x)", 0), guard, FlowOutcome::Falsy);
        assert_fact(
            &graph,
            range(source, "x", 3),
            range(source, "y", 1),
            FlowOutcome::Truthy,
        );
        assert_eq!(paths(&graph, range(source, "after(x)", 0)).len(), 3);
        assert_eq!(
            paths(&graph, range(source, "x ? left(x) : (y && no(x))", 0)),
            vec![vec![]]
        );
    }

    #[test]
    fn loops_keep_backedges_but_not_jumps_after_a_terminator() {
        for source in [
            "function f(x) { while (x) { read(x); } after(x); }",
            "function f(x) { do { read(x); } while (x); after(x); }",
            "function f(x) { for (; x;) { read(x); } after(x); }",
        ] {
            let graph = flows(source);
            assert!(graph.nodes.iter().enumerate().any(|(id, node)| {
                matches!(node, FlowNode::Join(predecessors) if predecessors.iter().any(|p| *p > id))
            }));
            assert!(
                graph
                    .expression_flows
                    .contains_key(&range(source, "read(x)", 0))
            );
            assert!(
                graph
                    .expression_flows
                    .contains_key(&range(source, "after(x)", 0))
            );
        }
        for source in [
            "function f(x) { for (;;) { return; } dead(x); }",
            "function f(x) { while (true) { continue; dead(x); } after(x); }",
            "function f(x) { do { continue; dead(x); } while (true); after(x); }",
        ] {
            let graph = flows(source);
            assert!(
                !graph
                    .expression_flows
                    .contains_key(&range(source, "dead(x)", 0))
            );
            if source.contains("after(x)") {
                assert!(
                    !graph
                        .expression_flows
                        .contains_key(&range(source, "after(x)", 0))
                );
            }
        }
    }

    #[test]
    fn nested_execution_roots_are_isolated() {
        let source = "function f(x) { if (x) { const g = () => { try { inner(x); } catch {} }; outer(x); } }";
        let graph = flows(source);
        assert!(
            !graph
                .expression_flows
                .contains_key(&range(source, "inner(x)", 0))
        );
        assert_fact(
            &graph,
            range(source, "outer(x)", 0),
            range(source, "x", 1),
            FlowOutcome::Truthy,
        );
        assert!(
            narrowing_flow_graph(&cfg(source, JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION))
                .is_none()
        );

        let source = "function f(x) { if (x) { const g = () => inner(x); outer(x); } }";
        let outer = flows(source);
        let inner =
            narrowing_flow_graph(&cfg(source, JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION)).unwrap();
        assert!(
            !outer
                .expression_flows
                .contains_key(&range(source, "inner(x)", 0))
        );
        assert_eq!(paths(&inner, range(source, "inner(x)", 0)), vec![vec![]]);
        assert!(call_continuations(&inner).is_empty());
        assert_eq!(
            call_continuations(&outer),
            vec![range(source, "outer(x)", 0)]
        );
        assert!(
            !inner
                .expression_flows
                .contains_key(&range(source, "outer(x)", 0))
        );

        let source = "function f(x) { if (x) { outer(x); const g = () => { inner(x); after; }; } }";
        let outer = flows(source);
        let inner =
            narrowing_flow_graph(&cfg(source, JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION)).unwrap();
        assert_eq!(
            call_continuations(&outer),
            vec![range(source, "outer(x)", 0)]
        );
        assert_eq!(
            call_continuations(&inner),
            vec![range(source, "inner(x)", 0)]
        );
        assert_eq!(paths(&inner, range(source, "inner(x)", 0)), vec![vec![]]);
        assert_eq!(paths(&inner, range(source, "after", 0)), vec![vec![]]);
    }

    #[test]
    fn expression_bodied_arrow_and_shorthand_reads() {
        let source = "const f = x => x && ({ value: read(x), x });";
        let graph =
            narrowing_flow_graph(&cfg(source, JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION)).unwrap();
        let guard = range(source, "x", 1);
        assert_eq!(paths(&graph, guard), vec![vec![]]);
        assert_fact(
            &graph,
            range(source, "read(x)", 0),
            guard,
            FlowOutcome::Truthy,
        );
        assert_fact(&graph, range(source, "x", 3), guard, FlowOutcome::Truthy);
        let module = narrowing_flow_graph(&cfg(source, JsSyntaxKind::JS_MODULE)).unwrap();
        assert!(!module.expression_flows.contains_key(&guard));
    }

    #[test]
    fn optional_chains_preserve_outer_facts_without_assuming_a_receiver() {
        for call in [
            "x?.(nested(x))",
            "x?.read(nested(x))",
            "x.read?.(nested(x))",
            "x?.[nested(x)](last(x))",
            "x?.read.call(nested(x))",
            "x?.read()(nested(x))",
            "((x?.read(nested(x))))",
            "(x?.read)(nested(x))",
            "x?.read!(nested(x))",
            "(x?.read)!(nested(x))",
        ] {
            let source = format!("function f(x, y) {{ if (y) {call}; after(x); }}");
            let graph = flows(&source);
            let expression = range(&source, call, 0);
            let guard = range(&source, "y", 1);
            for occurrence in graph.expression_flows.keys() {
                if expression.contains_range(*occurrence) {
                    assert_fact(&graph, *occurrence, guard, FlowOutcome::Truthy);
                    assert_eq!(paths(&graph, *occurrence)[0].len(), 1);
                }
            }
            assert!(
                graph
                    .expression_flows
                    .contains_key(&range(&source, "nested(x)", 0))
            );
            assert_eq!(
                call_continuations(&graph),
                vec![range(&source, "after(x)", 0)]
            );
            assert_eq!(paths(&graph, range(&source, "after(x)", 0)).len(), 2);
        }
    }

    #[test]
    fn rejects_unsupported_evaluation_and_control_flow() {
        for body in [
            "try { read(x); } catch {}",
            "try { read(x); } finally {}",
            "switch (x) { case 1: read(x); }",
            "for (const key in x) read(key);",
            "for (const item of x) read(item);",
            "x &&= read(x);",
            "x ||= read(x);",
            "x ??= read(x);",
            "const { value = read(x) } = x;",
            "class C { field = read(x); }",
        ] {
            let source = format!("function f(x) {{ {body} }}");
            assert!(
                narrowing_flow_graph(&cfg(&source, JsSyntaxKind::JS_FUNCTION_DECLARATION))
                    .is_none(),
                "{body}"
            );
        }
    }

    #[test]
    fn rejects_malformed_operands_foreign_snapshots_and_unknown_conditions() {
        for source in ["x &&", "x ? y :"] {
            let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
            assert!(parsed.has_errors());
            let expression = parsed
                .syntax()
                .descendants()
                .find_map(AnyJsExpression::cast)
                .unwrap();
            let mut builder = FunctionBuilder::new(parsed.syntax());
            builder
                .append_statement()
                .with_node(expression.into_syntax());
            assert!(narrowing_flow_graph(&builder.finish()).is_none());
        }
        let source = "function f(x) { if (x) read(x); }";
        let mut graph = cfg(source, JsSyntaxKind::JS_FUNCTION_DECLARATION);
        let foreign = cfg(source, JsSyntaxKind::JS_FUNCTION_DECLARATION);
        graph.blocks[0].instructions[0].node = foreign.blocks[0].instructions[0].node.clone();
        assert!(narrowing_flow_graph(&graph).is_none());

        let source = "function f(x) { read(x); }";
        let graph = cfg(source, JsSyntaxKind::JS_FUNCTION_DECLARATION);
        let expression = graph
            .node
            .descendants()
            .find_map(AnyJsExpression::cast)
            .unwrap();
        let mut builder = FunctionBuilder::new(graph.node);
        let target = builder.append_block();
        builder
            .append_jump(true, target)
            .with_node(expression.into_syntax());
        builder.append_return();
        builder.set_cursor(target);
        assert!(narrowing_flow_graph(&builder.finish()).is_none());
    }

    #[test]
    fn construction_limits_fail_closed() {
        let source = format!(
            "function f(x) {{ {}x{}; }}",
            "(".repeat(MAX_DEPTH),
            ")".repeat(MAX_DEPTH)
        );
        assert!(
            narrowing_flow_graph(&cfg(&source, JsSyntaxKind::JS_FUNCTION_DECLARATION)).is_none()
        );
        let mut graph = cfg(
            "function f(x) { read(x); }",
            JsSyntaxKind::JS_FUNCTION_DECLARATION,
        );
        graph.blocks[0].instructions =
            vec![graph.blocks[0].instructions[0].clone(); MAX_INSTRUCTIONS + 1];
        assert!(narrowing_flow_graph(&graph).is_none());
        graph.blocks[0].instructions.clear();
        graph.blocks = vec![graph.blocks[0].clone(); MAX_BLOCKS + 1];
        assert!(narrowing_flow_graph(&graph).is_none());

        let mut builder = FlowBuilder {
            graph: NarrowingFlowGraph {
                nodes: vec![FlowNode::Start; MAX_FLOW_NODES],
                expression_flows: FxHashMap::default(),
            },
            remaining_visits: MAX_SYNTAX_NODES,
        };
        assert!(builder.push(FlowNode::Start).is_none());
    }
}
