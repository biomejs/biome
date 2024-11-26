use std::{cmp::Ordering, collections::VecDeque, num::NonZeroU32, vec::IntoIter};

use crate::services::control_flow::{ControlFlowGraph, JsControlFlowGraph};
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_control_flow::{
    builder::{BlockId, ROOT_BLOCK_ID},
    ExceptionHandler, ExceptionHandlerKind, Instruction, InstructionKind,
};
use biome_diagnostics::Severity;
use biome_js_syntax::{
    JsBlockStatement, JsCaseClause, JsDefaultClause, JsDoWhileStatement, JsForInStatement,
    JsForOfStatement, JsForStatement, JsFunctionBody, JsIfStatement, JsLabeledStatement,
    JsLanguage, JsReturnStatement, JsSwitchStatement, JsSyntaxElement, JsSyntaxKind, JsSyntaxNode,
    JsTryFinallyStatement, JsTryStatement, JsVariableStatement, JsWhileStatement, TextRange,
};
use biome_rowan::{declare_node_union, AstNode, NodeOrToken};
use roaring::bitmap::RoaringBitmap;
use rustc_hash::FxHashMap;

declare_lint_rule! {
    /// Disallow unreachable code
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function example() {
    ///     return;
    ///     neverCalled();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function example() {
    ///     for(let i = 0; i < 10; ++i) {
    ///         break;
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function example() {
    ///     for(const key in value) {
    ///         continue;
    ///         neverCalled();
    ///     }
    /// }
    /// ```
    pub NoUnreachable {
        version: "1.0.0",
        name: "noUnreachable",
        language: "js",
        sources: &[RuleSource::Eslint("no-unreachable")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoUnreachable {
    type Query = ControlFlowGraph;
    type State = UnreachableRange;
    type Signals = UnreachableRanges;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut signals = UnreachableRanges::new();

        let cfg = ctx.query();

        if exceeds_complexity_threshold(cfg) {
            analyze_simple(cfg, &mut signals)
        } else {
            analyze_fine(cfg, &mut signals)
        }

        signals
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.text_trimmed_range,
            if state.terminators.is_empty() {
                "This code is unreachable"
            } else {
                "This code will never be reached ..."
            },
        )
        .description("This code is unreachable")
        .unnecessary();

        // Pluralize and adapt the error message accordingly based on the
        // number and position of secondary labels
        match state.terminators.as_slice() {
            // The CFG didn't contain enough information to determine a cause
            // for this range being unreachable
            [] => {}
            // A single node is responsible for this range being unreachable
            [node] => {
                diagnostic = diagnostic.detail(
                    node.range,
                    format_args!(
                        "... because this statement will {} beforehand",
                        node.reason()
                    ),
                );
            }
            // The range has two dominating terminator instructions
            [node_a, node_b] => {
                if node_a.kind == node_b.kind {
                    diagnostic = diagnostic
                        .detail(node_a.range, "... because either this statement ...")
                        .detail(
                            node_b.range,
                            format_args!(
                                "... or this statement will {} beforehand",
                                node_b.reason()
                            ),
                        );
                } else {
                    diagnostic = diagnostic
                        .detail(
                            node_a.range,
                            format_args!(
                                "... because either this statement will {} ...",
                                node_a.reason()
                            ),
                        )
                        .detail(
                            node_b.range,
                            format_args!(
                                "... or this statement will {} beforehand",
                                node_b.reason()
                            ),
                        );
                }
            }
            // The range has three or more dominating terminator instructions
            terminators => {
                // SAFETY: This subtraction is safe since the match expression
                // ensures the slice has at least 3 elements
                let last = terminators.len() - 1;

                // Do not repeat the reason for each terminator if they all have the same kind
                let (_, has_homogeneous_kind) = terminators
                    .iter()
                    .fold(None, |prev_kind, terminator| match prev_kind {
                        Some((kind, state)) => Some((kind, state && terminator.kind == kind)),
                        None => Some((terminator.kind, true)),
                    })
                    // SAFETY: terminators has at least 3 elements
                    .unwrap();

                if has_homogeneous_kind {
                    for (index, node) in terminators.iter().enumerate() {
                        if index == 0 {
                            diagnostic = diagnostic
                                .detail(node.range, "... because either this statement, ...");
                        } else if index < last {
                            diagnostic = diagnostic.detail(node.range, "... this statement, ...");
                        } else {
                            diagnostic = diagnostic.detail(
                                node.range,
                                format_args!(
                                    "... or this statement will {} beforehand",
                                    node.reason()
                                ),
                            );
                        }
                    }
                } else {
                    for (index, node) in terminators.iter().enumerate() {
                        if index == 0 {
                            diagnostic = diagnostic.detail(
                                node.range,
                                format_args!(
                                    "... because either this statement will {}, ...",
                                    node.reason()
                                ),
                            );
                        } else if index < last {
                            diagnostic = diagnostic.detail(
                                node.range,
                                format_args!("... this statement will {}, ...", node.reason()),
                            );
                        } else {
                            diagnostic = diagnostic.detail(
                                node.range,
                                format_args!(
                                    "... or this statement will {} beforehand",
                                    node.reason()
                                ),
                            );
                        }
                    }
                }
            }
        }

        Some(diagnostic)
    }
}

/// Any function with a complexity score higher than this value will use the
/// simple reachability analysis instead of the fine analysis
const COMPLEXITY_THRESHOLD: u32 = 20;

/// Returns true if the "complexity score" for the [JsControlFlowGraph] is higher
/// than [COMPLEXITY_THRESHOLD]. This score is an arbitrary value (the formula
/// is similar to the cyclomatic complexity of the function but this is only
/// approximative) used to determine whether the NoDeadCode rule should perform
/// a fine reachability analysis or fall back to a simpler algorithm to avoid
/// spending too much time analyzing exceedingly complex functions
fn exceeds_complexity_threshold(cfg: &JsControlFlowGraph) -> bool {
    let nodes = cfg.blocks.len() as u32;

    let mut edges: u32 = 0;
    let mut conditionals: u32 = 0;

    for block in &cfg.blocks {
        let mut exception_handlers = NonZeroU32::new(block.exception_handlers.len() as u32);
        let mut cleanup_handlers = NonZeroU32::new(block.cleanup_handlers.len() as u32);

        for inst in &block.instructions {
            if has_side_effects(inst) {
                if let Some(handlers) = exception_handlers.take() {
                    edges += handlers.get();
                    conditionals += 1;
                }
            }

            match inst.kind {
                InstructionKind::Statement => {}
                InstructionKind::Jump { conditional, .. } => {
                    edges += 1;

                    if conditional {
                        conditionals += 1;
                    }
                }
                InstructionKind::Return => {
                    if let Some(handlers) = cleanup_handlers.take() {
                        edges += handlers.get();
                        conditionals += 1;
                    }
                }
            }

            let complexity = edges.saturating_sub(nodes) + conditionals / 2;
            if complexity > COMPLEXITY_THRESHOLD {
                return true;
            }
        }
    }

    false
}

/// Perform a simple reachability analysis, does not attempt to determine a
/// terminator instruction for unreachable ranges allowing blocks to be visited
/// at most once and ensuring the algorithm finishes in a bounded time
fn analyze_simple(cfg: &JsControlFlowGraph, signals: &mut UnreachableRanges) {
    // Perform a simple reachability analysis on the control flow graph by
    // traversing the function starting at the entry point
    let mut reachable_blocks = RoaringBitmap::new();
    let mut queue = VecDeque::new();

    if !cfg.blocks.is_empty() {
        reachable_blocks.insert(ROOT_BLOCK_ID.index());
        queue.push_back((ROOT_BLOCK_ID, None));
    }

    while let Some((block_id, handlers)) = queue.pop_front() {
        let block = cfg.get(block_id);

        // Lookup the existence of an exception edge for this block but
        // defer its creation until an instruction that can throw is encountered
        let mut exception_handlers = block.exception_handlers.split_first();

        // Tracks whether this block is "terminated", if an instruction
        // that unconditionally aborts the control flow of this block has
        // been encountered
        let mut has_terminator = false;

        for inst in &block.instructions {
            // If this block is terminated, mark this instruction as unreachable and continue
            if has_terminator {
                if let Some(node) = &inst.node {
                    signals.push(node, None);
                }
                continue;
            }

            // Do not create exception edges for instructions with no side effects
            if has_side_effects(inst) {
                // If this block has a pending exception edge, create an
                // additional path diverging towards the corresponding
                // catch or finally block
                if let Some((handler, handlers)) = exception_handlers.take() {
                    if reachable_blocks.insert(handler.target.index()) {
                        queue.push_back((handler.target, find_catch_handlers(handlers)));
                    }
                }
            }

            match inst.kind {
                InstructionKind::Statement => {}
                InstructionKind::Jump {
                    conditional,
                    block,
                    finally_fallthrough,
                } => {
                    if finally_fallthrough && handlers.is_some() {
                        // Jump towards the corresponding block if there are pending exception
                        // handlers, otherwise return from the function
                        let handlers = handlers.and_then(<[_]>::split_first);

                        if let Some((handler, handlers)) = handlers {
                            if reachable_blocks.insert(handler.target.index()) {
                                queue.push_back((handler.target, Some(handlers)));
                            }
                        }
                    }

                    if reachable_blocks.insert(block.index()) {
                        // Insert an edge if this jump is reachable
                        queue.push_back((block, handlers));
                    }

                    // Jump is a terminator instruction if it's unconditional
                    if !conditional {
                        has_terminator = true;
                    }
                }
                InstructionKind::Return => {
                    if let Some((handler, handlers)) = block.cleanup_handlers.split_first() {
                        if reachable_blocks.insert(handler.target.index()) {
                            queue.push_back((handler.target, Some(handlers)));
                        }
                    }

                    has_terminator = true;
                }
            }
        }
    }

    // Detect blocks that were never reached by the above traversal
    for (index, block) in cfg.blocks.iter().enumerate() {
        let index = index as u32;
        if reachable_blocks.contains(index) {
            continue;
        }

        for inst in &block.instructions {
            if let Some(node) = &inst.node {
                signals.push(node, None);
            }
        }
    }
}

/// Performs a fine reachability analysis of the control flow graph: this
/// algorithm traverse all the possible paths through the function to determine
/// the reachability of each block and instruction but also find one or more
/// "terminator instructions" for each unreachable range of code that cause it
/// to be impossible to reach
fn analyze_fine(cfg: &JsControlFlowGraph, signals: &mut UnreachableRanges) {
    // Traverse the CFG and calculate block / instruction reachability
    let block_paths = traverse_cfg(cfg, signals);

    // Detect unreachable blocks using the result of the above traversal
    'blocks: for (block_id, block) in cfg.block_id_iter() {
        match block_paths.get(&block_id) {
            // Block has incoming paths, but may be unreachable if they all
            // have a dominating terminator intruction
            Some(paths) => {
                let mut terminators = Vec::new();
                for path in paths {
                    if let Some(terminator) = *path {
                        terminators.push(terminator);
                    } else {
                        // This path has no terminator, the block is reachable
                        continue 'blocks;
                    }
                }

                // Mark each instruction in the block as unreachable with
                // the appropriate terminator labels
                for inst in &block.instructions {
                    if let Some(node) = &inst.node {
                        for terminator in &terminators {
                            signals.push(node, *terminator);
                        }
                    }
                }
            }
            // Block has no incoming paths, is completely cut off from the CFG
            // In theory this shouldn't happen as our CFG also stores
            // unreachable edges, if we get here there might be a bug in
            // the control flow analysis
            None => {
                for inst in &block.instructions {
                    if let Some(node) = &inst.node {
                        // There is no incoming control flow so we can't
                        // determine a terminator instruction for this
                        // unreachable range
                        signals.push(node, None);
                    }
                }
            }
        }
    }
}

/// Individual entry in the traversal queue, holding the state for a
/// single "linearly independent path" through the function as it gets
/// created during the control flow traversal
struct PathState<'cfg> {
    /// Index of the next block to visit
    next_block: BlockId,
    /// Set of all blocks already visited on this path
    visited: RoaringBitmap,
    /// Current terminating instruction for the path, if one was
    /// encountered
    terminator: Option<Option<PathTerminator>>,
    exception_handlers: Option<&'cfg [ExceptionHandler]>,
}

/// Perform a simple reachability analysis on the control flow graph by
/// traversing the function starting at the entry points
fn traverse_cfg(
    cfg: &JsControlFlowGraph,
    signals: &mut UnreachableRanges,
) -> FxHashMap<BlockId, Vec<Option<Option<PathTerminator>>>> {
    let mut queue = VecDeque::new();

    queue.push_back(PathState {
        next_block: ROOT_BLOCK_ID,
        visited: RoaringBitmap::new(),
        terminator: None,
        exception_handlers: None,
    });

    // This maps holds a list of "path state", the active terminator
    // intruction for each path that can reach the block
    let mut block_paths = FxHashMap::default();

    while let Some(mut path) = queue.pop_front() {
        // Add the block to the visited set for the path, and the current
        // state of the path to the global reachable blocks map
        path.visited.insert(path.next_block.index());

        block_paths
            .entry(path.next_block)
            .or_insert_with(Vec::new)
            .push(path.terminator);

        let block = cfg.get(path.next_block);

        // Lookup the existence of an exception edge for this block but
        // defer its creation until an instruction that can throw is encountered
        let mut exception_handlers = block.exception_handlers.split_first();

        // Set to true if the `terminator` is found inside of this block
        let mut has_direct_terminator = false;

        for inst in &block.instructions {
            // Do not create exception edges for instructions with no side effects
            if has_side_effects(inst) {
                // If this block has a pending exception edge, create an
                // additional path diverging towards the corresponding
                // catch or finally block
                if let Some((handler, handlers)) = exception_handlers.take() {
                    if !path.visited.contains(handler.target.index()) {
                        queue.push_back(PathState {
                            next_block: handler.target,
                            visited: path.visited.clone(),
                            terminator: path.terminator,
                            exception_handlers: find_catch_handlers(handlers),
                        });
                    }
                }
            }

            // If this block has already ended, immediately mark this instruction as unreachable
            if let Some(terminator) = path.terminator.filter(|_| has_direct_terminator) {
                if let Some(node) = &inst.node {
                    signals.push(node, terminator);
                }
            }

            match inst.kind {
                InstructionKind::Statement => {}
                InstructionKind::Jump {
                    conditional,
                    block,
                    finally_fallthrough,
                } => {
                    handle_jump(&mut queue, &path, block, finally_fallthrough);

                    // Jump is a terminator instruction if it's unconditional
                    if path.terminator.is_none() && !conditional {
                        path.terminator = Some(inst.node.as_ref().map(|node| PathTerminator {
                            kind: node.kind(),
                            range: node.text_trimmed_range(),
                        }));
                        has_direct_terminator = true;
                    }
                }
                InstructionKind::Return => {
                    handle_return(&mut queue, &path, &block.cleanup_handlers);

                    if path.terminator.is_none() {
                        path.terminator = Some(inst.node.as_ref().map(|node| PathTerminator {
                            kind: node.kind(),
                            range: node.text_trimmed_range(),
                        }));
                        has_direct_terminator = true;
                    }
                }
            }
        }
    }

    block_paths
}

/// Returns `true` if `inst` can potentially have side effects. Due to the
/// dynamic nature of JavaScript this is a conservative check, biased towards
/// returning false positives
fn has_side_effects(inst: &Instruction<JsLanguage>) -> bool {
    let element = match inst.node.as_ref() {
        Some(element) => element,
        None => return false,
    };

    let NodeOrToken::Node(node) = element else {
        return false;
    };

    match node.kind() {
        JsSyntaxKind::JS_RETURN_STATEMENT => {
            let node = JsReturnStatement::unwrap_cast(node.clone());
            node.argument().is_some()
        }
        JsSyntaxKind::JS_BREAK_STATEMENT | JsSyntaxKind::JS_CONTINUE_STATEMENT => false,
        kind => !kind.is_literal(),
    }
}

/// Returns the list of all `finally` exception handlers up to and including
/// the first `catch` handler to be executed when an exception is thrown
fn find_catch_handlers(handlers: &[ExceptionHandler]) -> Option<&[ExceptionHandler]> {
    let handlers = handlers
        .iter()
        .position(|handler| matches!(handler.kind, ExceptionHandlerKind::Catch))
        .map_or(handlers, |index| &handlers[index..]);

    if handlers.is_empty() {
        None
    } else {
        Some(handlers)
    }
}

/// Create an additional visitor path from a jump instruction and push it to the queue
fn handle_jump<'cfg>(
    queue: &mut VecDeque<PathState<'cfg>>,
    path: &PathState<'cfg>,
    block: BlockId,
    finally_fallthrough: bool,
) {
    // If this jump is exiting a finally clause and this path is visiting
    // an exception handlers chain
    if finally_fallthrough && path.exception_handlers.is_some() {
        // Jump towards the corresponding block if there are pending exception
        // handlers, otherwise return from the function
        let handlers = path.exception_handlers.and_then(<[_]>::split_first);

        if let Some((handler, handlers)) = handlers {
            if !path.visited.contains(handler.target.index()) {
                queue.push_back(PathState {
                    next_block: handler.target,
                    visited: path.visited.clone(),
                    terminator: path.terminator,
                    exception_handlers: Some(handlers),
                });
            }
        }
    } else if !path.visited.contains(block.index()) {
        // Push the jump target block to the queue if it hasn't
        // been visited yet in this path
        queue.push_back(PathState {
            next_block: block,
            visited: path.visited.clone(),
            terminator: path.terminator,
            exception_handlers: path.exception_handlers,
        });
    }
}

/// Create an additional visitor path from a return instruction and push it to
/// the queue if necessary
fn handle_return<'cfg>(
    queue: &mut VecDeque<PathState<'cfg>>,
    path: &PathState<'cfg>,
    cleanup_handlers: &'cfg [ExceptionHandler],
) {
    if let Some((handler, handlers)) = cleanup_handlers.split_first() {
        if !path.visited.contains(handler.target.index()) {
            queue.push_back(PathState {
                next_block: handler.target,
                visited: path.visited.clone(),
                terminator: path.terminator,
                exception_handlers: Some(handlers),
            });
        }
    }
}

/// Stores a list of unreachable code ranges, sorted in ascending source order
#[derive(Debug)]
pub struct UnreachableRanges {
    ranges: Vec<UnreachableRange>,
}

impl UnreachableRanges {
    fn new() -> Self {
        UnreachableRanges { ranges: Vec::new() }
    }

    fn push(&mut self, node: &JsSyntaxElement, terminator: Option<PathTerminator>) {
        let text_range = node.text_range();
        let text_trimmed_range = node.text_trimmed_range();

        // Perform a binary search on the ranges already in storage to find an
        // appropriate position for either merging or inserting the incoming range
        let insertion = self.ranges.binary_search_by(|entry| {
            if entry.text_range.end() < text_range.start() {
                Ordering::Less
            } else if text_range.end() < entry.text_range.start() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        let index = match insertion {
            // The search returned an existing overlapping range, extend it to
            // cover the incoming range
            Ok(index) => {
                let entry = &mut self.ranges[index];
                entry.text_range = entry.text_range.cover(text_range);
                entry.text_trimmed_range = entry.text_trimmed_range.cover(text_trimmed_range);

                if let Some(terminator) = terminator {
                    // Terminator labels are also stored in ascending order to
                    // facilitate the generation of labels when the diagnostic
                    // gets emitted
                    let terminator_insertion = entry
                        .terminators
                        .binary_search_by_key(&terminator.range.start(), |node| node.range.start());

                    if let Err(index) = terminator_insertion {
                        entry.terminators.insert(index, terminator);
                    }
                }

                index
            }
            // No overlapping range was found, insert at the appropriate
            // position to preserve the ordering instead
            Err(index) => {
                self.ranges.insert(
                    index,
                    UnreachableRange {
                        text_range,
                        text_trimmed_range,
                        terminators: terminator.into_iter().collect(),
                    },
                );

                index
            }
        };

        let node = match node.parent() {
            Some(parent) => parent,
            None => return,
        };

        self.propagate_ranges(node, index);
    }

    /// Propagate unreachable ranges upward in the tree by detecting and
    /// merging disjoint ranges that cover all the fields of a certain node
    /// type. This requires specialized logic for each control flow node type,
    /// for instance an if-statement is considered fully unreachable if its
    /// test expression, consequent statement and optional else clause are all
    /// fully unreachable.
    fn propagate_ranges(&mut self, mut node: JsSyntaxNode, mut index: usize) -> Option<()> {
        while let Some(parent) = node.ancestors().find_map(JsControlFlowNode::cast) {
            // Merge the adjacent and overlapping ranges
            self.merge_adjacent_ranges();

            let fields = match &parent {
                JsControlFlowNode::JsFunctionBody(_) => break,

                JsControlFlowNode::JsBlockStatement(stmt) => {
                    let statements = stmt.statements().into_syntax();
                    if statements.text_trimmed_range().is_empty() {
                        vec![]
                    } else {
                        vec![statements.text_range_with_trivia()]
                    }
                }

                JsControlFlowNode::JsVariableStatement(stmt) => {
                    let declaration = stmt.declaration().ok()?;
                    declaration
                        .declarators()
                        .into_iter()
                        .filter_map(|declarator| match declarator {
                            Ok(declarator) => match declarator.initializer()?.expression() {
                                Ok(expression) => {
                                    Some(Ok(expression.syntax().text_range_with_trivia()))
                                }
                                Err(err) => Some(Err(err)),
                            },
                            Err(err) => Some(Err(err)),
                        })
                        .collect::<Result<Vec<_>, _>>()
                        .ok()?
                }
                JsControlFlowNode::JsLabeledStatement(stmt) => {
                    vec![stmt.body().ok()?.syntax().text_range_with_trivia()]
                }
                JsControlFlowNode::JsDoWhileStatement(stmt) => vec![
                    stmt.body().ok()?.syntax().text_range_with_trivia(),
                    stmt.test().ok()?.syntax().text_range_with_trivia(),
                ],
                JsControlFlowNode::JsForInStatement(stmt) => vec![
                    stmt.initializer().ok()?.syntax().text_range_with_trivia(),
                    stmt.body().ok()?.syntax().text_range_with_trivia(),
                ],
                JsControlFlowNode::JsForOfStatement(stmt) => vec![
                    stmt.initializer().ok()?.syntax().text_range_with_trivia(),
                    stmt.body().ok()?.syntax().text_range_with_trivia(),
                ],
                JsControlFlowNode::JsForStatement(stmt) => {
                    let mut res = Vec::new();

                    if let Some(initializer) = stmt.initializer() {
                        res.push(initializer.syntax().text_range_with_trivia());
                    }

                    if let Some(test) = stmt.test() {
                        res.push(test.syntax().text_range_with_trivia());
                    }

                    if let Some(update) = stmt.update() {
                        res.push(update.syntax().text_range_with_trivia());
                    }

                    res.push(stmt.body().ok()?.syntax().text_range_with_trivia());
                    res
                }
                JsControlFlowNode::JsIfStatement(stmt) => {
                    let mut res = vec![
                        stmt.test().ok()?.syntax().text_range_with_trivia(),
                        stmt.consequent().ok()?.syntax().text_range_with_trivia(),
                    ];

                    if let Some(else_clause) = stmt.else_clause() {
                        res.push(
                            else_clause
                                .alternate()
                                .ok()?
                                .syntax()
                                .text_range_with_trivia(),
                        );
                    }

                    res
                }
                JsControlFlowNode::JsSwitchStatement(stmt) => {
                    let mut res = vec![stmt.discriminant().ok()?.syntax().text_range_with_trivia()];

                    let cases = stmt.cases().into_syntax();
                    if !cases.text_trimmed_range().is_empty() {
                        res.push(cases.text_range_with_trivia());
                    }

                    res
                }
                JsControlFlowNode::JsTryStatement(stmt) => vec![
                    stmt.body().ok()?.syntax().text_range_with_trivia(),
                    stmt.catch_clause()
                        .ok()?
                        .body()
                        .ok()?
                        .syntax()
                        .text_range_with_trivia(),
                ],
                JsControlFlowNode::JsTryFinallyStatement(stmt) => {
                    let mut res = vec![stmt.body().ok()?.syntax().text_range_with_trivia()];

                    if let Some(catch_clause) = stmt.catch_clause() {
                        res.push(catch_clause.body().ok()?.syntax().text_range_with_trivia());
                    }

                    res.push(
                        stmt.finally_clause()
                            .ok()?
                            .body()
                            .ok()?
                            .syntax()
                            .text_range_with_trivia(),
                    );

                    res
                }
                JsControlFlowNode::JsWhileStatement(stmt) => vec![
                    stmt.test().ok()?.syntax().text_range_with_trivia(),
                    stmt.body().ok()?.syntax().text_range_with_trivia(),
                ],
                JsControlFlowNode::JsCaseClause(stmt) => {
                    let mut res = vec![stmt.test().ok()?.syntax().text_range_with_trivia()];

                    let consequent = stmt.consequent().into_syntax();
                    if !consequent.text_trimmed_range().is_empty() {
                        res.push(consequent.text_range_with_trivia());
                    }

                    res
                }
                JsControlFlowNode::JsDefaultClause(stmt) => {
                    let mut res = vec![stmt.default_token().ok()?.text_range()];

                    let consequent = stmt.consequent().into_syntax();
                    if !consequent.text_trimmed_range().is_empty() {
                        res.push(consequent.text_range_with_trivia());
                    }

                    res
                }
            };

            let next_index = check_neighbors(&self.ranges, index, &fields)?;

            // Extend the range at the specific index to cover the whole parent node
            let entry = &mut self.ranges[next_index];
            entry.text_range = entry
                .text_range
                .cover(parent.syntax().text_range_with_trivia());
            entry.text_trimmed_range = entry
                .text_trimmed_range
                .cover(parent.syntax().text_trimmed_range());

            index = next_index;
            node = parent.syntax().parent()?;
        }

        // Merge the adjacent and overlapping ranges
        self.merge_adjacent_ranges();

        Some(())
    }

    /// Merge adjacent unreachable ranges into a single entry
    fn merge_adjacent_ranges(&mut self) {
        let mut index = 0;
        while index < self.ranges.len().saturating_sub(1) {
            let text_range = self.ranges[index].text_range;

            if self.ranges[index + 1].text_range.start() <= text_range.end() {
                let prev_entry = self.ranges.remove(index + 1);

                let entry = &mut self.ranges[index];
                entry.text_range = entry.text_range.cover(prev_entry.text_range);
                entry.text_trimmed_range = entry
                    .text_trimmed_range
                    .cover(prev_entry.text_trimmed_range);

                continue;
            }

            index += 1;
        }
    }
}

declare_node_union! {
    JsControlFlowNode =
        JsFunctionBody |
        JsVariableStatement |
        JsLabeledStatement |
        JsBlockStatement |
        JsDoWhileStatement |
        JsForInStatement |
        JsForOfStatement |
        JsForStatement |
        JsIfStatement |
        JsSwitchStatement |
        JsTryStatement |
        JsTryFinallyStatement |
        JsWhileStatement |
        JsCaseClause |
        JsDefaultClause
}

/// Try to find a section of `ranges` that matches `fields`, and returns an
/// index `i` into `ranges` such that the ranges from `i` to `i + fields.len()`
/// cover the corresponding entry in `fields`.
///
/// To avoid having to iterate over the whole length of `ranges`, the search is
/// guided using `index` to only try ranges starting between
/// `index - fields.len()` and `index`, clamped within the limits of `ranges`.
fn check_neighbors(
    ranges: &[UnreachableRange],
    index: usize,
    fields: &[TextRange],
) -> Option<usize> {
    if fields.len() > ranges.len() {
        return None;
    }

    let fields_end = fields.len().saturating_sub(1);
    let min_start = index.saturating_sub(fields_end);
    let max_start = (min_start + fields.len()).min(ranges.len().saturating_sub(fields_end));

    for start in min_start..max_start {
        let end = start + fields.len();
        let slice = &ranges[start..end];

        let is_matching = slice
            .iter()
            .zip(fields.iter().filter(|field| !field.is_empty()))
            .all(|(range, field)| range.text_range.contains_range(*field));

        if is_matching {
            return Some(start);
        }
    }

    None
}

impl IntoIterator for UnreachableRanges {
    type Item = UnreachableRange;
    type IntoIter = IntoIter<UnreachableRange>;

    fn into_iter(self) -> Self::IntoIter {
        self.ranges.into_iter()
    }
}

/// Stores the trimmed and un-trimmed ranges for a block of unreachable source
/// code, along with a list of secondary labels pointing to the dominating
/// terminator instructions that cause it to be unreachable
#[derive(Debug)]
pub struct UnreachableRange {
    text_range: TextRange,
    text_trimmed_range: TextRange,
    terminators: Vec<PathTerminator>,
}

#[derive(Debug, Clone, Copy)]
struct PathTerminator {
    kind: JsSyntaxKind,
    range: TextRange,
}

impl PathTerminator {
    /// Returns a message explaining why this paths is unreachable
    fn reason(&self) -> &'static str {
        match self.kind {
            JsSyntaxKind::JS_BREAK_STATEMENT => "break the flow of the code",
            JsSyntaxKind::JS_CONTINUE_STATEMENT => "continue the loop",
            JsSyntaxKind::JS_RETURN_STATEMENT => "return from the function",
            JsSyntaxKind::JS_THROW_STATEMENT => "throw an exception",
            _ => "stop the flow of the code",
        }
    }
}
