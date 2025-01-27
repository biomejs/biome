use std::collections::VecDeque;

use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_control_flow::{
    builder::{BlockId, ROOT_BLOCK_ID},
    ExceptionHandlerKind, InstructionKind,
};
use biome_diagnostics::Severity;
use biome_js_syntax::{JsDefaultClause, JsLanguage, JsSwitchStatement, JsSyntaxNode};
use biome_rowan::{AstNode, AstNodeList, TextRange, WalkEvent};
use roaring::RoaringBitmap;
use rustc_hash::FxHashMap;

use crate::{services::control_flow::AnyJsControlFlowRoot, ControlFlowGraph};

declare_lint_rule! {
    /// Disallow fallthrough of `switch` clauses.
    ///
    /// Switch clauses in `switch` statements fall through by default.
    /// This can lead to unexpected behavior when forgotten.
    ///
    /// > The rule doesn't take `process.exit()` in consideration.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// switch (bar) {
    /// 	case 0:
    /// 		a();
    /// 	case 1:
    /// 		b();
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// switch (foo) {
    /// 	case 1:
    ///     case 2:
    /// 		doSomething();
    /// 		break;
    ///     case 3: {
    ///         if (cond) {
    ///             break;
    ///         } else {
    ///             break;
    ///         }
    ///     }
    /// 	case 4:
    /// 		doSomething();
    /// }
    /// ```
    ///
    pub NoFallthroughSwitchClause {
        version: "1.0.0",
        name: "noFallthroughSwitchClause",
        language: "js",
        sources: &[RuleSource::Eslint("no-fallthrough")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoFallthroughSwitchClause {
    type Query = ControlFlowGraph;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let cfg = ctx.query();
        let mut fallthrough = Vec::new();
        // Return early if the graph doesn't contain any switch statements.
        // This avoids to allocate some memory.
        if !has_switch_statement(&cfg.node) {
            return fallthrough.into_boxed_slice();
        }
        // block to process.
        let mut block_stack = vec![ROOT_BLOCK_ID];
        let mut visited_blocks = RoaringBitmap::new();
        visited_blocks.insert(ROOT_BLOCK_ID.index());
        let mut switch_clauses = VecDeque::new();
        let mut block_to_switch_clause_range = FxHashMap::default();
        // Traverse the control flow graph and search for switch statements.
        while let Some(block_id) = block_stack.pop() {
            let block = cfg.get(block_id);
            // Register exception handlers as blocks to process
            // Ignore finally handler: they are already in the Control Flow Graph.
            for exception_handler in block
                .exception_handlers
                .iter()
                .filter(|x| matches!(x.kind, ExceptionHandlerKind::Catch))
            {
                // If the block was already visited, skip it.
                // This avoid cycles and checking twice the same block.
                if visited_blocks.insert(exception_handler.target.index()) {
                    block_stack.push(exception_handler.target);
                }
            }
            // Traverse the instructions of the block searching for a switch statement.
            // A switch statements is followed by conditional jumps (the cases),
            // and a last unconditional jump (maybe the default clause)
            let mut is_switch = false;
            let mut has_default_clause = false;
            for instruction in block.instructions.iter() {
                match instruction.kind {
                    InstructionKind::Statement => {
                        if let Some(node) = &instruction.node {
                            if let Some(switch_stmt) =
                                node.parent().and_then(JsSwitchStatement::cast)
                            {
                                // We assume that a block has a single switch.
                                // If this assertion fails, then it is likely a change in the
                                // implementation of the Control Flow Graph
                                debug_assert!(!is_switch);
                                is_switch = true;
                                let switch_clause_nodes = switch_stmt.cases();
                                let mut default_clause = None;
                                switch_clauses.reserve_exact(switch_clause_nodes.len());
                                block_to_switch_clause_range.reserve(switch_clause_nodes.len());
                                // Register in-order the switch cases, but the default clause which
                                // is inserted at the end.
                                // This mimics the order of the jumps in a blocks.
                                // The default clause, if any is the last (unconditional) jump.
                                for switch_clause in switch_clause_nodes {
                                    if JsDefaultClause::can_cast(switch_clause.syntax().kind()) {
                                        has_default_clause = true;
                                        default_clause = Some(switch_clause);
                                    } else {
                                        switch_clauses.push_back(switch_clause);
                                    }
                                }
                                if let Some(default_clause) = default_clause.take() {
                                    switch_clauses.push_back(default_clause);
                                }
                            }
                        }
                    }
                    InstructionKind::Jump {
                        conditional,
                        block: jump_block_id,
                        ..
                    } => {
                        // If the block was already visited, skip it.
                        // This avoid cycles and checking twice the same block.
                        if visited_blocks.insert(jump_block_id.index()) {
                            block_stack.push(jump_block_id);
                        }
                        // If we are in a block of a switch statements,
                        // then any conditional jump is a case and an unconditional jump
                        // is a default clause if the switch has a default clause.
                        if is_switch && (conditional || has_default_clause) {
                            // Take the unconditional jump into account only if a default clause is present.
                            let Some(switch_clause) = switch_clauses.pop_front() else {
                                break;
                            };
                            block_to_switch_clause_range.insert(
                                jump_block_id,
                                if switch_clause.consequent().is_empty() {
                                    // Ignore empty switch clauses
                                    None
                                } else {
                                    Some(switch_clause.range())
                                },
                            );
                        }
                        if !conditional {
                            // The next instructions are unreachable.
                            break;
                        }
                    }
                    InstructionKind::Return => {
                        // The next instructions are unreachable.
                        break;
                    }
                }
            }
            if !block_to_switch_clause_range.is_empty() {
                // Analyze the found switch clauses to detect any fallthrough.
                register_fallthrough_switch_clauses(
                    &block_to_switch_clause_range,
                    &visited_blocks,
                    cfg,
                    &mut fallthrough,
                );
                block_to_switch_clause_range.clear();
            }
            switch_clauses.clear();
        }
        fallthrough.into_boxed_slice()
    }

    fn diagnostic(
        _: &RuleContext<Self>,
        switch_clause_range: &Self::State,
    ) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                switch_clause_range,
                markup! {
                    "This case is falling through to the next case."
                },
            )
            .note(markup! {
                "Add a `break` or `return` statement to the end of this case to prevent fallthrough."
            }),
        )
    }
}

fn has_switch_statement(control_flow_root: &JsSyntaxNode) -> bool {
    let mut iter = control_flow_root.preorder();
    while let Some(node) = iter.next() {
        if let WalkEvent::Enter(node) = node {
            if JsSwitchStatement::can_cast(node.kind()) {
                return true;
            } else if AnyJsControlFlowRoot::can_cast(node.kind()) && control_flow_root != &node {
                iter.skip_subtree()
            }
        }
    }
    false
}

fn register_fallthrough_switch_clauses(
    block_to_switch_clause_range: &FxHashMap<BlockId, Option<TextRange>>,
    visited_blocks: &RoaringBitmap,
    cfg: &biome_control_flow::ControlFlowGraph<JsLanguage>,
    fallthrough: &mut Vec<TextRange>,
) {
    let mut current_switch_clause = None;
    // Register all switch clauses as block to process.
    let mut block_stack: Vec<_> = block_to_switch_clause_range.keys().copied().collect();
    let mut visited_blocks = visited_blocks.clone();
    // Traverse the control flow graph and detect fallthrough
    while let Some(block_id) = block_stack.pop() {
        current_switch_clause = block_to_switch_clause_range
            .get(&block_id)
            .or(current_switch_clause);
        let block = cfg.get(block_id);
        // Register exception handlers as blocks to process
        // Ignore finally handler: they are already in the Control Flow Graph.
        for exception_handler in block
            .exception_handlers
            .iter()
            .filter(|x| matches!(x.kind, ExceptionHandlerKind::Catch))
        {
            // If the block was already visited, skip it.
            // This avoid cycles and checking twice the same block.
            if visited_blocks.insert(exception_handler.target.index()) {
                block_stack.push(exception_handler.target);
            }
        }
        for instruction in block.instructions.iter() {
            match instruction.kind {
                InstructionKind::Statement => {}
                InstructionKind::Jump {
                    conditional,
                    block: jump_block_id,
                    ..
                } => {
                    // If the block was already visited, skip it.
                    // This avoid cycles and checking twice the same block.
                    if visited_blocks.insert(jump_block_id.index()) {
                        block_stack.push(jump_block_id);
                    }
                    if !conditional {
                        if block_to_switch_clause_range.contains_key(&jump_block_id) {
                            let Some(switch_clause_range) = current_switch_clause else {
                                unreachable!("The current switch clause should be known.");
                            };
                            // Ignore empty switch clauses
                            if let Some(switch_clause_range) = switch_clause_range {
                                fallthrough.push(*switch_clause_range);
                            }
                        }
                        // The next instructions are unreachable.
                        break;
                    }
                }
                InstructionKind::Return => {
                    // The next instructions are unreachable.
                    break;
                }
            }
        }
    }
}
