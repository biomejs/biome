use super::guards::{
    DecomposedCondition, FlowGuard, FlowOutcome, GuardFunctions, decompose_assertion,
    decompose_condition,
};
use super::{FlowNode, FlowNodeId, FlowSubject, FlowSubjectId, FlowSubjectTable};
use biome_js_control_flow::{AnyJsControlFlowRoot, is_truthy_literal};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsExpression, AnyJsFunction, AnyJsFunctionBody, AnyJsStatement, AnyTsType,
    JsDoWhileStatement, JsForStatement, JsIdentifierExpression, JsIfStatement, JsSyntaxNode,
    JsWhileStatement, T,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, TokenText};

const MAX_SYNTAX_NODES: usize = 16_384;
/// Limits how deeply syntax can nest below the execution root that gets flow.
pub(super) const MAX_DEPTH: usize = 128;
const MAX_FLOW_NODES: usize = 65_536;

/// The flow of one execution root before relevance filtering.
pub(super) struct BuiltRoot {
    pub(super) nodes: Vec<FlowNode>,
    pub(super) guards: Vec<FlowGuard>,
    /// The subjects that guards, mentions, and reads refer to, each at the
    /// index of its id.
    pub(super) subjects: Vec<FlowSubject>,
    /// Condition nodes paired with each subject their guard may refine.
    pub(super) mentions: Vec<(FlowNodeId, FlowSubjectId)>,
    /// Reachable reads of local bindings with their incoming flow.
    pub(super) reads: Vec<BuiltRead>,
}

pub(super) struct BuiltRead {
    pub(super) identifier: JsIdentifierExpression,
    pub(super) point: FlowNodeId,
    pub(super) subject: FlowSubjectId,
}

/// Marks a root whose syntax or control flow narrowing does not support.
struct Unsupported;

type Build<T> = Result<T, Unsupported>;

/// Builds the flow of `root`, excluding nested execution roots.
///
/// `root` must be selected by a [`FlowRootScanner`], which rejects the roots
/// that the builder cannot build, such as a root with a `with` statement.
/// Expression-bodied arrows are supported. Parameter initializers, class
/// members, `declare` statements, and the expressions of module export
/// declarations other than classes and enums are not evaluated, and
/// unreachable code records no reads.
///
/// The builder models the control flow of blocks, `if`, `while`, `do...while`,
/// `for`, labeled statements, `break`, `continue`, `return`, and `throw`. Any
/// other statement, such as `switch`, `try`, `for...in`, or `for...of`, is an
/// opaque region: the builder evaluates its expressions in source order from
/// the flow before it, and the code after it continues from that evaluation.
/// A condition inside a region narrows only the rest of the expression that
/// contains it, as in `x && x.length`. Each `break` or `continue` that leaves
/// the region joins its target with the flow before the region. Because only
/// variables that are never written narrow, the flow before a region is never
/// narrower than the real flow at any point inside or after it.
///
/// Returns `None` for inputs exceeding construction limits. Like the statement
/// control-flow graph, it also returns `None` when any statement, including
/// unreachable code, lacks syntax that graph requires, such as a jump target or
/// an `if` test.
///
/// [`FlowRootScanner`]: super::FlowRootScanner
pub(super) fn build_root(
    root: &AnyJsControlFlowRoot,
    model: &SemanticModel,
    functions: &mut GuardFunctions,
) -> Option<BuiltRoot> {
    if !has_complete_statements(root) {
        return None;
    }
    let mut builder = RootBuilder {
        model,
        functions,
        nodes: vec![FlowNode::Start],
        guards: Vec::new(),
        subjects: FlowSubjectTable::default(),
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
        subjects: builder.subjects.into_subjects(),
        mentions: builder.mentions,
        reads: builder.reads,
    })
}

/// Checks every statement of `root`, reachable or not, for the syntax that the
/// statement control-flow graph requires: tests of `if`, `while` and `do`
/// statements, labels of labeled blocks, variable initializers, the
/// discriminant and case tests of `switch` statements, the heads of `for...in`
/// and `for...of` loops, and a target for each `break` and `continue`.
fn has_complete_statements(root: &AnyJsControlFlowRoot) -> bool {
    let mut check = StatementCheck::default();
    check.statements(root.syntax()) && check.escaping.is_empty()
}

/// A `break` or `continue` statement.
struct Jump {
    label: Option<TokenText>,
    is_continue: bool,
}

impl Jump {
    /// Returns the jump that `statement` performs, or `None` when it is not a
    /// `break` or `continue` statement.
    fn from_statement(statement: &AnyJsStatement) -> Option<Self> {
        let (label, is_continue) = if let AnyJsStatement::JsBreakStatement(statement) = statement {
            (statement.label_token(), false)
        } else if let AnyJsStatement::JsContinueStatement(statement) = statement {
            (statement.label_token(), true)
        } else {
            return None;
        };
        Some(Self {
            label: label.map(|token| token.token_text_trimmed()),
            is_continue,
        })
    }

    /// Returns whether a statement of `kind` labeled `label` can be the target
    /// of this jump. The target is the innermost enclosing statement for which
    /// this holds. An unlabeled `break` targets the innermost loop or `switch`,
    /// whether or not it has a label; only a labeled `break` leaves a block.
    fn can_target(&self, kind: TargetKind, label: Option<&TokenText>) -> bool {
        match (&self.label, self.is_continue) {
            (Some(own), false) => label == Some(own),
            (None, false) => kind != TargetKind::Block,
            (own, true) => {
                kind == TargetKind::Loop && own.as_ref().is_none_or(|own| label == Some(own))
            }
        }
    }
}

/// The kind of statement that `break` or `continue` can target.
#[derive(Clone, Copy, Eq, PartialEq)]
enum TargetKind {
    /// A labeled block, which only a labeled `break` targets.
    Block,
    /// A `switch` statement, which `break` but not `continue` targets.
    Switch,
    Loop,
}

/// A statement that `break` or `continue` can target during a
/// [`StatementCheck`].
struct CheckedTarget {
    label: Option<TokenText>,
    kind: TargetKind,
}

/// Checks the syntax of statements, as [`has_complete_statements`] describes,
/// and collects the jumps that leave them.
#[derive(Default)]
struct StatementCheck {
    targets: Vec<CheckedTarget>,
    /// The jumps that target a statement outside the checked statements.
    escaping: Vec<Jump>,
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
        reason = "Other statements require no syntax beyond what parsing guarantees, so only the statements they contain are checked."
    )]
    fn statement(&mut self, statement: AnyJsStatement, label: Option<TokenText>) -> bool {
        match statement {
            AnyJsStatement::JsBlockStatement(block) => match label {
                Some(label) => self.scoped(Some(label), TargetKind::Block, |check| {
                    check.statements(block.syntax())
                }),
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
            AnyJsStatement::JsForInStatement(statement) => {
                statement.initializer().is_ok()
                    && statement.expression().is_ok()
                    && self.loop_body(statement.body().ok(), label)
            }
            AnyJsStatement::JsForOfStatement(statement) => {
                statement.initializer().is_ok()
                    && statement.expression().is_ok()
                    && self.loop_body(statement.body().ok(), label)
            }
            AnyJsStatement::JsSwitchStatement(statement) => {
                statement.discriminant().is_ok()
                    && statement.cases().iter().all(|clause| {
                        clause
                            .as_js_case_clause()
                            .is_none_or(|clause| clause.test().is_ok())
                    })
                    && self.scoped(label, TargetKind::Switch, |check| {
                        check.statements(statement.syntax())
                    })
            }
            AnyJsStatement::JsBreakStatement(_) | AnyJsStatement::JsContinueStatement(_) => {
                if let Some(jump) = Jump::from_statement(&statement)
                    && !self
                        .targets
                        .iter()
                        .any(|target| jump.can_target(target.kind, target.label.as_ref()))
                {
                    self.escaping.push(jump);
                }
                true
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
            // Nested execution roots are checked when they are built.
            AnyJsStatement::JsFunctionDeclaration(_) | AnyJsStatement::TsModuleDeclaration(_) => {
                true
            }
            _ => self.statements(statement.syntax()),
        }
    }

    fn loop_body(&mut self, body: Option<AnyJsStatement>, label: Option<TokenText>) -> bool {
        self.scoped(label, TargetKind::Loop, |check| {
            body.is_none_or(|body| check.statement(body, None))
        })
    }

    fn scoped(
        &mut self,
        label: Option<TokenText>,
        kind: TargetKind,
        check: impl FnOnce(&mut Self) -> bool,
    ) -> bool {
        self.targets.push(CheckedTarget { label, kind });
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

impl JumpTarget {
    fn kind(&self) -> TargetKind {
        match self.continue_to {
            Some(_) => TargetKind::Loop,
            None => TargetKind::Block,
        }
    }
}

struct RootBuilder<'a, 'raw> {
    model: &'a SemanticModel,
    functions: &'a mut GuardFunctions<'raw>,
    nodes: Vec<FlowNode>,
    guards: Vec<FlowGuard>,
    subjects: FlowSubjectTable,
    mentions: Vec<(FlowNodeId, FlowSubjectId)>,
    reads: Vec<BuiltRead>,
    jumps: Vec<JumpTarget>,
    /// The incoming flow of the next statement, or `None` when unreachable.
    current: Option<FlowNodeId>,
    remaining_visits: usize,
}

impl RootBuilder<'_, '_> {
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
        reason = "Every other statement is an opaque region."
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
            AnyJsStatement::JsExpressionStatement(statement) => {
                let flow = self.evaluate(statement.syntax(), current, 0)?;
                let expression = statement.expression().map_err(|_| Unsupported)?;
                // An assertion call that returns has established its condition.
                self.current = Some(
                    match decompose_assertion(
                        &expression,
                        self.model,
                        self.functions,
                        &mut self.subjects,
                        &mut self.guards,
                    ) {
                        Some(condition) => self.push_condition(flow, condition)?,
                        None => flow,
                    },
                );
                Ok(())
            }
            AnyJsStatement::JsDebuggerStatement(_) | AnyJsStatement::JsEmptyStatement(_) => {
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
            AnyJsStatement::JsBreakStatement(_) | AnyJsStatement::JsContinueStatement(_) => {
                let jump = Jump::from_statement(&statement).ok_or(Unsupported)?;
                let target = self.jump_target(&jump)?;
                self.connect_jump(target, &jump, current)?;
                self.current = None;
                Ok(())
            }
            AnyJsStatement::JsFunctionDeclaration(_)
            | AnyJsStatement::TsTypeAliasDeclaration(_)
            | AnyJsStatement::TsInterfaceDeclaration(_)
            | AnyJsStatement::TsDeclareFunctionDeclaration(_) => Ok(()),
            // Ambient declarations never run, and TypeScript analyzes the body
            // of a module or global declaration apart from the code around it.
            AnyJsStatement::TsDeclareStatement(_)
            | AnyJsStatement::TsExternalModuleDeclaration(_)
            | AnyJsStatement::TsGlobalDeclaration(_) => Ok(()),
            _ => self.opaque_region(&statement, label, current),
        }
    }

    /// Builds `statement` as an opaque region that starts at `current`, as
    /// [`build_root`] describes. `label` is the label of `statement`, if any.
    fn opaque_region(
        &mut self,
        statement: &AnyJsStatement,
        label: Option<TokenText>,
        current: FlowNodeId,
    ) -> Build<()> {
        let mut check = StatementCheck::default();
        if !check.statement(statement.clone(), label) {
            return Err(Unsupported);
        }
        let mut connected = Vec::new();
        for jump in check.escaping {
            let target = self.jump_target(&jump)?;
            if !connected.contains(&(target, jump.is_continue)) {
                connected.push((target, jump.is_continue));
                self.connect_jump(target, &jump, current)?;
            }
        }
        self.current = Some(self.evaluate(statement.syntax(), current, 0)?);
        Ok(())
    }

    /// Returns the index in `jumps` of the target of `jump`.
    fn jump_target(&self, jump: &Jump) -> Build<usize> {
        self.jumps
            .iter()
            .rposition(|target| jump.can_target(target.kind(), target.label.as_ref()))
            .ok_or(Unsupported)
    }

    /// Adds `from` to the flow that `jump` reaches at the target at `index` in
    /// `jumps`: the flow after the target for a `break`, or its continue point
    /// for a `continue`.
    fn connect_jump(&mut self, index: usize, jump: &Jump, from: FlowNodeId) -> Build<()> {
        let target = self.jumps.get_mut(index).ok_or(Unsupported)?;
        if jump.is_continue {
            let continue_to = target.continue_to.ok_or(Unsupported)?;
            self.add_predecessor(continue_to, from)
        } else {
            target.breaks.push(from);
            Ok(())
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
        // Class members are left out of the enclosing flow: methods,
        // accessors, constructors, and static blocks are execution roots of
        // their own, and TypeScript analyzes a class field that has an
        // initializer, including its decorators and computed key, apart from
        // the code around the class. Only the decorators and computed keys of
        // the other members lose the narrowing that TypeScript gives them.
        if AnyJsControlFlowRoot::can_cast(kind)
            || AnyJsClassMember::can_cast(kind)
            || AnyTsType::can_cast(kind)
        {
            return Ok(incoming);
        }
        if let Some(identifier) = JsIdentifierExpression::cast_ref(node) {
            if let Some(subject) = identifier
                .name()
                .ok()
                .and_then(|name| self.subjects.reference(self.model, &name))
            {
                self.reads.push(BuiltRead {
                    identifier,
                    point: incoming,
                    subject,
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
        let condition = decompose_condition(
            expression,
            outcome,
            self.model,
            self.functions,
            &mut self.subjects,
            &mut self.guards,
        );
        self.push_condition(antecedent, condition)
    }

    /// Adds a node where `condition` holds after `antecedent`.
    fn push_condition(
        &mut self,
        antecedent: FlowNodeId,
        condition: DecomposedCondition,
    ) -> Build<FlowNodeId> {
        let node = self.push(FlowNode::Condition {
            antecedent,
            guard: condition.guard,
        })?;
        self.mentions.extend(
            condition
                .mentions
                .into_iter()
                .map(|subject| (node, subject)),
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
        let id = FlowNodeId::try_from(self.nodes.len()).map_err(|_| Unsupported)?;
        self.nodes.push(node);
        Ok(id)
    }

    fn add_predecessor(&mut self, join: FlowNodeId, predecessor: FlowNodeId) -> Build<()> {
        let Some(FlowNode::Join(predecessors)) = self.nodes.get_mut(join as usize) else {
            return Err(Unsupported);
        };
        predecessors.push(predecessor);
        Ok(())
    }

    fn has_predecessors(&self, join: FlowNodeId) -> bool {
        matches!(self.nodes.get(join as usize), Some(FlowNode::Join(predecessors)) if !predecessors.is_empty())
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
    use biome_rowan::{TextRange, TextSize};
    use rustc_hash::FxHashMap;

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
        let raw_binding_types = FxHashMap::default();
        Built {
            source: source.to_owned(),
            root: build_root(
                &root,
                &model,
                &mut GuardFunctions::new(&[], &raw_binding_types),
            ),
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
            let mut pending = vec![(start, Vec::new(), vec![start])];
            let mut budget = 1_000usize;
            while let Some((node, mut facts, visited)) = pending.pop() {
                budget = budget.checked_sub(1).expect("excessive test graph");
                let mut follow = |predecessor: FlowNodeId, facts: Vec<String>| {
                    if !visited.contains(&predecessor) {
                        let mut visited = visited.clone();
                        visited.push(predecessor);
                        pending.push((predecessor, facts, visited));
                    }
                };
                match &root.nodes[node as usize] {
                    FlowNode::Start => result.push(facts),
                    FlowNode::Join(predecessors) => {
                        for predecessor in predecessors {
                            follow(*predecessor, facts.clone());
                        }
                    }
                    FlowNode::Condition { antecedent, guard } => {
                        self.facts(*guard, &mut facts);
                        follow(*antecedent, facts);
                    }
                }
            }
            result
        }

        fn facts(&self, guard: usize, facts: &mut Vec<String>) {
            match &self.root().guards[guard] {
                FlowGuard::Keep | FlowGuard::Incomplete => {}
                FlowGuard::Test {
                    subject,
                    test,
                    positive,
                } => {
                    let FlowSubject::Binding(binding) = &self.root().subjects[*subject as usize];
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
                (0..).zip(&built.root().nodes).any(|(id, node)| {
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
    }

    #[test]
    fn unlabeled_jumps_target_the_innermost_loop_even_when_labeled() {
        for source in [
            "function f(x) { outer: while (x) { break; } }",
            "function f(x) { outer: while (x) { continue; } }",
        ] {
            assert!(function(source).root.is_some(), "{source}");
        }

        let built = function(
            "function f(x) { while (true) { inner: while (x) { break; } read(x); return; } }",
        );
        let paths = built.paths("x", 2);
        assert_eq!(paths.len(), 2);
        assert!(paths.iter().any(|path| path == &["x:Truthy"]));
        assert!(paths.iter().any(|path| path == &["!x:Truthy"]));

        let built = function(
            "function f(x) { while (true) { inner: do { continue; } while (x); read(x); return; } }",
        );
        assert!(built.point("x", 2).is_some());
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
        let arrow = build(source, JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION);
        assert_eq!(arrow.paths("x", 2), [Vec::<String>::new()]);
        assert_eq!(arrow.point("x", 3), None);
    }

    #[test]
    fn reads_in_regions_keep_the_facts_from_before_them() {
        let built = function(
            "function f(q, y) { if (q) { switch (y) { case read(q): read(q); } try { read(q); } catch (e) { read(q); } finally { read(q); } for (const v of y) read(q); for (const k in y) read(q); class C extends read(q) {} enum E { A = read(q) } after(q); } }",
        );
        for occurrence in 2..=11 {
            assert_eq!(
                built.paths("q", occurrence),
                [vec!["q:Truthy".to_owned()]],
                "{occurrence}"
            );
        }
    }

    #[test]
    fn regions_narrow_only_within_expressions() {
        let built = function(
            "function f(x, b) { try { b && read(b); if (!x) return; read(x); } catch {} after(x); }",
        );
        assert!(contains(&built.paths("b", 2), "b:Truthy"));
        // The `if` and the `return` inside the region are not modeled.
        for occurrence in [2, 3] {
            let paths = built.paths("x", occurrence);
            assert!(
                paths.iter().flatten().all(|fact| !fact.contains('x')),
                "{occurrence}: {paths:?}"
            );
        }
    }

    #[test]
    fn jumps_leaving_regions_carry_the_entry_state() {
        // `break` out of a `try`: the exit after the loop joins the loop's own
        // exit with the state before the region.
        let built = function(
            "function f(x, y) { while (y) { if (x) { try { break; } catch {} } return; } after(x); }",
        );
        let paths = built.paths("x", 2);
        assert_eq!(paths.len(), 2, "{paths:?}");
        assert!(paths.iter().any(|path| path == &["!y:Truthy"]));
        assert!(paths.iter().any(|path| path == &["x:Truthy", "y:Truthy"]));

        // `continue` out of a `switch` reaches the loop update.
        let built = function(
            "function f(x, y) { for (;; read(x)) { if (!x) { switch (y) { case 1: continue; } } return; } }",
        );
        assert_eq!(built.paths("x", 1), [vec!["!x:Truthy".to_owned()]]);

        // `continue` out of a `try` reaches the test of a `do...while` loop.
        let built = function(
            "function f(x) { do { if (x) { try { continue; } finally {} } return; } while (read(x)); }",
        );
        assert_eq!(built.paths("x", 2), [vec!["x:Truthy".to_owned()]]);

        // A labeled `break` out of a `switch` leaves the labeled loop.
        let built = function(
            "function f(x, y) { outer: while (true) { if (x) { switch (y) { case 1: break outer; } } return; } after(x); }",
        );
        assert_eq!(built.paths("x", 2), [vec!["x:Truthy".to_owned()]]);
    }

    #[test]
    fn labeled_breaks_leave_regions() {
        let built = function(
            "function f(x) { done: { if (x) { try { break done; } catch {} } return; } after(x); }",
        );
        assert_eq!(built.paths("x", 2), [vec!["x:Truthy".to_owned()]]);
    }

    #[test]
    fn regions_connect_each_target_once() {
        let built = function(
            "function f(x, y) { while (true) { if (x) { try { if (y) break; break; } catch { break; } } return; } after(x); }",
        );
        assert_eq!(built.paths("x", 2), [vec!["x:Truthy".to_owned()]]);
    }

    #[test]
    fn jumps_to_statements_inside_a_region_do_not_leave_it() {
        for source in [
            "function f(x, y) { while (true) { switch (y) { case 1: break; } return; } after(x); }",
            "function f(x, y) { while (true) { label: switch (y) { case 1: break label; } return; } after(x); }",
            "function f(x, y) { while (true) { try { while (y) { break; } } catch {} return; } after(x); }",
            "function f(x, y) { while (true) { try { for (const v of y) { continue; } } finally {} return; } after(x); }",
            "function f(x, y) { while (true) { try { each: for (const v in y) { for (const w of v) { continue each; } } } catch {} return; } after(x); }",
            "function f(x, y) { while (true) { try { inner: { break inner; } } catch {} return; } after(x); }",
        ] {
            let built = function(source);
            assert!(built.root.is_some(), "{source}");
            assert_eq!(built.point("x", 1), None, "{source}");
        }
    }

    #[test]
    fn regions_need_a_target_for_each_jump() {
        for source in [
            "function f(x) { if (x) read(x); return; try { if () dead(x); } catch {} }",
            "function f(x) { if (x) read(x); return; switch () {} }",
        ] {
            assert!(
                build_parsed(source, JsSyntaxKind::JS_FUNCTION_DECLARATION)
                    .root
                    .is_none(),
                "{source}"
            );
        }
        // The statement control-flow graph gives no target to a label on a
        // `try` statement.
        let built =
            function("function f(x) { if (x) read(x); out: try { break out; } finally {} }");
        assert!(built.root.is_none());
    }

    #[test]
    fn class_expressions_evaluate_only_their_decorators_and_heritage() {
        let built = function(
            "function f(v) { if (v) { const C = @use(v) class extends use(v) { [v] = v; static w = v; m() { v; } static { v; } }; after(v); } }",
        );
        for occurrence in [2, 3, 9] {
            assert!(
                contains(&built.paths("v", occurrence), "v:Truthy"),
                "{occurrence}"
            );
        }
        for occurrence in 4..=8 {
            assert_eq!(built.point("v", occurrence), None, "{occurrence}");
        }
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
