use super::guards::{FlowGuard, FlowOutcome, decompose_condition};
use super::{FlowNode, FlowNodeId};
use biome_js_control_flow::{AnyJsControlFlowRoot, is_truthy_literal};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsFunctionBody, AnyJsStatement, AnyTsType,
    JsDoWhileStatement, JsForStatement, JsIdentifierExpression, JsIfStatement, JsSyntaxNode,
    JsWhileStatement, T,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TokenText};

const MAX_SYNTAX_NODES: usize = 16_384;
/// Limits how deeply syntax can nest below the execution root that gets flow.
pub(super) const MAX_DEPTH: usize = 128;
const MAX_FLOW_NODES: usize = 65_536;

/// The flow of one execution root before relevance filtering.
pub(super) struct BuiltRoot {
    pub(super) nodes: Vec<FlowNode>,
    pub(super) guards: Vec<FlowGuard>,
    /// Condition nodes paired with each binding their guard may refine.
    pub(super) mentions: Vec<(FlowNodeId, TextRange)>,
    /// Reachable reads of local bindings with their incoming flow.
    pub(super) reads: Vec<BuiltRead>,
}

pub(super) struct BuiltRead {
    pub(super) identifier: JsIdentifierExpression,
    pub(super) point: FlowNodeId,
    pub(super) binding: TextRange,
}

/// Marks a root whose syntax or control flow narrowing does not support.
struct Unsupported;

type Build<T> = Result<T, Unsupported>;

/// Builds the flow of `root`, excluding nested execution roots.
///
/// `root` must be selected by a [`FlowRootScanner`], which only selects roots
/// whose syntax the builder supports. Expression-bodied arrows are supported.
/// Parameter initializers and module export declarations are not evaluated,
/// and unreachable code records no reads.
///
/// Returns `None` for inputs exceeding construction limits. Like the statement
/// control-flow graph, it also returns `None` when any statement, including
/// unreachable code, lacks syntax that graph requires, such as a jump target or
/// an `if` test. An unlabeled `break` or `continue` only targets an unlabeled
/// loop.
///
/// [`FlowRootScanner`]: super::FlowRootScanner
pub(super) fn build_root(root: &AnyJsControlFlowRoot, model: &SemanticModel) -> Option<BuiltRoot> {
    if !has_complete_statements(root) {
        return None;
    }
    let mut builder = RootBuilder {
        model,
        nodes: vec![FlowNode::Start],
        guards: Vec::new(),
        mentions: Vec::new(),
        reads: Vec::new(),
        jumps: Vec::new(),
        current: Some(0),
        remaining_visits: MAX_SYNTAX_NODES,
    };
    builder.root(root).ok()?;
    Some(BuiltRoot {
        nodes: builder.nodes,
        guards: builder.guards,
        mentions: builder.mentions,
        reads: builder.reads,
    })
}

/// Checks every statement of `root`, reachable or not, for the syntax that the
/// statement control-flow graph requires: tests of `if`, `while` and `do`
/// statements, labels of labeled blocks, variable initializers, and a target
/// for each `break` and `continue`.
fn has_complete_statements(root: &AnyJsControlFlowRoot) -> bool {
    StatementCheck {
        targets: Vec::new(),
    }
    .statements(root.syntax())
}

/// A statement that `break` or `continue` can target during
/// [`has_complete_statements`].
struct CheckedTarget {
    label: Option<TokenText>,
    is_loop: bool,
}

struct StatementCheck {
    targets: Vec<CheckedTarget>,
}

impl StatementCheck {
    fn statements(&mut self, node: &JsSyntaxNode) -> bool {
        node.children().all(|child| {
            let kind = child.kind();
            if AnyJsControlFlowRoot::can_cast(kind)
                || AnyJsExpression::can_cast(kind)
                || AnyTsType::can_cast(kind)
            {
                return true;
            }
            match AnyJsStatement::cast_ref(&child) {
                Some(statement) => self.statement(statement, None),
                None => self.statements(&child),
            }
        })
    }

    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "Other statements require no syntax beyond what parsing guarantees."
    )]
    fn statement(&mut self, statement: AnyJsStatement, label: Option<TokenText>) -> bool {
        match statement {
            AnyJsStatement::JsBlockStatement(block) => match label {
                Some(label) => {
                    self.scoped(Some(label), false, |check| check.statements(block.syntax()))
                }
                None => self.statements(block.syntax()),
            },
            AnyJsStatement::JsLabeledStatement(labeled) => {
                let label = labeled
                    .label_token()
                    .ok()
                    .map(|token| token.token_text_trimmed());
                match labeled.body() {
                    // A labeled block needs its label as a `break` target.
                    Ok(AnyJsStatement::JsBlockStatement(_)) if label.is_none() => false,
                    Ok(body) => self.statement(body, label),
                    Err(_) => true,
                }
            }
            AnyJsStatement::JsIfStatement(statement) => {
                statement.test().is_ok()
                    && statement
                        .consequent()
                        .ok()
                        .is_none_or(|consequent| self.statement(consequent, None))
                    && statement
                        .else_clause()
                        .and_then(|else_clause| else_clause.alternate().ok())
                        .is_none_or(|alternate| self.statement(alternate, None))
            }
            AnyJsStatement::JsWhileStatement(statement) => {
                statement.test().is_ok() && self.loop_body(statement.body().ok(), label)
            }
            AnyJsStatement::JsDoWhileStatement(statement) => {
                statement.test().is_ok() && self.loop_body(statement.body().ok(), label)
            }
            AnyJsStatement::JsForStatement(statement) => {
                self.loop_body(statement.body().ok(), label)
            }
            AnyJsStatement::JsBreakStatement(statement) => {
                let label = statement
                    .label_token()
                    .map(|token| token.token_text_trimmed());
                self.targets.iter().any(|target| target.label == label)
            }
            AnyJsStatement::JsContinueStatement(statement) => {
                let label = statement
                    .label_token()
                    .map(|token| token.token_text_trimmed());
                self.targets
                    .iter()
                    .any(|target| target.is_loop && target.label == label)
            }
            AnyJsStatement::JsVariableStatement(statement) => {
                statement.declaration().is_ok_and(|declaration| {
                    declaration.declarators().iter().all(|declarator| {
                        declarator.is_ok_and(|declarator| {
                            declarator
                                .initializer()
                                .is_none_or(|initializer| initializer.expression().is_ok())
                        })
                    })
                })
            }
            _ => true,
        }
    }

    fn loop_body(&mut self, body: Option<AnyJsStatement>, label: Option<TokenText>) -> bool {
        self.scoped(label, true, |check| {
            body.is_none_or(|body| check.statement(body, None))
        })
    }

    fn scoped(
        &mut self,
        label: Option<TokenText>,
        is_loop: bool,
        check: impl FnOnce(&mut Self) -> bool,
    ) -> bool {
        self.targets.push(CheckedTarget { label, is_loop });
        let complete = check(self);
        self.targets.pop();
        complete
    }
}

/// A statement that `break` or `continue` can target.
struct JumpTarget {
    label: Option<TokenText>,
    /// The loop's continue point; `None` for a labeled block.
    continue_to: Option<FlowNodeId>,
    breaks: Vec<FlowNodeId>,
}

struct RootBuilder<'model> {
    model: &'model SemanticModel,
    nodes: Vec<FlowNode>,
    guards: Vec<FlowGuard>,
    mentions: Vec<(FlowNodeId, TextRange)>,
    reads: Vec<BuiltRead>,
    jumps: Vec<JumpTarget>,
    /// The incoming flow of the next statement, or `None` when unreachable.
    current: Option<FlowNodeId>,
    remaining_visits: usize,
}

impl RootBuilder<'_> {
    fn root(&mut self, root: &AnyJsControlFlowRoot) -> Build<()> {
        if let AnyJsControlFlowRoot::AnyJsFunction(AnyJsFunction::JsArrowFunctionExpression(arrow)) =
            root
            && let AnyJsFunctionBody::AnyJsExpression(body) =
                arrow.body().map_err(|_| Unsupported)?
        {
            self.current = Some(self.evaluate(body.syntax(), 0, 0)?);
            return Ok(());
        }
        self.statements(root.syntax())
    }

    /// Builds the statements found below `node`, without entering
    /// expressions, types, or nested execution roots.
    fn statements(&mut self, node: &JsSyntaxNode) -> Build<()> {
        for child in node.children() {
            let kind = child.kind();
            if AnyJsControlFlowRoot::can_cast(kind)
                || AnyJsExpression::can_cast(kind)
                || AnyTsType::can_cast(kind)
            {
                continue;
            }
            match AnyJsStatement::cast_ref(&child) {
                Some(statement) => self.statement(statement, None)?,
                None => self.statements(&child)?,
            }
        }
        Ok(())
    }

    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "Syntax validation rejects every other statement."
    )]
    fn statement(&mut self, statement: AnyJsStatement, label: Option<TokenText>) -> Build<()> {
        let Some(current) = self.current else {
            return Ok(());
        };
        match statement {
            AnyJsStatement::JsBlockStatement(block) => match label {
                Some(label) => {
                    self.jumps.push(JumpTarget {
                        label: Some(label),
                        continue_to: None,
                        breaks: Vec::new(),
                    });
                    self.statements(block.syntax())?;
                    let target = self.jumps.pop().ok_or(Unsupported)?;
                    self.current = self.join(self.current.into_iter().chain(target.breaks))?;
                    Ok(())
                }
                None => self.statements(block.syntax()),
            },
            AnyJsStatement::JsLabeledStatement(labeled) => {
                let label = labeled
                    .label_token()
                    .ok()
                    .map(|token| token.token_text_trimmed());
                match labeled.body() {
                    Ok(body) => self.statement(body, label),
                    Err(_) => Ok(()),
                }
            }
            AnyJsStatement::JsExpressionStatement(_)
            | AnyJsStatement::JsDebuggerStatement(_)
            | AnyJsStatement::JsEmptyStatement(_) => {
                self.current = Some(self.evaluate(statement.syntax(), current, 0)?);
                Ok(())
            }
            AnyJsStatement::JsVariableStatement(variable) => {
                let declaration = variable.declaration().map_err(|_| Unsupported)?;
                let mut flow = current;
                for declarator in declaration.declarators() {
                    let declarator = declarator.map_err(|_| Unsupported)?;
                    if let Some(initializer) = declarator.initializer() {
                        let expression = initializer.expression().map_err(|_| Unsupported)?;
                        flow = self.evaluate(expression.syntax(), flow, 0)?;
                    }
                }
                self.current = Some(flow);
                Ok(())
            }
            AnyJsStatement::JsReturnStatement(_) | AnyJsStatement::JsThrowStatement(_) => {
                self.evaluate(statement.syntax(), current, 0)?;
                self.current = None;
                Ok(())
            }
            AnyJsStatement::JsIfStatement(statement) => self.if_statement(&statement, current),
            AnyJsStatement::JsWhileStatement(statement) => {
                self.while_statement(&statement, current, label)
            }
            AnyJsStatement::JsDoWhileStatement(statement) => {
                self.do_while_statement(&statement, current, label)
            }
            AnyJsStatement::JsForStatement(statement) => {
                self.for_statement(&statement, current, label)
            }
            AnyJsStatement::JsBreakStatement(statement) => {
                let label = statement
                    .label_token()
                    .map(|token| token.token_text_trimmed());
                let target = self
                    .jumps
                    .iter_mut()
                    .rev()
                    .find(|target| target.label == label)
                    .ok_or(Unsupported)?;
                target.breaks.push(current);
                self.current = None;
                Ok(())
            }
            AnyJsStatement::JsContinueStatement(statement) => {
                let label = statement
                    .label_token()
                    .map(|token| token.token_text_trimmed());
                let continue_to = self
                    .jumps
                    .iter()
                    .rev()
                    .filter(|target| target.label == label)
                    .find_map(|target| target.continue_to)
                    .ok_or(Unsupported)?;
                self.add_predecessor(continue_to, current)?;
                self.current = None;
                Ok(())
            }
            AnyJsStatement::JsFunctionDeclaration(_)
            | AnyJsStatement::TsTypeAliasDeclaration(_)
            | AnyJsStatement::TsInterfaceDeclaration(_)
            | AnyJsStatement::TsDeclareFunctionDeclaration(_) => Ok(()),
            _ => Err(Unsupported),
        }
    }

    fn if_statement(&mut self, statement: &JsIfStatement, current: FlowNodeId) -> Build<()> {
        let test = statement.test().map_err(|_| Unsupported)?;
        let flow = self.evaluate(test.syntax(), current, 0)?;
        let positive = self.condition(flow, &test, FlowOutcome::Truthy)?;
        let negative = self.condition(flow, &test, FlowOutcome::Falsy)?;
        self.current = Some(positive);
        if let Ok(consequent) = statement.consequent() {
            self.statement(consequent, None)?;
        }
        let consequent_end = self.current;
        self.current = Some(negative);
        if let Some(alternate) = statement
            .else_clause()
            .and_then(|else_clause| else_clause.alternate().ok())
        {
            self.statement(alternate, None)?;
        }
        self.current = self.join(consequent_end.into_iter().chain(self.current))?;
        Ok(())
    }

    fn while_statement(
        &mut self,
        statement: &JsWhileStatement,
        current: FlowNodeId,
        label: Option<TokenText>,
    ) -> Build<()> {
        let test = statement.test().map_err(|_| Unsupported)?;
        let header = self.push(FlowNode::Join(vec![current]))?;
        let flow = self.evaluate(test.syntax(), header, 0)?;
        let exit = if is_truthy_literal(&test) {
            self.current = Some(flow);
            None
        } else {
            self.current = Some(self.condition(flow, &test, FlowOutcome::Truthy)?);
            Some(self.condition(flow, &test, FlowOutcome::Falsy)?)
        };
        self.loop_body(statement.body().ok(), label, header)?;
        if let Some(end) = self.current {
            self.add_predecessor(header, end)?;
        }
        let target = self.jumps.pop().ok_or(Unsupported)?;
        self.current = self.join(exit.into_iter().chain(target.breaks))?;
        Ok(())
    }

    fn do_while_statement(
        &mut self,
        statement: &JsDoWhileStatement,
        current: FlowNodeId,
        label: Option<TokenText>,
    ) -> Build<()> {
        let body_entry = self.push(FlowNode::Join(vec![current]))?;
        let test_entry = self.push(FlowNode::Join(Vec::new()))?;
        self.current = Some(body_entry);
        self.loop_body(statement.body().ok(), label, test_entry)?;
        if let Some(end) = self.current {
            self.add_predecessor(test_entry, end)?;
        }
        let target = self.jumps.pop().ok_or(Unsupported)?;
        let mut exit = None;
        if self.has_predecessors(test_entry) {
            let test = statement.test().map_err(|_| Unsupported)?;
            let flow = self.evaluate(test.syntax(), test_entry, 0)?;
            if is_truthy_literal(&test) {
                self.add_predecessor(body_entry, flow)?;
            } else {
                let repeat = self.condition(flow, &test, FlowOutcome::Truthy)?;
                self.add_predecessor(body_entry, repeat)?;
                exit = Some(self.condition(flow, &test, FlowOutcome::Falsy)?);
            }
        }
        self.current = self.join(exit.into_iter().chain(target.breaks))?;
        Ok(())
    }

    fn for_statement(
        &mut self,
        statement: &JsForStatement,
        current: FlowNodeId,
        label: Option<TokenText>,
    ) -> Build<()> {
        let mut flow = current;
        if let Some(initializer) = statement.initializer() {
            flow = self.evaluate(initializer.syntax(), flow, 0)?;
        }
        let header = self.push(FlowNode::Join(vec![flow]))?;
        let update_entry = self.push(FlowNode::Join(Vec::new()))?;
        let exit = match statement.test() {
            Some(test) => {
                let flow = self.evaluate(test.syntax(), header, 0)?;
                self.current = Some(self.condition(flow, &test, FlowOutcome::Truthy)?);
                Some(self.condition(flow, &test, FlowOutcome::Falsy)?)
            }
            None => {
                self.current = Some(header);
                None
            }
        };
        self.loop_body(statement.body().ok(), label, update_entry)?;
        if let Some(end) = self.current {
            self.add_predecessor(update_entry, end)?;
        }
        let target = self.jumps.pop().ok_or(Unsupported)?;
        if self.has_predecessors(update_entry) {
            let flow = match statement.update() {
                Some(update) => self.evaluate(update.syntax(), update_entry, 0)?,
                None => update_entry,
            };
            self.add_predecessor(header, flow)?;
        }
        self.current = self.join(exit.into_iter().chain(target.breaks))?;
        Ok(())
    }

    /// Builds a loop body with a jump target that the caller pops after
    /// connecting the body's end.
    fn loop_body(
        &mut self,
        body: Option<AnyJsStatement>,
        label: Option<TokenText>,
        continue_to: FlowNodeId,
    ) -> Build<()> {
        self.jumps.push(JumpTarget {
            label,
            continue_to: Some(continue_to),
            breaks: Vec::new(),
        });
        match body {
            Some(body) => self.statement(body, None),
            None => Ok(()),
        }
    }

    /// Records reads and branches while evaluating `node` from `incoming`, and
    /// returns the flow after it.
    fn evaluate(
        &mut self,
        node: &JsSyntaxNode,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        self.remaining_visits = self.remaining_visits.checked_sub(1).ok_or(Unsupported)?;
        if depth > MAX_DEPTH {
            return Err(Unsupported);
        }
        let kind = node.kind();
        if AnyJsControlFlowRoot::can_cast(kind) || AnyTsType::can_cast(kind) {
            return Ok(incoming);
        }
        if let Some(identifier) = JsIdentifierExpression::cast_ref(node) {
            if let Some(binding) = identifier
                .name()
                .ok()
                .and_then(|name| self.model.binding(&name))
            {
                self.reads.push(BuiltRead {
                    identifier,
                    point: incoming,
                    binding: binding.range(),
                });
            }
            return Ok(incoming);
        }
        match AnyJsExpression::cast_ref(node) {
            Some(AnyJsExpression::JsLogicalExpression(logical)) => {
                let left = logical.left().map_err(|_| Unsupported)?;
                let right = logical.right().map_err(|_| Unsupported)?;
                let (evaluate_right, skip_right) =
                    match logical.operator_token().map_err(|_| Unsupported)?.kind() {
                        T![&&] => (FlowOutcome::Truthy, FlowOutcome::Falsy),
                        T![||] => (FlowOutcome::Falsy, FlowOutcome::Truthy),
                        T![??] => (FlowOutcome::Nullish, FlowOutcome::NonNullish),
                        _ => return Err(Unsupported),
                    };
                let left_flow = self.evaluate(left.syntax(), incoming, depth + 1)?;
                let right_flow = self.condition(left_flow, &left, evaluate_right)?;
                let right_flow = self.evaluate(right.syntax(), right_flow, depth + 1)?;
                let skip_flow = self.condition(left_flow, &left, skip_right)?;
                self.push(FlowNode::Join(vec![skip_flow, right_flow]))
            }
            Some(AnyJsExpression::JsConditionalExpression(conditional)) => {
                let test = conditional.test().map_err(|_| Unsupported)?;
                let consequent = conditional.consequent().map_err(|_| Unsupported)?;
                let alternate = conditional.alternate().map_err(|_| Unsupported)?;
                let test_flow = self.evaluate(test.syntax(), incoming, depth + 1)?;
                let positive = self.condition(test_flow, &test, FlowOutcome::Truthy)?;
                let positive = self.evaluate(consequent.syntax(), positive, depth + 1)?;
                let negative = self.condition(test_flow, &test, FlowOutcome::Falsy)?;
                let negative = self.evaluate(alternate.syntax(), negative, depth + 1)?;
                self.push(FlowNode::Join(vec![positive, negative]))
            }
            _ => {
                let mut flow = incoming;
                for child in node.children() {
                    flow = self.evaluate(&child, flow, depth + 1)?;
                }
                Ok(flow)
            }
        }
    }

    fn condition(
        &mut self,
        antecedent: FlowNodeId,
        expression: &AnyJsExpression,
        outcome: FlowOutcome,
    ) -> Build<FlowNodeId> {
        let condition = decompose_condition(expression, outcome, self.model, &mut self.guards);
        let node = self.push(FlowNode::Condition {
            antecedent,
            guard: condition.guard,
        })?;
        self.mentions.extend(
            condition
                .mentions
                .into_iter()
                .map(|binding| (node, binding)),
        );
        Ok(node)
    }

    /// Returns the flow where all `predecessors` meet, or `None` when there is
    /// no reachable predecessor.
    fn join(
        &mut self,
        predecessors: impl IntoIterator<Item = FlowNodeId>,
    ) -> Build<Option<FlowNodeId>> {
        let predecessors: Vec<_> = predecessors.into_iter().collect();
        match predecessors.as_slice() {
            [] => Ok(None),
            [single] => Ok(Some(*single)),
            _ => self.push(FlowNode::Join(predecessors)).map(Some),
        }
    }

    fn push(&mut self, node: FlowNode) -> Build<FlowNodeId> {
        if self.nodes.len() >= MAX_FLOW_NODES {
            return Err(Unsupported);
        }
        self.nodes.push(node);
        Ok(self.nodes.len() - 1)
    }

    fn add_predecessor(&mut self, join: FlowNodeId, predecessor: FlowNodeId) -> Build<()> {
        let Some(FlowNode::Join(predecessors)) = self.nodes.get_mut(join) else {
            return Err(Unsupported);
        };
        predecessors.push(predecessor);
        Ok(())
    }

    fn has_predecessors(&self, join: FlowNodeId) -> bool {
        matches!(self.nodes.get(join), Some(FlowNode::Join(predecessors)) if !predecessors.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_module_info::flow::FlowTest;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_semantic::{SemanticModelOptions, semantic_model};
    use biome_js_syntax::JsSyntaxKind;
    use biome_languages::JsFileSource;
    use biome_rowan::TextSize;

    struct Built {
        source: String,
        root: Option<BuiltRoot>,
    }

    /// Builds the first execution root of `kind` in `source`.
    fn build(source: &str, kind: JsSyntaxKind) -> Built {
        let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
        assert!(!parsed.has_errors(), "{source}");
        build_parsed(source, kind)
    }

    /// Builds like [`build`], also accepting sources with syntax errors.
    fn build_parsed(source: &str, kind: JsSyntaxKind) -> Built {
        let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let root = parsed
            .syntax()
            .descendants()
            .filter(|node| node.kind() == kind)
            .find_map(AnyJsControlFlowRoot::cast)
            .unwrap();
        Built {
            source: source.to_owned(),
            root: build_root(&root, &model),
        }
    }

    fn function(source: &str) -> Built {
        build(source, JsSyntaxKind::JS_FUNCTION_DECLARATION)
    }

    impl Built {
        fn root(&self) -> &BuiltRoot {
            self.root.as_ref().expect("root must have flow")
        }

        fn range(&self, text: &str, occurrence: usize) -> TextRange {
            let start = self.source.match_indices(text).nth(occurrence).unwrap().0;
            TextRange::at(
                TextSize::try_from(start).unwrap(),
                TextSize::try_from(text.len()).unwrap(),
            )
        }

        /// Returns the incoming flow of the read `name` at `occurrence`, or
        /// `None` when no reachable read was recorded there.
        fn point(&self, name: &str, occurrence: usize) -> Option<FlowNodeId> {
            let range = self.range(name, occurrence);
            self.root()
                .reads
                .iter()
                .find(|read| read.identifier.range() == range)
                .map(|read| read.point)
        }

        /// Lists, for each acyclic path from the start to the read, the tests
        /// that path assumed, written as `name:Test` or `!name:Test`.
        fn paths(&self, name: &str, occurrence: usize) -> Vec<Vec<String>> {
            let root = self.root();
            let mut result = Vec::new();
            let start = self.point(name, occurrence).expect("read must be recorded");
            let mut pending = vec![(start, Vec::new())];
            let mut budget = 1_000usize;
            while let Some((node, mut facts)) = pending.pop() {
                budget = budget
                    .checked_sub(1)
                    .expect("cyclic or excessive test graph");
                match &root.nodes[node] {
                    FlowNode::Start => result.push(facts),
                    FlowNode::Join(predecessors) => {
                        for predecessor in predecessors {
                            pending.push((*predecessor, facts.clone()));
                        }
                    }
                    FlowNode::Condition { antecedent, guard } => {
                        self.facts(*guard, &mut facts);
                        pending.push((*antecedent, facts));
                    }
                }
            }
            result
        }

        fn facts(&self, guard: usize, facts: &mut Vec<String>) {
            match &self.root().guards[guard] {
                FlowGuard::Keep | FlowGuard::Incomplete => {}
                FlowGuard::Test {
                    binding,
                    test,
                    positive,
                } => {
                    let name =
                        &self.source[usize::from(binding.start())..usize::from(binding.end())];
                    let test = match test {
                        FlowTest::Truthy => "Truthy",
                        FlowTest::Nullish => "Nullish",
                        _ => "Other",
                    };
                    let negation = if *positive { "" } else { "!" };
                    facts.push(format!("{negation}{name}:{test}"));
                }
                FlowGuard::Both { left, right, .. } => {
                    self.facts(*left, facts);
                    self.facts(*right, facts);
                }
            }
        }
    }

    fn contains(paths: &[Vec<String>], fact: &str) -> bool {
        paths
            .iter()
            .all(|path| path.iter().any(|candidate| candidate == fact))
    }

    #[test]
    fn if_else_branches_rejoin() {
        let built = function("function f(x) { if (x) yes(x); else no(x); after(x); }");
        assert_eq!(built.paths("x", 1), [Vec::<String>::new()]);
        assert!(contains(&built.paths("x", 2), "x:Truthy"));
        assert!(contains(&built.paths("x", 3), "!x:Truthy"));
        let joined = built.paths("x", 4);
        assert_eq!(joined.len(), 2);
        assert!(
            joined
                .iter()
                .any(|path| path.contains(&"x:Truthy".to_owned()))
        );
        assert!(
            joined
                .iter()
                .any(|path| path.contains(&"!x:Truthy".to_owned()))
        );
    }

    #[test]
    fn early_returns_leave_later_reads_on_the_other_branch() {
        let built = function("function f(x) { if (x) { return x; dead(x); } after(x); }");
        assert!(contains(&built.paths("x", 2), "x:Truthy"));
        assert_eq!(built.point("x", 3), None);
        assert_eq!(built.paths("x", 4), [vec!["!x:Truthy".to_owned()]]);
    }

    #[test]
    fn guard_operands_use_incoming_facts() {
        let built = function("function f(x) { if (x) { if (x === null) read(x); } }");
        let inner = built.paths("x", 2);
        assert!(contains(&inner, "x:Truthy"));
        assert!(inner.iter().all(|path| path.len() == 1));
        assert_eq!(built.paths("x", 3).first().map(Vec::len), Some(2));
    }

    #[test]
    fn logical_operators_branch_on_their_left_operand() {
        for (operator, fact) in [("&&", "x:Truthy"), ("||", "!x:Truthy"), ("??", "x:Nullish")] {
            let built = function(&format!(
                "function f(x) {{ x {operator} read(x); after(x); }}"
            ));
            assert_eq!(built.paths("x", 1), [Vec::<String>::new()], "{operator}");
            assert!(contains(&built.paths("x", 2), fact), "{operator}");
            let joined = built.paths("x", 3);
            assert_eq!(joined.len(), 2, "{operator}");
            assert!(joined.iter().any(|path| !path.contains(&fact.to_owned())));
        }
    }

    #[test]
    fn conditional_expressions_rejoin_before_sibling_reads() {
        let built = function("function f(x, y) { use(x ? left(x) : (y && no(x)), after(x)); }");
        assert!(contains(&built.paths("x", 2), "x:Truthy"));
        let no = built.paths("x", 3);
        assert!(contains(&no, "!x:Truthy") && contains(&no, "y:Truthy"));
        assert_eq!(built.paths("x", 4).len(), 3);
    }

    #[test]
    fn loops_keep_backedges() {
        for source in [
            "function f(x) { while (x) { read(x); } after(x); }",
            "function f(x) { do { read(x); } while (x); after(x); }",
            "function f(x) { for (; x;) { read(x); } after(x); }",
        ] {
            let built = function(source);
            assert!(
                built.root().nodes.iter().enumerate().any(|(id, node)| {
                    matches!(node, FlowNode::Join(predecessors) if predecessors.iter().any(|p| *p > id))
                }),
                "{source}"
            );
            assert!(built.point("x", 2).is_some(), "{source}");
            assert!(built.point("x", 3).is_some(), "{source}");
        }
    }

    #[test]
    fn jumps_end_reachability() {
        for (source, unreachable) in [
            ("function f(x) { for (;;) { return; } dead(x); }", vec![1]),
            (
                "function f(x) { while (true) { continue; dead(x); } after(x); }",
                vec![1, 2],
            ),
            (
                "function f(x) { do { continue; dead(x); } while (true); after(x); }",
                vec![1, 2],
            ),
        ] {
            let built = function(source);
            for occurrence in unreachable {
                assert_eq!(built.point("x", occurrence), None, "{source}");
            }
        }
    }

    #[test]
    fn labeled_jumps_reach_their_targets() {
        let built = function("function f(x) { done: { if (x) break done; read(x); } after(x); }");
        assert_eq!(built.paths("x", 2), [vec!["!x:Truthy".to_owned()]]);
        assert_eq!(built.paths("x", 3).len(), 2);

        let built =
            function("function f(x) { outer: while (x) { while (x) { break outer; } } after(x); }");
        assert!(built.point("x", 3).is_some());

        let built = function(
            "function f(x) { outer: while (x) { while (x) { continue outer; } } after(x); }",
        );
        assert!(built.point("x", 3).is_some());

        // As in the statement control-flow graph, an unlabeled jump never
        // targets a labeled loop, and jumps in unreachable code still need a
        // target.
        for source in [
            "function f(x) { outer: while (x) { break; } }",
            "function f(x) { outer: while (x) { continue; } }",
            "function f(x, flag) { if (x) read(x); outer: while (flag) { return; break; } }",
        ] {
            assert!(function(source).root.is_none(), "{source}");
        }
    }

    #[test]
    fn unreachable_statements_need_the_syntax_the_statement_graph_requires() {
        let source = "function f(x) { if (x) read(x); return; if () dead(x); }";
        assert!(
            build_parsed(source, JsSyntaxKind::JS_FUNCTION_DECLARATION)
                .root
                .is_none()
        );
    }

    #[test]
    fn for_loops_keep_their_exit_for_truthy_literal_tests() {
        let built = function("function f(x) { for (; true;) { read(x); } after(x); }");
        assert!(built.point("x", 2).is_some());
        let built = function("function f(x) { while (true) { read(x); } after(x); }");
        assert_eq!(built.point("x", 2), None);
    }

    #[test]
    fn optional_chains_keep_outer_facts_without_a_receiver_condition() {
        let built = function("function f(x, y) { if (y) x?.read(x); after(x); }");
        assert_eq!(built.paths("x", 2), [vec!["y:Truthy".to_owned()]]);
        assert_eq!(built.paths("x", 3).len(), 2);
    }

    #[test]
    fn nested_roots_are_isolated() {
        let source = "function f(x) { if (x) { const g = () => { try { inner(x); } catch {} }; outer(x); } }";
        let built = function(source);
        assert_eq!(built.point("x", 2), None);
        assert!(contains(&built.paths("x", 3), "x:Truthy"));
        assert!(
            build(source, JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION)
                .root
                .is_none()
        );
    }

    #[test]
    fn expression_bodied_arrows_have_flow() {
        let built = build(
            "const f = x => x && ({ value: read(x) });",
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION,
        );
        assert_eq!(built.paths("x", 1), [Vec::<String>::new()]);
        assert!(contains(&built.paths("x", 2), "x:Truthy"));
    }
}
