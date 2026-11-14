use super::guards::{
    DecomposedCondition, FlowGuard, FlowOutcome, GuardFunctions, SwitchCase, SwitchKind,
    decompose_assertion, decompose_condition, decompose_switch_case, exhaustive_operand,
    is_unit_case,
};
use super::subjects::subject_of;
use super::{
    AnyFlowExpression, FlowNode, FlowNodeId, FlowSubjectId, FlowSubjectTable, FlowSwitch, FlowTest,
    MAX_SWITCHES, typeof_kind_bit,
};
use biome_js_control_flow::{AnyJsControlFlowRoot, is_truthy_literal};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsExpression, AnyJsFunction, AnyJsFunctionBody, AnyJsStatement,
    AnyJsSwitchClause, AnyTsType, JsDoWhileStatement, JsForStatement, JsIdentifierExpression,
    JsIfStatement, JsLanguage, JsSwitchStatement, JsSyntaxNode, JsWhileStatement, T,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxKindSet, TokenText};
use rustc_hash::FxHashSet;
use std::ops::Range;

const MAX_SYNTAX_NODES: usize = 16_384;
/// Limits how deeply syntax can nest below the execution root that gets flow.
pub(super) const MAX_DEPTH: usize = 128;
const MAX_FLOW_NODES: usize = 65_536;

/// Nodes below a statement list that contain no statement of the execution
/// root: expressions, types, and nested execution roots.
const NON_STATEMENT_KINDS: SyntaxKindSet<JsLanguage> = AnyJsControlFlowRoot::KIND_SET
    .union(AnyJsExpression::KIND_SET)
    .union(AnyTsType::KIND_SET);

/// Nodes that evaluation leaves out of the enclosing flow: nested execution
/// roots, class members, and types.
const UNEVALUATED_KINDS: SyntaxKindSet<JsLanguage> = AnyJsControlFlowRoot::KIND_SET
    .union(AnyJsClassMember::KIND_SET)
    .union(AnyTsType::KIND_SET);

/// The flow of one execution root before relevance filtering.
pub(super) struct BuiltRoot {
    pub(super) nodes: Vec<FlowNode>,
    pub(super) guards: Vec<FlowGuard>,
    /// The subjects that guards, mentions, and reads refer to.
    pub(super) subjects: FlowSubjectTable,
    /// Condition nodes paired with each subject their guard may narrow.
    pub(super) mentions: Vec<(FlowNodeId, FlowSubjectId)>,
    /// The subjects that conditions narrow only through
    /// [`FlowGuard::Property`], such as `s` when the only condition that
    /// mentions it is `s.kind === "c"`, or through `in`, `instanceof`, or a
    /// type-guard function whose target has only object types, as in
    /// `"a" in s`, `s instanceof Date`, or `isFoo(s)`.
    pub(super) property_only: FxHashSet<FlowSubjectId>,
    /// The subjects in `property_only` that `instanceof` or a type-guard
    /// function whose target has only object types narrows.
    pub(super) instance_tested: FxHashSet<FlowSubjectId>,
    /// The condition nodes, each with a subject in `mentions`, whose
    /// condition mentions the subject only as the subject of `instanceof` or
    /// of a type-guard function whose target has only object types.
    pub(super) instance_mentions: FxHashSet<(FlowNodeId, FlowSubjectId)>,
    /// Reachable reads of subjects with their incoming flow.
    pub(super) reads: Vec<BuiltRead>,
    /// The `switch` statements that the root's [`FlowGuard::Bypass`] guards
    /// stand for, in the order of their `switch` index.
    pub(super) switches: Vec<BuiltSwitch>,
}

/// A `switch` statement whose [`FlowGuard::Bypass`] gives `never` when the
/// statement is exhaustive.
pub(super) struct BuiltSwitch {
    pub(super) switch: FlowSwitch,
    /// The condition node of the path on which no case matches.
    pub(super) bypass: FlowNodeId,
}

pub(super) struct BuiltRead {
    /// The identifier or member expression that reads the subject.
    pub(super) expression: AnyFlowExpression,
    pub(super) point: FlowNodeId,
    pub(super) subject: FlowSubjectId,
}

/// Marks a root whose syntax or control flow narrowing does not support.
struct Unsupported;

type Build<T> = Result<T, Unsupported>;

/// Builds the flow of `root`, excluding nested execution roots.
///
/// Records reads and runtime tests of paths on `this`, such as `this.v`, only
/// when `this_can_narrow` is true. The caller computes it for `root` with
/// [`this_can_narrow`].
///
/// `root` must be selected by a [`FlowRootScanner`], which rejects the roots
/// that the builder cannot build, such as a root with a `with` statement.
/// Expression-bodied arrows are supported. Parameter initializers, class
/// members, `declare` statements, and the expressions of module export
/// declarations other than classes and enums are not evaluated, and
/// unreachable code records no reads.
///
/// The builder models the control flow of blocks, `if`, `while`, `do...while`,
/// `for`, `switch`, labeled statements, `break`, `continue`, `return`, and
/// `throw`. Any other statement, such as `try`, `for...in`, or `for...of`, is
/// an opaque region: the builder evaluates its expressions in source order
/// from the flow before it, and the code after it continues from that
/// evaluation.
/// A condition inside a region narrows only the rest of the expression that
/// contains it, as in `x && x.length`. Each `break` or `continue` that leaves
/// the region joins its target with the flow before the region. Because only
/// subjects that the root never writes narrow, the flow before a region is
/// never narrower than the real flow at any point inside or after it.
///
/// Returns `None` for inputs exceeding construction limits. Like the statement
/// control-flow graph, it also returns `None` when any statement, including
/// unreachable code, lacks syntax that graph requires, such as a jump target or
/// an `if` test.
///
/// [`FlowRootScanner`]: super::FlowRootScanner
/// [`this_can_narrow`]: super::subjects::this_can_narrow
pub(super) fn build_root(
    root: &AnyJsControlFlowRoot,
    model: &SemanticModel,
    functions: &mut GuardFunctions,
    this_can_narrow: bool,
) -> Option<BuiltRoot> {
    if !has_complete_statements(root) {
        return None;
    }
    let mut builder = RootBuilder {
        model,
        functions,
        nodes: vec![FlowNode::Start],
        guards: Vec::new(),
        subjects: FlowSubjectTable::new(root, this_can_narrow),
        mentions: Vec::new(),
        tested: FxHashSet::default(),
        parents: FxHashSet::default(),
        instance_tested: FxHashSet::default(),
        instance_mentions: FxHashSet::default(),
        reads: Vec::new(),
        switches: Vec::new(),
        jumps: Vec::new(),
        current: Some(0),
        remaining_visits: MAX_SYNTAX_NODES,
    };
    builder.root(root).ok()?;
    let property_only: FxHashSet<_> = builder
        .parents
        .difference(&builder.tested)
        .copied()
        .collect();
    let instance_tested = builder
        .instance_tested
        .intersection(&property_only)
        .copied()
        .collect();
    Some(BuiltRoot {
        nodes: builder.nodes,
        guards: builder.guards,
        subjects: builder.subjects,
        mentions: builder.mentions,
        property_only,
        instance_tested,
        instance_mentions: builder.instance_mentions,
        reads: builder.reads,
        switches: builder.switches,
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
            if NON_STATEMENT_KINDS.matches(child.kind()) {
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

/// The cases of a `switch` statement under construction.
struct SwitchCases {
    kind: SwitchKind,
    discriminant: AnyJsExpression,
    /// The test of each clause in source order, or `None` for `default`.
    tests: Vec<Option<AnyJsExpression>>,
    /// The flow after the discriminant, where every case is tested.
    pre: FlowNodeId,
    /// For `switch (true)`, a number of clauses and the flow on which none of
    /// the cases of the clauses before that number matched.
    unmatched: (usize, FlowNodeId),
}

impl SwitchCases {
    /// Returns the case whose test is `test`, assumed to match or not.
    fn case<'a>(&'a self, test: &'a AnyJsExpression, matched: bool) -> SwitchCase<'a> {
        SwitchCase {
            discriminant: &self.discriminant,
            test,
            kind: self.kind,
            matched,
        }
    }
}

/// A statement that `break` or `continue` can target.
struct JumpTarget {
    label: Option<TokenText>,
    kind: TargetKind,
    /// The loop's continue point; `None` for a labeled block or a `switch`.
    continue_to: Option<FlowNodeId>,
    breaks: Vec<FlowNodeId>,
}

struct RootBuilder<'a, 'raw> {
    model: &'a SemanticModel,
    functions: &'a mut GuardFunctions<'raw>,
    nodes: Vec<FlowNode>,
    guards: Vec<FlowGuard>,
    subjects: FlowSubjectTable,
    mentions: Vec<(FlowNodeId, FlowSubjectId)>,
    /// The subjects that a condition mentions other than as the object of a
    /// [`FlowGuard::Property`] or the subject of `in`, `instanceof`, or a
    /// type-guard function whose target has only object types.
    tested: FxHashSet<FlowSubjectId>,
    /// The objects of the [`FlowGuard::Property`] guards and the subjects of
    /// the `in` and `instanceof` tests and of the calls to type-guard
    /// functions whose targets have only object types, in all conditions.
    parents: FxHashSet<FlowSubjectId>,
    /// The subjects of the `instanceof` tests and of the calls to type-guard
    /// functions whose targets have only object types, in all conditions.
    instance_tested: FxHashSet<FlowSubjectId>,
    instance_mentions: FxHashSet<(FlowNodeId, FlowSubjectId)>,
    reads: Vec<BuiltRead>,
    switches: Vec<BuiltSwitch>,
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
            if NON_STATEMENT_KINDS.matches(child.kind()) {
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
                        kind: TargetKind::Block,
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
            AnyJsStatement::JsSwitchStatement(statement) => {
                self.switch_statement(&statement, current, label)
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
            .rposition(|target| jump.can_target(target.kind, target.label.as_ref()))
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
            kind: TargetKind::Loop,
            continue_to: Some(continue_to),
            breaks: Vec::new(),
        });
        match body {
            Some(body) => self.statement(body, None),
            None => Ok(()),
        }
    }

    /// Builds `statement` from `current`, as TypeScript's
    /// `bindSwitchStatement` and `bindCaseBlock` do. `label` is the label of
    /// `statement`, if any.
    ///
    /// Every case test is evaluated from the flow after the discriminant, so
    /// a read in a case test never sees the cases before it. The clauses form
    /// groups: a run of clauses without statements, followed by the next
    /// clause. A group's statements start where one of its cases matches,
    /// where no case of the statement matches when the group contains
    /// `default`, and where the statements of the group before fall through.
    /// The code after the statement continues from the end of the last
    /// group, from each `break` that targets the statement, and, without
    /// `default`, from the bypass: the path on which no case matches.
    fn switch_statement(
        &mut self,
        statement: &JsSwitchStatement,
        current: FlowNodeId,
        label: Option<TokenText>,
    ) -> Build<()> {
        let discriminant = statement.discriminant().map_err(|_| Unsupported)?;
        let pre = self.evaluate(discriminant.syntax(), current, 0)?;
        let clauses: Vec<_> = statement.cases().iter().collect();
        let mut tests = Vec::with_capacity(clauses.len());
        for clause in &clauses {
            let test = match clause {
                AnyJsSwitchClause::JsCaseClause(clause) => {
                    let test = clause.test().map_err(|_| Unsupported)?;
                    self.evaluate(test.syntax(), pre, 0)?;
                    Some(test)
                }
                AnyJsSwitchClause::JsDefaultClause(_) => None,
            };
            tests.push(test);
        }
        let mut cases = SwitchCases {
            kind: SwitchKind::new(statement),
            discriminant,
            tests,
            pre,
            unmatched: (0, pre),
        };
        self.jumps.push(JumpTarget {
            label,
            kind: TargetKind::Switch,
            continue_to: None,
            breaks: Vec::new(),
        });
        let mut fallthrough = None;
        let mut start = 0;
        for (index, clause) in clauses.iter().enumerate() {
            let consequent = clause.consequent();
            if consequent.is_empty() && index + 1 < clauses.len() {
                continue;
            }
            let entry = self.case_group_entry(&mut cases, start..index + 1)?;
            self.current = self.join(entry.into_iter().chain(fallthrough))?;
            self.statements(consequent.syntax())?;
            fallthrough = self.current;
            start = index + 1;
        }
        let target = self.jumps.pop().ok_or(Unsupported)?;
        let bypass = if cases.tests.iter().any(Option::is_none) {
            None
        } else {
            Some(self.switch_bypass(statement, &mut cases)?)
        };
        self.current = self.join(fallthrough.into_iter().chain(target.breaks).chain(bypass))?;
        Ok(())
    }

    /// Returns the flows where the statements of the case group made of the
    /// clauses at `group` start, apart from the fall-through, following
    /// TypeScript's `getTypeAtSwitchClause`.
    fn case_group_entry(
        &mut self,
        cases: &mut SwitchCases,
        group: Range<usize>,
    ) -> Build<Vec<FlowNodeId>> {
        let has_default = cases.tests[group.clone()].iter().any(Option::is_none);
        match cases.kind {
            SwitchKind::Opaque => Ok(vec![cases.pre]),
            // As in `narrowTypeBySwitchOnTrue`, no case before the group
            // matched. With `default` in the group, no case after it matched
            // either, while one of the group's own cases may have.
            SwitchKind::True => {
                let unmatched = self.unmatched_cases_before(cases, group.start)?;
                if has_default {
                    let mut flow = unmatched;
                    for index in group.end..cases.tests.len() {
                        flow = self.case_condition(cases, index, flow, false)?;
                    }
                    Ok(vec![flow])
                } else {
                    group
                        .map(|index| self.case_condition(cases, index, unmatched, true))
                        .collect()
                }
            }
            SwitchKind::Value | SwitchKind::Typeof => {
                let mut entry = Vec::with_capacity(group.len());
                for index in group {
                    if cases.tests[index].is_some() {
                        entry.push(self.case_condition(cases, index, cases.pre, true)?);
                    }
                }
                if has_default {
                    let unmatched = self.unmatched_cases(cases);
                    entry.push(match unmatched {
                        Some(unmatched) => self.push_condition(cases.pre, unmatched)?,
                        None => cases.pre,
                    });
                }
                Ok(entry)
            }
        }
    }

    /// Returns the flow after `antecedent` where the case of the clause at
    /// `index` matched or, when `matched` is false, did not, which is
    /// `antecedent` itself for `default` and for a case that narrows nothing.
    fn case_condition(
        &mut self,
        cases: &SwitchCases,
        index: usize,
        antecedent: FlowNodeId,
        matched: bool,
    ) -> Build<FlowNodeId> {
        let Some(Some(test)) = cases.tests.get(index) else {
            return Ok(antecedent);
        };
        match decompose_switch_case(
            cases.case(test, matched),
            self.model,
            self.functions,
            &mut self.subjects,
            &mut self.guards,
        ) {
            Some(condition) => self.push_condition(antecedent, condition),
            None => Ok(antecedent),
        }
    }

    /// Returns the flow of `switch (true)` on which none of the cases of the
    /// clauses before `end` matched, extending the chain that `cases` keeps.
    fn unmatched_cases_before(&mut self, cases: &mut SwitchCases, end: usize) -> Build<FlowNodeId> {
        let (mut count, mut flow) = cases.unmatched;
        while count < end {
            flow = self.case_condition(cases, count, flow, false)?;
            count += 1;
        }
        cases.unmatched = (count, flow);
        Ok(flow)
    }

    /// Returns the condition that no case of `cases` matched, or `None` when
    /// no case narrows anything.
    fn unmatched_cases(&mut self, cases: &SwitchCases) -> Option<DecomposedCondition> {
        let mut unmatched: Option<DecomposedCondition> = None;
        for test in cases.tests.iter().flatten() {
            let Some(case) = decompose_switch_case(
                cases.case(test, false),
                self.model,
                self.functions,
                &mut self.subjects,
                &mut self.guards,
            ) else {
                continue;
            };
            unmatched = Some(match unmatched {
                Some(unmatched) => unmatched.then(case, &mut self.guards),
                None => case,
            });
        }
        unmatched
    }

    /// Returns the bypass of `statement`, a `switch` without `default`: the
    /// flow on which none of its cases matched.
    ///
    /// When TypeScript can find the statement exhaustive, the bypass is a
    /// [`FlowGuard::Bypass`] that inference drops for every subject once it
    /// finds the statement exhaustive, as TypeScript's
    /// `getTypeAtFlowBranchLabel` does.
    fn switch_bypass(
        &mut self,
        statement: &JsSwitchStatement,
        cases: &mut SwitchCases,
    ) -> Build<FlowNodeId> {
        match cases.kind {
            SwitchKind::Opaque => Ok(cases.pre),
            // `switch (true)` is never exhaustive, because its cases have type
            // `boolean`, which is no unit type.
            SwitchKind::True => self.unmatched_cases_before(cases, cases.tests.len()),
            SwitchKind::Value | SwitchKind::Typeof => {
                let Some(mut unmatched) = self.unmatched_cases(cases) else {
                    return Ok(cases.pre);
                };
                let Some(switch) = self.exhaustive_switch(statement, cases, unmatched.guard) else {
                    return self.push_condition(cases.pre, unmatched);
                };
                let index = u32::try_from(self.switches.len()).map_err(|_| Unsupported)?;
                self.guards.push(FlowGuard::Bypass {
                    switch: index,
                    guard: unmatched.guard,
                });
                unmatched.guard = self.guards.len() - 1;
                let bypass = self.push_condition(cases.pre, unmatched)?;
                self.switches.push(BuiltSwitch { switch, bypass });
                Ok(bypass)
            }
        }
    }

    /// Returns the record of `statement` that lets inference decide whether
    /// it is exhaustive, given that `unmatched` is the index of the guard that
    /// tests that none of its cases matched, as [`FlowSwitch`] describes.
    ///
    /// Returns `None` when TypeScript can't find the statement exhaustive or
    /// Biome can't tell, and once the root records [`MAX_SWITCHES`]
    /// statements.
    fn exhaustive_switch(
        &mut self,
        statement: &JsSwitchStatement,
        cases: &SwitchCases,
        unmatched: usize,
    ) -> Option<FlowSwitch> {
        if self.switches.len() >= MAX_SWITCHES {
            return None;
        }
        let operand = exhaustive_operand(&cases.discriminant, cases.kind)?;
        if !cases
            .tests
            .iter()
            .flatten()
            .all(|test| is_unit_case(&cases.discriminant, test, cases.kind, self.model))
        {
            return None;
        }
        let subject = subject_of(&operand, self.model, &mut self.subjects)?;
        let typeof_kinds =
            (cases.kind == SwitchKind::Typeof).then(|| self.typeof_kinds(unmatched, subject));
        Some(FlowSwitch {
            discriminant: operand.range(),
            subject,
            guard: unmatched,
            start: statement.l_curly_token().ok()?.text_trimmed_range().start(),
            typeof_kinds,
        })
    }

    /// Returns the kinds of value, one bit each as [`typeof_kind_bit`] gives
    /// them, that the guard at `guard` tests `subject` with `typeof` for.
    fn typeof_kinds(&self, guard: usize, subject: FlowSubjectId) -> u8 {
        let mut kinds = 0;
        let mut pending = vec![guard];
        while let Some(index) = pending.pop() {
            match self.guards.get(index) {
                Some(FlowGuard::Both { left, right, .. }) => pending.extend([*left, *right]),
                Some(FlowGuard::Test {
                    subject: tested,
                    test: FlowTest::Typeof(kind),
                    ..
                }) if *tested == subject => kinds |= typeof_kind_bit(*kind),
                _ => {}
            }
        }
        kinds
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
        if UNEVALUATED_KINDS.matches(kind) {
            return Ok(incoming);
        }
        if let Some(identifier) = JsIdentifierExpression::cast_ref(node) {
            if let Some(subject) = identifier
                .name()
                .ok()
                .and_then(|name| self.subjects.reference(self.model, &name))
            {
                self.reads.push(BuiltRead {
                    expression: AnyFlowExpression::JsIdentifierExpression(identifier),
                    point: incoming,
                    subject,
                });
            }
            return Ok(incoming);
        }
        match AnyJsExpression::cast_ref(node) {
            // The object and the key are evaluated before the property is read.
            Some(
                expression @ (AnyJsExpression::JsStaticMemberExpression(_)
                | AnyJsExpression::JsComputedMemberExpression(_)),
            ) => {
                let flow = self.children(node, incoming, depth)?;
                if let Some(subject) = subject_of(&expression, self.model, &mut self.subjects)
                    && let Some(expression) = AnyFlowExpression::cast(expression.into_syntax())
                {
                    self.reads.push(BuiltRead {
                        expression,
                        point: flow,
                        subject,
                    });
                }
                Ok(flow)
            }
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
            _ => self.children(node, incoming, depth),
        }
    }

    /// Evaluates the children of `node`, which lies at `depth`, in source
    /// order from `incoming`, and returns the flow after them.
    fn children(
        &mut self,
        node: &JsSyntaxNode,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        let mut flow = incoming;
        for child in node.children() {
            flow = self.evaluate(&child, flow, depth + 1)?;
        }
        Ok(flow)
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
        for subject in &condition.instance_tested {
            if !condition.mentions.contains(subject) {
                self.instance_mentions.insert((node, *subject));
            }
        }
        self.tested.extend(condition.mentions.iter().copied());
        self.parents.extend(condition.parents.iter().copied());
        self.instance_tested.extend(condition.instance_tested);
        self.mentions.extend(
            condition
                .mentions
                .into_iter()
                .chain(condition.parents)
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
    use crate::js_module_info::flow::{FlowSubject, FlowTest, PropertyKey};
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

    /// Builds like [`build`], also accepting sources with syntax errors, and
    /// records paths on `this` where [`this_can_narrow`] holds.
    ///
    /// [`this_can_narrow`]: super::super::subjects::this_can_narrow
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
                super::super::subjects::this_can_narrow(&root),
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
                .find(|read| read.expression.range() == range)
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
                    let name = self.subject_name(*subject);
                    let test = self.test_name(*test);
                    let negation = if *positive { "" } else { "!" };
                    facts.push(format!("{negation}{name}:{test}"));
                }
                // A test of `o.k` that narrows `o` is written `o[k]:Test`.
                FlowGuard::Property {
                    object,
                    key,
                    test,
                    positive,
                    ..
                } => {
                    let object = self.subject_name(*object);
                    let key = match key {
                        PropertyKey::Name(name) | PropertyKey::Private(name) => name,
                    };
                    let test = self.test_name(*test);
                    let negation = if *positive { "" } else { "!" };
                    facts.push(format!("{negation}{object}[{key}]:{test}"));
                }
                // A test that applies only to a readonly path is written
                // `readonly(Test)`.
                FlowGuard::ReadonlyPath(inner) => {
                    let mut inner_facts = Vec::new();
                    self.facts(*inner, &mut inner_facts);
                    facts.extend(
                        inner_facts
                            .into_iter()
                            .map(|fact| format!("readonly({fact})")),
                    );
                }
                FlowGuard::Both { left, right, .. } => {
                    self.facts(*left, facts);
                    self.facts(*right, facts);
                }
                // The bypass of a `switch` that may be exhaustive is written
                // `bypass(Test)`.
                FlowGuard::Bypass { guard, .. } => {
                    let mut inner_facts = Vec::new();
                    self.facts(*guard, &mut inner_facts);
                    facts.extend(
                        inner_facts
                            .into_iter()
                            .map(|fact| format!("bypass({fact})")),
                    );
                }
            }
        }
    }

    impl Built {
        /// Names `test` as `Truthy`, `Nullish`, or `Other`, or as `=` and
        /// the source of the literal that a test of equality compares with.
        fn test_name(&self, test: FlowTest) -> String {
            match test {
                FlowTest::Truthy => "Truthy".to_owned(),
                FlowTest::Nullish => "Nullish".to_owned(),
                FlowTest::Literal(range) => format!("={}", &self.source[range]),
                _ => "Other".to_owned(),
            }
        }

        /// Returns the source of the variable or the pattern that `subject`
        /// starts at, or `this`, followed by the names along its path, as in
        /// `o.v` or `this.#w`.
        fn subject_name(&self, subject: FlowSubjectId) -> String {
            match &self.root().subjects.as_slice()[subject as usize] {
                FlowSubject::Binding(binding) | FlowSubject::Pattern(binding) => {
                    self.source[*binding].to_owned()
                }
                FlowSubject::This => "this".to_owned(),
                FlowSubject::Member {
                    object,
                    key: PropertyKey::Name(name),
                } => format!("{}.{name}", self.subject_name(*object)),
                FlowSubject::Member {
                    object,
                    key: PropertyKey::Private(name),
                } => format!("{}.#{name}", self.subject_name(*object)),
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
    fn property_paths_are_read_and_tested_like_variables() {
        let built =
            function("function f(o) { if (o.v) read(o.v.w, (o).v); if (o?.v == null) read(o.v); }");
        assert!(contains(&built.paths("o.v", 1), "o.v:Truthy"));
        assert!(contains(&built.paths("o.v.w", 0), "o.v:Truthy"));
        assert!(contains(&built.paths("(o).v", 0), "o.v:Truthy"));
        assert!(contains(&built.paths("o.v", 2), "o.v:Nullish"));
    }

    #[test]
    fn paths_on_this_are_read_and_tested_where_this_can_narrow() {
        const SOURCE: &str = "class K { #w; m() { if (this.v) read(this.v); if (this.#w) read(this.#w); } constructor() { if (this.v) read(this.v); } }";
        let method = build(SOURCE, JsSyntaxKind::JS_METHOD_CLASS_MEMBER);
        assert!(contains(&method.paths("this.v", 1), "this.v:Truthy"));
        assert!(contains(&method.paths("this.#w", 1), "this.#w:Truthy"));
        let constructor = build(SOURCE, JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER);
        assert_eq!(constructor.point("this.v", 3), None);
        assert!(constructor.root().mentions.is_empty());
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
            "function f(q, y) { if (q) { try { read(q); } catch (e) { read(q); } finally { read(q); } for (const v of y) read(q); for (const k in y) read(q); class C extends read(q) {} enum E { A = read(q) } after(q); } }",
        );
        for occurrence in 2..=9 {
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

        // `continue` out of a `try` reaches the test of a `do...while` loop.
        let built = function(
            "function f(x) { do { if (x) { try { continue; } finally {} } return; } while (read(x)); }",
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

    /// Returns the facts of each path to the read `name` at `occurrence`,
    /// each path's facts sorted, and the paths sorted.
    fn sorted_paths(built: &Built, name: &str, occurrence: usize) -> Vec<Vec<String>> {
        let mut paths = built.paths(name, occurrence);
        for path in &mut paths {
            path.sort();
        }
        paths.sort();
        paths
    }

    fn facts(facts: &[&str]) -> Vec<String> {
        facts.iter().map(|fact| (*fact).to_owned()).collect()
    }

    #[test]
    fn switch_cases_narrow_their_group() {
        let built = function(
            "function f(x) { switch (x) { case 1: read(x); case 2: case 3: read(x); break; default: read(x); } after(x); }",
        );
        assert_eq!(sorted_paths(&built, "x", 2), [facts(&["x:=1"])]);
        // The group of `2` and `3` also continues the statements of `1`.
        assert_eq!(
            sorted_paths(&built, "x", 3),
            [facts(&["x:=1"]), facts(&["x:=2"]), facts(&["x:=3"])]
        );
        assert_eq!(
            sorted_paths(&built, "x", 4),
            [facts(&["!x:=1", "!x:=2", "!x:=3"])]
        );
        // With `default`, no path skips every case.
        assert_eq!(
            sorted_paths(&built, "x", 5),
            [
                facts(&["!x:=1", "!x:=2", "!x:=3"]),
                facts(&["x:=1"]),
                facts(&["x:=2"]),
                facts(&["x:=3"])
            ]
        );
    }

    #[test]
    fn default_joins_the_group_that_contains_it() {
        let built =
            function("function f(x) { switch (x) { case 1: break; default: case 2: read(x); } }");
        assert_eq!(
            sorted_paths(&built, "x", 2),
            [facts(&["!x:=1", "!x:=2"]), facts(&["x:=2"])]
        );
    }

    #[test]
    fn case_tests_start_from_the_discriminant() {
        let built = function("function f(x) { switch (x) { case 1: break; case read(x): } }");
        assert_eq!(built.paths("x", 2), [Vec::<String>::new()]);
    }

    #[test]
    fn switches_without_default_continue_where_no_case_matches() {
        let built = function("function f(x) { switch (x) { case 1: return; } after(x); }");
        assert_eq!(built.paths("x", 2), [facts(&["bypass(!x:=1)"])]);
        assert_eq!(built.root().switches.len(), 1);
        // A case that isn't a unit value keeps the switch from being
        // exhaustive, and a case that narrows nothing adds no condition.
        let built = function("function f(x) { switch (x) { case f(): return; } after(x); }");
        assert_eq!(built.paths("x", 2), [Vec::<String>::new()]);
        assert!(built.root().switches.is_empty());
        let built =
            function("function f(x) { switch (x) { case 1: return; case f(): } after(x); }");
        assert_eq!(
            sorted_paths(&built, "x", 2),
            [Vec::new(), facts(&["!x:=1"])]
        );
        assert!(built.root().switches.is_empty());
        // An empty switch has nothing to narrow.
        let built = function("function f(x) { switch (x) {} after(x); }");
        assert_eq!(built.paths("x", 2), [Vec::<String>::new()]);
        assert!(built.root().switches.is_empty());
    }

    #[test]
    fn breaks_leave_the_switch_and_continues_reach_the_loop() {
        let built = function(
            "function f(x, y) { for (;; read(x)) { if (!x) { switch (y) { case 1: continue; } } return; } }",
        );
        assert_eq!(
            sorted_paths(&built, "x", 1),
            [facts(&["!x:Truthy", "y:=1"])]
        );
        let built = function(
            "function f(x, y) { outer: while (true) { if (x) { switch (y) { case 1: break outer; } } return; } after(x); }",
        );
        assert_eq!(sorted_paths(&built, "x", 2), [facts(&["x:Truthy", "y:=1"])]);
        for source in [
            "function f(x, y) { while (true) { switch (y) { case 1: break; } return; } after(x); }",
            "function f(x, y) { while (true) { label: switch (y) { case 1: break label; } return; } after(x); }",
        ] {
            let built = function(source);
            assert_eq!(built.point("x", 1), None, "{source}");
        }
        let built = function(
            "function f(x, y) { label: switch (x) { case 1: if (y) break label; read(y); break; case 2: if (y) break; read(y); } after(y); }",
        );
        assert_eq!(built.paths("y", 2).len(), 1);
        assert_eq!(built.paths("y", 4).len(), 1);
        // Both `break` statements, the end of the second case, the end of the
        // first case's statements before its `break`, and the bypass.
        assert_eq!(built.paths("y", 5).len(), 5);
    }

    #[test]
    fn switch_true_assumes_the_cases_before_a_group_failed() {
        let built = function(
            "function f(x, y) { switch (true) { case x === 1: read(x); break; case y === 2: read(x); break; default: read(x); } after(x); }",
        );
        assert_eq!(sorted_paths(&built, "x", 2), [facts(&["x:=1"])]);
        assert_eq!(sorted_paths(&built, "x", 3), [facts(&["!x:=1", "y:=2"])]);
        assert_eq!(sorted_paths(&built, "x", 4), [facts(&["!x:=1", "!y:=2"])]);
        // `switch (true)` is never exhaustive.
        assert!(built.root().switches.is_empty());
        // A case that may give a truthy value other than `true` narrows only
        // where it matched.
        let built = function(
            "function f(x) { switch (true) { case x: read(x); break; default: read(x); } }",
        );
        assert_eq!(built.paths("x", 2), [facts(&["x:Truthy"])]);
        assert_eq!(built.paths("x", 3), [Vec::<String>::new()]);
        // A group with `default` assumes only the cases outside it failed.
        let built = function(
            "function f(x) { switch (true) { case x === 1: default: read(x); break; case x === 2: } }",
        );
        assert_eq!(sorted_paths(&built, "x", 2), [facts(&["!x:=2"])]);
    }

    #[test]
    fn default_narrows_no_receiver_of_an_optional_chain() {
        let built = function(
            "function f(z) { switch (z?.k) { case undefined: break; case 1: read(z); break; default: read(z); } }",
        );
        assert!(contains(&built.paths("z", 2), "!z:Nullish"));
        let default = built.paths("z", 3);
        assert!(
            default.iter().flatten().all(|fact| fact != "!z:Nullish"),
            "{default:?}"
        );
    }

    #[test]
    fn typeof_cases_narrow_only_their_operand() {
        let built = function(
            "function f(z) { switch (typeof z.k) { case \"string\": read(z); } if (typeof z.k === \"string\") read(z); }",
        );
        let case = built.paths("z", 2);
        assert!(
            case.iter().flatten().all(|fact| !fact.contains("z[k]")),
            "{case:?}"
        );
        assert!(contains(&built.paths("z", 4), "z[k]:Other"));
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
