use super::guards::{
    DecomposedCondition, FlowGuard, FlowOutcome, GuardFunctions, SwitchCase, SwitchKind,
    decompose_assignment_target, decompose_condition, decompose_for_in, decompose_statement_call,
    decompose_switch_case, exhaustive_operand, is_unit_case,
};
use super::subjects::{identifier_key, literal_key, member_name_key, property_key, subject_of};
use super::writes::WrittenTargets;
use super::{
    AnyFlowExpression, FlowAssigned, FlowAssignedValue, FlowCall, FlowCallee, FlowDestructureStep,
    FlowEvolving, FlowNode, FlowNodeId, FlowSubject, FlowSubjectId, FlowSubjectTable, FlowSwitch,
    FlowTest, FlowWrite, FlowWriteEffect, MAX_SWITCHES, PropertyKey, evolving_declaration,
    typeof_kind_bit,
};
use biome_js_control_flow::AnyJsControlFlowRoot;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsArrayAssignmentPatternElement, AnyJsArrayBindingPatternElement,
    AnyJsArrowFunctionParameters, AnyJsAssignment, AnyJsAssignmentPattern, AnyJsBinding,
    AnyJsBindingPattern, AnyJsCallArgument, AnyJsClass, AnyJsClassMember,
    AnyJsConstructorParameter, AnyJsExpression, AnyJsForInOrOfInitializer, AnyJsForInitializer,
    AnyJsFormalParameter, AnyJsFunction, AnyJsFunctionBody, AnyJsLiteralExpression, AnyJsName,
    AnyJsObjectAssignmentPatternMember, AnyJsObjectBindingPatternMember, AnyJsObjectMemberName,
    AnyJsOptionalChainExpression, AnyJsParameter, AnyJsStatement, AnyJsSwitchClause,
    AnyJsTryStatement, AnyJsVariableDeclaration, AnyTsName, AnyTsType, AnyTsTypeMember,
    ClassMemberName, JsArrowFunctionExpression, JsAssignmentExpression, JsAssignmentOperator,
    JsBinaryOperator, JsCallExpression, JsClassExpression, JsClassMemberList,
    JsComputedMemberAssignment, JsComputedMemberName, JsDecorator, JsDoWhileStatement, JsExport,
    JsForInStatement, JsForOfStatement, JsForStatement, JsFunctionExpression, JsGetterClassMember,
    JsGetterObjectMember, JsIdentifierAssignment, JsIdentifierExpression, JsIfStatement,
    JsInitializerClause, JsLanguage, JsMethodClassMember, JsMethodObjectMember, JsParameters,
    JsParenthesizedAssignment, JsParenthesizedExpression, JsReferenceIdentifier,
    JsSetterClassMember, JsSetterObjectMember, JsStaticInitializationBlockClassMember,
    JsStaticMemberExpression, JsSwitchStatement, JsSyntaxNode, JsThisExpression, JsUnaryExpression,
    JsUnaryOperator, JsVariableDeclarator, JsWhileStatement, T, TsDeclareStatement,
    TsExternalModuleDeclaration, TsGlobalDeclaration, TsInterfaceDeclaration,
    TsNonNullAssertionAssignment, TsNonNullAssertionExpression, TsTypeAnnotation,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_js_type_info::{DestructureField, own_returns};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, SyntaxKindSet, SyntaxResult, TextRange, TokenText,
    WalkEvent, declare_node_union,
};
use rustc_hash::{FxHashMap, FxHashSet};
use std::ops::Range;

const MAX_SYNTAX_NODES: usize = 16_384;
/// Limits how deeply syntax can nest below the execution root that gets flow.
pub(super) const MAX_DEPTH: usize = 128;
const MAX_FLOW_NODES: usize = 65_536;

/// Nodes below a statement list that the statement walk doesn't enter:
/// expressions, types, and nested execution roots. Evaluation builds the
/// statements of inline code among them where it runs.
const NON_STATEMENT_KINDS: SyntaxKindSet<JsLanguage> = AnyJsControlFlowRoot::KIND_SET
    .union(AnyJsExpression::KIND_SET)
    .union(AnyTsType::KIND_SET);

/// Nodes that evaluation leaves out of the enclosing flow: nested execution
/// roots, class members, and types. Inline code among them, a static block or
/// a function called where it is defined, still runs in the enclosing flow
/// (see [`AnyInlineCode::runs_inline`]).
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
    /// The condition nodes, each with a subject in `mentions`, whose
    /// condition mentions the subject only through call points.
    pub(super) call_mentions: FxHashSet<(FlowNodeId, FlowSubjectId)>,
    /// The call points that the root's [`FlowGuard::Call`] guards stand for,
    /// each with the subjects that it may narrow.
    pub(super) calls: Vec<(FlowCall, Box<[FlowSubjectId]>)>,
    /// Reachable reads of subjects with their incoming flow.
    pub(super) reads: Vec<BuiltRead>,
    /// The `switch` statements that the root's [`FlowGuard::Bypass`] guards
    /// stand for, in the order of their `switch` index.
    pub(super) switches: Vec<BuiltSwitch>,
    /// The writes that the [`FlowNode::Write`] nodes stand for.
    pub(super) writes: Vec<FlowWrite>,
    /// The values that the writes assign.
    pub(super) assigned: Vec<FlowAssigned>,
    /// The assignment targets and `delete` expressions that have a write
    /// point in the graph.
    pub(super) written: WrittenTargets,
    /// The closures that the root creates in its reachable code, with their
    /// capture points, in the order the builder evaluates them.
    pub(super) captures: Vec<BuiltCapture>,
    /// The write points of the initializers of variables whose type evolves
    /// with their writes, as `null` in `let x = null`, which give such a
    /// variable the type that its declaration already gives it.
    pub(super) initializers: FxHashSet<FlowNodeId>,
    /// The variables whose type evolves with their writes and that a `var`
    /// declares with an initializer. A read before the declarator sees such a
    /// variable as `undefined` instead of the type that its declaration gives.
    pub(super) hoisted: FxHashSet<FlowSubjectId>,
    /// Each statement of the root's own code that the builder met, in the
    /// order it met them, with its incoming flow, or `None` where no path
    /// reaches it. Empty unless the build records statements. A statement
    /// nested in one that no path reaches is never met. A finally block built
    /// twice lists its statements twice.
    pub(super) statements: Vec<(TextRange, Option<FlowNodeId>)>,
    /// The flow that reaches the end of the root, or `None` when every path
    /// ends before it, as at `return` or `throw`.
    pub(super) end: Option<FlowNodeId>,
}

declare_node_union! {
    /// A function that can continue the flow of the code that creates it, as
    /// TypeScript lets a read inside it do: a function or arrow expression,
    /// and a method or accessor of an object literal or of a class. A method
    /// or accessor of a class continues the flow only in a class expression
    /// (see [`Self::continues_outer_flow`]). TypeScript starts the flow of a
    /// function declaration, a constructor, and a method or accessor of a
    /// class declaration at their own start. A function or arrow expression
    /// that runs inline is no closure: the builder evaluates it where it runs
    /// (see [`AnyInlineCode::runs_inline`]).
    pub(super) AnyContinuingClosure = JsFunctionExpression
        | JsArrowFunctionExpression
        | JsMethodObjectMember
        | JsGetterObjectMember
        | JsSetterObjectMember
        | JsMethodClassMember
        | JsGetterClassMember
        | JsSetterClassMember
}

impl AnyContinuingClosure {
    /// Returns whether a read inside the closure can see the flow where it
    /// is created. A read always can in a function or arrow expression and
    /// in an object literal member, and in a class member only when the
    /// class is a class expression, as in TypeScript.
    pub(super) fn continues_outer_flow(&self) -> bool {
        let member = match self {
            Self::JsFunctionExpression(_)
            | Self::JsArrowFunctionExpression(_)
            | Self::JsMethodObjectMember(_)
            | Self::JsGetterObjectMember(_)
            | Self::JsSetterObjectMember(_) => return true,
            Self::JsMethodClassMember(member) => member.syntax(),
            Self::JsGetterClassMember(member) => member.syntax(),
            Self::JsSetterClassMember(member) => member.syntax(),
        };
        member
            .parent()
            .and_then(|members| JsClassMemberList::cast(members)?.parent::<JsClassExpression>())
            .is_some()
    }
}

declare_node_union! {
    /// Code that may run inline, as part of the flow of the execution root
    /// around it: a function or arrow expression called where it is defined,
    /// or a class static block. See [`Self::runs_inline`].
    pub(super) AnyInlineCode = JsFunctionExpression
        | JsArrowFunctionExpression
        | JsStaticInitializationBlockClassMember
}

/// Code inside which an inline function or static block never runs in the
/// flow of the root around it: class members other than static blocks,
/// decorators, types, and ambient declarations. Collection treats a function
/// called where it is defined inside such code as a root of its own.
const INLINE_BARRIER_KINDS: SyntaxKindSet<JsLanguage> = AnyJsClassMember::KIND_SET
    .union(JsDecorator::KIND_SET)
    .union(AnyTsType::KIND_SET)
    .union(TsDeclareStatement::KIND_SET)
    .union(TsExternalModuleDeclaration::KIND_SET)
    .union(TsGlobalDeclaration::KIND_SET);

impl AnyInlineCode {
    /// Returns whether this code runs as part of the flow of the execution
    /// root around it, as TypeScript's binder runs it, instead of being a
    /// root of its own.
    ///
    /// A static block runs where its class is evaluated. A function or arrow
    /// expression that is neither `async` nor a generator runs where a call
    /// directly invokes it, through parentheses only, as in `(() => {})()`,
    /// `(function () {})?.()`, or `!(() => {})()`. TypeScript runs an
    /// `async` function or a generator called in place, a function called
    /// through `.call`, a non-null assertion, or `as`, and a constructed one
    /// apart from the code around them.
    ///
    /// The code must also lie where the builder of that root evaluates it:
    /// in a statement or an expression of the root's own code, including
    /// other inline code, but not in a class member other than a static
    /// block, a decorator, the computed key of a method or accessor, a type,
    /// an ambient declaration, or an export clause that is not a statement,
    /// such as `export const v = (() => 1)();`. A function called in place
    /// there stays a root of its own, so its own tests still narrow its
    /// reads.
    pub(super) fn runs_inline(&self) -> bool {
        if !self.is_called_in_place() {
            return false;
        }
        let mut in_statement = false;
        for ancestor in self.syntax().ancestors().skip(1) {
            let kind = ancestor.kind();
            if AnyJsControlFlowRoot::can_cast(kind) {
                match Self::cast(ancestor) {
                    // Its context decides for both, so the walk goes on.
                    Some(code) if code.is_called_in_place() => {
                        in_statement = false;
                        continue;
                    }
                    _ => return true,
                }
            }
            if INLINE_BARRIER_KINDS.matches(kind)
                || (JsComputedMemberName::can_cast(kind)
                    && ancestor
                        .parent()
                        .is_some_and(|member| AnyJsControlFlowRoot::can_cast(member.kind())))
            {
                return false;
            }
            if AnyJsStatement::can_cast(kind) {
                in_statement = true;
            } else if JsExport::can_cast(kind) && !in_statement {
                return false;
            }
        }
        false
    }

    /// Returns whether this is a static block, or a function or arrow
    /// expression that is neither `async` nor a generator and that a call
    /// invokes where it is defined, whatever the code around the call.
    fn is_called_in_place(&self) -> bool {
        match self {
            Self::JsStaticInitializationBlockClassMember(_) => true,
            Self::JsFunctionExpression(_) | Self::JsArrowFunctionExpression(_) => {
                let mut parent = self.syntax().parent();
                while let Some(node) = parent
                    .as_ref()
                    .filter(|node| JsParenthesizedExpression::can_cast(node.kind()))
                {
                    parent = node.parent();
                }
                parent
                    .and_then(JsCallExpression::cast)
                    .and_then(|call| inline_callee(&call))
                    .is_some_and(|callee| callee.syntax() == self.syntax())
            }
        }
    }
}

/// Returns the function or arrow expression that `call` invokes, through
/// parentheses, when it is neither `async` nor a generator, without checking
/// where the call is.
fn inline_callee(call: &JsCallExpression) -> Option<AnyJsFunction> {
    match call.callee().ok()?.omit_parentheses() {
        AnyJsExpression::JsFunctionExpression(function)
            if function.async_token().is_none() && function.star_token().is_none() =>
        {
            Some(function.into())
        }
        AnyJsExpression::JsArrowFunctionExpression(arrow) if arrow.async_token().is_none() => {
            Some(arrow.into())
        }
        _ => None,
    }
}

/// Returns the function or arrow expression that `call` runs inline, as
/// [`AnyInlineCode::runs_inline`] decides, or `None` when `call` invokes
/// anything else.
fn inline_function(call: &JsCallExpression) -> Option<AnyJsFunction> {
    inline_callee(call).filter(|function| {
        AnyInlineCode::cast_ref(function.syntax()).is_some_and(|code| code.runs_inline())
    })
}

/// A closure that a root creates, with its capture point, the flow where the
/// root creates it. A read inside the closure continues from that point, as
/// in TypeScript.
pub(super) struct BuiltCapture {
    pub(super) closure: AnyContinuingClosure,
    pub(super) point: FlowNodeId,
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
/// Expression-bodied arrows are supported. Parameters run before the body.
/// Class members other than static blocks, `declare` statements, and the
/// expressions of module export declarations other than classes and enums
/// are not evaluated, and unreachable code records no reads and adds no write
/// points. A write there has no write point, so collection counts it as a
/// write that the graph doesn't model, as [`BuiltRoot::written`] describes.
///
/// A function or arrow expression called where it is defined and a class
/// static block run inline, as [`AnyInlineCode::runs_inline`] decides: a call
/// evaluates the function's parameters and body where it runs it, and the
/// code after the call continues from the end of the body joined with each
/// `return`. A static block runs where its class is evaluated.
///
/// The builder models the control flow of blocks, `if`, `while`, `do...while`,
/// `for`, `for...in`, `for...of`, `switch`, `try`, labeled statements,
/// `break`, `continue`, `return`, and `throw`, and evaluates expressions in
/// the order in which JavaScript runs them, with a write point after each
/// write to a subject and a condition node after each statement call (see
/// [`decompose_statement_call`]). Any other statement, such as a class or
/// enum declaration, is an opaque region: the builder evaluates its
/// expressions from the flow before it, and the code after it continues from
/// that evaluation. A condition inside a region narrows only the rest of the
/// expression that contains it, as in `x && x.length`. Each `break` or
/// `continue` that leaves the region joins its target with the flow before
/// the region. That flow is never narrower than the flow at any point inside
/// the region while the region has no write point, so a region with both a
/// write point and a `break` or `continue` that leaves it is not supported.
///
/// A branch that a condition written as the literal `true` or `false` never
/// takes has no flow, as in TypeScript's binder, so it records no read and
/// adds nothing where it joins other paths. The statements after a call of a
/// local function that never returns have no flow either.
///
/// When `records_statements` is true, the build lists the statements that
/// it meets with their incoming flow in [`BuiltRoot::statements`].
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
    records_statements: bool,
) -> Option<BuiltRoot> {
    if !has_complete_statements(root) {
        return None;
    }
    // A build that failed may have left the call points of its root.
    functions.take_calls();
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
        call_mentions: FxHashSet::default(),
        reads: Vec::new(),
        switches: Vec::new(),
        writes: Vec::new(),
        write_ids: FxHashMap::default(),
        assigned: Vec::new(),
        assigned_ids: FxHashMap::default(),
        value_callees: FxHashMap::default(),
        written: WrittenTargets::default(),
        captures: Vec::new(),
        evolving: FxHashMap::default(),
        evolving_values: FxHashMap::default(),
        initializers: FxHashSet::default(),
        hoisted: FxHashSet::default(),
        write_points: 0,
        jumps: Vec::new(),
        exceptions: Vec::new(),
        returns: Vec::new(),
        records_reads: true,
        statements: records_statements.then(Vec::new),
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
        call_mentions: builder.call_mentions,
        calls: builder.functions.take_calls(),
        reads: builder.reads,
        switches: builder.switches,
        writes: builder.writes,
        assigned: builder.assigned,
        written: builder.written,
        captures: builder.captures,
        initializers: builder.initializers,
        hoisted: builder.hoisted,
        statements: builder.statements.unwrap_or_default(),
        end: builder.current,
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

/// Returns the value of `expression` when it is the literal `true` or
/// `false` itself, without parentheses, as TypeScript's binder reads a
/// condition.
pub(super) fn literal_condition(expression: &AnyJsExpression) -> Option<bool> {
    let AnyJsExpression::AnyJsLiteralExpression(
        AnyJsLiteralExpression::JsBooleanLiteralExpression(literal),
    ) = expression
    else {
        return None;
    };
    literal
        .value_token()
        .ok()
        .map(|token| token.kind() == T![true])
}

/// Limits the operators of `&&`, `||`, `!`, and parentheses that
/// [`never_has_outcome`] visits in one condition.
const MAX_LITERAL_STEPS: usize = 256;

/// Returns whether the condition `expression` is never truthy, when `truthy`
/// is true, or never falsy otherwise, as TypeScript's binder finds it. The
/// branch that needs that outcome has no flow.
///
/// The literal `true` is never falsy and `false` never truthy. TypeScript
/// binds each operand of `&&` or `||` as a condition of its own, so
/// `c && false` is never truthy and `c || true` never falsy. That also holds
/// inside parentheses and `!`, which inverts the outcome. Any other
/// condition can be either, including `(false)`, `!false`, `0`, a `const`
/// that holds `false`, and the operands of `??`. Returns false when the walk
/// runs out of work.
pub(super) fn never_has_outcome(expression: &AnyJsExpression, truthy: bool) -> bool {
    let mut remaining = MAX_LITERAL_STEPS;
    never_has(expression, truthy, &mut remaining)
}

fn never_has(expression: &AnyJsExpression, truthy: bool, remaining: &mut usize) -> bool {
    if let Some(value) = literal_condition(expression) {
        return value != truthy;
    }
    // TypeScript looks through parentheses and `!` for a logical operator,
    // which gets the outcome, inverted by each `!`. Any other expression
    // inside them is one ordinary condition.
    let mut truthy = truthy;
    let mut expression = expression.clone();
    let logical = loop {
        let Some(left) = remaining.checked_sub(1) else {
            return false;
        };
        *remaining = left;
        expression = match expression {
            AnyJsExpression::JsParenthesizedExpression(parenthesized) => {
                match parenthesized.expression() {
                    Ok(inner) => inner,
                    Err(_) => return false,
                }
            }
            AnyJsExpression::JsUnaryExpression(unary)
                if unary.operator() == Ok(JsUnaryOperator::LogicalNot) =>
            {
                truthy = !truthy;
                match unary.argument() {
                    Ok(inner) => inner,
                    Err(_) => return false,
                }
            }
            AnyJsExpression::JsLogicalExpression(logical) => break logical,
            _ => return false,
        };
    };
    let (Ok(left), Ok(right), Ok(operator)) =
        (logical.left(), logical.right(), logical.operator_token())
    else {
        return false;
    };
    match (operator.kind(), truthy) {
        // Both operands must be truthy.
        (T![&&], true) => never_has(&left, true, remaining) || never_has(&right, true, remaining),
        // The left operand is falsy, or truthy with a falsy right one.
        (T![&&], false) => {
            never_has(&left, false, remaining)
                && (never_has(&left, true, remaining) || never_has(&right, false, remaining))
        }
        (T![||], true) => {
            never_has(&left, true, remaining)
                && (never_has(&left, false, remaining) || never_has(&right, true, remaining))
        }
        (T![||], false) => {
            never_has(&left, false, remaining) || never_has(&right, false, remaining)
        }
        _ => false,
    }
}

/// Returns whether `link`, one link of an optional chain, evaluates an
/// operand: the key of a computed member access, as `k` in `x?.[k]`, or the
/// arguments of a call, as `y` in `x?.f(y)`. A `?.` before such a link
/// evaluates the operand only when its object is not nullish.
/// Returns the link of a chain that `object`, the object of another link,
/// is, looking through non-null assertions, as `x?.a` in `x?.a!.b`. A
/// non-null assertion doesn't end an optional chain, because `x?.a!.b` runs
/// as `x?.a.b`, while parentheses do.
pub(super) fn inner_link(object: &AnyJsExpression) -> Option<AnyJsOptionalChainExpression> {
    let mut object = object.clone();
    while let AnyJsExpression::TsNonNullAssertionExpression(assertion) = object {
        object = assertion.expression().ok()?;
    }
    AnyJsOptionalChainExpression::cast(object.into_syntax())
}

/// Returns whether `link` belongs to an optional chain: it or a link inside
/// it, through non-null assertions as [`inner_link`] finds them, has `?.`.
fn is_optional_chain(link: &AnyJsOptionalChainExpression) -> bool {
    let mut link = link.clone();
    loop {
        if link.is_optional() {
            return true;
        }
        match link.object().ok().and_then(|object| inner_link(&object)) {
            Some(inner) => link = inner,
            None => return false,
        }
    }
}

pub(super) fn has_operands(link: &AnyJsOptionalChainExpression) -> bool {
    match link {
        AnyJsOptionalChainExpression::JsStaticMemberExpression(_) => false,
        AnyJsOptionalChainExpression::JsComputedMemberExpression(_) => true,
        AnyJsOptionalChainExpression::JsCallExpression(call) => call
            .arguments()
            .is_ok_and(|arguments| !arguments.args().is_empty()),
    }
}

/// Returns whether `right`, the right operand of `=`, makes the assignment
/// compound-like, as in TypeScript. That is the case for a binary expression,
/// in parentheses or not, with a shift or arithmetic operator, as in
/// `x = x + 1`.
fn is_compound_like(right: &AnyJsExpression) -> bool {
    let AnyJsExpression::JsBinaryExpression(binary) = right.clone().omit_parentheses() else {
        return false;
    };
    binary.operator().is_ok_and(|operator| {
        matches!(
            operator,
            JsBinaryOperator::LeftShift
                | JsBinaryOperator::RightShift
                | JsBinaryOperator::UnsignedRightShift
                | JsBinaryOperator::Plus
                | JsBinaryOperator::Minus
                | JsBinaryOperator::Times
                | JsBinaryOperator::Divide
                | JsBinaryOperator::Remainder
                | JsBinaryOperator::Exponent
        )
    })
}

/// Returns whether `declarator` is part of a `var` declaration, whose
/// variables a read can reach before the declarator.
pub(super) fn is_var(declarator: &JsVariableDeclarator) -> bool {
    match declarator.declaration() {
        Some(AnyJsVariableDeclaration::JsVariableDeclaration(declaration)) => declaration.is_var(),
        Some(AnyJsVariableDeclaration::JsForVariableDeclaration(declaration)) => {
            declaration.is_var()
        }
        None => false,
    }
}

/// Returns the variable that `call` adds values to as an array, matching the
/// calls that TypeScript treats as array mutations. The callee must be
/// written as `.push` or `.unshift` on a variable, in parentheses or not, or
/// with `!`, as `a` in `a.push(v)`, `a?.unshift(v)`, or `(a)!.push(v)`.
/// TypeScript doesn't look at the type of the object or of the method.
pub(super) fn mutated_array(call: &JsCallExpression) -> Option<JsIdentifierExpression> {
    let AnyJsExpression::JsStaticMemberExpression(callee) = call.callee().ok()? else {
        return None;
    };
    let AnyJsName::JsName(name) = callee.member().ok()? else {
        return None;
    };
    let name = name.value_token().ok()?;
    if !matches!(name.text_trimmed(), "push" | "unshift") {
        return None;
    }
    array_variable(callee.object().ok()?)
}

/// Returns the variable that `expression` reads, without parentheses and
/// `!`, which TypeScript skips when it matches the object of an array
/// mutation with a variable.
fn array_variable(mut expression: AnyJsExpression) -> Option<JsIdentifierExpression> {
    loop {
        expression = match expression {
            AnyJsExpression::JsParenthesizedExpression(parenthesized) => {
                parenthesized.expression().ok()?
            }
            AnyJsExpression::TsNonNullAssertionExpression(asserted) => {
                asserted.expression().ok()?
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => return Some(identifier),
            _ => return None,
        };
    }
}

/// Returns whether `identifier` is the object of a `push` or `unshift` call
/// that [`mutated_array`] matches, or of the target of an element assignment
/// with `=` and any key, as `a` in `a[i] = v`. Other writes of an element,
/// as `a[0] += 1` or `[a[0]] = t`, add nothing to an array in TypeScript.
pub(super) fn is_array_mutation(identifier: &JsIdentifierExpression) -> bool {
    let mut object = identifier.syntax().clone();
    let parent = loop {
        let Some(parent) = object.parent() else {
            return false;
        };
        if JsParenthesizedExpression::can_cast(parent.kind())
            || TsNonNullAssertionExpression::can_cast(parent.kind())
        {
            object = parent;
        } else {
            break parent;
        }
    };
    if let Some(member) = JsComputedMemberAssignment::cast_ref(&parent) {
        return member
            .object()
            .is_ok_and(|member_object| *member_object.syntax() == object)
            && member
                .parent::<JsAssignmentExpression>()
                .is_some_and(|assignment| {
                    assignment.operator() == Ok(JsAssignmentOperator::Assign)
                        && assignment
                            .left()
                            .is_ok_and(|left| left.syntax() == member.syntax())
                });
    }
    JsStaticMemberExpression::cast(parent)
        .and_then(|member| member.parent::<JsCallExpression>())
        .and_then(|call| mutated_array(&call))
        .is_some_and(|mutated| mutated.syntax() == identifier.syntax())
}

/// Returns whether `expression` is an empty array literal without
/// parentheses. TypeScript doesn't count an empty array literal in
/// parentheses either.
fn is_empty_array_literal(expression: &AnyJsExpression) -> bool {
    matches!(expression, AnyJsExpression::JsArrayExpression(array) if array.elements().is_empty())
}

/// What the key of an element assignment target, as `0` in `a[0] = v`, tells
/// about the write to an array whose type evolves with its writes.
enum ElementKey {
    /// A number literal, so the value joins the element type, as TypeScript
    /// adds it when the key's type is a number.
    Number,
    /// A string literal or a template without substitutions, whose type is
    /// no number, so TypeScript adds nothing.
    String,
    /// A key whose type may or may not be a number.
    Other,
}

impl ElementKey {
    fn new(key: &AnyJsExpression) -> Self {
        match key.clone().omit_parentheses() {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(_),
            ) => Self::Number,
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(_),
            ) => Self::String,
            AnyJsExpression::JsTemplateExpression(template)
                if template.tag().is_none()
                    && template
                        .elements()
                        .iter()
                        .all(|element| element.as_js_template_element().is_none()) =>
            {
                Self::String
            }
            _ => Self::Other,
        }
    }
}

/// The reads that [`RootBuilder::reads_evolving`] looks for, and the nested
/// roots that it skips.
const EVOLVING_READ_KINDS: SyntaxKindSet<JsLanguage> =
    JsIdentifierExpression::KIND_SET.union(AnyJsControlFlowRoot::KIND_SET);

/// Limits the operands that [`value_callees`] visits, including the values
/// that the local functions it follows return.
const MAX_VALUE_STEPS: usize = 32;

/// Limits the nested calls of local functions without a return type
/// annotation through which [`value_callees`] follows returned values.
const MAX_LOCAL_CALL_DEPTH: usize = 4;

/// Why Biome may type a value narrower than TypeScript does, as
/// [`value_callees`] finds it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UntypedValue {
    /// The value can be the result of a call with an inferred return type
    /// ([`CalleeReturn::Inferred`]), and [`value_callees`] doesn't follow the
    /// values that the callee returns. Examples are a function expression
    /// called where it is defined and an element call as `a[0]()`.
    /// [`UntypedValue::Other`] takes precedence over this reason.
    ///
    /// Unlike [`UntypedValue::Other`], this reason still lets an assignment
    /// count as covered (see [`RootBuilder::is_covered`]). A covered
    /// assignment writes a [`FlowEvolving::Variable`] from a scope other than
    /// the variable's declaration. The variable's declared type already
    /// includes the type that Biome infers for such a value.
    InferredReturn,
    /// The value can be the result of `typeof`, or of a call to a local
    /// function that returns a value that reads a variable whose type evolves
    /// with its writes, or its operands exceed [`MAX_VALUE_STEPS`].
    Other,
}

/// Returns whether `expression`, outside the functions that it creates, reads
/// a variable whose type evolves with its writes, as `evolving` gives it for
/// the variable's declaration.
fn reads_evolving_variable(
    expression: &AnyJsExpression,
    model: &SemanticModel,
    mut evolving: impl FnMut(TextRange) -> FlowEvolving,
) -> bool {
    let mut events = expression.syntax().preorder_of_kinds(EVOLVING_READ_KINDS);
    while let Some(event) = events.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if AnyJsControlFlowRoot::can_cast(node.kind()) {
            events.skip_subtree();
            continue;
        }
        if let Some(binding) = JsIdentifierExpression::cast(node)
            .and_then(|identifier| identifier.name().ok())
            .and_then(|name| model.binding(&name))
            && evolving(binding.range()) != FlowEvolving::None
        {
            return true;
        }
    }
    false
}

/// Returns whether the declared type of `binding`, a variable whose type
/// evolves with its writes as a variable, includes the type that Biome gives
/// `right`, the value that an assignment with `=`, `??=`, `||=`, or `&&=` to
/// `identifier` writes. `reads_evolving` tells whether a value reads a
/// variable whose type evolves with its writes.
///
/// The declared type of such a variable joins its initializer with the values
/// of these assignments when they lie in a scope other than the one that
/// declares the variable, as `x = v` in `let x = null; if (c) { x = v; }`, so
/// the assignment must lie in such a scope. The value must not read a
/// variable whose type evolves with its writes, directly or through a value
/// that a local function it calls returns, and Biome must not type it
/// narrower than TypeScript for a reason other than a call to a function
/// whose return type Biome infers; see [`value_callees`].
fn covers_value(
    identifier: &JsIdentifierAssignment,
    binding: &Binding,
    right: &AnyJsExpression,
    model: &SemanticModel,
    mut reads_evolving: impl FnMut(&AnyJsExpression) -> bool,
) -> bool {
    model.scope(identifier.syntax()).id() != binding.scope().id()
        && !reads_evolving(right)
        && !matches!(
            value_callees(right, model, reads_evolving),
            Err(UntypedValue::Other)
        )
}

/// Returns whether the declared type of `binding`, a variable whose type
/// evolves with its writes as a variable, includes the value of every write
/// of it after its initializer. Each write must be an assignment with `=`,
/// `??=`, `||=`, or `&&=` to the variable, in parentheses, with `!`, or
/// neither, for which [`covers_value`] holds. The declared type may lack the
/// value of any other write, as a compound assignment, `++`, destructuring,
/// or the head of a `for...of` loop.
pub(super) fn covers_writes(binding: &Binding, model: &SemanticModel) -> bool {
    let mut evolving: FxHashMap<TextRange, FlowEvolving> = FxHashMap::default();
    binding.all_writes().all(|write| {
        let Some(identifier) = JsIdentifierAssignment::cast(write.syntax()) else {
            return false;
        };
        let Some(right) = assigned_operand(&identifier) else {
            return false;
        };
        covers_value(&identifier, binding, &right, model, |value| {
            reads_evolving_variable(value, model, |range| {
                *evolving.entry(range).or_insert_with(|| {
                    model
                        .as_binding_by_range(range)
                        .map_or(FlowEvolving::None, |binding| {
                            evolving_declaration(&binding, model)
                        })
                })
            })
        })
    })
}

/// Returns the value that an assignment with `=`, `??=`, `||=`, or `&&=`
/// writes to `identifier`, in parentheses, with `!`, or neither, or `None`
/// when `identifier` is the target of another write.
fn assigned_operand(identifier: &JsIdentifierAssignment) -> Option<AnyJsExpression> {
    let mut parent = identifier.syntax().parent()?;
    while JsParenthesizedAssignment::can_cast(parent.kind())
        || TsNonNullAssertionAssignment::can_cast(parent.kind())
    {
        parent = parent.parent()?;
    }
    let assignment = JsAssignmentExpression::cast(parent)?;
    matches!(
        assignment.operator().ok()?,
        JsAssignmentOperator::Assign
            | JsAssignmentOperator::NullishCoalescingAssign
            | JsAssignmentOperator::LogicalOrAssign
            | JsAssignmentOperator::LogicalAndAssign
    )
    .then(|| assignment.right().ok())
    .flatten()
}

/// Returns the callees that the reading context must confirm as annotated
/// before a write reduces by the value of `expression`, or an
/// [`UntypedValue`] when Biome may type the value narrower than TypeScript.
/// A write of such a value gives the subject its declared type, or Unknown
/// for an evolving variable, one whose type evolves with its writes.
/// `reads_evolving` tells whether a value reads an evolving variable; see
/// [`RootBuilder::reads_evolving`].
///
/// It visits the values that `expression` can give through:
///
/// - parentheses, `await`, and the last operand of the comma operator;
/// - the branches of a conditional and the operands of `&&`, `||`, and `??`;
/// - the objects along an optional chain, because `?.` gives `undefined`
///   only when Biome's type of its object has it;
/// - the returned values of a call when [`inferred_local_body`] finds the
///   callee's body, through at most [`MAX_LOCAL_CALL_DEPTH`] nested calls.
///   Biome infers the callee's return type from them as TypeScript does,
///   including the `undefined` of a reachable end.
///
/// A call adds its callee when [`callee_return`] gives
/// [`CalleeReturn::Resolved`]. When a call has an inferred return type and
/// its returned values aren't visited, it gives
/// [`UntypedValue::InferredReturn`]. `typeof` gives [`UntypedValue::Other`],
/// because Biome types it as the one kind of value that the operand's type
/// has, where TypeScript gives every kind. The variant lists its other cases.
fn value_callees(
    expression: &AnyJsExpression,
    model: &SemanticModel,
    mut reads_evolving: impl FnMut(&AnyJsExpression) -> bool,
) -> Result<Vec<FlowCallee>, UntypedValue> {
    let mut pending = vec![(expression.clone(), MAX_LOCAL_CALL_DEPTH)];
    let mut callees = Vec::new();
    let mut remaining = MAX_VALUE_STEPS;
    let mut inferred = false;
    while let Some((expression, depth)) = pending.pop() {
        remaining = remaining.checked_sub(1).ok_or(UntypedValue::Other)?;
        let operands = match expression {
            AnyJsExpression::JsParenthesizedExpression(parenthesized) => {
                [parenthesized.expression().ok(), None]
            }
            AnyJsExpression::JsConditionalExpression(conditional) => {
                [conditional.consequent().ok(), conditional.alternate().ok()]
            }
            AnyJsExpression::JsLogicalExpression(logical) => {
                [logical.left().ok(), logical.right().ok()]
            }
            AnyJsExpression::JsSequenceExpression(sequence) => [sequence.right().ok(), None],
            AnyJsExpression::JsAwaitExpression(await_expression) => {
                [await_expression.argument().ok(), None]
            }
            AnyJsExpression::JsUnaryExpression(unary)
                if unary.operator() == Ok(JsUnaryOperator::Typeof) =>
            {
                return Err(UntypedValue::Other);
            }
            AnyJsExpression::JsCallExpression(call) => {
                let callee = call.callee().map_err(|_| UntypedValue::Other)?;
                match callee_return(&callee, model) {
                    CalleeReturn::Annotated => {}
                    CalleeReturn::Inferred => match depth
                        .checked_sub(1)
                        .and_then(|depth| Some((depth, inferred_local_body(&callee, model)?)))
                    {
                        Some((depth, body)) => {
                            let values: Vec<_> = match body {
                                AnyJsFunctionBody::AnyJsExpression(value) => Vec::from([value]),
                                AnyJsFunctionBody::JsFunctionBody(body) => own_returns(&body)
                                    .filter_map(|statement| statement.argument())
                                    .collect(),
                            };
                            for value in values {
                                if reads_evolving(&value) {
                                    return Err(UntypedValue::Other);
                                }
                                pending.push((value, depth));
                            }
                        }
                        None => inferred = true,
                    },
                    CalleeReturn::Resolved(callee) => callees.push(callee),
                }
                [call.is_optional_chain().then_some(callee), None]
            }
            AnyJsExpression::JsStaticMemberExpression(member) if member.is_optional_chain() => {
                [member.object().ok(), None]
            }
            AnyJsExpression::JsComputedMemberExpression(member) if member.is_optional_chain() => {
                [member.object().ok(), None]
            }
            _ => continue,
        };
        pending.extend(
            operands
                .into_iter()
                .flatten()
                .map(|operand| (operand, depth)),
        );
    }
    if inferred {
        Err(UntypedValue::InferredReturn)
    } else {
        Ok(callees)
    }
}

/// Returns the body of the local function that `callee` names, when Biome
/// infers a call's return type from that body. The name comes from a
/// function declaration, or from a variable initialized with a function or
/// arrow expression and declared without a type annotation. The function
/// has no return type annotation and isn't a generator, because calling a
/// generator gives a generator object instead of its returned values.
/// Returns `None` for any other callee, such as a method, an import, or a
/// function expression called where it is defined.
fn inferred_local_body(
    callee: &AnyJsExpression,
    model: &SemanticModel,
) -> Option<AnyJsFunctionBody> {
    let AnyJsExpression::JsIdentifierExpression(identifier) = callee.clone().omit_parentheses()
    else {
        return None;
    };
    let binding = model.binding(&identifier.name().ok()?)?;
    let function = match binding.tree().declaration()? {
        AnyJsBindingDeclaration::JsFunctionDeclaration(function) => AnyJsFunction::from(function),
        AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(function) => {
            AnyJsFunction::from(function)
        }
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            if declarator.variable_annotation().is_some() {
                return None;
            }
            match declarator
                .initializer()?
                .expression()
                .ok()?
                .omit_parentheses()
            {
                AnyJsExpression::JsArrowFunctionExpression(function) => {
                    AnyJsFunction::from(function)
                }
                AnyJsExpression::JsFunctionExpression(function) => AnyJsFunction::from(function),
                _ => return None,
            }
        }
        _ => return None,
    };
    if function.return_type_annotation().is_some() || function.is_generator() {
        return None;
    }
    function.body().ok()
}

/// Where the return type of the callee of a call comes from, as far as the
/// syntax of one module tells.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CalleeReturn {
    /// An annotation that the syntax shows, or a declaration in the lib or in
    /// a declaration file, which has no function body to infer from.
    Annotated,
    /// A function body, or a type that the syntax doesn't show. A write of
    /// the call's value gives the subject its declared type, unless the
    /// callee is a local function that [`inferred_local_body`] finds. For
    /// such a callee, [`value_callees`] checks the values that it returns.
    Inferred,
    /// A declaration that the reading context finds.
    ///
    /// For an import, it is the declaration that the import names, in the
    /// module that declares it. That declaration is annotated in a
    /// declaration file, unless its type annotation is `typeof` of a
    /// variable, which counts as that variable does (see
    /// [`declared_return`]). Elsewhere it follows the rules for a local
    /// variable (see [`declaration_return`]).
    ///
    /// For a member, every type that its object can have must get the member
    /// from the lib, from a declaration file, or from methods with a return
    /// type annotation. Such a type is a primitive, a tuple, a lib global, a
    /// type that a declaration file declares, or a class or an interface that
    /// declares such methods or inherits them, without a merged interface
    /// that another module declares. [`member_return`] finds the methods in a
    /// class or an interface. A property that a declaration file types as
    /// `typeof` of a variable counts as that variable does (see
    /// [`declared_member_returns`]).
    Resolved(FlowCallee),
}

/// Returns where the return type of `callee`, the callee of a call, comes
/// from:
///
/// - A function or arrow expression is annotated when it has a return type
///   annotation.
/// - A variable that the module doesn't declare is a global that the lib or
///   a declaration file declares, so it is annotated.
/// - A variable that the module declares depends on its declaration; see
///   [`declaration_return`].
/// - A member, as `a.m`, depends on the types that its object can have, so
///   the reading context resolves it.
///
/// Other callees are inferred, including an element, as `a[0]`, whose type
/// can come from a type argument, as in `[g][0]()`.
pub(crate) fn callee_return(callee: &AnyJsExpression, model: &SemanticModel) -> CalleeReturn {
    match callee.clone().omit_parentheses() {
        AnyJsExpression::JsArrowFunctionExpression(function) => {
            annotated_if(function.return_type_annotation().is_some())
        }
        AnyJsExpression::JsFunctionExpression(function) => {
            annotated_if(function.return_type_annotation().is_some())
        }
        AnyJsExpression::JsIdentifierExpression(identifier) => {
            let Ok(name) = identifier.name() else {
                return CalleeReturn::Inferred;
            };
            model
                .binding(&name)
                .map_or(CalleeReturn::Annotated, |binding| {
                    declaration_return(&binding)
                })
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            CalleeReturn::Resolved(FlowCallee::Member(member.range()))
        }
        _ => CalleeReturn::Inferred,
    }
}

declare_node_union! {
    /// A class or an interface, whose own members [`member_return`] reads.
    pub(crate) AnyMemberDeclaration = AnyJsClass | TsInterfaceDeclaration
}

/// Returns how many members named `name` `owner` declares itself, and where
/// the return type of a call to them comes from: [`CalleeReturn::Annotated`]
/// when each of them is a method that isn't `static` and has a return type
/// annotation, and [`CalleeReturn::Inferred`] otherwise, as for a property,
/// an accessor, or a method without an annotation. A private name is spelled
/// with its `#`, as local inference stores member names.
pub(crate) fn member_return(owner: &AnyMemberDeclaration, name: &str) -> (usize, CalleeReturn) {
    let mut declared = 0;
    let mut annotated = true;
    match owner {
        AnyMemberDeclaration::AnyJsClass(class) => {
            for member in class.members() {
                let is_named = member
                    .name()
                    .ok()
                    .flatten()
                    .and_then(|member_name| member_name.name())
                    .is_some_and(|member_name| match member_name {
                        ClassMemberName::Private(own) => name.strip_prefix('#') == Some(own.text()),
                        ClassMemberName::Public(own) => own.text() == name,
                    });
                if !is_named {
                    continue;
                }
                declared += 1;
                annotated &= match member {
                    AnyJsClassMember::JsMethodClassMember(method) => {
                        method.return_type_annotation().is_some()
                            && !method
                                .modifiers()
                                .iter()
                                .any(|modifier| modifier.as_js_static_modifier().is_some())
                    }
                    AnyJsClassMember::TsMethodSignatureClassMember(method) => {
                        method.return_type_annotation().is_some()
                            && !method
                                .modifiers()
                                .iter()
                                .any(|modifier| modifier.as_js_static_modifier().is_some())
                    }
                    _ => false,
                };
            }
        }
        AnyMemberDeclaration::TsInterfaceDeclaration(interface) => {
            for member in interface.members() {
                let (member_name, is_annotated) = match member {
                    AnyTsTypeMember::TsMethodSignatureTypeMember(method) => {
                        (method.name(), method.return_type_annotation().is_some())
                    }
                    AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
                        (property.name(), false)
                    }
                    AnyTsTypeMember::TsGetterSignatureTypeMember(getter) => (getter.name(), false),
                    AnyTsTypeMember::TsSetterSignatureTypeMember(setter) => (setter.name(), false),
                    _ => continue,
                };
                if member_name
                    .ok()
                    .and_then(|member_name| member_name.name())
                    .is_some_and(|member_name| member_name.text() == name)
                {
                    declared += 1;
                    annotated &= is_annotated;
                }
            }
        }
    }
    (declared, annotated_if(annotated))
}

/// Returns how many members named `name` `owner` declares itself, and
/// whether each of them has an explicit type, which TypeScript requires of a
/// member along the callee of a statement call. Only the `static` members
/// count when `is_static` is true, and only the others otherwise.
///
/// A method, and a property with a type annotation, have an explicit type.
/// An accessor and a property without a type annotation have none. A private
/// name is spelled with its `#`, as local inference stores member names.
pub(crate) fn member_explicitness(
    owner: &AnyMemberDeclaration,
    name: &str,
    is_static: bool,
) -> (usize, bool) {
    let mut declared = 0;
    let mut explicit = true;
    match owner {
        AnyMemberDeclaration::AnyJsClass(class) => {
            for member in class.members() {
                let is_named = member
                    .name()
                    .ok()
                    .flatten()
                    .and_then(|member_name| member_name.name())
                    .is_some_and(|member_name| match member_name {
                        ClassMemberName::Private(own) => name.strip_prefix('#') == Some(own.text()),
                        ClassMemberName::Public(own) => own.text() == name,
                    });
                if !is_named {
                    continue;
                }
                let (member_is_static, member_is_explicit) = match &member {
                    AnyJsClassMember::JsMethodClassMember(method) => (
                        method
                            .modifiers()
                            .iter()
                            .any(|modifier| modifier.as_js_static_modifier().is_some()),
                        true,
                    ),
                    AnyJsClassMember::TsMethodSignatureClassMember(method) => (
                        method
                            .modifiers()
                            .iter()
                            .any(|modifier| modifier.as_js_static_modifier().is_some()),
                        true,
                    ),
                    AnyJsClassMember::JsPropertyClassMember(property) => (
                        property
                            .modifiers()
                            .iter()
                            .any(|modifier| modifier.as_js_static_modifier().is_some()),
                        property.property_annotation().is_some_and(|annotation| {
                            matches!(annotation.type_annotation(), Ok(Some(_)))
                        }),
                    ),
                    AnyJsClassMember::TsPropertySignatureClassMember(property) => (
                        property
                            .modifiers()
                            .iter()
                            .any(|modifier| modifier.as_js_static_modifier().is_some()),
                        property.property_annotation().is_some_and(|annotation| {
                            matches!(annotation.type_annotation(), Ok(Some(_)))
                        }),
                    ),
                    // An accessor, or a member whose staticness Biome doesn't
                    // read, counts as declared without an explicit type.
                    _ => (is_static, false),
                };
                if member_is_static != is_static {
                    continue;
                }
                declared += 1;
                explicit &= member_is_explicit;
            }
        }
        AnyMemberDeclaration::TsInterfaceDeclaration(interface) => {
            if is_static {
                return (0, false);
            }
            for member in interface.members() {
                let (member_name, member_is_explicit) = match member {
                    AnyTsTypeMember::TsMethodSignatureTypeMember(method) => (method.name(), true),
                    AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
                        (property.name(), property.type_annotation().is_some())
                    }
                    AnyTsTypeMember::TsGetterSignatureTypeMember(getter) => (getter.name(), false),
                    AnyTsTypeMember::TsSetterSignatureTypeMember(setter) => (setter.name(), false),
                    _ => continue,
                };
                if member_name
                    .ok()
                    .and_then(|member_name| member_name.name())
                    .is_some_and(|member_name| member_name.text() == name)
                {
                    declared += 1;
                    explicit &= member_is_explicit;
                }
            }
        }
    }
    (declared, explicit)
}

/// Returns where the return type of a call to the variable that `binding`
/// declares comes from: the return type annotation of a function
/// declaration, or of the function or arrow expression that initializes a
/// variable without a type annotation; the function type that annotates a
/// variable or a parameter; or, for an import, the declaration it names, as
/// [`CalleeReturn::Resolved`] describes. Other declarations are inferred.
pub(crate) fn declaration_return(binding: &Binding) -> CalleeReturn {
    match binding.tree().declaration() {
        Some(AnyJsBindingDeclaration::JsFunctionDeclaration(function)) => {
            annotated_if(function.return_type_annotation().is_some())
        }
        Some(AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(function)) => {
            annotated_if(function.return_type_annotation().is_some())
        }
        Some(AnyJsBindingDeclaration::JsFunctionExpression(function)) => {
            annotated_if(function.return_type_annotation().is_some())
        }
        Some(
            AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_)
            | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_),
        ) => CalleeReturn::Annotated,
        Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) => {
            if let Some(annotation) = declarator.variable_annotation() {
                return annotated_if(is_function_type(
                    annotation.type_annotation().ok().flatten(),
                ));
            }
            let initializer = declarator
                .initializer()
                .and_then(|initializer| initializer.expression().ok())
                .map(AnyJsExpression::omit_parentheses);
            match initializer {
                Some(AnyJsExpression::JsArrowFunctionExpression(function)) => {
                    annotated_if(function.return_type_annotation().is_some())
                }
                Some(AnyJsExpression::JsFunctionExpression(function)) => {
                    annotated_if(function.return_type_annotation().is_some())
                }
                _ => CalleeReturn::Inferred,
            }
        }
        Some(AnyJsBindingDeclaration::JsFormalParameter(parameter)) => {
            annotated_if(is_function_type(parameter.type_annotation()))
        }
        Some(
            AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_),
        ) => CalleeReturn::Resolved(FlowCallee::Import(binding.range())),
        _ => CalleeReturn::Inferred,
    }
}

fn annotated_if(annotated: bool) -> CalleeReturn {
    if annotated {
        CalleeReturn::Annotated
    } else {
        CalleeReturn::Inferred
    }
}

/// Returns whether `annotation` is a function type, as `() => number`,
/// whose return type the syntax shows.
fn is_function_type(annotation: Option<TsTypeAnnotation>) -> bool {
    annotation
        .and_then(|annotation| annotation.ty().ok())
        .is_some_and(|ty| matches!(ty, AnyTsType::TsFunctionType(_)))
}

/// What the type annotation of a value in a declaration file tells about the
/// return type of a call of the value. A declaration file has no function
/// bodies, so the return type comes from an annotation, unless the value's
/// annotation is `typeof` of another value: a call then returns what a call
/// of that value returns, which can come from a function body elsewhere.
pub(crate) enum DeclaredReturn {
    /// The annotation isn't such a query, so it states the return type.
    Stated,
    /// The annotation is `typeof` of this variable, as `f` in `typeof f`.
    Queried(JsReferenceIdentifier),
    /// The annotation is `typeof` of a qualified name or of an import, as in
    /// `typeof ns.f` and `typeof import("./a")`, which Biome doesn't follow.
    Unfollowed,
}

impl DeclaredReturn {
    fn from_annotation(annotation: Option<TsTypeAnnotation>) -> Self {
        let mut ty = annotation.and_then(|annotation| annotation.ty().ok());
        while let Some(AnyTsType::TsParenthesizedType(parenthesized)) = &ty {
            ty = parenthesized.ty().ok();
        }
        match ty {
            Some(AnyTsType::TsTypeofType(query)) => match query.expression_name() {
                Ok(AnyTsName::JsReferenceIdentifier(name)) => Self::Queried(name),
                Ok(AnyTsName::TsQualifiedName(_)) | Err(_) => Self::Unfollowed,
            },
            Some(AnyTsType::TsImportType(import)) if import.typeof_token().is_some() => {
                Self::Unfollowed
            }
            _ => Self::Stated,
        }
    }
}

/// Returns what the type annotation of the variable that `binding` declares
/// tells about the return type of a call of the variable, for a declaration
/// in a declaration file. A declaration other than a variable states its
/// return type.
pub(crate) fn declared_return(binding: &Binding) -> DeclaredReturn {
    let Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) =
        binding.tree().declaration()
    else {
        return DeclaredReturn::Stated;
    };
    DeclaredReturn::from_annotation(
        declarator
            .variable_annotation()
            .and_then(|annotation| annotation.type_annotation().ok().flatten()),
    )
}

/// Returns what the type annotations of the properties named `name` that
/// `owner` declares itself tell about the return type of a call of the
/// property, for a class or an interface in a declaration file. A method
/// states its return type, so only properties count, `static` or not. A
/// private name is spelled with its `#`, as local inference stores member
/// names.
pub(crate) fn declared_member_returns(
    owner: &AnyMemberDeclaration,
    name: &str,
) -> Vec<DeclaredReturn> {
    let mut returns = Vec::new();
    match owner {
        AnyMemberDeclaration::AnyJsClass(class) => {
            for member in class.members() {
                let is_named = member
                    .name()
                    .ok()
                    .flatten()
                    .and_then(|member_name| member_name.name())
                    .is_some_and(|member_name| match member_name {
                        ClassMemberName::Private(own) => name.strip_prefix('#') == Some(own.text()),
                        ClassMemberName::Public(own) => own.text() == name,
                    });
                if !is_named {
                    continue;
                }
                let annotation = match member {
                    AnyJsClassMember::JsPropertyClassMember(property) => property
                        .property_annotation()
                        .and_then(|annotation| annotation.type_annotation().ok().flatten()),
                    AnyJsClassMember::TsPropertySignatureClassMember(property) => property
                        .property_annotation()
                        .and_then(|annotation| annotation.type_annotation().ok().flatten()),
                    _ => continue,
                };
                returns.push(DeclaredReturn::from_annotation(annotation));
            }
        }
        AnyMemberDeclaration::TsInterfaceDeclaration(interface) => {
            for member in interface.members() {
                let AnyTsTypeMember::TsPropertySignatureTypeMember(property) = member else {
                    continue;
                };
                if property
                    .name()
                    .ok()
                    .and_then(|member_name| member_name.name())
                    .is_some_and(|member_name| member_name.text() == name)
                {
                    returns.push(DeclaredReturn::from_annotation(property.type_annotation()));
                }
            }
        }
    }
    returns
}

declare_node_union! {
    /// A `for...in` or `for...of` statement, which runs its body once for
    /// each key or element of the value of its expression.
    AnyJsForInOrOfStatement = JsForInStatement | JsForOfStatement
}

impl AnyJsForInOrOfStatement {
    /// Returns the head of the loop, such as `const k` in `for (const k in o)`.
    fn initializer(&self) -> SyntaxResult<AnyJsForInOrOfInitializer> {
        match self {
            Self::JsForInStatement(statement) => statement.initializer(),
            Self::JsForOfStatement(statement) => statement.initializer(),
        }
    }

    fn expression(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            Self::JsForInStatement(statement) => statement.expression(),
            Self::JsForOfStatement(statement) => statement.expression(),
        }
    }

    fn body(&self) -> SyntaxResult<AnyJsStatement> {
        match self {
            Self::JsForInStatement(statement) => statement.body(),
            Self::JsForOfStatement(statement) => statement.body(),
        }
    }

    /// Returns the value that each iteration assigns to an assignment target
    /// in the head, given the loop's `expression`: a key of a `for...in`
    /// loop, or an element of the iterable of a `for...of` loop, or `None`
    /// for `for await`, whose elements Biome doesn't await.
    fn assigned_value(&self, expression: &AnyJsExpression) -> Option<FlowAssignedValue> {
        match self {
            Self::JsForInStatement(_) => Some(FlowAssignedValue::Key),
            Self::JsForOfStatement(statement) => statement
                .await_token()
                .is_none()
                .then(|| FlowAssignedValue::IterableValue(expression.range())),
        }
    }

    /// Returns whether the body assumes that the expression is not nullish.
    /// Only a `for...in` loop whose head declares one variable, such as
    /// `const k`, `let k`, or `var k`, makes that assumption.
    ///
    /// As in TypeScript, an assignment target, as in `for (k in o)`, narrows
    /// nothing. Neither does a destructuring pattern, which `for...in`
    /// rejects (TS2491).
    fn assumes_non_nullish(&self) -> bool {
        let Self::JsForInStatement(statement) = self else {
            return false;
        };
        let Ok(AnyJsForInOrOfInitializer::JsForVariableDeclaration(declaration)) =
            statement.initializer()
        else {
            return false;
        };
        declaration
            .declarator()
            .and_then(|declarator| declarator.id())
            .is_ok_and(|id| matches!(id, AnyJsBindingPattern::AnyJsBinding(_)))
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
    call_mentions: FxHashSet<(FlowNodeId, FlowSubjectId)>,
    reads: Vec<BuiltRead>,
    switches: Vec<BuiltSwitch>,
    writes: Vec<FlowWrite>,
    /// The index of each write in `writes`, so that a write that a second
    /// build of a finally block meets again keeps its index.
    write_ids: FxHashMap<FlowWrite, u32>,
    assigned: Vec<FlowAssigned>,
    /// The index of each value in `assigned`.
    assigned_ids: FxHashMap<FlowAssigned, u32>,
    /// The callees that the reading context must confirm for each value and
    /// default that a write assigns, by the range of its expression, as
    /// [`Self::check_value`] records them.
    value_callees: FxHashMap<TextRange, Box<[FlowCallee]>>,
    written: WrittenTargets,
    captures: Vec<BuiltCapture>,
    /// How the type of each variable met so far evolves with its writes, by
    /// the range of its declaration.
    evolving: FxHashMap<TextRange, FlowEvolving>,
    /// Whether each value checked so far reads a variable whose type evolves
    /// with its writes, by the value's range.
    evolving_values: FxHashMap<TextRange, bool>,
    initializers: FxHashSet<FlowNodeId>,
    hoisted: FxHashSet<FlowSubjectId>,
    /// The number of write points added so far, which tells whether an
    /// expression contains one.
    write_points: usize,
    jumps: Vec<JumpTarget>,
    /// The joins that exceptions reach, one for each enclosing try block or
    /// catch clause, innermost last. Each join is the start of the catch clause
    /// or finally block that handles an exception thrown inside its block.
    ///
    /// Each join starts with the flow where its block starts. It gains each
    /// write point inside the block, as in TypeScript, and the end of each
    /// finally block nested in the block. Conditions only narrow, so no other
    /// point inside the block is narrower than one of these.
    exceptions: Vec<FlowNodeId>,
    /// The flows that reach a finally block from a `return`, one list for each
    /// enclosing `try` statement with a finally block, innermost last.
    ///
    /// A list holds the flow at each `return` in the statement's try block or
    /// catch clause, because the finally block runs before the function
    /// returns. It also holds the end of each finally block nested there that
    /// such a `return` reached.
    returns: Vec<Vec<FlowNodeId>>,
    /// Whether evaluation records reads. It is false while the builder builds
    /// a finally block a second time, because the first build recorded them.
    records_reads: bool,
    /// The statements met so far with their incoming flow, for
    /// [`BuiltRoot::statements`], or `None` when the build doesn't record
    /// them.
    statements: Option<Vec<(TextRange, Option<FlowNodeId>)>>,
    /// The incoming flow of the next statement, or `None` when unreachable.
    current: Option<FlowNodeId>,
    /// The visits left before the build gives up. Every statement and every
    /// evaluated node spends one each time the builder reaches it, so the
    /// limit also bounds the builds of finally blocks.
    remaining_visits: usize,
}

impl RootBuilder<'_, '_> {
    fn root(&mut self, root: &AnyJsControlFlowRoot) -> Build<()> {
        let start = self.parameters(root, 0)?;
        if let AnyJsControlFlowRoot::AnyJsFunction(AnyJsFunction::JsArrowFunctionExpression(arrow)) =
            root
            && let AnyJsFunctionBody::AnyJsExpression(body) =
                arrow.body().map_err(|_| Unsupported)?
        {
            self.current = Some(self.evaluate(body.syntax(), start, 0)?);
            return Ok(());
        }
        self.current = Some(start);
        self.statements(root.syntax())
    }

    /// Evaluates the parameters of `root` in order from `incoming`, because
    /// they run before its body, and returns the flow after them.
    ///
    /// Each parameter evaluates its default, then its destructuring pattern,
    /// so `x.length` in `(x, y = x && x.length)` sees the test of `x`. A
    /// default runs only when the argument is `undefined`. The decorators of a
    /// parameter run where its class is defined, not where the function is
    /// called, so they are not evaluated.
    fn parameters(
        &mut self,
        root: &AnyJsControlFlowRoot,
        incoming: FlowNodeId,
    ) -> Build<FlowNodeId> {
        let parameters = match root {
            AnyJsControlFlowRoot::AnyJsFunction(function) => match function.parameters() {
                Ok(AnyJsArrowFunctionParameters::JsParameters(parameters)) => parameters,
                Ok(AnyJsArrowFunctionParameters::AnyJsBinding(_)) | Err(_) => return Ok(incoming),
            },
            AnyJsControlFlowRoot::JsMethodObjectMember(method) => match method.parameters() {
                Ok(parameters) => parameters,
                Err(_) => return Ok(incoming),
            },
            AnyJsControlFlowRoot::JsMethodClassMember(method) => match method.parameters() {
                Ok(parameters) => parameters,
                Err(_) => return Ok(incoming),
            },
            AnyJsControlFlowRoot::JsConstructorClassMember(constructor) => {
                let Ok(parameters) = constructor.parameters() else {
                    return Ok(incoming);
                };
                let mut flow = incoming;
                for parameter in parameters.parameters().iter().flatten() {
                    flow = match parameter {
                        AnyJsConstructorParameter::AnyJsFormalParameter(parameter) => {
                            self.formal_parameter(&parameter, flow)?
                        }
                        AnyJsConstructorParameter::JsRestParameter(parameter) => {
                            self.binding_element(parameter.binding().ok(), None, flow, 1)?
                        }
                        AnyJsConstructorParameter::TsPropertyParameter(parameter) => {
                            match parameter.formal_parameter() {
                                Ok(parameter) => self.formal_parameter(&parameter, flow)?,
                                Err(_) => flow,
                            }
                        }
                    };
                }
                return Ok(flow);
            }
            AnyJsControlFlowRoot::JsSetterObjectMember(setter) => {
                return match setter.parameter() {
                    Ok(parameter) => self.formal_parameter(&parameter, incoming),
                    Err(_) => Ok(incoming),
                };
            }
            AnyJsControlFlowRoot::JsSetterClassMember(setter) => {
                return match setter.parameter() {
                    Ok(parameter) => self.formal_parameter(&parameter, incoming),
                    Err(_) => Ok(incoming),
                };
            }
            AnyJsControlFlowRoot::JsModule(_)
            | AnyJsControlFlowRoot::JsScript(_)
            | AnyJsControlFlowRoot::JsGetterObjectMember(_)
            | AnyJsControlFlowRoot::JsGetterClassMember(_)
            | AnyJsControlFlowRoot::JsStaticInitializationBlockClassMember(_)
            | AnyJsControlFlowRoot::TsModuleDeclaration(_)
            | AnyJsControlFlowRoot::TsPropertySignatureTypeMember(_) => return Ok(incoming),
        };
        self.parameter_list(&parameters, incoming)
    }

    /// Evaluates `parameters` in order from `incoming` and returns the flow
    /// after them.
    fn parameter_list(
        &mut self,
        parameters: &JsParameters,
        incoming: FlowNodeId,
    ) -> Build<FlowNodeId> {
        let mut flow = incoming;
        for parameter in parameters.items().iter().flatten() {
            flow = match parameter {
                AnyJsParameter::AnyJsFormalParameter(parameter) => {
                    self.formal_parameter(&parameter, flow)?
                }
                AnyJsParameter::JsRestParameter(parameter) => {
                    self.binding_element(parameter.binding().ok(), None, flow, 1)?
                }
                AnyJsParameter::TsThisParameter(_) => flow,
            };
        }
        Ok(flow)
    }

    /// Evaluates the default of `parameter`, then its destructuring pattern,
    /// from `incoming`, and returns the flow after them.
    fn formal_parameter(
        &mut self,
        parameter: &AnyJsFormalParameter,
        incoming: FlowNodeId,
    ) -> Build<FlowNodeId> {
        match parameter {
            AnyJsFormalParameter::JsFormalParameter(parameter) => self.binding_element(
                parameter.binding().ok(),
                parameter.initializer(),
                incoming,
                1,
            ),
            AnyJsFormalParameter::JsBogusParameter(_) | AnyJsFormalParameter::JsMetavariable(_) => {
                Err(Unsupported)
            }
        }
    }

    /// Builds the statements found below `node`, without entering
    /// expressions, types, or nested execution roots.
    ///
    /// A statement whose evaluation reaches only a join without
    /// predecessors, as after a function called in place that always throws,
    /// ends the path, so the statements after it are unreachable.
    fn statements(&mut self, node: &JsSyntaxNode) -> Build<()> {
        for child in node.children() {
            if NON_STATEMENT_KINDS.matches(child.kind()) {
                continue;
            }
            match AnyJsStatement::cast_ref(&child) {
                Some(statement) => self.statement(statement, None)?,
                None => self.statements(&child)?,
            }
            if let Some(current) = self.current
                && matches!(self.nodes.get(current as usize), Some(FlowNode::Join(predecessors)) if predecessors.is_empty())
            {
                self.current = None;
            }
        }
        Ok(())
    }

    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "Every other statement is an opaque region."
    )]
    fn statement(&mut self, statement: AnyJsStatement, label: Option<TokenText>) -> Build<()> {
        // A statement that evaluates nothing, such as a type alias or
        // unreachable code, also spends a visit, because a finally block is
        // built more than once.
        self.visit(0)?;
        if let Some(statements) = &mut self.statements {
            statements.push((statement.range(), self.current));
        }
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
                self.visit(0)?;
                let expression = statement.expression().map_err(|_| Unsupported)?;
                self.current = self.statement_calls(&expression, current, 1)?;
                Ok(())
            }
            AnyJsStatement::JsDebuggerStatement(_) | AnyJsStatement::JsEmptyStatement(_) => {
                self.current = Some(self.evaluate(statement.syntax(), current, 0)?);
                Ok(())
            }
            AnyJsStatement::JsVariableStatement(variable) => {
                let declaration = variable.declaration().map_err(|_| Unsupported)?;
                self.current = Some(self.declaration(&declaration.into(), current, 0)?);
                Ok(())
            }
            AnyJsStatement::JsReturnStatement(_) => {
                let flow = self.evaluate(statement.syntax(), current, 0)?;
                if let Some(returns) = self.returns.last_mut() {
                    returns.push(flow);
                }
                self.current = None;
                Ok(())
            }
            // Inside a try block or catch clause, the exception needs no edge
            // to the innermost join in `exceptions`. That join already holds
            // the start of the block, which is never narrower than the flow at
            // the `throw`.
            AnyJsStatement::JsThrowStatement(_) => {
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
            AnyJsStatement::JsForInStatement(statement) => {
                self.loop_statement(&statement.into(), current, label)
            }
            AnyJsStatement::JsForOfStatement(statement) => {
                self.loop_statement(&statement.into(), current, label)
            }
            AnyJsStatement::JsSwitchStatement(statement) => {
                self.switch_statement(&statement, current, label)
            }
            // A label on a `try` statement is not a jump target. A root with a
            // `break` to such a label has no flow, because
            // `has_complete_statements` rejects it before the build.
            AnyJsStatement::JsTryStatement(statement) => {
                self.try_statement(&statement.into(), current)
            }
            AnyJsStatement::JsTryFinallyStatement(statement) => {
                self.try_statement(&statement.into(), current)
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

    /// Evaluates `expression`, the expression of an expression statement at
    /// `depth`, from `incoming`, and returns the flow after it.
    ///
    /// Each statement call adds a condition node right after it, which holds
    /// what the call establishes when it returns (see
    /// [`decompose_statement_call`]), so the call's own arguments still see
    /// the types from before it. The statement calls are those that
    /// TypeScript adds to the flow: the expression itself when it is a call,
    /// and each operand of the comma operator that is a call, through any
    /// parentheses around the comma expression. [`Self::evaluate`] adds the
    /// operands of a comma expression elsewhere, as in `return (assert(x), x)`.
    /// A call in parentheses, as in `(assert(x));`, is none. The operands run
    /// in order, so a write in an operand comes before the next operand.
    ///
    /// Returns `None` when a call of a local function that never returns ends
    /// the path, so the operands after it don't run. A call point whose
    /// signature never returns ends the path only where inference finds that
    /// signature.
    fn statement_calls(
        &mut self,
        expression: &AnyJsExpression,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<Option<FlowNodeId>> {
        match expression {
            AnyJsExpression::JsSequenceExpression(sequence) => {
                self.visit(depth)?;
                let left = sequence.left().map_err(|_| Unsupported)?;
                let right = sequence.right().map_err(|_| Unsupported)?;
                let Some(flow) = self.statement_calls(&left, incoming, depth + 1)? else {
                    return Ok(None);
                };
                self.statement_calls(&right, flow, depth + 1)
            }
            AnyJsExpression::JsParenthesizedExpression(parenthesized)
                if matches!(
                    parenthesized
                        .expression()
                        .map(AnyJsExpression::omit_parentheses),
                    Ok(AnyJsExpression::JsSequenceExpression(_))
                ) =>
            {
                self.visit(depth)?;
                let inner = parenthesized.expression().map_err(|_| Unsupported)?;
                self.statement_calls(&inner, incoming, depth + 1)
            }
            AnyJsExpression::JsCallExpression(call) => {
                let flow = self.evaluate(call.syntax(), incoming, depth)?;
                if self.functions.local_call_ends(call, self.model) {
                    return Ok(None);
                }
                match decompose_statement_call(
                    call,
                    self.model,
                    self.functions,
                    &mut self.subjects,
                    &mut self.guards,
                ) {
                    Some(condition) => self.push_condition(flow, condition).map(Some),
                    None => Ok(Some(flow)),
                }
            }
            _ => self
                .evaluate(expression.syntax(), incoming, depth)
                .map(Some),
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
        let write_points = self.write_points;
        self.current = Some(self.evaluate(statement.syntax(), current, 0)?);
        if !check.escaping.is_empty() && self.write_points != write_points {
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
        Ok(())
    }

    /// Evaluates `function`, which a call runs inline (see
    /// [`AnyInlineCode::runs_inline`]), from `incoming`, the flow where the
    /// call runs it, and returns the flow after the call: the end of its body
    /// joined with each `return`. Returns `None` when no path leaves the
    /// body, as when every path throws. `depth` is the depth of the call.
    ///
    /// The parameters run first, as for a root, and then the body. A `throw`
    /// reaches the joins of the `try` statements around the call, and each
    /// write point joins them, as anywhere in the root. Each build of the
    /// body, as in a finally block that is built twice, has its own return
    /// join, and the jump targets and the finally blocks around the call
    /// never apply inside the body.
    fn inline_body(
        &mut self,
        function: &AnyJsFunction,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<Option<FlowNodeId>> {
        let root = AnyJsControlFlowRoot::AnyJsFunction(function.clone());
        if !has_complete_statements(&root) {
            return Err(Unsupported);
        }
        let jumps = std::mem::take(&mut self.jumps);
        let returns = std::mem::replace(&mut self.returns, vec![Vec::new()]);
        let current = self.current;
        let start = self.parameters(&root, incoming)?;
        let end = match function.body().map_err(|_| Unsupported)? {
            AnyJsFunctionBody::AnyJsExpression(body) => {
                Some(self.evaluate(body.syntax(), start, depth + 1)?)
            }
            AnyJsFunctionBody::JsFunctionBody(body) => {
                self.current = Some(start);
                self.statements(body.syntax())?;
                self.current
            }
        };
        let returned = std::mem::replace(&mut self.returns, returns)
            .pop()
            .ok_or(Unsupported)?;
        self.jumps = jumps;
        self.current = current;
        self.join(end.into_iter().chain(returned))
    }

    /// Evaluates the statements of `block`, a static block that runs inline
    /// where its class is evaluated, from `incoming`, and returns the flow
    /// after them, or `None` when no path leaves them.
    fn static_block(
        &mut self,
        block: &JsStaticInitializationBlockClassMember,
        incoming: FlowNodeId,
    ) -> Build<Option<FlowNodeId>> {
        if !has_complete_statements(&block.clone().into()) {
            return Err(Unsupported);
        }
        let jumps = std::mem::take(&mut self.jumps);
        let returns = std::mem::take(&mut self.returns);
        let current = self.current.replace(incoming);
        self.statements(block.syntax())?;
        let end = std::mem::replace(&mut self.current, current);
        self.returns = returns;
        self.jumps = jumps;
        Ok(end)
    }

    /// Returns `end`, the flow after inline code inside an expression, or,
    /// when no path leaves that code, a join without predecessors, which no
    /// value reaches. [`Self::statements`] ends the path at such a join.
    fn inline_end(&mut self, end: Option<FlowNodeId>) -> Build<FlowNodeId> {
        match end {
            Some(end) => Ok(end),
            None => self.push(FlowNode::Join(Vec::new())),
        }
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
        let positive = self.branch(flow, &test, FlowOutcome::Truthy)?;
        let negative = self.branch(flow, &test, FlowOutcome::Falsy)?;
        self.current = positive;
        if let Ok(consequent) = statement.consequent() {
            self.statement(consequent, None)?;
        }
        let consequent_end = self.current;
        self.current = negative;
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
        self.current = self.branch(flow, &test, FlowOutcome::Truthy)?;
        let exit = self.branch(flow, &test, FlowOutcome::Falsy)?;
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
            if let Some(repeat) = self.branch(flow, &test, FlowOutcome::Truthy)? {
                self.add_predecessor(body_entry, repeat)?;
            }
            exit = self.branch(flow, &test, FlowOutcome::Falsy)?;
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
        let flow = match statement.initializer() {
            Some(AnyJsForInitializer::JsVariableDeclaration(declaration)) => {
                self.declaration(&declaration.into(), current, 0)?
            }
            Some(AnyJsForInitializer::AnyJsExpression(expression)) => {
                self.evaluate(expression.syntax(), current, 0)?
            }
            None => current,
        };
        let header = self.push(FlowNode::Join(vec![flow]))?;
        let update_entry = self.push(FlowNode::Join(Vec::new()))?;
        let exit = match statement.test() {
            Some(test) => {
                let flow = self.evaluate(test.syntax(), header, 0)?;
                self.current = self.branch(flow, &test, FlowOutcome::Truthy)?;
                self.branch(flow, &test, FlowOutcome::Falsy)?
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

    /// Builds `statement` from `current`, with the control flow that
    /// TypeScript gives `for...in` and `for...of` loops. `label` is the label
    /// of `statement`, if any.
    ///
    /// The expression runs once, before the loop. Every iteration starts at
    /// the loop's header and evaluates the head from there, including the
    /// defaults of a destructuring pattern. The end of the body and each
    /// `continue` that targets the loop return to the header. The header is
    /// also the exit. The code after the loop continues from the header and
    /// from each `break` that targets the loop, so a test in the body never
    /// narrows the code after the loop. In a `for...in` loop whose head
    /// declares one variable, the body also assumes that the expression and
    /// the receivers of its optional chain are not nullish. See
    /// [`decompose_for_in`].
    fn loop_statement(
        &mut self,
        statement: &AnyJsForInOrOfStatement,
        current: FlowNodeId,
        label: Option<TokenText>,
    ) -> Build<()> {
        let expression = statement.expression().map_err(|_| Unsupported)?;
        let flow = self.evaluate(expression.syntax(), current, 0)?;
        let header = self.push(FlowNode::Join(vec![flow]))?;
        let mut flow = match statement.initializer().map_err(|_| Unsupported)? {
            // Each iteration assigns the variables of a declaration after the
            // defaults of its pattern. A `let`, `const`, or `using` declaration
            // needs no write point: its variables are scoped to the loop, so no
            // test or write before the loop can mention them. They have their
            // declared types at the entry, which the header joins. Facts on the
            // back edge from an earlier iteration therefore cannot narrow a
            // read below its declared type. A `var` is hoisted, so code before
            // the loop can test or write it. Each iteration writes a `var`
            // without a value, which gives it its declared type.
            AnyJsForInOrOfInitializer::JsForVariableDeclaration(declaration) => {
                let flow = self.declaration(&declaration.clone().into(), header, 0)?;
                match declaration
                    .declarator()
                    .and_then(|declarator| declarator.id())
                {
                    Ok(pattern) if declaration.is_var() => {
                        self.binding_writes(&pattern, None, &[], flow, 0)?
                    }
                    Ok(_) | Err(_) => flow,
                }
            }
            // An assignment target is written each iteration, after the
            // pattern is evaluated, with a key for `for...in` and an element
            // of the iterable for `for...of`, as in TypeScript.
            AnyJsForInOrOfInitializer::AnyJsAssignmentPattern(pattern) => {
                let flow = self.assignment_pattern(&pattern, header, 0)?;
                let value = match statement.assigned_value(&expression) {
                    Some(value) if !self.reads_evolving(&expression) => Some(value),
                    Some(_) | None => None,
                };
                self.pattern_writes(&pattern, value, &[], flow, 0)?
            }
        };
        if statement.assumes_non_nullish() {
            let condition = decompose_for_in(
                &expression,
                self.model,
                self.functions,
                &mut self.subjects,
                &mut self.guards,
            );
            flow = self.push_condition(flow, condition)?;
        }
        self.current = Some(flow);
        self.loop_body(statement.body().ok(), label, header)?;
        if let Some(end) = self.current {
            self.add_predecessor(header, end)?;
        }
        let target = self.jumps.pop().ok_or(Unsupported)?;
        self.current = self.join(std::iter::once(header).chain(target.breaks))?;
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

    /// Builds `statement` from `current`, with the control flow that
    /// TypeScript gives a `switch` statement. `label` is the label of
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
    /// clauses at `group` start, apart from the fall-through, as in
    /// TypeScript.
    fn case_group_entry(
        &mut self,
        cases: &mut SwitchCases,
        group: Range<usize>,
    ) -> Build<Vec<FlowNodeId>> {
        let has_default = cases.tests[group.clone()].iter().any(Option::is_none);
        match cases.kind {
            SwitchKind::Opaque => Ok(vec![cases.pre]),
            // As in TypeScript, no case before the group matched. With
            // `default` in the group, no case after it matched either, while
            // one of the group's own cases may have.
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
                        Some(unmatched) => {
                            let unmatched = self.unmatched_guard(unmatched);
                            self.push_condition(cases.pre, unmatched)?
                        }
                        None => cases.pre,
                    });
                }
                Ok(entry)
            }
        }
    }

    /// Returns the flow after `antecedent` where the case of the clause at
    /// `index` matched or, when `matched` is false, did not. The flow is
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
    /// no case narrows anything. Its guard is the conjunction of the cases
    /// that did not match, without the [`FlowGuard::Unmatched`] wrapper that
    /// [`Self::unmatched_guard`] adds.
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

    /// Returns `unmatched`, the condition that no case of a `switch` on a
    /// value or on `typeof` of one matched, with its guard wrapped in
    /// [`FlowGuard::Unmatched`].
    fn unmatched_guard(&mut self, mut unmatched: DecomposedCondition) -> DecomposedCondition {
        self.guards.push(FlowGuard::Unmatched(unmatched.guard));
        unmatched.guard = self.guards.len() - 1;
        unmatched
    }

    /// Returns the bypass of `statement`, a `switch` without `default`: the
    /// flow on which none of its cases matched.
    ///
    /// When TypeScript can find the statement exhaustive, the condition of
    /// the bypass is a [`FlowGuard::Bypass`]. Once inference finds the
    /// statement exhaustive, that guard gives `never` for every subject, so
    /// the bypass adds nothing to the code after the statement, as in
    /// TypeScript.
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
                let Some(unmatched) = self.unmatched_cases(cases) else {
                    return Ok(cases.pre);
                };
                let switch = self.exhaustive_switch(statement, cases, unmatched.guard);
                let mut unmatched = self.unmatched_guard(unmatched);
                let Some(switch) = switch else {
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
    /// it is exhaustive. `unmatched` is the index of the guard that tests
    /// that none of its cases matched.
    ///
    /// Returns `None` when the discriminant is neither a subject nor `typeof`
    /// of one, or is `typeof` in parentheses, when a case tests something
    /// other than a unit value or a kind of value, and once the root records
    /// [`MAX_SWITCHES`] statements.
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

    /// Returns the kinds of value that the guard at `guard` tests `subject`
    /// for with `typeof`, one bit each, as in [`FlowSwitch::typeof_kinds`].
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

    /// Builds `statement` from `current`, following TypeScript's control flow
    /// for `try` statements.
    ///
    /// An exception can leave the try block at any point. None of those
    /// points is narrower than the start of the block or one of its write
    /// points (see [`Self::exceptions`]), so the catch clause starts from the
    /// join of those. The normal exits are the ends of the try block and of the
    /// catch clause. `break` and `continue` reach their targets directly,
    /// without passing through the finally block, as in TypeScript.
    ///
    /// Without a finally block, the code after the statement continues from
    /// the normal exits. A finally block is built twice:
    /// - The first build starts from every way into the block: the normal
    ///   exits, the join of the exceptions from the try block, or from the
    ///   catch clause when there is one, and each `return` in either. It
    ///   records the reads of the block. Its end joins the enclosing
    ///   [`Self::exceptions`] and, when a `return` reached the block, the
    ///   enclosing [`Self::returns`]. These joins can be wider than
    ///   TypeScript's, because TypeScript joins there the block as reached
    ///   only from the exceptions, or only from the `return`s.
    /// - The second build starts from the normal exits only and records no
    ///   reads. The code after the statement continues from its end, because
    ///   that code runs only when the try block or the catch clause completed
    ///   normally. TypeScript gives that code the same flow.
    ///
    /// Without a normal exit, the code after the statement is unreachable.
    fn try_statement(&mut self, statement: &AnyJsTryStatement, current: FlowNodeId) -> Build<()> {
        let finally = statement.finally_clause();
        // A finally block is built twice, so the work doubles with each
        // finally block that encloses a `try` statement. Every statement
        // spends a visit each time it is built, so the visit limit bounds
        // that work even when the blocks evaluate nothing.
        let exception = self.push(FlowNode::Join(vec![current]))?;
        self.exceptions.push(exception);
        if finally.is_some() {
            self.returns.push(Vec::new());
        }
        if let Ok(body) = statement.body() {
            self.statements(body.syntax())?;
        }
        let mut normal_exits: Vec<_> = self.current.into_iter().collect();
        let mut exception = self.exceptions.pop().ok_or(Unsupported)?;
        if let Some(clause) = statement.catch_clause() {
            let start = exception;
            exception = self.push(FlowNode::Join(vec![start]))?;
            self.exceptions.push(exception);
            // The declaration runs the defaults of a destructuring pattern.
            self.current = Some(
                match clause
                    .declaration()
                    .and_then(|declaration| declaration.binding().ok())
                {
                    Some(pattern) => self.binding_pattern(&pattern, start, 0)?,
                    None => start,
                },
            );
            if let Ok(body) = clause.body() {
                self.statements(body.syntax())?;
            }
            normal_exits.extend(self.current);
            exception = self.exceptions.pop().ok_or(Unsupported)?;
        }
        let Some(finally) = finally else {
            self.current = self.join(normal_exits)?;
            return Ok(());
        };
        let returns = self.returns.pop().ok_or(Unsupported)?;
        let body = finally.body().ok();
        self.current = self.join(
            normal_exits
                .iter()
                .copied()
                .chain(std::iter::once(exception))
                .chain(returns.iter().copied()),
        )?;
        if let Some(body) = &body {
            self.statements(body.syntax())?;
        }
        let Some(end) = self.current else {
            return Ok(());
        };
        if let Some(outer) = self.exceptions.last().copied() {
            self.add_predecessor(outer, end)?;
        }
        if !returns.is_empty()
            && let Some(outer) = self.returns.last_mut()
        {
            outer.push(end);
        }
        self.current = self.join(normal_exits)?;
        if self.current.is_some()
            && let Some(body) = &body
        {
            let records_reads = std::mem::replace(&mut self.records_reads, false);
            self.statements(body.syntax())?;
            self.records_reads = records_reads;
        }
        Ok(())
    }

    /// Records reads and branches while evaluating `node` from `incoming`, and
    /// returns the flow after it.
    ///
    /// Expressions are evaluated in the order in which JavaScript runs them,
    /// except a destructuring assignment. It evaluates its pattern, defaults
    /// included, before its right operand, as TypeScript's binder does,
    /// although JavaScript runs the right operand first, and writes its
    /// targets after the right operand. Any other node evaluates its children
    /// in source order, which is the order in which they run.
    ///
    /// A part that runs only on some paths starts from the condition under
    /// which it runs, as in TypeScript. The right operand of `&&`, `||`, and
    /// `??` starts from a test of the left operand, and each branch of a
    /// conditional expression from a test of its condition. The key and the
    /// arguments after a `?.` start from a test that its object is not
    /// nullish, and the right operand of a logical assignment from a test of
    /// its target. [`Self::optional_chain`] and [`Self::assignment`] cover
    /// chains and assignments in detail.
    fn evaluate(
        &mut self,
        node: &JsSyntaxNode,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        self.visit(depth)?;
        let kind = node.kind();
        // Class members other than static blocks are left out of the
        // enclosing flow: methods, accessors, and constructors are execution
        // roots of their own, and TypeScript analyzes a class field that has
        // an initializer, including its decorators and computed key, apart
        // from the code around the class. Only the decorators and computed
        // keys of the other members lose the narrowing that TypeScript gives
        // them. A static block runs here, in member order, after the class's
        // `extends` clause.
        if UNEVALUATED_KINDS.matches(kind) {
            if let Some(block) = JsStaticInitializationBlockClassMember::cast_ref(node)
                && AnyInlineCode::cast_ref(node).is_some_and(|code| code.runs_inline())
            {
                let end = self.static_block(&block, incoming)?;
                return self.inline_end(end);
            }
            // The call that runs an inline function evaluates its body.
            if self.records_reads
                && let Some(closure) = AnyContinuingClosure::cast_ref(node)
                && closure.continues_outer_flow()
                && !AnyInlineCode::cast_ref(node).is_some_and(|code| code.runs_inline())
            {
                self.captures.push(BuiltCapture {
                    closure,
                    point: incoming,
                });
            }
            return Ok(incoming);
        }
        if let Some(identifier) = JsIdentifierExpression::cast_ref(node) {
            if self.records_reads
                && let Some(subject) = identifier
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
        if let Some(this) = JsThisExpression::cast_ref(node) {
            if self.records_reads
                && let Some(subject) = self.subjects.this(&this)
            {
                self.reads.push(BuiltRead {
                    expression: AnyFlowExpression::JsThisExpression(this),
                    point: incoming,
                    subject,
                });
            }
            return Ok(incoming);
        }
        if let Some(chain) = AnyJsOptionalChainExpression::cast_ref(node)
            && is_optional_chain(&chain)
        {
            return self.optional_chain(&chain, incoming, depth);
        }
        match AnyJsExpression::cast_ref(node) {
            // The object and the key are evaluated before the property is read.
            Some(
                expression @ (AnyJsExpression::JsStaticMemberExpression(_)
                | AnyJsExpression::JsComputedMemberExpression(_)),
            ) => {
                let flow = self.children(node, incoming, depth)?;
                self.member_read(expression, flow);
                Ok(flow)
            }
            Some(AnyJsExpression::JsAssignmentExpression(assignment)) => {
                self.assignment(&assignment, incoming, depth)
            }
            // The callee and the arguments run before the call adds them to
            // an array, or before the body of an inline function runs.
            Some(AnyJsExpression::JsCallExpression(call)) => {
                let flow = self.children(node, incoming, depth)?;
                match inline_function(&call) {
                    Some(function) => {
                        let end = self.inline_body(&function, flow, depth)?;
                        self.inline_end(end)
                    }
                    None => self.array_mutation(&call, flow),
                }
            }
            Some(AnyJsExpression::JsPreUpdateExpression(update)) => {
                let operand = update.operand().map_err(|_| Unsupported)?;
                self.update(&operand, incoming, depth)
            }
            Some(AnyJsExpression::JsPostUpdateExpression(update)) => {
                let operand = update.operand().map_err(|_| Unsupported)?;
                self.update(&operand, incoming, depth)
            }
            Some(AnyJsExpression::JsUnaryExpression(unary))
                if unary.operator() == Ok(JsUnaryOperator::Delete) =>
            {
                self.delete(&unary, incoming, depth)
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
                let right_flow = match self.branch(left_flow, &left, evaluate_right)? {
                    Some(flow) => Some(self.evaluate(right.syntax(), flow, depth + 1)?),
                    None => None,
                };
                let skip_flow = self.branch(left_flow, &left, skip_right)?;
                self.join_branches(left_flow, skip_flow, right_flow)
            }
            Some(AnyJsExpression::JsConditionalExpression(conditional)) => {
                let test = conditional.test().map_err(|_| Unsupported)?;
                let consequent = conditional.consequent().map_err(|_| Unsupported)?;
                let alternate = conditional.alternate().map_err(|_| Unsupported)?;
                let test_flow = self.evaluate(test.syntax(), incoming, depth + 1)?;
                let positive = match self.branch(test_flow, &test, FlowOutcome::Truthy)? {
                    Some(flow) => Some(self.evaluate(consequent.syntax(), flow, depth + 1)?),
                    None => None,
                };
                let negative = match self.branch(test_flow, &test, FlowOutcome::Falsy)? {
                    Some(flow) => Some(self.evaluate(alternate.syntax(), flow, depth + 1)?),
                    None => None,
                };
                self.join_branches(test_flow, positive, negative)
            }
            // Each operand of the comma operator that is a call is a statement
            // call, wherever the comma expression is.
            Some(AnyJsExpression::JsSequenceExpression(sequence)) => {
                let left = sequence.left().map_err(|_| Unsupported)?;
                let right = sequence.right().map_err(|_| Unsupported)?;
                let end = match self.statement_calls(&left, incoming, depth + 1)? {
                    Some(flow) => self.statement_calls(&right, flow, depth + 1)?,
                    None => None,
                };
                self.inline_end(end)
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

    /// Spends one of the remaining visits on a node at `depth`, failing when
    /// no visit remains or the node lies deeper than [`MAX_DEPTH`].
    fn visit(&mut self, depth: usize) -> Build<()> {
        self.remaining_visits = self.remaining_visits.checked_sub(1).ok_or(Unsupported)?;
        if depth > MAX_DEPTH {
            return Err(Unsupported);
        }
        Ok(())
    }

    /// Records the read of the subject that `expression`, a member access,
    /// reads, with the incoming flow `point`.
    fn member_read(&mut self, expression: AnyJsExpression, point: FlowNodeId) {
        if self.records_reads
            && let Some(subject) = subject_of(&expression, self.model, &mut self.subjects)
            && let Some(expression) = AnyFlowExpression::cast(expression.into_syntax())
        {
            self.reads.push(BuiltRead {
                expression,
                point,
                subject,
            });
        }
    }

    /// Evaluates the optional chain whose outermost link is `chain` from
    /// `incoming`, as TypeScript does, and returns the flow after it. `chain`
    /// lies at `depth`.
    ///
    /// The chain evaluates its root, the object of its innermost link, then
    /// the key or arguments of each link outward. Parentheses end a chain, as
    /// for [`AnyJsOptionalChainExpression::is_optional_chain`] and the
    /// receivers of a runtime test, so the root of `(x?.a)?.b(y)` is
    /// `(x?.a)`. A non-null assertion doesn't, as in TypeScript, so `x` is not
    /// nullish in the arguments of `x?.a!.b(x)`.
    ///
    /// When its object is nullish, a `?.` gives `undefined` without evaluating
    /// the rest of the chain. So each `?.` that a key or arguments follow adds
    /// a condition that its object is not nullish, and the rest of the chain
    /// runs from that condition. For example, `x` is not nullish in the
    /// arguments of `x?.f(x)`, `x?.[x.k]`, and `x?.a.f(x)`. The condition
    /// tests the object as not nullish rather than truthy, so `n` in
    /// `n?.toFixed(n)` keeps `0`. A `?.` adds no condition when its object is
    /// `this` or a variable whose collected type can't be nullish, such as `o`
    /// after `const o = { a: 1 }`. Each property read is recorded after its
    /// object and key.
    ///
    /// The code after the chain also runs where a `?.` gave `undefined`, so
    /// its flow joins the flow before each `?.` with the end of the chain.
    /// When no key or argument has a write point, that join has the type of
    /// the flow after the root for every subject, so the code after the chain
    /// continues from that flow instead. TypeScript likewise restores the flow
    /// from before a chain that changes no subject. A test inside the chain
    /// then reaches no read after it, so those reads need no flow.
    fn optional_chain(
        &mut self,
        chain: &AnyJsOptionalChainExpression,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        // Each link with its object, from the outermost link inward.
        let mut links = Vec::new();
        let mut link = chain.clone();
        let root = loop {
            let object = link.object().map_err(|_| Unsupported)?;
            let inner = inner_link(&object);
            links.push((link, object));
            match inner {
                Some(inner) => {
                    self.visit(depth + links.len())?;
                    link = inner;
                }
                None => break links.last().map(|(_, object)| object.clone()),
            }
        };
        let root = root.ok_or(Unsupported)?;
        let mut after_root = self.evaluate(root.syntax(), incoming, depth + links.len())?;
        // An inline function that the innermost link calls runs before the
        // rest of the chain, which continues from where the function ends. A
        // plain call evaluates its arguments, then the body, as JavaScript
        // does. For an optional call, as in `(() => {})?.(x)`, TypeScript's
        // binder runs the body before the arguments, which the link evaluates
        // afterwards. Biome follows it: a function expression is never
        // nullish, and running the body after the arguments would give a
        // read in them a type disjoint from TypeScript's.
        if let Some((AnyJsOptionalChainExpression::JsCallExpression(call), _)) = links.last()
            && let Some(function) = inline_function(call)
        {
            let link_depth = depth + links.len() - 1;
            let is_optional = call.is_optional();
            if !is_optional && let Ok(arguments) = call.arguments() {
                after_root = self.evaluate(arguments.syntax(), after_root, link_depth + 1)?;
            }
            let end = self.inline_body(&function, after_root, link_depth)?;
            after_root = self.inline_end(end)?;
            if !is_optional {
                links.pop();
            }
        }
        let write_points = self.write_points;
        // The flow where each `?.` gives `undefined` without evaluating the
        // rest of the chain.
        let mut short_circuits = Vec::new();
        // Only a `?.` that a key or arguments follow adds a condition, so `x`
        // in `x?.a.b` isn't tested.
        let tested = links
            .iter()
            .position(|(link, _)| has_operands(link))
            .map_or(0, |outermost| links.len() - outermost);
        let mut flow = after_root;
        for (index, (link, object)) in links.iter().rev().enumerate() {
            let link_depth = depth + links.len() - 1 - index;
            if link.is_optional() {
                short_circuits.push(flow);
            }
            if index < tested
                && link.is_optional()
                && self.functions.may_be_nullish(object, self.model)
            {
                let condition = decompose_condition(
                    object,
                    FlowOutcome::NonNullish,
                    self.model,
                    self.functions,
                    &mut self.subjects,
                    &mut self.guards,
                );
                flow = self.push_narrowing_condition(flow, condition)?;
            }
            match link {
                AnyJsOptionalChainExpression::JsStaticMemberExpression(member) => {
                    self.member_read(member.clone().into(), flow);
                }
                AnyJsOptionalChainExpression::JsComputedMemberExpression(member) => {
                    if let Ok(key) = member.member() {
                        flow = self.evaluate(key.syntax(), flow, link_depth + 1)?;
                    }
                    self.member_read(member.clone().into(), flow);
                }
                AnyJsOptionalChainExpression::JsCallExpression(call) => {
                    if let Ok(arguments) = call.arguments() {
                        flow = self.evaluate(arguments.syntax(), flow, link_depth + 1)?;
                    }
                    flow = self.array_mutation(call, flow)?;
                }
            }
        }
        if self.write_points == write_points {
            return Ok(after_root);
        }
        short_circuits.push(flow);
        Ok(self.join(short_circuits)?.unwrap_or(flow))
    }

    /// Evaluates `assignment`, which lies at `depth`, from `incoming`, and
    /// returns the flow after it.
    ///
    /// An assignment to a variable or a property, including a compound
    /// assignment such as `o[k] += v`, evaluates the object and the key of its
    /// target, as [`Self::assignment_target`] describes, then the right
    /// operand, after which a write point writes the target, as
    /// [`Self::write_target`] describes.
    ///
    /// A destructuring assignment follows TypeScript's binder rather than
    /// runtime order. It evaluates its pattern first, as
    /// [`Self::assignment_pattern`] describes, then the right operand, and
    /// then writes each target in source order, as [`Self::pattern_writes`]
    /// describes. At runtime, the right operand runs first and each default
    /// only for `undefined`, but TypeScript evaluates the defaults
    /// unconditionally before the value, so this order keeps every read
    /// within TypeScript's types.
    ///
    /// A logical assignment evaluates its right operand only where its target
    /// is nullish for `??=`, falsy for `||=`, or truthy for `&&=`, as in
    /// TypeScript, and writes the target there. The flow after it joins that
    /// path with the path on which the target's test skipped the right
    /// operand.
    fn assignment(
        &mut self,
        assignment: &JsAssignmentExpression,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        let right = assignment.right().map_err(|_| Unsupported)?;
        let target = match assignment.left().map_err(|_| Unsupported)? {
            AnyJsAssignmentPattern::AnyJsAssignment(target) => target,
            pattern => {
                let flow = self.assignment_pattern(&pattern, incoming, depth + 1)?;
                let flow = self.evaluate(right.syntax(), flow, depth + 1)?;
                let value = self.assigned_expression(&right);
                return self.pattern_writes(&pattern, value, &[], flow, depth + 1);
            }
        };
        let flow = self.assignment_target(&target, incoming, depth + 1)?;
        let (evaluate_right, skip_right) = match assignment.operator().map_err(|_| Unsupported)? {
            JsAssignmentOperator::LogicalAndAssign => (FlowOutcome::Truthy, FlowOutcome::Falsy),
            JsAssignmentOperator::LogicalOrAssign => (FlowOutcome::Falsy, FlowOutcome::Truthy),
            JsAssignmentOperator::NullishCoalescingAssign => {
                (FlowOutcome::Nullish, FlowOutcome::NonNullish)
            }
            JsAssignmentOperator::Assign => {
                let flow = self.evaluate(right.syntax(), flow, depth + 1)?;
                let value = self.assigned_expression(&right);
                let compound_like = is_compound_like(&right);
                // An empty array literal starts an array whose type evolves
                // with its writes again.
                let restarts = is_empty_array_literal(&right)
                    && match &target {
                        AnyJsAssignment::JsIdentifierAssignment(identifier) => {
                            self.model.binding(identifier).is_some_and(|binding| {
                                self.evolving(binding.range()) == FlowEvolving::Array
                            })
                        }
                        _ => false,
                    };
                let flow = self.write_target(&target, flow, |builder| {
                    if restarts {
                        return Ok(FlowWriteEffect::EmptyArray);
                    }
                    Ok(if compound_like {
                        FlowWriteEffect::CompoundLike(builder.assigned_value(value, &[])?)
                    } else {
                        FlowWriteEffect::Assign(builder.assignment_value(&target, &right, value)?)
                    })
                })?;
                return self.element_write(&target, value, flow);
            }
            JsAssignmentOperator::AddAssign
            | JsAssignmentOperator::SubtractAssign
            | JsAssignmentOperator::TimesAssign
            | JsAssignmentOperator::SlashAssign
            | JsAssignmentOperator::RemainderAssign
            | JsAssignmentOperator::ExponentAssign
            | JsAssignmentOperator::LeftShiftAssign
            | JsAssignmentOperator::RightShiftAssign
            | JsAssignmentOperator::UnsignedRightShiftAssign
            | JsAssignmentOperator::BitwiseAndAssign
            | JsAssignmentOperator::BitwiseOrAssign
            | JsAssignmentOperator::BitwiseXorAssign => {
                let flow = self.evaluate(right.syntax(), flow, depth + 1)?;
                return self.write_target(&target, flow, |_| Ok(FlowWriteEffect::Compound));
            }
        };
        let right_flow = self.target_condition(flow, &target, evaluate_right)?;
        let right_flow = self.evaluate(right.syntax(), right_flow, depth + 1)?;
        let value = self.assigned_expression(&right);
        let right_flow = self.write_target(&target, right_flow, |builder| {
            Ok(FlowWriteEffect::Assign(
                builder.assignment_value(&target, &right, value)?,
            ))
        })?;
        let skip_flow = self.target_condition(flow, &target, skip_right)?;
        self.join_two(skip_flow, right_flow)
    }

    /// Evaluates `++` or `--` on `operand`, which lies at `depth`, from
    /// `incoming`, and returns the flow after it. The object and the key of
    /// the operand run first, and then the operand is written, as a compound
    /// assignment writes its target.
    fn update(
        &mut self,
        operand: &AnyJsAssignment,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        let flow = self.assignment_target(operand, incoming, depth + 1)?;
        self.write_target(operand, flow, |_| Ok(FlowWriteEffect::Compound))
    }

    /// Evaluates `unary`, a `delete` expression, which lies at `depth`, from
    /// `incoming`, and returns the flow after it.
    ///
    /// The object and the key of the deleted property run, and then a write
    /// point removes the property. For `delete o.p`, the write assigns
    /// `undefined`, as in TypeScript. TypeScript ignores `delete o["p"]`, while
    /// Biome gives `o.p` its declared type there, and `delete o[k]` gives every
    /// path below `o` its declared type. Any other operand, as
    /// `delete (o.p)`, is evaluated as an expression and adds no write point,
    /// so collection counts it as a write that the graph doesn't model.
    fn delete(
        &mut self,
        unary: &JsUnaryExpression,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        self.visit(depth + 1)?;
        let argument = unary.argument().map_err(|_| Unsupported)?;
        let (object, key) = match &argument {
            AnyJsExpression::JsStaticMemberExpression(member) => {
                let object = member.object().map_err(|_| Unsupported)?;
                let flow = self.evaluate(object.syntax(), incoming, depth + 2)?;
                let key = member.member().ok().as_ref().and_then(property_key);
                (
                    object,
                    (flow, key.map(|key| (key, FlowWriteEffect::Undefined))),
                )
            }
            AnyJsExpression::JsComputedMemberExpression(member) => {
                let object = member.object().map_err(|_| Unsupported)?;
                let key = member.member().map_err(|_| Unsupported)?;
                let flow = self.evaluate(object.syntax(), incoming, depth + 2)?;
                let flow = self.evaluate(key.syntax(), flow, depth + 2)?;
                (
                    object,
                    (
                        flow,
                        literal_key(&key).map(|key| (key, FlowWriteEffect::Reset)),
                    ),
                )
            }
            _ => return self.evaluate(argument.syntax(), incoming, depth + 1),
        };
        let (flow, property) = key;
        let Some(object) = subject_of(&object, self.model, &mut self.subjects) else {
            return Ok(flow);
        };
        self.written.members.insert(unary.range());
        let write = match property {
            Some((key, effect)) => FlowWrite {
                subject: self
                    .subjects
                    .intern(FlowSubject::Member { object, key })
                    .ok_or(Unsupported)?,
                effect,
            },
            None => FlowWrite {
                subject: object,
                effect: FlowWriteEffect::Wildcard,
            },
        };
        self.push_write(flow, write)
    }

    /// Adds a node where the target `target` of a logical assignment has
    /// `outcome` after `antecedent`, and returns the flow after it. Adds no
    /// node when the test can narrow no subject, as for the target `o[k]`.
    fn target_condition(
        &mut self,
        antecedent: FlowNodeId,
        target: &AnyJsAssignment,
        outcome: FlowOutcome,
    ) -> Build<FlowNodeId> {
        let condition = decompose_assignment_target(
            target,
            outcome,
            self.model,
            self.functions,
            &mut self.subjects,
            &mut self.guards,
        );
        self.push_narrowing_condition(antecedent, condition)
    }

    /// Evaluates the object and the key of `target`, which lies at `depth`,
    /// from `incoming`, and returns the flow after them: the object of
    /// `o.v`, the object and the key of `o[k]`, and nothing for a variable.
    /// Parentheses, `!`, and type assertions around the target are skipped.
    fn assignment_target(
        &mut self,
        target: &AnyJsAssignment,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        self.visit(depth)?;
        let inner = match target {
            AnyJsAssignment::JsIdentifierAssignment(_) => return Ok(incoming),
            AnyJsAssignment::JsStaticMemberAssignment(member) => {
                let object = member.object().map_err(|_| Unsupported)?;
                return self.evaluate(object.syntax(), incoming, depth + 1);
            }
            AnyJsAssignment::JsComputedMemberAssignment(member) => {
                let object = member.object().map_err(|_| Unsupported)?;
                let flow = self.evaluate(object.syntax(), incoming, depth + 1)?;
                return match member.member() {
                    Ok(key) => self.evaluate(key.syntax(), flow, depth + 1),
                    Err(_) => Ok(flow),
                };
            }
            AnyJsAssignment::JsParenthesizedAssignment(assignment) => assignment.assignment(),
            AnyJsAssignment::TsNonNullAssertionAssignment(assignment) => assignment.assignment(),
            AnyJsAssignment::TsAsAssignment(assignment) => assignment.assignment(),
            AnyJsAssignment::TsSatisfiesAssignment(assignment) => assignment.assignment(),
            AnyJsAssignment::TsTypeAssertionAssignment(assignment) => assignment.assignment(),
            AnyJsAssignment::JsBogusAssignment(_) => return Err(Unsupported),
        };
        match inner {
            Ok(inner) => self.assignment_target(&inner, incoming, depth + 1),
            Err(_) => Ok(incoming),
        }
    }

    /// Evaluates `pattern`, the target of a destructuring assignment or of the
    /// head of a `for...in` or `for...of` loop, which lies at `depth`, from
    /// `incoming`, and returns the flow after it, before any target is
    /// written.
    ///
    /// The elements run in source order, as TypeScript's binder visits them.
    /// Each evaluates its computed key, the object and the key of its target
    /// as [`Self::assignment_target`] describes, and its default. A default
    /// runs unconditionally here, as in TypeScript, so a write in it, as in
    /// `[a = (x = 1)] = t`, holds after the pattern. An element whose target
    /// is a nested pattern evaluates its default, then the nested pattern.
    fn assignment_pattern(
        &mut self,
        pattern: &AnyJsAssignmentPattern,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        self.visit(depth)?;
        let mut flow = incoming;
        match pattern {
            AnyJsAssignmentPattern::AnyJsAssignment(target) => {
                flow = self.assignment_target(target, flow, depth + 1)?;
            }
            AnyJsAssignmentPattern::JsArrayAssignmentPattern(pattern) => {
                for element in pattern.elements().iter().flatten() {
                    flow = match element {
                        AnyJsArrayAssignmentPatternElement::JsArrayAssignmentPatternElement(
                            element,
                        ) => self.assignment_element(
                            element.pattern().ok(),
                            element.init(),
                            flow,
                            depth + 1,
                        )?,
                        AnyJsArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(
                            element,
                        ) => {
                            self.assignment_element(element.pattern().ok(), None, flow, depth + 1)?
                        }
                        AnyJsArrayAssignmentPatternElement::JsArrayHole(_) => flow,
                    };
                }
            }
            AnyJsAssignmentPattern::JsObjectAssignmentPattern(pattern) => {
                for property in pattern.properties().iter().flatten() {
                    flow = match property {
                        AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(
                            property,
                        ) => {
                            let flow = self.member_name(property.member().ok(), flow, depth + 1)?;
                            self.assignment_element(
                                property.pattern().ok(),
                                property.init(),
                                flow,
                                depth + 1,
                            )?
                        }
                        AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
                            property,
                        ) => self.assignment_element(None, property.init(), flow, depth + 1)?,
                        AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(rest) => {
                            match rest.target() {
                                Ok(target) => self.assignment_target(&target, flow, depth + 1)?,
                                Err(_) => flow,
                            }
                        }
                        AnyJsObjectAssignmentPatternMember::JsBogusAssignment(_) => {
                            return Err(Unsupported);
                        }
                    };
                }
            }
        }
        Ok(flow)
    }

    /// Evaluates one element of a destructuring assignment from `incoming` and
    /// returns the flow after it. `pattern` is the element's target and
    /// `default` its default. A target that is no pattern evaluates its object
    /// and key before the default, and a nested pattern runs after the
    /// default.
    fn assignment_element(
        &mut self,
        pattern: Option<AnyJsAssignmentPattern>,
        default: Option<JsInitializerClause>,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        let default = default.and_then(|default| default.expression().ok());
        let evaluate_default = |builder: &mut Self, flow| match &default {
            Some(default) => builder.evaluate(default.syntax(), flow, depth + 1),
            None => Ok(flow),
        };
        match pattern {
            Some(AnyJsAssignmentPattern::AnyJsAssignment(target)) => {
                let flow = self.assignment_target(&target, incoming, depth)?;
                evaluate_default(self, flow)
            }
            Some(pattern) => {
                let flow = evaluate_default(self, incoming)?;
                self.assignment_pattern(&pattern, flow, depth)
            }
            None => evaluate_default(self, incoming),
        }
    }

    /// Writes each target of `pattern`, the target of a destructuring
    /// assignment or of the head of a `for...in` or `for...of` loop, in source
    /// order from `incoming`, and returns the flow after them.
    ///
    /// Each target is assigned the field of `value` that the elements around
    /// it read, with `steps` for those outside `pattern` and the default of
    /// each element along the way, as in TypeScript. A target gets its
    /// declared type when `value` is `None`. A computed key that
    /// [`member_name_key`] rejects reads an unknown field, and so does a rest
    /// element after one, so their targets get their declared types.
    fn pattern_writes(
        &mut self,
        pattern: &AnyJsAssignmentPattern,
        value: Option<FlowAssignedValue>,
        steps: &[FlowDestructureStep],
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        self.visit(depth)?;
        let mut flow = incoming;
        match pattern {
            AnyJsAssignmentPattern::AnyJsAssignment(target) => {
                return self.write_target(target, flow, |builder| {
                    Ok(FlowWriteEffect::Assign(
                        builder.assigned_value(value, steps)?,
                    ))
                });
            }
            AnyJsAssignmentPattern::JsArrayAssignmentPattern(pattern) => {
                for (index, element) in pattern.elements().iter().enumerate() {
                    let (target, field, default) = match element.map_err(|_| Unsupported)? {
                        AnyJsArrayAssignmentPatternElement::JsArrayAssignmentPatternElement(
                            element,
                        ) => (
                            element.pattern(),
                            DestructureField::Index {
                                index,
                                has_initializer: false,
                            },
                            element.init(),
                        ),
                        AnyJsArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(
                            element,
                        ) => (element.pattern(), DestructureField::RestFrom(index), None),
                        AnyJsArrayAssignmentPatternElement::JsArrayHole(_) => continue,
                    };
                    let Ok(target) = target else {
                        continue;
                    };
                    let (value, steps) = self.with_step(value, steps, field, default.as_ref());
                    flow = self.pattern_writes(&target, value, &steps, flow, depth + 1)?;
                }
            }
            AnyJsAssignmentPattern::JsObjectAssignmentPattern(pattern) => {
                // The names of the properties before a rest element, which
                // it leaves out, or `None` once a key names no property and
                // when there is no rest element.
                let has_rest = matches!(
                    pattern.properties().iter().next_back(),
                    Some(Ok(
                        AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(_)
                    ))
                );
                let mut names = has_rest.then(Vec::new);
                for property in pattern.properties().iter() {
                    match property.map_err(|_| Unsupported)? {
                        AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(
                            property,
                        ) => {
                            let Ok(target) = property.pattern() else {
                                continue;
                            };
                            let key = property.member().ok().as_ref().and_then(member_name_key);
                            let (value, steps) = match key {
                                Some(PropertyKey::Name(name)) => {
                                    if let Some(names) = &mut names {
                                        names.push(name.clone());
                                    }
                                    let field = DestructureField::Name { name, has_initializer: false };
                                    self.with_step(value, steps, field, property.init().as_ref())
                                }
                                Some(PropertyKey::Private(_)) | None => {
                                    names = None;
                                    (None, Vec::new())
                                }
                            };
                            flow = self.pattern_writes(&target, value, &steps, flow, depth + 1)?;
                        }
                        AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
                            property,
                        ) => {
                            let Ok(identifier) = property.identifier() else {
                                continue;
                            };
                            let key = identifier.name_token().ok().as_ref().and_then(identifier_key);
                            let (value, steps) = match key {
                                Some(PropertyKey::Name(name)) => {
                                    if let Some(names) = &mut names {
                                        names.push(name.clone());
                                    }
                                    let field = DestructureField::Name { name, has_initializer: false };
                                    self.with_step(value, steps, field, property.init().as_ref())
                                }
                                Some(PropertyKey::Private(_)) | None => {
                                    names = None;
                                    (None, Vec::new())
                                }
                            };
                            let target = AnyJsAssignment::JsIdentifierAssignment(identifier);
                            flow = self.write_target(&target, flow, |builder| {
                                Ok(FlowWriteEffect::Assign(builder.assigned_value(value, &steps)?))
                            })?;
                        }
                        AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(rest) => {
                            let Ok(target) = rest.target() else {
                                continue;
                            };
                            let (value, steps) = match names.take() {
                                Some(names) => {
                                    let field = DestructureField::RestExcept(names.into());
                                    self.with_step(value, steps, field, None)
                                }
                                None => (None, Vec::new()),
                            };
                            flow = self.write_target(&target, flow, |builder| {
                                Ok(FlowWriteEffect::Assign(builder.assigned_value(value, &steps)?))
                            })?;
                        }
                        AnyJsObjectAssignmentPatternMember::JsBogusAssignment(_) => {
                            return Err(Unsupported);
                        }
                    }
                }
            }
        }
        Ok(flow)
    }

    /// Adds a write point of the subject that `target` writes after
    /// `incoming`, with the effect that `effect` gives, and returns the flow
    /// after it.
    ///
    /// Parentheses and `!` around the target are skipped, as in TypeScript.
    /// TypeScript doesn't narrow at a target behind `as`, `satisfies`, or
    /// `<T>`, as `(x as T) = v`, so the write gives the subject its declared
    /// type, which is wider. A write through a key that [`literal_key`]
    /// rejects, as `o[k] = v`, writes any property of `o`, as
    /// [`FlowWriteEffect::Wildcard`] describes.
    ///
    /// A target that is no subject and whose object is no subject, as `f().v`
    /// or a global, adds no point and isn't recorded in
    /// [`BuiltRoot::written`], so collection counts it as a write that the
    /// graph doesn't model, as for writes outside the root's flow.
    fn write_target(
        &mut self,
        target: &AnyJsAssignment,
        incoming: FlowNodeId,
        effect: impl FnOnce(&mut Self) -> Build<FlowWriteEffect>,
    ) -> Build<FlowNodeId> {
        let mut target = target.clone();
        let mut asserted = false;
        let (subject, wildcard) = loop {
            target = match target {
                AnyJsAssignment::JsParenthesizedAssignment(assignment) => assignment.assignment(),
                AnyJsAssignment::TsNonNullAssertionAssignment(assignment) => {
                    assignment.assignment()
                }
                AnyJsAssignment::TsAsAssignment(assignment) => {
                    asserted = true;
                    assignment.assignment()
                }
                AnyJsAssignment::TsSatisfiesAssignment(assignment) => {
                    asserted = true;
                    assignment.assignment()
                }
                AnyJsAssignment::TsTypeAssertionAssignment(assignment) => {
                    asserted = true;
                    assignment.assignment()
                }
                AnyJsAssignment::JsIdentifierAssignment(identifier) => {
                    let Some(subject) = self.subjects.reference(self.model, &identifier) else {
                        return Ok(incoming);
                    };
                    self.written
                        .variables
                        .insert(identifier.syntax().text_trimmed_range().start());
                    break (subject, false);
                }
                AnyJsAssignment::JsStaticMemberAssignment(member) => {
                    let (Ok(object), Ok(name)) = (member.object(), member.member()) else {
                        return Ok(incoming);
                    };
                    let (Some(object), Some(key)) = (
                        subject_of(&object, self.model, &mut self.subjects),
                        property_key(&name),
                    ) else {
                        return Ok(incoming);
                    };
                    let subject = self
                        .subjects
                        .intern(FlowSubject::Member { object, key })
                        .ok_or(Unsupported)?;
                    self.written.members.insert(member.range());
                    break (subject, false);
                }
                AnyJsAssignment::JsComputedMemberAssignment(member) => {
                    let Some(object) = member
                        .object()
                        .ok()
                        .and_then(|object| subject_of(&object, self.model, &mut self.subjects))
                    else {
                        return Ok(incoming);
                    };
                    self.written.members.insert(member.range());
                    match member.member().ok().as_ref().and_then(literal_key) {
                        Some(key) => {
                            let subject = self
                                .subjects
                                .intern(FlowSubject::Member { object, key })
                                .ok_or(Unsupported)?;
                            break (subject, false);
                        }
                        None => break (object, true),
                    }
                }
                AnyJsAssignment::JsBogusAssignment(_) => return Err(Unsupported),
            }
            .map_err(|_| Unsupported)?;
        };
        let effect = if wildcard {
            FlowWriteEffect::Wildcard
        } else if asserted {
            FlowWriteEffect::Reset
        } else {
            effect(self)?
        };
        self.push_write(incoming, FlowWrite { subject, effect })
    }

    /// Adds a [`FlowNode::Write`] of `write` after `antecedent` and returns
    /// it. The innermost exception join also gets the new node, as in
    /// TypeScript.
    fn push_write(&mut self, antecedent: FlowNodeId, write: FlowWrite) -> Build<FlowNodeId> {
        let index = match self.write_ids.get(&write) {
            Some(index) => *index,
            None => {
                let index = u32::try_from(self.writes.len()).map_err(|_| Unsupported)?;
                self.writes.push(write.clone());
                self.write_ids.insert(write, index);
                index
            }
        };
        let node = self.push(FlowNode::Write {
            antecedent,
            write: index,
        })?;
        self.write_points += 1;
        if let Some(exception) = self.exceptions.last().copied() {
            self.add_predecessor(exception, node)?;
        }
        Ok(node)
    }

    /// Returns the index in [`BuiltRoot::assigned`] of `value` read through
    /// `steps`, or `None` when `value` is `None`, because Biome can't type it
    /// as TypeScript does.
    fn assigned_value(
        &mut self,
        value: Option<FlowAssignedValue>,
        steps: &[FlowDestructureStep],
    ) -> Build<Option<u32>> {
        let Some(value) = value else {
            return Ok(None);
        };
        let ranges = match value {
            FlowAssignedValue::Expression(range) => Some(range),
            FlowAssignedValue::BooleanLiteral(_)
            | FlowAssignedValue::IterableValue(_)
            | FlowAssignedValue::Key => None,
        };
        let callees = ranges
            .into_iter()
            .chain(steps.iter().filter_map(|step| step.default))
            .filter_map(|range| self.value_callees.get(&range))
            .flatten()
            .copied()
            .collect();
        self.intern_assigned(FlowAssigned {
            value,
            steps: steps.into(),
            callees,
            covered: false,
        })
        .map(Some)
    }

    /// Returns the index of `assigned` in [`Self::assigned`], adding it when
    /// it is new.
    fn intern_assigned(&mut self, assigned: FlowAssigned) -> Build<u32> {
        if let Some(index) = self.assigned_ids.get(&assigned) {
            return Ok(*index);
        }
        let index = u32::try_from(self.assigned.len()).map_err(|_| Unsupported)?;
        self.assigned.push(assigned.clone());
        self.assigned_ids.insert(assigned, index);
        Ok(index)
    }

    /// Returns the index of the value that an assignment with `=`, `??=`,
    /// `||=`, or `&&=` writes to `target`, as `right` in `x = right`, given
    /// `value`, the value that [`Self::assigned_expression`] gave for
    /// `right`. The value is covered, as [`FlowAssigned::covered`] describes,
    /// when [`Self::is_covered`] holds.
    fn assignment_value(
        &mut self,
        target: &AnyJsAssignment,
        right: &AnyJsExpression,
        value: Option<FlowAssignedValue>,
    ) -> Build<Option<u32>> {
        if !self.is_covered(target, right) {
            return self.assigned_value(value, &[]);
        }
        self.intern_assigned(FlowAssigned {
            value: FlowAssignedValue::Expression(right.range()),
            steps: Box::default(),
            callees: Box::default(),
            covered: true,
        })
        .map(Some)
    }

    /// Returns whether the declared type of the variable that an assignment
    /// with `=`, `??=`, `||=`, or `&&=` writes includes the type that Biome
    /// gives `right`, the assigned value.
    ///
    /// The variable must be `target`, in parentheses or with `!`, its type
    /// must evolve with its writes as a variable, and [`covers_value`] must
    /// hold for the assignment.
    fn is_covered(&mut self, target: &AnyJsAssignment, right: &AnyJsExpression) -> bool {
        let mut target = target.clone();
        let identifier = loop {
            let inner = match target {
                AnyJsAssignment::JsIdentifierAssignment(identifier) => break identifier,
                AnyJsAssignment::JsParenthesizedAssignment(assignment) => assignment.assignment(),
                AnyJsAssignment::TsNonNullAssertionAssignment(assignment) => {
                    assignment.assignment()
                }
                AnyJsAssignment::JsBogusAssignment(_)
                | AnyJsAssignment::JsComputedMemberAssignment(_)
                | AnyJsAssignment::JsStaticMemberAssignment(_)
                | AnyJsAssignment::TsAsAssignment(_)
                | AnyJsAssignment::TsSatisfiesAssignment(_)
                | AnyJsAssignment::TsTypeAssertionAssignment(_) => return false,
            };
            let Ok(inner) = inner else {
                return false;
            };
            target = inner;
        };
        let Some(binding) = self.model.binding(&identifier) else {
            return false;
        };
        let model = self.model;
        self.evolving(binding.range()) == FlowEvolving::Variable
            && covers_value(&identifier, &binding, right, model, |value| {
                self.reads_evolving(value)
            })
    }

    /// Returns the value of `expression` as a write assigns it, or `None`
    /// when Biome can't type it as TypeScript does, as for `typeof` or a call
    /// to a method whose return type Biome infers; see [`value_callees`].
    fn assigned_expression(&mut self, expression: &AnyJsExpression) -> Option<FlowAssignedValue> {
        if matches!(
            expression.clone().omit_parentheses(),
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsBooleanLiteralExpression(_)
            )
        ) {
            return Some(FlowAssignedValue::BooleanLiteral(expression.range()));
        }
        self.check_value(expression)
            .then(|| FlowAssignedValue::Expression(expression.range()))
    }

    /// Returns whether Biome can type `expression`, a value or a default that
    /// a write assigns, as TypeScript does, once the reading context confirms
    /// the callees that [`value_callees`] gives. The callees are recorded by
    /// the range of `expression`, where [`Self::assigned_value`] finds them.
    fn check_value(&mut self, expression: &AnyJsExpression) -> bool {
        if self.reads_evolving(expression) {
            return false;
        }
        let model = self.model;
        let Ok(callees) = value_callees(expression, model, |value| self.reads_evolving(value))
        else {
            return false;
        };
        if !callees.is_empty() {
            self.value_callees
                .insert(expression.range(), callees.into_boxed_slice());
        }
        true
    }

    /// Returns how the type of the variable declared at `range` evolves with
    /// its writes, calling [`evolving_declaration`] once per variable.
    fn evolving(&mut self, range: TextRange) -> FlowEvolving {
        if let Some(evolving) = self.evolving.get(&range) {
            return *evolving;
        }
        let evolving = self
            .model
            .as_binding_by_range(range)
            .map_or(FlowEvolving::None, |binding| {
                evolving_declaration(&binding, self.model)
            });
        self.evolving.insert(range, evolving);
        evolving
    }

    /// Returns the subject of the variable that `identifier` reads when the
    /// variable's type evolves as an array.
    fn evolving_array(&mut self, identifier: &JsIdentifierExpression) -> Option<FlowSubjectId> {
        let binding = self.model.binding(&identifier.name().ok()?)?;
        if self.evolving(binding.range()) != FlowEvolving::Array {
            return None;
        }
        self.subjects.intern(FlowSubject::Binding(binding.range()))
    }

    /// Returns whether `expression`, a value that a write assigns or adds to
    /// an array, reads a variable whose type evolves with its writes outside
    /// the functions that it creates, once per expression.
    ///
    /// Biome types such a read in a value without narrowing, from a declared
    /// type that doesn't follow the variable's writes, as `null` for `x` in
    /// `let x = null; x = "a"; y = x`, so the value's type can lack what
    /// TypeScript gives it.
    fn reads_evolving(&mut self, expression: &AnyJsExpression) -> bool {
        let range = expression.range();
        if let Some(reads) = self.evolving_values.get(&range) {
            return *reads;
        }
        let model = self.model;
        let reads = reads_evolving_variable(expression, model, |range| self.evolving(range));
        self.evolving_values.insert(range, reads);
        reads
    }

    /// Adds the write points of `call` after `incoming`, the flow after its
    /// arguments, when `call` adds values to an array whose type evolves with
    /// its writes, and returns the flow after them. Each argument gets one
    /// write point, in order, as TypeScript adds each argument to the element
    /// type. A spread argument adds a value that Biome can't type as
    /// TypeScript does. [`mutated_array`] recognizes the calls that add values.
    fn array_mutation(
        &mut self,
        call: &JsCallExpression,
        incoming: FlowNodeId,
    ) -> Build<FlowNodeId> {
        let Some(object) = mutated_array(call) else {
            return Ok(incoming);
        };
        let Some(subject) = self.evolving_array(&object) else {
            return Ok(incoming);
        };
        self.written.mutations.insert(object.range().start());
        let mut flow = incoming;
        for argument in call.arguments().map_err(|_| Unsupported)?.args() {
            let value = match argument.map_err(|_| Unsupported)? {
                AnyJsCallArgument::AnyJsExpression(argument) => {
                    let value = self.assigned_expression(&argument);
                    self.assigned_value(value, &[])?
                }
                AnyJsCallArgument::JsSpread(_) => None,
            };
            let effect = FlowWriteEffect::Elements(value);
            flow = self.push_write(flow, FlowWrite { subject, effect })?;
        }
        Ok(flow)
    }

    /// Adds the write point that gives the value of the element assignment
    /// `target = value` to an array whose type evolves with its writes, as
    /// `a[0] = v` does, after `incoming`, and returns the flow after it.
    ///
    /// The value joins the element type only for a number literal key,
    /// because TypeScript adds a value only when the key's type is a number.
    /// A string key adds nothing. Any other key may be a number, so its value
    /// is one that Biome can't type as TypeScript does. Any other target adds
    /// no write point.
    fn element_write(
        &mut self,
        target: &AnyJsAssignment,
        value: Option<FlowAssignedValue>,
        incoming: FlowNodeId,
    ) -> Build<FlowNodeId> {
        let AnyJsAssignment::JsComputedMemberAssignment(member) = target else {
            return Ok(incoming);
        };
        let (Some(object), Ok(key)) = (
            member.object().ok().and_then(array_variable),
            member.member(),
        ) else {
            return Ok(incoming);
        };
        let Some(subject) = self.evolving_array(&object) else {
            return Ok(incoming);
        };
        self.written.mutations.insert(object.range().start());
        let value = match ElementKey::new(&key) {
            ElementKey::Number => self.assigned_value(value, &[])?,
            ElementKey::String => return Ok(incoming),
            ElementKey::Other => None,
        };
        let effect = FlowWriteEffect::Elements(value);
        self.push_write(incoming, FlowWrite { subject, effect })
    }

    /// Returns `value` and `steps` with a step that reads `field`, supplied
    /// by `default` when it is `undefined`, or `None` and no steps when
    /// Biome can't type the default as TypeScript does, as
    /// [`Self::check_value`] decides.
    fn with_step(
        &mut self,
        value: Option<FlowAssignedValue>,
        steps: &[FlowDestructureStep],
        field: DestructureField,
        default: Option<&JsInitializerClause>,
    ) -> (Option<FlowAssignedValue>, Vec<FlowDestructureStep>) {
        let default = default.and_then(|default| default.expression().ok());
        if value.is_none()
            || default
                .as_ref()
                .is_some_and(|default| !self.check_value(default))
        {
            return (None, Vec::new());
        }
        let mut steps = steps.to_vec();
        steps.push(FlowDestructureStep {
            field,
            default: default.map(|default| default.range()),
        });
        (value, steps)
    }

    /// Evaluates the declarators of `declaration`, which lies at `depth`, in
    /// order from `incoming`, and returns the flow after them.
    ///
    /// Each declarator evaluates its binding pattern, with
    /// [`Self::binding_pattern`], then its initializer. The initializer runs
    /// first at runtime, but TypeScript's types after a write in one of them,
    /// as in `const [a = (x = 1)] = [x]`, follow this order. A declarator with
    /// a type annotation then writes the variables it declares with the
    /// initializer's value. A `var` declarator without one also writes its
    /// variables, because a read before the declarator sees the variable as
    /// `undefined`. That write has no value and gives the variables their
    /// declared types. A write of the initializer's value would give the same
    /// types, because the declared type of such a variable is the type of the
    /// initializer. Other declarators add no write point, because no read
    /// comes before them. A declarator without an initializer, as in a
    /// `for...of` head or `let x!: T`, evaluates only its pattern. `using` and
    /// `await using` declarations evaluate as `const` does.
    fn declaration(
        &mut self,
        declaration: &AnyJsVariableDeclaration,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        match declaration {
            AnyJsVariableDeclaration::JsVariableDeclaration(declaration) => {
                let mut flow = incoming;
                for declarator in declaration.declarators() {
                    let declarator = declarator.map_err(|_| Unsupported)?;
                    flow = self.declarator(&declarator, flow, depth + 1)?;
                }
                Ok(flow)
            }
            AnyJsVariableDeclaration::JsForVariableDeclaration(declaration) => {
                let declarator = declaration.declarator().map_err(|_| Unsupported)?;
                self.declarator(&declarator, incoming, depth + 1)
            }
        }
    }

    /// Evaluates the binding pattern of `declarator`, defaults included, then
    /// its initializer, from `incoming`, as TypeScript's binder orders a
    /// declaration, and returns the flow after them. The writes of the
    /// declared variables follow the initializer.
    fn declarator(
        &mut self,
        declarator: &JsVariableDeclarator,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        let pattern = declarator.id().ok();
        let mut flow = self.binding_element(pattern.clone(), None, incoming, depth + 1)?;
        if let Some(initializer) = declarator.initializer() {
            let expression = initializer.expression().map_err(|_| Unsupported)?;
            flow = self.evaluate(expression.syntax(), flow, depth + 1)?;
            if let Some(pattern) = &pattern {
                if declarator.variable_annotation().is_some() {
                    let value = self.assigned_expression(&expression);
                    flow = self.binding_writes(pattern, value, &[], flow, depth + 1)?;
                } else if let AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
                    binding,
                )) = pattern
                    && let evolving @ (FlowEvolving::Variable | FlowEvolving::Array) =
                        self.evolving(binding.range())
                {
                    let subject = self
                        .subjects
                        .intern(FlowSubject::Binding(binding.range()))
                        .ok_or(Unsupported)?;
                    let effect = if evolving == FlowEvolving::Array {
                        FlowWriteEffect::EmptyArray
                    } else {
                        let value = self.assigned_expression(&expression);
                        FlowWriteEffect::Assign(self.assigned_value(value, &[])?)
                    };
                    flow = self.push_write(flow, FlowWrite { subject, effect })?;
                    self.initializers.insert(flow);
                    if is_var(declarator) {
                        self.hoisted.insert(subject);
                    }
                } else if is_var(declarator) {
                    flow = self.binding_writes(pattern, None, &[], flow, depth + 1)?;
                }
            }
        }
        Ok(flow)
    }

    /// Writes each variable that `pattern`, the binding pattern of a
    /// declarator, declares, in source order from `incoming`, and returns the
    /// flow after them.
    ///
    /// Each variable is assigned the field of `value` that the elements
    /// around it read, as [`Self::pattern_writes`] describes for the targets
    /// of a destructuring assignment.
    fn binding_writes(
        &mut self,
        pattern: &AnyJsBindingPattern,
        value: Option<FlowAssignedValue>,
        steps: &[FlowDestructureStep],
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        self.visit(depth)?;
        let mut flow = incoming;
        match pattern {
            AnyJsBindingPattern::AnyJsBinding(binding) => {
                return self.write_binding(binding, value, steps, flow);
            }
            AnyJsBindingPattern::JsArrayBindingPattern(pattern) => {
                for (index, element) in pattern.elements().iter().enumerate() {
                    let (element, field, default) = match element.map_err(|_| Unsupported)? {
                        AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(element) => (
                            element.pattern(),
                            DestructureField::Index {
                                index,
                                has_initializer: false,
                            },
                            element.init(),
                        ),
                        AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(
                            element,
                        ) => (element.pattern(), DestructureField::RestFrom(index), None),
                        AnyJsArrayBindingPatternElement::JsArrayHole(_) => continue,
                    };
                    let Ok(element) = element else {
                        continue;
                    };
                    let (value, steps) = self.with_step(value, steps, field, default.as_ref());
                    flow = self.binding_writes(&element, value, &steps, flow, depth + 1)?;
                }
            }
            AnyJsBindingPattern::JsObjectBindingPattern(pattern) => {
                // The names of the properties before a rest element, as for
                // an assignment pattern.
                let has_rest = matches!(
                    pattern.properties().iter().next_back(),
                    Some(Ok(
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(_)
                    ))
                );
                let mut names = has_rest.then(Vec::new);
                for property in pattern.properties().iter() {
                    match property.map_err(|_| Unsupported)? {
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(
                            property,
                        ) => {
                            let Ok(element) = property.pattern() else {
                                continue;
                            };
                            let key = property.member().ok().as_ref().and_then(member_name_key);
                            let (value, steps) = match key {
                                Some(PropertyKey::Name(name)) => {
                                    if let Some(names) = &mut names {
                                        names.push(name.clone());
                                    }
                                    let field = DestructureField::Name { name, has_initializer: false };
                                    self.with_step(value, steps, field, property.init().as_ref())
                                }
                                Some(PropertyKey::Private(_)) | None => {
                                    names = None;
                                    (None, Vec::new())
                                }
                            };
                            flow = self.binding_writes(&element, value, &steps, flow, depth + 1)?;
                        }
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                            property,
                        ) => {
                            let Ok(binding) = property.identifier() else {
                                continue;
                            };
                            let key = binding
                                .as_js_identifier_binding()
                                .and_then(|binding| binding.name_token().ok())
                                .as_ref()
                                .and_then(identifier_key);
                            let (value, steps) = match key {
                                Some(PropertyKey::Name(name)) => {
                                    if let Some(names) = &mut names {
                                        names.push(name.clone());
                                    }
                                    let field = DestructureField::Name { name, has_initializer: false };
                                    self.with_step(value, steps, field, property.init().as_ref())
                                }
                                Some(PropertyKey::Private(_)) | None => {
                                    names = None;
                                    (None, Vec::new())
                                }
                            };
                            flow = self.write_binding(&binding, value, &steps, flow)?;
                        }
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(rest) => {
                            let Ok(binding) = rest.binding() else {
                                continue;
                            };
                            let (value, steps) = match names.take() {
                                Some(names) => {
                                    let field = DestructureField::RestExcept(names.into());
                                    self.with_step(value, steps, field, None)
                                }
                                None => (None, Vec::new()),
                            };
                            flow = self.write_binding(&binding, value, &steps, flow)?;
                        }
                        AnyJsObjectBindingPatternMember::JsBogusBinding(_)
                        | AnyJsObjectBindingPatternMember::JsMetavariable(_) => {
                            return Err(Unsupported);
                        }
                    }
                }
            }
        }
        Ok(flow)
    }

    /// Adds a write point of the variable that `binding` declares after
    /// `incoming`, which assigns it `value` read through `steps`, and returns
    /// the flow after it.
    fn write_binding(
        &mut self,
        binding: &AnyJsBinding,
        value: Option<FlowAssignedValue>,
        steps: &[FlowDestructureStep],
        incoming: FlowNodeId,
    ) -> Build<FlowNodeId> {
        let Some(binding) = binding.as_js_identifier_binding() else {
            return Ok(incoming);
        };
        let subject = self
            .subjects
            .intern(FlowSubject::Binding(binding.range()))
            .ok_or(Unsupported)?;
        let effect = FlowWriteEffect::Assign(self.assigned_value(value, steps)?);
        self.push_write(incoming, FlowWrite { subject, effect })
    }

    /// Evaluates `pattern`, the binding pattern of a declaration, a
    /// parameter, or a `catch` clause, from `incoming` and returns the flow
    /// after it, before any variable that it declares is written. `pattern`
    /// lies at `depth`. A declaration evaluates its pattern before its
    /// initializer, so `incoming` is then the flow before the initializer.
    ///
    /// The elements run in source order. Each element evaluates its computed
    /// key, then its default, then its nested pattern. A default runs only
    /// when the value is `undefined`.
    fn binding_pattern(
        &mut self,
        pattern: &AnyJsBindingPattern,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        self.visit(depth)?;
        let mut flow = incoming;
        match pattern {
            AnyJsBindingPattern::AnyJsBinding(_) => {}
            AnyJsBindingPattern::JsArrayBindingPattern(pattern) => {
                for element in pattern.elements().iter().flatten() {
                    flow = match element {
                        AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(element) => {
                            self.binding_element(
                                element.pattern().ok(),
                                element.init(),
                                flow,
                                depth + 1,
                            )?
                        }
                        AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(
                            element,
                        ) => self.binding_element(element.pattern().ok(), None, flow, depth + 1)?,
                        AnyJsArrayBindingPatternElement::JsArrayHole(_) => flow,
                    };
                }
            }
            AnyJsBindingPattern::JsObjectBindingPattern(pattern) => {
                for property in pattern.properties().iter().flatten() {
                    flow = match property {
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(
                            property,
                        ) => {
                            let flow = self.member_name(property.member().ok(), flow, depth + 1)?;
                            self.binding_element(
                                property.pattern().ok(),
                                property.init(),
                                flow,
                                depth + 1,
                            )?
                        }
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                            property,
                        ) => self.default_value(property.init(), flow, depth + 1)?,
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(_) => flow,
                        AnyJsObjectBindingPatternMember::JsBogusBinding(_)
                        | AnyJsObjectBindingPatternMember::JsMetavariable(_) => {
                            return Err(Unsupported);
                        }
                    };
                }
            }
        }
        Ok(flow)
    }

    /// Evaluates one element of a binding pattern or one parameter from
    /// `incoming` and returns the flow after it. The element evaluates its
    /// default `default`, then its pattern `pattern`.
    fn binding_element(
        &mut self,
        pattern: Option<AnyJsBindingPattern>,
        default: Option<JsInitializerClause>,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        let flow = self.default_value(default, incoming, depth)?;
        match pattern {
            Some(pattern) => self.binding_pattern(&pattern, flow, depth),
            None => Ok(flow),
        }
    }

    /// Evaluates the expression of `name` when it is a computed property name,
    /// as `k` in `{ [k]: v }`, from `incoming`, and returns the flow after it.
    fn member_name(
        &mut self,
        name: Option<AnyJsObjectMemberName>,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        match name {
            Some(AnyJsObjectMemberName::JsComputedMemberName(name)) => match name.expression() {
                Ok(expression) => self.evaluate(expression.syntax(), incoming, depth + 1),
                Err(_) => Ok(incoming),
            },
            Some(AnyJsObjectMemberName::JsMetavariable(_)) => Err(Unsupported),
            Some(AnyJsObjectMemberName::JsLiteralMemberName(_)) | None => Ok(incoming),
        }
    }

    /// Evaluates `default`, the default of a destructuring element or of a
    /// parameter, from `incoming`, and returns the flow after it.
    ///
    /// A default runs only when the value is `undefined`, so the flow after
    /// it joins `incoming` with the end of the default, as in TypeScript. In
    /// `const { a = x && f(x) } = o`, for example, `f(x)` sees `x` as truthy,
    /// and the code after the declaration doesn't.
    fn default_value(
        &mut self,
        default: Option<JsInitializerClause>,
        incoming: FlowNodeId,
        depth: usize,
    ) -> Build<FlowNodeId> {
        let Some(expression) = default.and_then(|default| default.expression().ok()) else {
            return Ok(incoming);
        };
        let flow = self.evaluate(expression.syntax(), incoming, depth + 1)?;
        self.join_two(incoming, flow)
    }

    /// Returns the flow where `first` and `second` meet.
    fn join_two(&mut self, first: FlowNodeId, second: FlowNodeId) -> Build<FlowNodeId> {
        if first == second {
            Ok(first)
        } else {
            self.push(FlowNode::Join(vec![first, second]))
        }
    }

    /// Returns the flow where `expression`, a condition, has `outcome` after
    /// `antecedent`, or `None` when the branch is unreachable because the
    /// condition never has a truthy or falsy `outcome` (see
    /// [`never_has_outcome`]). A condition written as the literal `true` or
    /// `false` narrows nothing, so the branch that it takes continues from
    /// `antecedent`, as in TypeScript.
    fn branch(
        &mut self,
        antecedent: FlowNodeId,
        expression: &AnyJsExpression,
        outcome: FlowOutcome,
    ) -> Build<Option<FlowNodeId>> {
        let truthy = match outcome {
            FlowOutcome::Truthy => true,
            FlowOutcome::Falsy => false,
            FlowOutcome::Nullish | FlowOutcome::NonNullish => {
                return self.condition(antecedent, expression, outcome).map(Some);
            }
        };
        if never_has_outcome(expression, truthy) {
            return Ok(None);
        }
        if literal_condition(expression).is_some() {
            return Ok(Some(antecedent));
        }
        self.condition(antecedent, expression, outcome).map(Some)
    }

    /// Returns the flow after an expression whose two branches end at
    /// `first` and `second`, `None` for a branch that no path takes, or
    /// `fallback` when neither is taken.
    fn join_branches(
        &mut self,
        fallback: FlowNodeId,
        first: Option<FlowNodeId>,
        second: Option<FlowNodeId>,
    ) -> Build<FlowNodeId> {
        match (first, second) {
            (Some(first), Some(second)) => self.push(FlowNode::Join(vec![first, second])),
            (Some(flow), None) | (None, Some(flow)) => Ok(flow),
            (None, None) => Ok(fallback),
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

    /// Adds a node where `condition` holds after `antecedent` and returns the
    /// flow after it. Adds no node when the condition can narrow no subject,
    /// as for the object `f()` of `f()?.g(x)`.
    fn push_narrowing_condition(
        &mut self,
        antecedent: FlowNodeId,
        condition: DecomposedCondition,
    ) -> Build<FlowNodeId> {
        if condition.mentions.is_empty()
            && condition.parents.is_empty()
            && condition.call_mentions.is_empty()
            && self.guards.get(condition.guard) == Some(&FlowGuard::Keep)
        {
            return Ok(antecedent);
        }
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
        for subject in &condition.call_mentions {
            if !condition.mentions.contains(subject) && !condition.parents.contains(subject) {
                self.call_mentions.insert((node, *subject));
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
                .chain(condition.call_mentions)
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
                true,
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
            let start = self.point(name, occurrence).expect("read must be recorded");
            self.paths_from(start)
        }

        /// Lists the tests that each acyclic path from the start to `start`
        /// assumed, as [`Self::paths`] does for a read.
        fn paths_from(&self, start: FlowNodeId) -> Vec<Vec<String>> {
            let root = self.root();
            let mut result = Vec::new();
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
                    // A write of `x` is written `x:Write`, with the kind of
                    // its effect.
                    FlowNode::Write { antecedent, write } => {
                        let write = &root.writes[*write as usize];
                        let effect = match write.effect {
                            FlowWriteEffect::Assign(Some(_)) => "Assign",
                            FlowWriteEffect::Assign(None) => "Reset",
                            FlowWriteEffect::CompoundLike(_) => "CompoundLike",
                            FlowWriteEffect::Compound => "Compound",
                            FlowWriteEffect::Undefined => "Undefined",
                            FlowWriteEffect::Reset => "Reset",
                            FlowWriteEffect::Wildcard => "Wildcard",
                            FlowWriteEffect::Elements(Some(_)) => "Elements",
                            FlowWriteEffect::Elements(None) => "UnknownElements",
                            FlowWriteEffect::EmptyArray => "EmptyArray",
                        };
                        facts.push(format!("{}:{effect}", self.subject_name(write.subject)));
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
                FlowGuard::Unmatched(inner) => self.facts(*inner, facts),
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
                // A call point is written `call(callee)` for a call whose
                // result is truthy and `!call(callee)` otherwise. Its effect
                // depends on its callee, which only inference resolves.
                FlowGuard::Call { call, positive } => {
                    let root = self.root();
                    let (call, _) = &root.calls[*call as usize];
                    let text = &self.source[call.range];
                    let callee = &text[..text.find('(').unwrap_or(text.len())];
                    let negation = if *positive { "" } else { "!" };
                    facts.push(format!("{negation}call({callee})"));
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
                FlowTest::Literal { value, strict } => {
                    format!("{}{}", if strict { "=" } else { "==" }, &self.source[value])
                }
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
            "function f(x, y) { for (const v of y) { x && read(x); } after(x); }",
            "function f(x) { for (const k in x) { read(x); } after(x); }",
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
    fn loops_exit_unless_their_test_is_the_literal_true() {
        for source in [
            "function f(x) { while (true) { read(x); } after(x); }",
            "function f(x) { do { read(x); } while (true); after(x); }",
            "function f(x) { for (;;) { read(x); } after(x); }",
            "function f(x) { for (; true;) { read(x); } after(x); }",
            "function f(x, c) { while (c || true) { read(x); } after(x); }",
        ] {
            assert_eq!(function(source).point("x", 2), None, "{source}");
        }
        for source in [
            "function f(x) { while (1) { read(x); } after(x); }",
            r#"function f(x) { while ("a") { read(x); } after(x); }"#,
            "function f(x) { while ((true)) { read(x); } after(x); }",
            "function f(x) { while (!0) { read(x); } after(x); }",
            "function f(x) { do { read(x); } while (1); after(x); }",
            "function f(x) { for (; 1;) { read(x); } after(x); }",
        ] {
            assert!(function(source).point("x", 2).is_some(), "{source}");
        }
    }

    #[test]
    fn branches_that_a_literal_condition_never_takes_have_no_flow() {
        for source in [
            "function f(x) { if (false) { read(x); } after(x); }",
            "function f(x) { if (true) {} else { read(x); } after(x); }",
            "function f(x) { while (false) { read(x); } after(x); }",
            "function f(x) { for (; false;) { read(x); } after(x); }",
            "function f(x) { false && read(x); after(x); }",
            "function f(x) { true || read(x); after(x); }",
            "function f(x) { false ? read(x) : 0; after(x); }",
            "function f(x, c) { if (c && false) { read(x); } after(x); }",
            "function f(x, c) { if (!(c && false)) {} else { read(x); } after(x); }",
        ] {
            let built = function(source);
            assert_eq!(built.point("x", 1), None, "{source}");
            assert!(built.point("x", 2).is_some(), "{source}");
        }
        for source in [
            "function f(x) { if ((false)) { read(x); } after(x); }",
            "function f(x) { if (!false) {} else { read(x); } after(x); }",
            "function f(x) { if (0) { read(x); } after(x); }",
            "function f(x) { false ?? read(x); after(x); }",
        ] {
            assert!(function(source).point("x", 1).is_some(), "{source}");
        }
    }

    #[test]
    fn for_in_and_for_of_loops_exit_from_their_header() {
        for source in [
            "function f(x, y) { for (const v of y) { if (!x) return; read(x); } after(x); }",
            "function f(x, y) { for (const v in y()) { if (!x) continue; read(x); } after(x); }",
        ] {
            let built = function(source);
            assert_eq!(built.paths("x", 2), [facts(&["x:Truthy"])], "{source}");
            // The test in the body never reaches the code after the loop.
            assert_eq!(built.paths("x", 3), [Vec::<String>::new()], "{source}");
        }
        let built = function("function f(x, y) { for (const v of y) { if (x) break; } after(x); }");
        assert_eq!(
            sorted_paths(&built, "x", 2),
            [Vec::new(), facts(&["x:Truthy"])]
        );
        let built =
            function("function f(x, y) { for (const v of y) { continue; dead(x); } after(x); }");
        assert_eq!(built.point("x", 1), None);
        assert_eq!(built.paths("x", 2), [Vec::<String>::new()]);
    }

    #[test]
    fn for_in_and_for_of_loops_evaluate_the_expression_once_and_the_head_each_time() {
        let built = function(
            "function f(x) { for (const { a = read(x) } of read(x)) { if (!x) continue; } }",
        );
        // The expression runs once, from the start, and the default in the
        // head runs at the header, which the body's `continue` and end reach.
        assert_eq!(built.point("x", 2), Some(0));
        let header = built.point("x", 1).unwrap();
        assert_eq!(built.point("x", 3), Some(header));
        assert!(matches!(
            &built.root().nodes[header as usize],
            FlowNode::Join(predecessors) if predecessors.len() == 3
        ));
    }

    #[test]
    fn for_in_bodies_assume_that_the_expression_is_not_nullish() {
        let built = function("function f(obj) { for (const k in obj) read(obj); after(obj); }");
        assert_eq!(built.paths("obj", 2), [facts(&["!obj:Nullish"])]);
        assert_eq!(built.paths("obj", 3), [Vec::<String>::new()]);
        // Each iteration also writes a `var`, which code before the loop can
        // test.
        let built = function("function f(obj) { for (var k in obj?.v) read(obj); }");
        assert_eq!(
            sorted_paths(&built, "obj", 2),
            [facts(&["!obj.v:Nullish", "!obj:Nullish", "k:Reset"])]
        );
        // Only a head that declares one variable narrows.
        for source in [
            "function f(obj) { for (k in obj) read(obj); }",
            "function f(obj) { for (const [k] in obj) read(obj); }",
            "function f(obj) { for (const k of obj) read(obj); }",
        ] {
            let built = function(source);
            assert_eq!(built.paths("obj", 2), [Vec::<String>::new()], "{source}");
        }
        // The test of `obj.v` adds no guard on `obj`, which would be written
        // `!obj[v]:Nullish`.
        let built = function("function f(obj) { for (const k in obj.v) read(obj); }");
        assert_eq!(built.paths("obj", 2), [facts(&["!obj.v:Nullish"])]);
    }

    #[test]
    fn optional_chains_test_their_receivers_before_keys_and_arguments() {
        let built = function("function f(x, y) { if (y) x?.read(x); after(x); }");
        assert_eq!(
            sorted_paths(&built, "x", 2),
            [facts(&["!x:Nullish", "y:Truthy"])]
        );
        // The code after the chain continues from the flow after its root.
        assert_eq!(
            sorted_paths(&built, "x", 3),
            [facts(&["!y:Truthy"]), facts(&["y:Truthy"])]
        );
        for source in [
            "function f(x) { x?.[read(x)]; }",
            "function f(x) { x?.(x); }",
            "function f(x) { x?.a.b.read(x); }",
            "function f(x) { x?.read(0).then(x); }",
        ] {
            let built = function(source);
            assert_eq!(built.paths("x", 1), [Vec::<String>::new()], "{source}");
            assert_eq!(built.paths("x", 2), [facts(&["!x:Nullish"])], "{source}");
        }
        // Each `?.` tests its own object. The test of `x.a` also tests `x`
        // through its property `a`.
        let built = function("function f(x) { x?.a?.read(x, x.a); }");
        let both = [facts(&["!x.a:Nullish", "!x:Nullish", "!x[a]:Nullish"])];
        assert_eq!(sorted_paths(&built, "x", 2), both);
        assert_eq!(sorted_paths(&built, "x.a", 0), both);
        // Only a `?.` that a key or arguments follow tests its object, and an
        // object that is no subject adds no condition.
        for source in [
            "function f(x) { x?.a.b; x; }",
            "function f(x) { x?.read(); x; }",
            "function f(x) { (x?.a).read(x); }",
            "function f(x) { g()?.read(x); }",
        ] {
            assert_eq!(function(source).root().nodes.len(), 1, "{source}");
        }
    }

    #[test]
    fn nested_optional_chains_start_from_the_test_of_their_enclosing_chain() {
        let built = function("function f(x) { x?.read(x?.read(x)); }");
        assert_eq!(built.paths("x", 2), [facts(&["!x:Nullish"])]);
        assert_eq!(built.paths("x", 3), [facts(&["!x:Nullish", "!x:Nullish"])]);
    }

    #[test]
    fn defaults_join_the_flow_before_them() {
        let built = function("function f(x, o) { const { a = x && read(x) } = o; after(x); }");
        assert_eq!(built.paths("x", 2), [facts(&["x:Truthy"])]);
        assert_eq!(
            sorted_paths(&built, "x", 3),
            [Vec::new(), facts(&["!x:Truthy"]), facts(&["x:Truthy"])]
        );
        // A default without a condition adds no node.
        let built = function("function f(x, o) { const { a = read(x) } = o; }");
        assert_eq!(built.root().nodes.len(), 1);
    }

    #[test]
    fn declarations_evaluate_their_pattern_then_their_initializer() {
        // As in TypeScript's binder, the pattern runs before the initializer.
        let built = function(
            "function f(x, o) { const { [read(x)]: { b = read(x) } = read(x), ...rest } = x && o; }",
        );
        for occurrence in 1..=3 {
            assert_eq!(
                sorted_paths(&built, "x", occurrence),
                [Vec::<String>::new()],
                "{occurrence}"
            );
        }
        let built = function("function f(x, o) { for (let [a = x && read(x)] = o; ;) break; }");
        assert_eq!(built.paths("x", 2), [facts(&["x:Truthy"])]);
        // A default that writes a variable runs conditionally, before the
        // initializer, so the initializer sees both paths.
        let built = function("function f(x) { const [b = g(x = 1)] = read(x); }");
        assert_eq!(
            sorted_paths(&built, "x", 2),
            [Vec::new(), facts(&["x:Assign"])]
        );
    }

    #[test]
    fn annotated_declarators_write_their_variables_after_the_initializer() {
        let built = function(
            "function f(c) { let x: string | number = c; const [y, { z }]: [1, { z: 2 }] = c; read(x, y, z); }",
        );
        assert!(contains(&built.paths("x", 1), "x:Assign"));
        assert!(contains(&built.paths("y", 1), "y:Assign"));
        assert!(contains(&built.paths("z", 2), "z:Assign"));
        // The initializer equals the declared type of an unannotated
        // variable, so the declaration adds no write point.
        let built = function("function f(c) { let x = c; read(x); }");
        assert_eq!(built.paths("x", 1), [Vec::<String>::new()]);
        // A call to a local function without a return annotation assigns the
        // values that the function returns, when Biome types them as
        // TypeScript does. A function expression called where it is defined
        // assigns an undetermined value.
        let built = function(
            "function f() { let x: number | undefined = g(); read(x); function g() { return 1; } }",
        );
        assert_eq!(built.paths("x", 1), [facts(&["x:Assign"])]);
        let built = function("function f() { let x: number | undefined = (() => 1)(); read(x); }");
        assert_eq!(built.paths("x", 1), [facts(&["x:Reset"])]);
    }

    #[test]
    fn var_declarators_and_loop_heads_write_their_variables() {
        // A read can come before a `var` declarator, so a declarator without
        // an annotation writes its variables too, and so does each iteration
        // of a loop whose head declares them.
        let built = function(
            "function f(c, list) { read(vx, pa); var vx = c, { pa } = c; read(vx, pa); for (var vk of list) read(vk); }",
        );
        assert_eq!(built.paths("vx", 0), [Vec::<String>::new()]);
        for name in ["vx", "pa"] {
            assert_eq!(
                sorted_paths(&built, name, 2),
                [facts(&["call(read)", "pa:Reset", "vx:Reset"])],
                "{name}"
            );
        }
        assert!(contains(&built.paths("vk", 1), "vk:Reset"));
        let built =
            function("function f(c, list) { let y = c; for (const k of list) read(y, k); }");
        assert_eq!(built.paths("y", 1), [Vec::<String>::new()]);
        assert_eq!(built.paths("k", 1), [Vec::<String>::new()]);
    }

    #[test]
    fn parameters_run_before_the_body_in_order() {
        let built = function(
            "function f(x, y = x && read(x), { a = x || read(x) } = read(x), ...[b = read(x)]) { after(x); }",
        );
        assert_eq!(built.paths("x", 2), [facts(&["x:Truthy"])]);
        assert!(contains(&built.paths("x", 4), "!x:Truthy"));
        // The default of a parameter runs before the defaults of its pattern.
        assert!(built.point("x", 5).unwrap() < built.point("x", 4).unwrap());
        assert!(built.paths("x", 6).len() > 1);
        assert!(built.paths("x", 7).iter().any(Vec::is_empty));
        for (source, kind) in [
            (
                "const g = (x, y = x && read(x)) => x;",
                JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION,
            ),
            (
                "class K { m(x, y = x && read(x)) {} }",
                JsSyntaxKind::JS_METHOD_CLASS_MEMBER,
            ),
            (
                "class K { constructor(x, public y = x && read(x)) {} }",
                JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER,
            ),
        ] {
            assert_eq!(
                build(source, kind).paths("x", 2),
                [facts(&["x:Truthy"])],
                "{source}"
            );
        }
    }

    #[test]
    fn destructuring_assignments_evaluate_their_pattern_then_their_value() {
        // As in TypeScript's binder, the pattern, with its defaults, runs
        // before the value, and the targets are written after the value.
        let built = function(
            "function f(x, o, b) { ({ [read(x)]: o[read(x)] = read(x), b = x && read(x) } = x && o); read(b); }",
        );
        for occurrence in 1..=3 {
            assert_eq!(
                sorted_paths(&built, "x", occurrence),
                [Vec::<String>::new()],
                "{occurrence}"
            );
        }
        assert!(contains(&built.paths("x", 5), "x:Truthy"));
        assert!(contains(&built.paths("b", 2), "b:Assign"));
        let built = function("function f(x, y) { [y = x && read(x)] = read(x); after(y); }");
        assert_eq!(built.paths("x", 2), [facts(&["x:Truthy"])]);
        assert_eq!(
            sorted_paths(&built, "x", 3),
            [facts(&["!x:Truthy"]), facts(&["x:Truthy"])]
        );
        assert!(contains(&built.paths("y", 2), "y:Assign"));
        // A write in a default holds before the value.
        let built = function("function f(x, y) { [y = (x = 1)] = read(x); }");
        assert_eq!(built.paths("x", 2), [facts(&["x:Assign"])]);
    }

    #[test]
    fn writes_follow_the_operators_that_make_them() {
        for (body, fact) in [
            ("x = 1", "x:Assign"),
            ("x = x + 1", "x:CompoundLike"),
            ("x = (x * 2)", "x:CompoundLike"),
            ("x += 1", "x:Compound"),
            ("x++", "x:Compound"),
            ("--x", "x:Compound"),
            ("(x) = 1", "x:Assign"),
            ("x! = 1", "x:Assign"),
            ("(x as any) = 1", "x:Reset"),
            ("x = typeof x", "x:Reset"),
            // Each iteration writes the target, and the loop exits before it.
            ("for (x of xs) read(x)", "x:Assign"),
            ("for (x in xs) read(x)", "x:Assign"),
            ("o.x = 1", "o.x:Assign"),
            ("o['x'] = 1", "o.x:Assign"),
            ("o[k] = 1", "o:Wildcard"),
            ("delete o.x", "o.x:Undefined"),
            ("delete o['x']", "o.x:Reset"),
            ("delete o[k]", "o:Wildcard"),
        ] {
            let built = function(&format!(
                "function f(x, o, xs, k) {{ {body}; read(x); read(o.x); }}"
            ));
            let (path, read) = if fact.starts_with('o') {
                ("o.x", "read(o.x)")
            } else {
                ("x", "read(x)")
            };
            let start = built.source.find(read).unwrap() + "read(".len();
            let occurrence = built
                .source
                .match_indices(path)
                .position(|(index, _)| index == start)
                .unwrap();
            assert!(
                built
                    .paths(path, occurrence)
                    .iter()
                    .flatten()
                    .any(|candidate| candidate == fact),
                "{body}: {:?}",
                built.paths(path, occurrence)
            );
        }
    }

    #[test]
    fn evolving_declarations_write_their_initializers_and_their_elements() {
        // A variable whose type evolves with its writes is written by its
        // initializer, and an array by each value that a call or an element
        // assignment with `=` adds to it.
        let built = function(
            "function f(v, i) { let x = null; var y = undefined; read(x, y); let q = []; q.push(v, ...v); q[0] = v; q[i] = v; q['k'] = v; q[0] += v; q = []; read(q); }",
        );
        assert_eq!(built.paths("x", 1), [facts(&["y:Assign", "x:Assign"])]);
        assert_eq!(
            sorted_paths(&built, "q", 7),
            [facts(&[
                "call(read)",
                "q.0:Assign",
                "q.0:Compound",
                "q.k:Assign",
                "q:Elements",
                "q:Elements",
                "q:EmptyArray",
                "q:EmptyArray",
                "q:UnknownElements",
                "q:UnknownElements",
                "q:Wildcard",
                "x:Assign",
                "y:Assign",
            ])]
        );
        // `push` on an array that doesn't evolve is an ordinary call point,
        // and an empty array that another variable declares restarts nothing.
        let built = function("function f(w: number[], v) { w.push(v); w = []; read(w); }");
        assert_eq!(built.paths("w", 3), [facts(&["w:Assign", "call(w.push)"])]);
        // A value that reads a variable whose type evolves with its writes
        // has Biome's type for that variable, which doesn't follow its
        // writes, so the write gives the declared type.
        let built = function(
            "function f(c) { let e = null; e = c; let y: string | null = c; y = e; read(y); }",
        );
        assert!(contains(&built.paths("y", 2), "y:Reset"));
    }

    #[test]
    fn assignments_in_other_scopes_have_values_that_declared_types_cover() {
        // The declared type of a variable whose type evolves with its writes
        // includes the values of `=` and logical assignments in scopes other
        // than its declaration. Values that Biome may type narrower than
        // TypeScript for a reason other than an inferred return type, as
        // `typeof`, a read of such a variable, and a call of a local function
        // that returns such a read, are left out, and so are compound and
        // compound-like assignments.
        let built = function(
            "function f(c, o, n) { let x = null; let y = null; x = o.m(); if (c) { x = o.m(); x ??= h(); x = typeof c; x = y; x = n + 1; x += 1; x = g(); x = k(); } { let z = null; z = o.m(); read(z); } read(x); function g() { return y; } function k() { return 1; } }",
        );
        let covered: Vec<_> = built
            .root()
            .assigned
            .iter()
            .filter(|assigned| assigned.covered)
            .map(|assigned| match assigned.value {
                FlowAssignedValue::Expression(range) | FlowAssignedValue::BooleanLiteral(range) => {
                    &built.source[range]
                }
                FlowAssignedValue::IterableValue(_) | FlowAssignedValue::Key => "",
            })
            .collect();
        assert_eq!(covered, ["o.m()", "h()", "k()"]);
    }

    #[test]
    fn writes_reach_the_exceptions_of_their_try_block() {
        let built = function("function f(x) { try { x = 1; g(); } catch { read(x); } }");
        assert_eq!(
            sorted_paths(&built, "x", 2),
            [Vec::new(), facts(&["x:Assign"])]
        );
    }

    #[test]
    fn optional_chains_with_writes_join_their_short_circuits() {
        let built = function("function f(x, o) { o?.f(x = 1); after(x); }");
        assert_eq!(
            sorted_paths(&built, "x", 2),
            [Vec::new(), facts(&["!o:Nullish", "x:Assign"])]
        );
    }

    #[test]
    fn logical_assignments_evaluate_their_right_operand_from_a_test_of_their_target() {
        for (operator, right, skip) in [
            ("&&=", "x:Truthy", "!x:Truthy"),
            ("||=", "!x:Truthy", "x:Truthy"),
            ("??=", "x:Nullish", "!x:Nullish"),
        ] {
            // The target is the occurrence of `x` before the operator.
            let built = function(&format!(
                "function f(x) {{ x {operator} read(x); after(x); }}"
            ));
            assert_eq!(built.paths("x", 2), [facts(&[right])], "{operator}");
            // The target is written where the right operand runs.
            let mut written = facts(&["x:Assign", right]);
            written.sort();
            let mut joined = vec![facts(&[skip]), written];
            joined.sort();
            assert_eq!(sorted_paths(&built, "x", 3), joined, "{operator}");
        }
        // The object and the key of the target run before the test, and a
        // test of a property also tests its object.
        let built = function("function f(s, k) { s[read(k)].v ||= read(s); }");
        assert_eq!(built.paths("k", 1), [Vec::<String>::new()]);
        let built = function("function f(s) { s.kind ||= read(s); }");
        assert_eq!(
            sorted_paths(&built, "s", 2),
            [facts(&["!s.kind:Truthy", "!s[kind]:Truthy"])]
        );
        // A compound assignment evaluates its target, then its right operand.
        let built = function("function f(x, o) { o[x && read(x)] += read(x); }");
        assert_eq!(built.paths("x", 3).len(), 2);
        let built = function("function f(s) { s.kind ||= read(s); after(s); }");
        assert!(contains(&built.paths("s", 3), "s.kind:Assign") || built.paths("s", 3).len() == 2);
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
        const SOURCE: &str = "class K { #w; m() { if (this.v) read(this.v); if (this.#w) read(this.#w); } constructor() { if (this.v) read(this.v); this.v = 1; read(this.v); } static s() { if (this.v) read(this.v); } }";
        let method = build(SOURCE, JsSyntaxKind::JS_METHOD_CLASS_MEMBER);
        assert!(contains(&method.paths("this.v", 1), "this.v:Truthy"));
        assert!(contains(&method.paths("this.#w", 1), "this.#w:Truthy"));
        let constructor = build(SOURCE, JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER);
        assert!(contains(&constructor.paths("this.v", 3), "this.v:Truthy"));
        assert!(contains(&constructor.paths("this.v", 5), "this.v:Assign"));
        let method = build(
            "class K { static s() { if (this.v) read(this.v); } }",
            JsSyntaxKind::JS_METHOD_CLASS_MEMBER,
        );
        assert!(method.root().mentions.is_empty());
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
            "function f(q, y) { if (q) { class C extends read(q) {} enum E { A = read(q) } after(q); } }",
        );
        for occurrence in 2..=4 {
            assert_eq!(
                built.paths("q", occurrence),
                [vec!["q:Truthy".to_owned()]],
                "{occurrence}"
            );
        }
    }

    #[test]
    fn catch_clauses_start_from_the_flow_before_try() {
        let built = function(
            "function f(x) { try { if (!x) return; read(x); } catch { read(x); } after(x); }",
        );
        assert_eq!(built.paths("x", 2), [facts(&["x:Truthy"])]);
        assert_eq!(built.paths("x", 3), [Vec::<String>::new()]);
        // The code after the statement continues from the ends of the try
        // block and of the catch clause, after the call points there.
        assert_eq!(
            sorted_paths(&built, "x", 4),
            [facts(&["call(read)"]), facts(&["call(read)", "x:Truthy"])]
        );
        // A catch clause that ends the path leaves the try block's end.
        let built = function(
            "function f(x) { try { if (!x) throw x; } catch ({ a = read(x) }) { return; } after(x); }",
        );
        assert_eq!(built.paths("x", 3), [Vec::<String>::new()]);
        assert_eq!(built.paths("x", 4), [facts(&["x:Truthy"])]);
    }

    #[test]
    fn finally_blocks_are_built_twice() {
        let built =
            function("function f(x) { try { if (!x) return; } finally { read(x); } after(x); }");
        // The first build starts from the end of the try block, its
        // exceptions, and its `return`.
        assert_eq!(
            sorted_paths(&built, "x", 2),
            [Vec::new(), facts(&["!x:Truthy"]), facts(&["x:Truthy"])]
        );
        let range = built.range("x", 2);
        assert_eq!(
            built
                .root()
                .reads
                .iter()
                .filter(|read| read.expression.range() == range)
                .count(),
            1
        );
        // The second build starts from the normal exits only, and the code
        // after the statement continues from its end.
        assert_eq!(built.paths("x", 3), [facts(&["call(read)", "x:Truthy"])]);
        let built = function(
            "function f(x, y) { try { if (!x) return; } catch { return; } finally { if (!y) return; } after(x); }",
        );
        assert_eq!(
            sorted_paths(&built, "x", 2),
            [facts(&["x:Truthy", "y:Truthy"])]
        );
        // Without a normal exit, the code after the statement is
        // unreachable.
        let built = function("function f(x) { try { return; } finally { read(x); } after(x); }");
        assert!(built.point("x", 1).is_some());
        assert_eq!(built.point("x", 2), None);
    }

    #[test]
    fn finally_blocks_reach_the_joins_of_the_enclosing_try() {
        let built =
            function("function f(x) { try { try { return; } finally { x; } } finally { x; } }");
        let inner = built.point("x", 1).unwrap();
        let outer = built.point("x", 2).unwrap();
        let FlowNode::Join(predecessors) = &built.root().nodes[outer as usize] else {
            panic!("a finally block starts at a join");
        };
        // The `return` passes through both finally blocks.
        assert!(predecessors.contains(&inner), "{predecessors:?}");
        // So does an exception, through the join of the outer try block.
        assert!(predecessors.iter().any(|predecessor| matches!(
            &built.root().nodes[*predecessor as usize],
            FlowNode::Join(exceptions) if exceptions.contains(&inner)
        )));
    }

    #[test]
    fn nested_finally_blocks_stop_at_the_node_limit() {
        let nested = |depth: usize| {
            format!(
                "function f(x) {{ if (x) read(x); {}{} }}",
                "try {} finally { ".repeat(depth),
                "}".repeat(depth)
            )
        };
        assert!(function(&nested(3)).root.is_some());
        // Each build of a finally block builds the one nested in it twice, so
        // the innermost one would be built 2^20 times.
        assert!(function(&nested(20)).root.is_none());
    }

    #[test]
    fn statements_that_evaluate_nothing_count_against_the_visit_limit() {
        let declarations: String = (0..100)
            .map(|index| format!("type T{index} = 0; "))
            .collect();
        let unreachable = format!("if (x) {{ return; {} }}", "x; ".repeat(100));
        for body in [declarations, unreachable] {
            assert!(
                function(&nested_finally_blocks(2, &body)).root.is_some(),
                "{body}"
            );
            // The innermost finally block is built 2^8 times, so its 100
            // statements are built more often than the visit limit allows,
            // although they evaluate nothing.
            assert!(
                function(&nested_finally_blocks(8, &body)).root.is_none(),
                "{body}"
            );
        }
    }

    /// Returns a function whose body tests `x`, then nests `depth` finally
    /// blocks around `body`.
    fn nested_finally_blocks(depth: usize, body: &str) -> String {
        format!(
            "function f(x) {{ if (x) read(x); {}{body} {} }}",
            "try {} finally { ".repeat(depth),
            "}".repeat(depth)
        )
    }

    #[test]
    fn jumps_leave_try_statements_with_the_flow_where_they_are() {
        let built = function(
            "function f(x, y) { while (y) { try { if (x) break; } catch {} return; } after(x); }",
        );
        assert_eq!(
            sorted_paths(&built, "x", 2),
            [facts(&["!y:Truthy"]), facts(&["x:Truthy", "y:Truthy"])]
        );
        // `continue` reaches the test of a `do...while` loop without passing
        // through the finally block.
        let built = function(
            "function f(x) { do { try { if (x) continue; } finally {} return; } while (read(x)); }",
        );
        assert_eq!(built.paths("x", 2), [facts(&["x:Truthy"])]);
    }

    #[test]
    fn labeled_breaks_leave_try_statements() {
        let built = function(
            "function f(x) { done: { try { if (x) break done; } catch {} return; } after(x); }",
        );
        assert_eq!(built.paths("x", 2), [facts(&["x:Truthy"])]);
    }

    #[test]
    fn each_jump_leaves_a_try_statement_from_its_own_flow() {
        let built = function(
            "function f(x, y) { while (true) { if (x) { try { if (y) break; break; } catch { break; } } return; } after(x); }",
        );
        assert_eq!(
            sorted_paths(&built, "x", 2),
            [
                facts(&["!y:Truthy", "x:Truthy"]),
                facts(&["x:Truthy"]),
                facts(&["x:Truthy", "y:Truthy"])
            ]
        );
    }

    #[test]
    fn jumps_to_statements_inside_a_try_statement_stay_inside_it() {
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
    fn statements_need_their_syntax_and_a_target_for_each_jump() {
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
        // The group of `2` and `3` also continues the statements of `1`,
        // after its call point.
        assert_eq!(
            sorted_paths(&built, "x", 3),
            [
                facts(&["call(read)", "x:=1"]),
                facts(&["x:=2"]),
                facts(&["x:=3"])
            ]
        );
        assert_eq!(
            sorted_paths(&built, "x", 4),
            [facts(&["!x:=1", "!x:=2", "!x:=3"])]
        );
        // With `default`, no path skips every case.
        assert_eq!(
            sorted_paths(&built, "x", 5),
            [
                facts(&["!x:=1", "!x:=2", "!x:=3", "call(read)"]),
                facts(&["call(read)", "call(read)", "x:=1"]),
                facts(&["call(read)", "x:=2"]),
                facts(&["call(read)", "x:=3"])
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
    fn class_expressions_evaluate_their_decorators_heritage_and_static_blocks() {
        let built = function(
            "function f(v) { if (v) { const C = @use(v) class extends use(v) { [v] = v; static w = v; m() { v; } static { v; } }; after(v); } }",
        );
        for occurrence in [2, 3, 8, 9] {
            assert!(
                contains(&built.paths("v", occurrence), "v:Truthy"),
                "{occurrence}"
            );
        }
        for occurrence in 4..=7 {
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

    #[test]
    fn inline_functions_run_where_they_are_called() {
        let built = function("function f(x) { if (x) { (() => { read(x); })(); } after(x); }");
        assert!(contains(&built.paths("x", 2), "x:Truthy"));
        // The code after the call continues from the end of the body and from
        // each `return`.
        let built = function("function f(x) { (() => { if (x) return; x; })(); after(x); }");
        assert_eq!(built.paths("x", 2), [vec!["!x:Truthy".to_owned()]]);
        assert_eq!(
            sorted_paths(&built, "x", 3),
            [vec!["!x:Truthy".to_owned()], vec!["x:Truthy".to_owned()]]
        );
        let built = function("function f(x) { (function () { if (!x) throw 0; })(); after(x); }");
        assert_eq!(built.paths("x", 2), [vec!["x:Truthy".to_owned()]]);
        // A parameter's default runs where the argument is `undefined`.
        let built = function("function f(x, y) { if (x) ((a = read(x)) => a)(y); }");
        assert!(contains(&built.paths("x", 2), "x:Truthy"));
    }

    #[test]
    fn statements_after_inline_functions_that_always_throw_are_unreachable() {
        let built =
            function("function f(x) { if (x) { (() => { throw 0; })(); read(x); } after(x); }");
        assert_eq!(built.point("x", 2), None);
        assert_eq!(built.paths("x", 3), [vec!["!x:Truthy".to_owned()]]);
    }

    #[test]
    fn returns_in_inline_functions_never_reach_finally_blocks_around_the_call() {
        let built = function(
            "function f(x) { try { (() => { if (x) return; x; })(); after(x); } finally { last(x); } }",
        );
        assert_eq!(
            sorted_paths(&built, "x", 3),
            [vec!["!x:Truthy".to_owned()], vec!["x:Truthy".to_owned()]]
        );
        // Each build of a finally block joins the returns of the inline
        // function that it calls.
        let built = function(
            "function f(x) { try { c; } finally { (() => { if (x) return; })(); x; } after(x); }",
        );
        for occurrence in [2, 3] {
            let mut paths = sorted_paths(&built, "x", occurrence);
            paths.dedup();
            assert_eq!(
                paths,
                [vec!["!x:Truthy".to_owned()], vec!["x:Truthy".to_owned()]],
                "{occurrence}"
            );
        }
    }

    #[test]
    fn optional_inline_calls_run_their_body_before_their_arguments() {
        let built =
            function("function f(x) { (() => { if (!x) throw 0; })?.(read(x)); after(x); }");
        assert_eq!(built.paths("x", 2), [vec!["x:Truthy".to_owned()]]);
        assert_eq!(built.paths("x", 3), [vec!["x:Truthy".to_owned()]]);
        let built = function("function f(x) { (() => { if (!x) throw 0; })(read(x)); after(x); }");
        assert_eq!(built.paths("x", 2), [Vec::<String>::new()]);
        assert_eq!(built.paths("x", 3), [vec!["x:Truthy".to_owned()]]);
    }

    #[test]
    fn static_blocks_run_where_their_class_is_evaluated() {
        let built = function(
            "function f(v) { class C extends use(v) { static { if (!v) throw 0; } m() { v; } } after(v); }",
        );
        assert_eq!(built.paths("v", 1), [Vec::<String>::new()]);
        assert_eq!(built.point("v", 3), None);
        assert_eq!(built.paths("v", 4), [vec!["v:Truthy".to_owned()]]);
    }

    #[test]
    fn inline_code_where_the_root_doesnt_run_it_stays_out_of_the_flow() {
        let built = function(
            "function f(x) { if (x) { class C { y = (() => read(x))(); static z = (() => read(x))(); } (async () => read(x))(); } }",
        );
        for occurrence in 2..=4 {
            assert_eq!(built.point("x", occurrence), None, "{occurrence}");
        }
    }

    #[test]
    fn closures_inside_inline_functions_capture_the_inline_flow() {
        let source = "function f(x) { if (x) { (() => { const g = () => x; })(); } }";
        let built = function(source);
        let captures = &built.root().captures;
        assert_eq!(captures.len(), 1);
        assert_eq!(captures[0].closure.range(), built.range("() => x", 0));
        assert!(contains(&built.paths_from(captures[0].point), "x:Truthy"));
    }

    #[test]
    fn this_in_inline_function_expressions_is_their_own() {
        let built = build(
            "class K { m() { if (this.v) { (function () { read(this.v); })(); (() => read(this.v))(); } } }",
            JsSyntaxKind::JS_METHOD_CLASS_MEMBER,
        );
        assert_eq!(built.point("this.v", 1), None);
        assert!(contains(&built.paths("this.v", 2), "this.v:Truthy"));
    }
}
