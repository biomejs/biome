use crate::ControlFlowGraph;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_control_flow::{ExceptionHandlerKind, InstructionKind};
use rome_js_syntax::{JsGetterClassMember, JsGetterObjectMember, JsReturnStatement};
use rome_rowan::{AstNode, NodeOrToken, TextRange};
use rustc_hash::FxHashSet;

declare_rule! {
    /// Enforces the presence of non-empty `return` statements in getters.
    ///
    /// A _getter_ allows defining a property which is dynamically computed.
    /// Thus, it is desirable that a _getter_ returns a value.
    ///
    /// Source: https://eslint.org/docs/latest/rules/getter-return
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class Person {
    ///     get firstName() {}
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const obj = {
    ///     get firstName() {
    ///         return;
    ///     },
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// class Person {
    ///     get firstName() {
    ///         return this.fullname.split(" ")[0];
    ///     }
    /// }
    /// ```
    ///
    /// ```js
    /// const obj = {
    ///     get firstName() {
    ///         return this.fullname.split(" ")[0];
    ///     },
    /// }
    /// ```
    ///
    pub(crate) UseGetterReturn {
        version: "1.0.0",
        name: "useGetterReturn",
        recommended: true,
    }
}

impl Rule for UseGetterReturn {
    type Query = ControlFlowGraph;
    type State = InvalidGetterReturn;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let cfg = ctx.query();
        let node_kind = cfg.node.kind();
        let mut invalid_returns = Vec::new();
        if !JsGetterClassMember::can_cast(node_kind) && !JsGetterObjectMember::can_cast(node_kind) {
            // The node is not a getter.
            return invalid_returns;
        }
        // stack of blocks to process
        let mut block_stack = Vec::new();
        let mut visited_blocks = FxHashSet::default();
        block_stack.push(0u32);
        visited_blocks.insert(0u32);
        while let Some(block_index) = block_stack.pop() {
            // SAFETY: this is a safe conversion because it is already an index for `cfg.blocks`.
            let block_index = block_index as usize;
            let Some(block) = cfg.blocks.get(block_index) else {
                continue;
            };
            for exception_handler in block.exception_handlers.iter() {
                // Ignore finally handler: they are already in the Control Flow Graph.
                if matches!(exception_handler.kind, ExceptionHandlerKind::Catch) {
                    // Avoid cycles and redundant checks.
                    if visited_blocks.insert(exception_handler.target) {
                        block_stack.push(exception_handler.target);
                    }
                }
            }
            for instruction in block.instructions.iter() {
                match instruction.kind {
                    InstructionKind::Statement => {}
                    InstructionKind::Jump {
                        block, conditional, ..
                    } => {
                        let jump_block_index = block.index();
                        // Avoid cycles and redundant checks.
                        if visited_blocks.insert(jump_block_index) {
                            block_stack.push(jump_block_index);
                        }
                        if !conditional {
                            // The next instructions are unreachable.
                            break;
                        }
                    }
                    InstructionKind::Return => {
                        if let Some(NodeOrToken::Node(node)) = instruction.node.clone() {
                            if let Some(return_stmt) = JsReturnStatement::cast(node) {
                                if return_stmt.argument().is_none() {
                                    invalid_returns.push(InvalidGetterReturn::EmptyReturn(
                                        return_stmt.range(),
                                    ));
                                }
                            }
                        } else {
                            invalid_returns.push(InvalidGetterReturn::MissingReturn);
                        }
                        // The next instructions are unreachable.
                        break;
                    }
                }
            }
        }
        invalid_returns
    }

    fn diagnostic(ctx: &RuleContext<Self>, invalid_return: &Self::State) -> Option<RuleDiagnostic> {
        let cfg = ctx.query();
        let diagnostic = match invalid_return {
            InvalidGetterReturn::MissingReturn => {
                let getter_range = cfg.node.text_trimmed_range();
                RuleDiagnostic::new(
                    rule_category!(),
                    getter_range,
                    markup! {
                        "This "<Emphasis>"getter"</Emphasis>" should "<Emphasis>"return"</Emphasis>" a value."
                    },
                )
            }
            InvalidGetterReturn::EmptyReturn(return_stmt_range) => RuleDiagnostic::new(
                rule_category!(),
                return_stmt_range,
                markup! {
                    "This "<Emphasis>"return"</Emphasis>" should return a value because it is located in a "<Emphasis>"return"</Emphasis>"."
                },
            ),
        };
        Some(diagnostic)
    }
}

#[derive(Debug)]
pub(crate) enum InvalidGetterReturn {
    /// No `return` statement.
    MissingReturn,
    // A `return` statement without argument.
    EmptyReturn(TextRange),
}
