use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_control_flow::{Instruction, InstructionKind, builder::ROOT_BLOCK_ID};
use biome_db::AnyParsedSource;
use biome_diagnostics::Severity;
use biome_js_control_flow::{
    AnyJsControlFlowRoot, JsControlFlowGraph, control_flow_model, js_control_flow_model,
};
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsArrayAssignmentPatternElement, AnyJsAssignmentPattern, AnyJsExpression,
    AnyJsObjectAssignmentPatternMember, JsArrayAssignmentPatternElementList, JsAssignmentOperator,
    JsExpressionStatement, JsIdentifierAssignment, JsIdentifierBinding, JsLanguage,
    JsObjectAssignmentPatternPropertyList, JsSyntaxToken, TextRange,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_languages::javascript::JsEmbeddingKind;
use biome_languages::{JsFileSource, LanguageDb};
use biome_rowan::{AstNode, NodeOrToken, SyntaxKindSet, WalkEvent};
use biome_rule_options::no_useless_assignment::NoUselessAssignmentOptions;
use roaring::RoaringBitmap;
use std::rc::Rc;

declare_lint_rule! {
    /// Disallow assignments whose value is never read.
    ///
    /// If a variable gets a value that is never read before the variable is assigned
    /// again, or before the variable stops being used, the assignment has no effect.
    /// This often hints at a mistake: the value may have been meant to be used, or to
    /// be assigned to a different variable.
    ///
    /// ```js,expect_diagnostic
    /// let id = "x1234"; // this value is never read
    /// id = generateId();
    /// doSomethingWith(id);
    /// ```
    ///
    /// The rule checks every way the code can run, including loops, `break`,
    /// `continue`, `return`, and errors caught by `try` statements.
    ///
    /// Variables that are never read are reported by [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/)
    /// instead. The rule also ignores variables that are used inside nested functions,
    /// because it can't know when those functions run. Exported variables and
    /// top-level variables of Astro, Svelte, and Vue files are ignored too, because
    /// other files or the component template can read them.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function fn() {
    ///     let v = "used";
    ///     doSomething(v);
    ///     v = "unused";
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function fn() {
    ///     let v = "used";
    ///     if (condition) {
    ///         v = "unused";
    ///         return;
    ///     }
    ///     doSomething(v);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function fn() {
    ///     let v = "unused";
    ///     v = "used";
    ///     doSomething(v);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function fn() {
    ///     let count = 0;
    ///     doSomething(count);
    ///     count++;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function fn1() {
    ///     let v = "used";
    ///     doSomething(v);
    ///     v = "used-2";
    ///     doSomething(v);
    /// }
    ///
    /// function fn2() {
    ///     let v = "used";
    ///     for (let i = 0; i < 10; i++) {
    ///         doSomething(v);
    ///         v = "used in next iteration";
    ///     }
    /// }
    ///
    /// function fn3() {
    ///     let result = "fallback";
    ///     try {
    ///         result = compute();
    ///     } catch {
    ///         // `result` keeps its previous value when `compute()` throws.
    ///     }
    ///     return result;
    /// }
    /// ```
    ///
    pub NoUselessAssignment {
        version: "next",
        name: "noUselessAssignment",
        language: "js",
        sources: &[RuleSource::Eslint("no-useless-assignment").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoUselessAssignment {
    type Query = Semantic<AnyJsControlFlowRoot>;
    type State = JsSyntaxToken;
    type Signals = Box<[Self::State]>;
    type Options = NoUselessAssignmentOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.query();
        // Framework templates read top-level variables of the script.
        if matches!(
            root,
            AnyJsControlFlowRoot::JsModule(_) | AnyJsControlFlowRoot::JsScript(_)
        ) && !matches!(
            ctx.source_type::<JsFileSource>().as_embedding_kind(),
            JsEmbeddingKind::None
        ) {
            return Box::default();
        }
        let Some(cfg) = control_flow_graph(ctx) else {
            return Box::default();
        };

        let mut accesses = Accesses::new(&cfg);
        for binding in declared_bindings(root, ctx.model()) {
            accesses.add_binding(&binding, root);
        }
        if accesses.names.is_empty() {
            return Box::default();
        }

        let reachable = reachable_blocks(&cfg);
        let mut live_in = vec![RoaringBitmap::new(); cfg.blocks.len()];
        let mut changed = true;
        while changed {
            changed = false;
            // Most edges point forward, so visiting blocks backward converges faster.
            for block in reachable.iter().rev() {
                let live = accesses.live_before(&cfg, block as usize, &live_in, |_| {});
                if live != live_in[block as usize] {
                    live_in[block as usize] = live;
                    changed = true;
                }
            }
        }

        let mut useless = Vec::new();
        for block in &reachable {
            accesses.live_before(&cfg, block as usize, &live_in, |name| {
                useless.push(accesses.names[name as usize].clone());
            });
        }
        useless.sort_unstable_by_key(|name| name.text_trimmed_range().start());
        useless.into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, name: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                name.text_trimmed_range(),
                markup! {
                    "The value assigned to "<Emphasis>{name.text_trimmed()}</Emphasis>" is never read."
                },
            )
            .note(markup! {
                "After this assignment, the variable is either assigned again or not read anymore, so this value is thrown away."
            })
            .note(markup! {
                "Remove the assignment if it's unnecessary. Otherwise, check whether the value should be used, or assigned to a different variable."
            }),
        )
    }
}

/// Returns the control flow graph of the queried execution root.
fn control_flow_graph(ctx: &RuleContext<NoUselessAssignment>) -> Option<JsControlFlowGraph> {
    let root = ctx.query();
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

/// Returns the bindings declared by `root`, excluding the ones of nested
/// execution roots.
fn declared_bindings(
    root: &AnyJsControlFlowRoot,
    model: &SemanticModel,
) -> impl Iterator<Item = Binding> {
    let mut events = root.syntax().preorder();
    std::iter::from_fn(move || {
        loop {
            let WalkEvent::Enter(node) = events.next()? else {
                continue;
            };
            if &node != root.syntax() && AnyJsControlFlowRoot::can_cast(node.kind()) {
                events.skip_subtree();
            } else if let Some(binding) = JsIdentifierBinding::cast(node) {
                return Some(model.as_binding(&binding));
            }
        }
    })
}

/// Reads and writes of the tracked variables, grouped by graph instruction.
struct Accesses {
    /// Ranges of the instruction nodes, sorted by start, with their instruction index.
    ranges: Vec<(TextRange, u32)>,
    /// Range of the node of every instruction.
    instruction_ranges: Vec<Option<TextRange>>,
    /// The first instruction index of every block.
    block_starts: Vec<u32>,
    /// Accesses of every instruction.
    accesses: Vec<Vec<Access>>,
    /// Names of the written variables, indexed by [AccessKind::Write::name].
    names: Vec<JsSyntaxToken>,
    variable_count: u32,
}

struct Access {
    variable: u32,
    range: TextRange,
    kind: AccessKind,
}

enum AccessKind {
    Read,
    Write {
        /// Whether the write always overwrites the previous value.
        overwrites: bool,
        /// Whether the write reads the previous value, such as `x += 1`.
        reads_previous: bool,
        /// The assigned value, which is evaluated before the write.
        value: Option<TextRange>,
        name: u32,
    },
}

impl Accesses {
    fn new(cfg: &JsControlFlowGraph) -> Self {
        let mut ranges = Vec::new();
        let mut instruction_ranges = Vec::new();
        let mut block_starts = Vec::with_capacity(cfg.blocks.len());
        let mut index = 0;
        for block in &cfg.blocks {
            block_starts.push(index);
            for instruction in &block.instructions {
                let range = match &instruction.node {
                    Some(NodeOrToken::Node(node)) => Some(node.text_trimmed_range()),
                    _ => None,
                };
                if let Some(range) = range {
                    ranges.push((range, index));
                }
                instruction_ranges.push(range);
                index += 1;
            }
        }
        ranges.sort_unstable_by_key(|(range, _)| range.start());
        Self {
            ranges,
            instruction_ranges,
            block_starts,
            accesses: (0..index).map(|_| Vec::new()).collect(),
            names: Vec::new(),
            variable_count: 0,
        }
    }

    /// Returns the instruction that evaluates the node at `range`.
    fn instruction_at(&self, range: TextRange) -> Option<u32> {
        let index = self
            .ranges
            .partition_point(|(instruction, _)| instruction.start() <= range.start());
        let (instruction, id) = self.ranges.get(index.checked_sub(1)?)?;
        instruction.contains_range(range).then_some(*id)
    }

    /// Records the accesses of `binding`, if all of them are evaluated by the graph of `root`.
    ///
    /// Values of variables that are never read are reported by `noUnusedVariables`.
    fn add_binding(&mut self, binding: &Binding, root: &AnyJsControlFlowRoot) {
        if binding.is_exported() {
            return;
        }
        let variable = self.variable_count;
        let mut accesses = Vec::new();
        let mut names = Vec::new();
        let mut has_reads = false;

        if let Some(declarator) = binding
            .tree()
            .declaration()
            .map(|declaration| {
                declaration
                    .parent_binding_pattern_declaration()
                    .unwrap_or(declaration)
            })
            .and_then(|declaration| match declaration {
                AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => Some(declarator),
                _ => None,
            })
            && let Some(initializer) = declarator.initializer()
        {
            let Ok(value) = initializer
                .expression()
                .map(|expression| expression.range())
            else {
                return;
            };
            let Some(instruction) = self.instruction_at(value) else {
                return;
            };
            let Ok(name) = binding.tree().name_token() else {
                return;
            };
            accesses.push((
                instruction,
                Access {
                    variable,
                    range: binding.range(),
                    kind: AccessKind::Write {
                        overwrites: true,
                        reads_previous: false,
                        value: Some(value),
                        name: names.len() as u32,
                    },
                },
            ));
            names.push(name);
        }

        for reference in binding.all_references() {
            let node = reference.syntax();
            // References in nested functions can run at any time.
            let is_in_root = node
                .ancestors()
                .find(|ancestor| AnyJsControlFlowRoot::can_cast(ancestor.kind()))
                .is_some_and(|ancestor| &ancestor == root.syntax());
            let range = node.text_trimmed_range();
            let Some(instruction) = self.instruction_at(range).filter(|_| is_in_root) else {
                return;
            };
            let kind = if reference.is_read() {
                has_reads = true;
                AccessKind::Read
            } else if let Some(identifier) = JsIdentifierAssignment::cast_ref(&node)
                && let Some(kind) = self.write_kind(&identifier, instruction, names.len() as u32)
            {
                let Ok(name) = identifier.name_token() else {
                    return;
                };
                names.push(name);
                kind
            } else {
                // Other writes, such as the head of a `for...of` loop, are not reported
                // and are not assumed to overwrite the previous value.
                continue;
            };
            accesses.push((
                instruction,
                Access {
                    variable,
                    range,
                    kind,
                },
            ));
        }

        if !has_reads || names.is_empty() {
            return;
        }
        let first_name = self.names.len() as u32;
        self.names.extend(names);
        for (instruction, mut access) in accesses {
            if let AccessKind::Write { name, .. } = &mut access.kind {
                *name += first_name;
            }
            self.accesses[instruction as usize].push(access);
        }
        self.variable_count += 1;
    }

    /// Classifies a write reference by the expression that writes it.
    fn write_kind(
        &self,
        identifier: &JsIdentifierAssignment,
        instruction: u32,
        name: u32,
    ) -> Option<AccessKind> {
        let writer = identifier
            .syntax()
            .ancestors()
            .skip(1)
            .find(|ancestor| !ASSIGNMENT_TARGET_KINDS.matches(ancestor.kind()))
            .and_then(AnyJsExpression::cast)?;
        let is_statement = self.instruction_ranges[instruction as usize]
            .is_some_and(|range| is_whole_instruction(range, &writer));
        match writer {
            AnyJsExpression::JsAssignmentExpression(assignment) => {
                let operator = assignment.operator().ok()?;
                let is_logical = matches!(
                    operator,
                    JsAssignmentOperator::LogicalAndAssign
                        | JsAssignmentOperator::LogicalOrAssign
                        | JsAssignmentOperator::NullishCoalescingAssign
                );
                Some(AccessKind::Write {
                    overwrites: is_statement && !is_logical,
                    reads_previous: operator != JsAssignmentOperator::Assign,
                    value: assignment.right().ok().map(|right| right.range()),
                    name,
                })
            }
            AnyJsExpression::JsPreUpdateExpression(_)
            | AnyJsExpression::JsPostUpdateExpression(_) => Some(AccessKind::Write {
                overwrites: is_statement,
                reads_previous: true,
                value: None,
                name,
            }),
            _ => None,
        }
    }

    /// Returns the variables that are live at the start of a block, calling
    /// `on_useless_write` with the name of every write whose value is never read.
    ///
    /// The graph doesn't order the accesses inside an instruction, so a read is
    /// assumed to happen after a write of the same instruction, unless it's part
    /// of the assigned value.
    fn live_before(
        &self,
        cfg: &JsControlFlowGraph,
        block: usize,
        live_in: &[RoaringBitmap],
        mut on_useless_write: impl FnMut(u32),
    ) -> RoaringBitmap {
        let data = &cfg.blocks[block];
        // Any expression of the block may throw, so the variables read by the
        // exception handlers are live everywhere in the block.
        let mut handlers = RoaringBitmap::new();
        for handler in &data.exception_handlers {
            handlers |= &live_in[handler.target.index() as usize];
        }
        let end = data
            .instructions
            .iter()
            .position(is_terminator)
            .map_or(data.instructions.len(), |index| index + 1);
        let mut live = handlers.clone();
        for (index, instruction) in data.instructions[..end].iter().enumerate().rev() {
            match instruction.kind {
                InstructionKind::Statement => {}
                InstructionKind::Jump {
                    conditional: true,
                    block,
                    ..
                } => live |= &live_in[block.index() as usize],
                InstructionKind::Jump { block, .. } => {
                    live.clone_from(&live_in[block.index() as usize]);
                }
                InstructionKind::Return => {
                    // `finally` clauses run before leaving the function.
                    live.clear();
                    for handler in &data.cleanup_handlers {
                        live |= &live_in[handler.target.index() as usize];
                    }
                }
            }
            live |= &handlers;

            let accesses = &self.accesses[(self.block_starts[block] as usize) + index];
            for access in accesses {
                if let AccessKind::Write { value, name, .. } = access.kind
                    && !live.contains(access.variable)
                    && !accesses.iter().any(|other| {
                        other.variable == access.variable
                            && !std::ptr::eq(other, access)
                            && other.reads()
                            && !value.is_some_and(|value| value.contains_range(other.range))
                    })
                {
                    on_useless_write(name);
                }
            }
            for access in accesses {
                if let AccessKind::Write {
                    overwrites: true, ..
                } = access.kind
                {
                    live.remove(access.variable);
                }
            }
            for access in accesses {
                if access.reads() {
                    live.insert(access.variable);
                }
            }
            live |= &handlers;
        }
        live
    }
}

impl Access {
    fn reads(&self) -> bool {
        matches!(
            self.kind,
            AccessKind::Read
                | AccessKind::Write {
                    reads_previous: true,
                    ..
                }
        )
    }
}

fn is_terminator(instruction: &Instruction<JsLanguage>) -> bool {
    matches!(
        instruction.kind,
        InstructionKind::Return
            | InstructionKind::Jump {
                conditional: false,
                ..
            }
    )
}

/// Kinds of the nodes between a written identifier and the expression that writes it.
const ASSIGNMENT_TARGET_KINDS: SyntaxKindSet<JsLanguage> = AnyJsAssignmentPattern::KIND_SET
    .union(AnyJsArrayAssignmentPatternElement::KIND_SET)
    .union(AnyJsObjectAssignmentPatternMember::KIND_SET)
    .union(JsArrayAssignmentPatternElementList::KIND_SET)
    .union(JsObjectAssignmentPatternPropertyList::KIND_SET);

/// Returns whether `writer` is the whole expression evaluated by the instruction
/// at `range`, so that it always runs when the instruction runs.
fn is_whole_instruction(range: TextRange, writer: &AnyJsExpression) -> bool {
    if range == writer.range() {
        return true;
    }
    writer
        .syntax()
        .parent()
        .and_then(JsExpressionStatement::cast)
        .is_some_and(|statement| statement.range() == range)
}

/// Returns the blocks reachable from the entry of the graph.
fn reachable_blocks(cfg: &JsControlFlowGraph) -> RoaringBitmap {
    let mut reachable = RoaringBitmap::new();
    let mut queue = vec![ROOT_BLOCK_ID];
    reachable.insert(ROOT_BLOCK_ID.index());
    while let Some(id) = queue.pop() {
        let block = cfg.get(id);
        let end = block
            .instructions
            .iter()
            .position(is_terminator)
            .map_or(block.instructions.len(), |index| index + 1);
        let jumps =
            block.instructions[..end]
                .iter()
                .filter_map(|instruction| match instruction.kind {
                    InstructionKind::Jump { block, .. } => Some(block),
                    _ => None,
                });
        let handlers = block
            .exception_handlers
            .iter()
            .chain(&block.cleanup_handlers)
            .map(|handler| handler.target);
        for target in jumps.chain(handlers) {
            if reachable.insert(target.index()) {
                queue.push(target);
            }
        }
    }
    reachable
}
