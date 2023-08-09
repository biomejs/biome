use std::collections::VecDeque;

use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_control_flow::{ExceptionHandlerKind, InstructionKind};
use rome_js_syntax::{JsDefaultClause, JsLanguage, JsSwitchStatement, JsSyntaxNode};
use rome_rowan::{AstNode, AstNodeList, TextRange, WalkEvent};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{control_flow::AnyJsControlFlowRoot, ControlFlowGraph};

declare_rule! {
    /// Disallow fallthrough of `switch` clauses.
    ///
    /// Switch clauses in `switch` statements fall through by default.
    /// This can lead to unexpected behavior when forgotten.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-fallthrough
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
    /// ## Valid
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
    pub(crate) NoFallthroughSwitchClause {
        version: "1.0.0",
        name: "noFallthroughSwitchClause",
        recommended: false,
    }
}

impl Rule for NoFallthroughSwitchClause {
    type Query = ControlFlowGraph;
    type State = TextRange;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let cfg = ctx.query();
        let mut fallthrough: Vec<TextRange> = vec![];
        // Return early if the graph doesn't contain any switch statements.
        // This avoids to allocate some memory.
        if !has_switch_statement(&cfg.node) {
            return fallthrough;
        }
        // block to process.
        let mut block_stack = Vec::new();
        let mut visited_blocks = FxHashSet::default();
        block_stack.push(0u32);
        visited_blocks.insert(0u32);
        // Traverse the control flow graph and search for switch statements.
        while let Some(block_index) = block_stack.pop() {
            // SAFETY: this is a safe conversion because it is already an index for `cfg.blocks`.
            let block_index = block_index as usize;
            let Some(block) = cfg.blocks.get(block_index) else {
                continue;
            };
            // Register exception handlers as blocks to process
            // Ignore finally handler: they are already in the Control Flow Graph.
            for exception_handler in block
                .exception_handlers
                .iter()
                .filter(|x| matches!(x.kind, ExceptionHandlerKind::Catch))
            {
                // Avoid cycles and redundant checks.
                if visited_blocks.insert(exception_handler.target) {
                    block_stack.push(exception_handler.target);
                }
            }
            // Traverse the instructions of the block searching for a switch statement.
            // A switch statements is followed by conditional jumps (the cases),
            // and a last unconditional jump (maybe the default clause)
            let mut is_switch = false;
            let mut switch_clauses = VecDeque::new();
            let mut has_default_clause = false;
            let mut switch_clause_blocks = FxHashSet::default();
            let mut block_to_switch_clause_range = FxHashMap::default();
            for instruction in block.instructions.iter() {
                match instruction.kind {
                    InstructionKind::Statement => {
                        if let Some(node) = &instruction.node {
                            if let Some(switch_stmt) =
                                node.parent().and_then(JsSwitchStatement::cast)
                            {
                                if is_switch {
                                    unreachable!("A block cannot contain two switch statements.")
                                }
                                is_switch = true;
                                let switch_clause_nodes = switch_stmt.cases();
                                let mut default_clause = None;
                                switch_clauses = VecDeque::with_capacity(switch_clause_nodes.len());
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
                        conditional, block, ..
                    } => {
                        let jump_block_index = block.index();
                        // Avoid cycles and redundant checks.
                        if visited_blocks.insert(jump_block_index) {
                            block_stack.push(jump_block_index);
                        }
                        // If we are in a block of a switch statements,
                        // then any conditional jump is a case and an unconditional jump
                        // is a default clause if the switch has a default clause.
                        if is_switch && (conditional || has_default_clause) {
                            // Take the unconditional jump into account only if a default clause is present.
                            switch_clause_blocks.insert(jump_block_index);
                            let Some(switch_clause) = switch_clauses.pop_front() else {
                                unreachable!("Missing switch clause.")
                            };
                            block_to_switch_clause_range.insert(
                                jump_block_index,
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
            if !switch_clause_blocks.is_empty() {
                // Analyze the found switch clauses to detect any fallthrough.
                register_fallthrough_switch_clauses(
                    &block_to_switch_clause_range,
                    &visited_blocks,
                    cfg,
                    &mut fallthrough,
                );
            }
        }
        fallthrough
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
    block_to_switch_clause_range: &FxHashMap<u32, Option<TextRange>>,
    visited_blocks: &FxHashSet<u32>,
    cfg: &rome_control_flow::ControlFlowGraph<JsLanguage>,
    fallthrough: &mut Vec<TextRange>,
) {
    let mut current_switch_clause = None;
    // Register all switch clauses as block to process.
    let mut block_stack: Vec<u32> = block_to_switch_clause_range.keys().copied().collect();
    let mut visited_blocks = visited_blocks.clone();
    // Traverse the control flow graph
    while let Some(block_index) = block_stack.pop() {
        current_switch_clause = block_to_switch_clause_range
            .get(&block_index)
            .or(current_switch_clause);
        // SAFETY: this is a safe conversion because it is already an index for `cfg.blocks`.
        let block_index = block_index as usize;
        let Some(block) = cfg.blocks.get(block_index) else {
            continue;
        };
        // Register exception handlers as blocks to process
        // Ignore finally handler: they are already in the Control Flow Graph.
        for exception_handler in block
            .exception_handlers
            .iter()
            .filter(|x| matches!(x.kind, ExceptionHandlerKind::Catch))
        {
            // Avoid cycles and redundant checks.
            if visited_blocks.insert(exception_handler.target) {
                block_stack.push(exception_handler.target);
            }
        }
        for instruction in block.instructions.iter() {
            match instruction.kind {
                InstructionKind::Statement => {}
                InstructionKind::Jump {
                    conditional, block, ..
                } => {
                    let jump_block_index = block.index();
                    // Avoid cycles and redundant checks.
                    if visited_blocks.insert(jump_block_index) {
                        block_stack.push(jump_block_index);
                    }
                    if !conditional {
                        if block_to_switch_clause_range.contains_key(&jump_block_index) {
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
