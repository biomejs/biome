use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_control_flow::{
    ExceptionHandlerKind, Instruction, InstructionKind, builder::ROOT_BLOCK_ID,
};
use biome_db::AnyParsedSource;
use biome_js_control_flow::{
    AnyJsControlFlowRoot, JsControlFlowGraph, control_flow_model, js_control_flow_model,
};
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyFunctionLike, AnyJsAssignment, AnyJsAssignmentPattern, AnyJsExpression, AnyTsType,
    JsAssignmentExpression, JsAssignmentOperator, JsAwaitExpression, JsConditionalExpression,
    JsIdentifierAssignment, JsLanguage, JsLogicalExpression, JsParenthesizedAssignment,
    JsParenthesizedExpression, JsPostUpdateExpression, JsPreUpdateExpression,
    JsPropertyClassMember, JsReferenceIdentifier, JsSyntaxNode, JsSyntaxToken, JsYieldExpression,
    TextSize, TsAsAssignment, TsNonNullAssertionAssignment, TsSatisfiesAssignment,
    TsTypeAssertionAssignment, assign_ext::AnyJsMemberAssignment,
    binding_ext::AnyJsBindingDeclaration, expr_ext::AnyJsMemberExpression,
};
use biome_languages::LanguageDb;
use biome_rowan::{AstNode, SyntaxKindSet, WalkEvent};
use biome_rule_options::use_atomic_updates::UseAtomicUpdatesOptions;
use roaring::RoaringBitmap;
use std::rc::Rc;

declare_lint_rule! {
    /// Disallow assignments that can be based on outdated values because of `await` or `yield`.
    ///
    /// A function pauses at every `await` and `yield`, and other code can run
    /// during the pause. If the function reads a variable, pauses, and then
    /// assigns a new value computed from what it read before the pause, the
    /// assignment can undo a change that other code made in the meantime.
    ///
    /// ```js,expect_diagnostic
    /// let total = 0;
    ///
    /// async function addToTotal(getAmount) {
    ///     // `total` is read here, before `getAmount()` finishes.
    ///     total += await getAmount();
    /// }
    ///
    /// // Both calls read `total` while it's still 0, so one of the amounts is lost.
    /// await Promise.all([addToTotal(getFirstAmount), addToTotal(getSecondAmount)]);
    /// ```
    ///
    /// The rule also reports assignments to properties of objects that other code
    /// can access, such as parameters and variables declared outside of the function.
    ///
    /// Variables that are declared inside the function and aren't used by any
    /// other function are ignored, because no other code can change them.
    /// Variables that aren't declared in the file, such as `window` or `process`,
    /// are ignored too.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let total = 0;
    ///
    /// async function addToTotal(getAmount) {
    ///     total = total + await getAmount();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let count = 0;
    ///
    /// async function refresh() {
    ///     const previous = count;
    ///     await reload();
    ///     count = previous + 1;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// async function addBonus(player) {
    ///     player.score += await fetchBonus();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let ids = [];
    ///
    /// function* collect() {
    ///     ids = ids.concat(yield);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let total = 0;
    ///
    /// async function addToTotal(getAmount) {
    ///     const amount = await getAmount();
    ///     total += amount;
    /// }
    ///
    /// async function sum(getAmount) {
    ///     let localTotal = 0;
    ///     localTotal += await getAmount();
    ///     return localTotal;
    /// }
    ///
    /// async function addBonus(player) {
    ///     const bonus = await fetchBonus();
    ///     player.score += bonus;
    /// }
    /// ```
    ///
    pub UseAtomicUpdates {
        version: "next",
        name: "useAtomicUpdates",
        language: "js",
        sources: &[RuleSource::Eslint("require-atomic-updates").inspired()],
        recommended: false,
    }
}

impl Rule for UseAtomicUpdates {
    type Query = Semantic<AnyFunctionLike>;
    type State = NonAtomicUpdate;
    type Signals = Box<[Self::State]>;
    type Options = UseAtomicUpdatesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let function = ctx.query();
        if !function.is_async() && !function.is_generator() {
            return Box::default();
        }
        let Some(root) = AnyJsControlFlowRoot::cast_ref(function.syntax()) else {
            return Box::default();
        };
        let Some(cfg) = control_flow_graph(ctx, &root) else {
            return Box::default();
        };
        let mut updates = Function {
            model: ctx.model(),
            root: &root,
        }
        .analyze(&cfg);
        // Blocks of loops are evaluated until their state stops changing, so
        // the same assignment can be found more than once.
        updates.sort_unstable_by_key(|update| {
            (
                update.assignment.syntax().text_trimmed_range().start(),
                update.variable.text_trimmed_range().start(),
            )
        });
        updates.dedup_by(|a, b| a.assignment == b.assignment && a.variable == b.variable);
        updates.into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, update: &Self::State) -> Option<RuleDiagnostic> {
        let name = update.variable.text_trimmed();
        let range = update.assignment.range();
        let diagnostic = if update.is_property {
            let left = update.assignment.left().ok()?;
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    <Emphasis>{AssignmentTarget(&left)}</Emphasis>" may be assigned based on an outdated state of "<Emphasis>{name}</Emphasis>"."
                },
            )
        } else {
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    <Emphasis>{name}</Emphasis>" may be reassigned based on an outdated value of "<Emphasis>{name}</Emphasis>"."
                },
            )
        };
        Some(
            diagnostic
            .note(markup! {
                "This function reads "<Emphasis>{name}</Emphasis>", then pauses at an "<Emphasis>"await"</Emphasis>" or "<Emphasis>"yield"</Emphasis>" before this assignment. Other code can change "<Emphasis>{name}</Emphasis>" during the pause, and this assignment would overwrite that change."
            })
            .note(markup! {
                "Store the result of the "<Emphasis>"await"</Emphasis>" or "<Emphasis>"yield"</Emphasis>" in a local variable first, then read "<Emphasis>{name}</Emphasis>" and assign the new value without pausing in between."
            }),
        )
    }
}

/// An assignment that may use a value of a variable read before the function paused.
pub struct NonAtomicUpdate {
    assignment: JsAssignmentExpression,
    /// The name of the variable read before the pause.
    variable: JsSyntaxToken,
    /// Whether the assignment writes a property of the variable, instead of the variable itself.
    is_property: bool,
}

/// Writes the source text of an assignment target, without leading or trailing trivia.
struct AssignmentTarget<'a>(&'a AnyJsAssignmentPattern);

impl biome_console::fmt::Display for AssignmentTarget<'_> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        self.0
            .syntax()
            .text_trimmed()
            .try_for_each_chunk(|chunk| fmt.write_str(chunk))
    }
}

/// Returns the control flow graph of the queried execution root.
fn control_flow_graph(
    ctx: &RuleContext<UseAtomicUpdates>,
    root: &AnyJsControlFlowRoot,
) -> Option<JsControlFlowGraph> {
    if let Some(db) = ctx.get_service::<Rc<dyn LanguageDb>>()
        && let Some(source) = ctx.get_service::<AnyParsedSource>()
    {
        js_control_flow_model(db.as_ref(), source).graph(root)
    } else {
        // Fix passes and standalone callers analyze trees that are not stored
        // in the database.
        control_flow_model(&ctx.root()).graph(root)
    }
}

/// The variables read since the last pause, and the ones read before it.
///
/// Variables are identified by the start of their declaration. A variable can
/// be in both sets after code paths merge.
#[derive(Clone, Default)]
struct FlowState {
    fresh: RoaringBitmap,
    outdated: RoaringBitmap,
}

impl FlowState {
    /// Merges `other` into `self`, returning whether `self` changed.
    fn join(&mut self, other: &Self) -> bool {
        let len = self.fresh.len() + self.outdated.len();
        self.fresh |= &other.fresh;
        self.outdated |= &other.outdated;
        self.fresh.len() + self.outdated.len() != len
    }

    fn read(&mut self, binding: &Binding) {
        let variable = variable_id(binding);
        self.fresh.insert(variable);
        self.outdated.remove(variable);
    }

    fn pause(&mut self) {
        let fresh = std::mem::take(&mut self.fresh);
        self.outdated |= fresh;
    }
}

fn variable_id(binding: &Binding) -> u32 {
    binding.range().start().into()
}

/// The states at the start of the blocks, and the blocks whose start state changed.
///
/// Every block has two start states: one for the normal execution, and one
/// for the execution of `finally` clauses that run because of a `return` or
/// an error. They are stored at the indices `2 * block` and `2 * block + 1`.
struct Worklist {
    entries: Vec<Option<FlowState>>,
    queue: Vec<u32>,
    queued: RoaringBitmap,
}

impl Worklist {
    /// Merges `state` into the start state of `block`, and queues `block` if it changed.
    fn propagate(&mut self, block: u32, unwinding: bool, state: &FlowState) {
        let key = block * 2 + u32::from(unwinding);
        let entry = &mut self.entries[key as usize];
        let changed = match entry {
            Some(entry) => entry.join(state),
            None => {
                *entry = Some(state.clone());
                true
            }
        };
        if changed && self.queued.insert(key) {
            self.queue.push(key);
        }
    }

    /// Returns the next queued block, whether it runs because of a `return` or
    /// an error, and its start state.
    fn pop(&mut self) -> Option<(u32, bool, FlowState)> {
        let key = self.queue.pop()?;
        self.queued.remove(key);
        Some((key / 2, key % 2 == 1, self.entries[key as usize].clone()?))
    }
}

/// An operand of a conditional or logical expression that may not run, and
/// the state saved to merge the code paths at the end of the expression.
struct Branch {
    /// The start of the right operand, or of the consequent and the alternate
    /// of a conditional expression.
    starts: [Option<TextSize>; 2],
    /// The number of `starts` the evaluation already reached.
    reached: usize,
    /// The state before the first branch, or at the end of the consequent once
    /// the alternate is reached.
    saved: Option<FlowState>,
}

impl Branch {
    fn new(node: &JsSyntaxNode) -> Option<Self> {
        let start = |node: &JsSyntaxNode| Some(node.text_trimmed_range().start());
        let starts = if let Some(conditional) = JsConditionalExpression::cast_ref(node) {
            [
                start(conditional.consequent().ok()?.syntax()),
                start(conditional.alternate().ok()?.syntax()),
            ]
        } else if let Some(logical) = JsLogicalExpression::cast_ref(node) {
            [start(logical.right().ok()?.syntax()), None]
        } else if let Some(assignment) = JsAssignmentExpression::cast_ref(node)
            && is_logical_assignment(&assignment)
        {
            [start(assignment.right().ok()?.syntax()), None]
        } else {
            return None;
        };
        Some(Self {
            starts,
            reached: 0,
            saved: None,
        })
    }

    /// Updates the saved state before the evaluation reaches `position`.
    fn advance(&mut self, position: TextSize, state: &mut FlowState) {
        while let Some(Some(start)) = self.starts.get(self.reached)
            && *start <= position
        {
            match &mut self.saved {
                Some(saved) => std::mem::swap(saved, state),
                None => self.saved = Some(state.clone()),
            }
            self.reached += 1;
        }
    }

    /// Merges the code paths at the end of the expression.
    fn finish(self, state: &mut FlowState) {
        if let Some(saved) = self.saved {
            state.join(&saved);
        }
    }
}

/// The analyzed function.
struct Function<'a> {
    model: &'a SemanticModel,
    root: &'a AnyJsControlFlowRoot,
}

impl Function<'_> {
    /// Propagates the states through `cfg` until they stop changing, and
    /// returns the assignments that may use outdated values.
    fn analyze(&self, cfg: &JsControlFlowGraph) -> Vec<NonAtomicUpdate> {
        let mut updates = Vec::new();
        let mut worklist = Worklist {
            entries: vec![None; cfg.blocks.len() * 2],
            queue: Vec::new(),
            queued: RoaringBitmap::new(),
        };
        worklist.propagate(ROOT_BLOCK_ID.index(), false, &FlowState::default());

        while let Some((block, unwinding, mut state)) = worklist.pop() {
            let data = &cfg.blocks[block as usize];
            let propagate_exception = |worklist: &mut Worklist, state: &FlowState| {
                for handler in &data.exception_handlers {
                    let unwinding = handler.kind == ExceptionHandlerKind::Finally;
                    worklist.propagate(handler.target.index(), unwinding, state);
                }
            };
            // Any expression of the block may throw.
            propagate_exception(&mut worklist, &state);
            for instruction in &data.instructions {
                self.evaluate(instruction, &mut state, &mut updates);
                propagate_exception(&mut worklist, &state);
                match instruction.kind {
                    InstructionKind::Statement => {}
                    InstructionKind::Jump {
                        finally_fallthrough: true,
                        ..
                    } if unwinding => {
                        // A `finally` clause that runs because of a `return` or an
                        // error continues with the enclosing `finally` clauses
                        // instead of the code after the `try` statement.
                        for handler in &data.cleanup_handlers {
                            worklist.propagate(handler.target.index(), true, &state);
                        }
                        break;
                    }
                    InstructionKind::Jump {
                        conditional, block, ..
                    } => {
                        worklist.propagate(block.index(), unwinding, &state);
                        if !conditional {
                            break;
                        }
                    }
                    InstructionKind::Return => {
                        // `finally` clauses run before leaving the function.
                        for handler in &data.cleanup_handlers {
                            worklist.propagate(handler.target.index(), true, &state);
                        }
                        break;
                    }
                }
            }
        }
        updates
    }

    /// Updates `state` with the reads, pauses, and writes of the node evaluated
    /// by `instruction`, in execution order. Appends to `updates` the
    /// assignments that may use outdated values.
    ///
    /// Nested functions, class fields, and TypeScript types are skipped, because
    /// they don't run when the node is evaluated.
    fn evaluate(
        &self,
        instruction: &Instruction<JsLanguage>,
        state: &mut FlowState,
        updates: &mut Vec<NonAtomicUpdate>,
    ) {
        let Some(node) = instruction.node.as_ref().and_then(|node| node.as_node()) else {
            return;
        };
        let mut branches: Vec<Branch> = Vec::new();
        let mut events = node.preorder_of_kinds(EVALUATED_KINDS);
        while let Some(event) = events.next() {
            match event {
                WalkEvent::Enter(node) => {
                    let position = node.text_trimmed_range().start();
                    for branch in &mut branches {
                        branch.advance(position, state);
                    }
                    if SKIPPED_KINDS.matches(node.kind()) {
                        events.skip_subtree();
                        continue;
                    }
                    if let Some(branch) = Branch::new(&node) {
                        branches.push(branch);
                    }
                    if let Some(reference) = JsReferenceIdentifier::cast_ref(&node) {
                        if !is_assigned_object(&reference)
                            && let Some(binding) = self.model.binding(&reference)
                        {
                            state.read(&binding);
                        }
                    } else if let Some(identifier) = JsIdentifierAssignment::cast_ref(&node)
                        && reads_previous_value(&identifier)
                        && let Some(binding) = self.model.binding(&identifier)
                    {
                        state.read(&binding);
                    }
                }
                WalkEvent::Leave(node) => {
                    if JsAwaitExpression::can_cast(node.kind())
                        || JsYieldExpression::can_cast(node.kind())
                    {
                        state.pause();
                    } else if let Some(assignment) = JsAssignmentExpression::cast_ref(&node) {
                        self.check_writes(&assignment, state, updates);
                        if is_logical_assignment(&assignment)
                            && let Some(branch) = branches.pop()
                        {
                            branch.finish(state);
                        }
                    } else if (JsConditionalExpression::can_cast(node.kind())
                        || JsLogicalExpression::can_cast(node.kind()))
                        && let Some(branch) = branches.pop()
                    {
                        branch.finish(state);
                    }
                }
            }
        }
    }

    /// Appends `assignment` to `updates` for every variable it writes, directly
    /// or through a property, that is outdated in `state`.
    fn check_writes(
        &self,
        assignment: &JsAssignmentExpression,
        state: &FlowState,
        updates: &mut Vec<NonAtomicUpdate>,
    ) {
        let mut check = |name: JsSyntaxToken, binding: Option<Binding>, is_property: bool| {
            if let Some(binding) = binding
                && state.outdated.contains(variable_id(&binding))
                && self.escapes_function(&binding, is_property)
            {
                updates.push(NonAtomicUpdate {
                    assignment: assignment.clone(),
                    variable: name,
                    is_property,
                });
            }
        };
        let Ok(left) = assignment.left() else {
            return;
        };
        match left {
            AnyJsAssignmentPattern::AnyJsAssignment(target) => match omit_wrappers(target) {
                Some(AnyJsAssignment::JsIdentifierAssignment(identifier)) => {
                    if let Ok(name) = identifier.name_token() {
                        check(name, self.model.binding(&identifier), false);
                    }
                }
                Some(AnyJsAssignment::JsStaticMemberAssignment(member)) => {
                    if let Some(object) = member.object().ok().and_then(object_root)
                        && let Ok(name) = object.value_token()
                    {
                        check(name, self.model.binding(&object), true);
                    }
                }
                Some(AnyJsAssignment::JsComputedMemberAssignment(member)) => {
                    if let Some(object) = member.object().ok().and_then(object_root)
                        && let Ok(name) = object.value_token()
                    {
                        check(name, self.model.binding(&object), true);
                    }
                }
                _ => {}
            },
            AnyJsAssignmentPattern::JsArrayAssignmentPattern(_)
            | AnyJsAssignmentPattern::JsObjectAssignmentPattern(_) => {
                let mut targets = left.syntax().preorder_of_kinds(PATTERN_TARGET_KINDS);
                while let Some(event) = targets.next() {
                    let WalkEvent::Enter(node) = event else {
                        continue;
                    };
                    if let Some(identifier) = JsIdentifierAssignment::cast(node) {
                        if let Ok(name) = identifier.name_token() {
                            check(name, self.model.binding(&identifier), false);
                        }
                    } else {
                        // Default values and computed keys may contain other assignments.
                        targets.skip_subtree();
                    }
                }
            }
        }
    }

    /// Returns whether code outside of the analyzed function can change the
    /// variable, or one of its properties if `is_property` is set.
    fn escapes_function(&self, binding: &Binding, is_property: bool) -> bool {
        let declaration = binding.tree().declaration().map(|declaration| {
            declaration
                .parent_binding_pattern_declaration()
                .unwrap_or(declaration)
        });
        // Callers can access the objects passed as arguments.
        if is_property
            && matches!(
                declaration,
                Some(
                    AnyJsBindingDeclaration::JsFormalParameter(_)
                        | AnyJsBindingDeclaration::JsRestParameter(_)
                        | AnyJsBindingDeclaration::JsArrowFunctionExpression(_)
                        | AnyJsBindingDeclaration::TsPropertyParameter(_)
                )
            )
        {
            return true;
        }
        // The name of a function declaration belongs to the enclosing scope.
        let declared_in = match &declaration {
            Some(
                declaration @ (AnyJsBindingDeclaration::JsFunctionDeclaration(_)
                | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_)),
            ) => declaration.syntax().parent(),
            _ => Some(binding.syntax()),
        };
        let is_in_root = |node: Option<JsSyntaxNode>| {
            node.and_then(|node| {
                // Class fields are initialized when an instance is created.
                node.ancestors().find(|ancestor| {
                    AnyJsControlFlowRoot::can_cast(ancestor.kind())
                        || JsPropertyClassMember::can_cast(ancestor.kind())
                })
            })
            .is_some_and(|ancestor| &ancestor == self.root.syntax())
        };
        !is_in_root(declared_in)
            || !binding
                .all_references()
                .all(|reference| is_in_root(Some(reference.syntax())))
    }
}

fn is_logical_assignment(assignment: &JsAssignmentExpression) -> bool {
    matches!(
        assignment.operator(),
        Ok(JsAssignmentOperator::LogicalAndAssign
            | JsAssignmentOperator::LogicalOrAssign
            | JsAssignmentOperator::NullishCoalescingAssign)
    )
}

/// Returns `target` without the parentheses and type assertions around it.
fn omit_wrappers(mut target: AnyJsAssignment) -> Option<AnyJsAssignment> {
    loop {
        target = match target {
            AnyJsAssignment::JsParenthesizedAssignment(target) => target.assignment().ok()?,
            AnyJsAssignment::TsAsAssignment(target) => target.assignment().ok()?,
            AnyJsAssignment::TsNonNullAssertionAssignment(target) => target.assignment().ok()?,
            AnyJsAssignment::TsSatisfiesAssignment(target) => target.assignment().ok()?,
            AnyJsAssignment::TsTypeAssertionAssignment(target) => target.assignment().ok()?,
            target => return Some(target),
        };
    }
}

/// Returns the variable at the start of a chain of property accesses.
///
/// For example, returns `a` for the object `a.b[c]` of the assignment target `a.b[c].d`.
fn object_root(object: AnyJsExpression) -> Option<JsReferenceIdentifier> {
    let mut object = object.omit_parentheses();
    loop {
        object = match object {
            AnyJsExpression::JsIdentifierExpression(identifier) => return identifier.name().ok(),
            AnyJsExpression::JsStaticMemberExpression(member) => member.object().ok()?,
            AnyJsExpression::JsComputedMemberExpression(member) => member.object().ok()?,
            _ => return None,
        }
        .omit_parentheses();
    }
}

/// Returns whether `reference` is the variable at the start of the left side of
/// an `=` assignment to a property, such as `a` in `a.b.c = value`.
///
/// The assignment replaces the property without reading its previous value.
fn is_assigned_object(reference: &JsReferenceIdentifier) -> bool {
    let Some(mut object) = reference.syntax().parent() else {
        return false;
    };
    for parent in object.ancestors().skip(1) {
        let is_object = |object_of: Option<AnyJsExpression>| {
            object_of.is_some_and(|expression| expression.syntax() == &object)
        };
        if JsParenthesizedExpression::can_cast(parent.kind())
            || is_object(
                AnyJsMemberExpression::cast_ref(&parent).and_then(|member| member.object().ok()),
            )
        {
            object = parent;
        } else if is_object(
            AnyJsMemberAssignment::cast_ref(&parent).and_then(|member| member.object().ok()),
        ) {
            return parent
                .ancestors()
                .skip(1)
                .find(|ancestor| !ASSIGNMENT_WRAPPER_KINDS.matches(ancestor.kind()))
                .and_then(JsAssignmentExpression::cast)
                .is_some_and(|assignment| {
                    matches!(assignment.operator(), Ok(JsAssignmentOperator::Assign))
                });
        } else {
            return false;
        }
    }
    false
}

/// Returns whether writing `identifier` reads its previous value, such as in `x += 1` or `x++`.
fn reads_previous_value(identifier: &JsIdentifierAssignment) -> bool {
    let Some(writer) = identifier
        .syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| !ASSIGNMENT_WRAPPER_KINDS.matches(ancestor.kind()))
    else {
        return false;
    };
    if let Some(assignment) = JsAssignmentExpression::cast_ref(&writer) {
        !matches!(assignment.operator(), Ok(JsAssignmentOperator::Assign))
    } else {
        JsPreUpdateExpression::can_cast(writer.kind())
            || JsPostUpdateExpression::can_cast(writer.kind())
    }
}

/// Kinds of the nodes that don't run when the code around them is evaluated.
const SKIPPED_KINDS: SyntaxKindSet<JsLanguage> = AnyJsControlFlowRoot::KIND_SET
    .union(JsPropertyClassMember::KIND_SET)
    .union(AnyTsType::KIND_SET);

/// Kinds of the nodes [Function::evaluate] acts on or skips.
const EVALUATED_KINDS: SyntaxKindSet<JsLanguage> = SKIPPED_KINDS
    .union(JsReferenceIdentifier::KIND_SET)
    .union(JsIdentifierAssignment::KIND_SET)
    .union(JsAssignmentExpression::KIND_SET)
    .union(JsAwaitExpression::KIND_SET)
    .union(JsYieldExpression::KIND_SET)
    .union(JsConditionalExpression::KIND_SET)
    .union(JsLogicalExpression::KIND_SET);

/// Kinds of the variables written by a destructuring assignment, and of the
/// expressions inside the pattern that don't assign the pattern's targets.
const PATTERN_TARGET_KINDS: SyntaxKindSet<JsLanguage> =
    JsIdentifierAssignment::KIND_SET.union(AnyJsExpression::KIND_SET);

/// Kinds of the nodes that can wrap an assignment target without changing it.
const ASSIGNMENT_WRAPPER_KINDS: SyntaxKindSet<JsLanguage> = JsParenthesizedAssignment::KIND_SET
    .union(TsAsAssignment::KIND_SET)
    .union(TsNonNullAssertionAssignment::KIND_SET)
    .union(TsSatisfiesAssignment::KIND_SET)
    .union(TsTypeAssertionAssignment::KIND_SET);
