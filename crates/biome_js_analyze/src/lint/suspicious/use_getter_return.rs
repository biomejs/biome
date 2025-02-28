use crate::ControlFlowGraph;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_control_flow::{builder::ROOT_BLOCK_ID, ExceptionHandlerKind, InstructionKind};
use biome_diagnostics::Severity;
use biome_js_syntax::{JsGetterClassMember, JsGetterObjectMember, JsReturnStatement};
use biome_rowan::{AstNode, NodeOrToken, TextRange};
use roaring::RoaringBitmap;

declare_lint_rule! {
    /// Enforce `get` methods to always return a value.
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
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Option {
    ///     get value() {
    ///         if (this.hasValue) {
    ///             log();
    ///         } else {
    ///             return null;
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
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
    ///     }
    /// }
    /// ```
    ///
    pub UseGetterReturn {
        version: "1.0.0",
        name: "useGetterReturn",
        language: "js",
        sources: &[RuleSource::Eslint("getter-return")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseGetterReturn {
    type Query = ControlFlowGraph;
    type State = InvalidGetterReturn;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let cfg = ctx.query();
        let node_kind = cfg.node.kind();
        let mut invalid_returns = Vec::new();
        if !JsGetterClassMember::can_cast(node_kind) && !JsGetterObjectMember::can_cast(node_kind) {
            // The node is not a getter.
            return invalid_returns.into_boxed_slice();
        }
        // stack of blocks to process
        let mut block_stack = vec![ROOT_BLOCK_ID];
        let mut visited_blocks = RoaringBitmap::new();
        visited_blocks.insert(ROOT_BLOCK_ID.index());
        while let Some(block_id) = block_stack.pop() {
            let block = cfg.get(block_id);
            for exception_handler in block.exception_handlers.iter() {
                // Ignore finally handler: they are already in the Control Flow Graph.
                if matches!(exception_handler.kind, ExceptionHandlerKind::Catch) {
                    // Avoid cycles and redundant checks.
                    if visited_blocks.insert(exception_handler.target.index()) {
                        block_stack.push(exception_handler.target);
                    }
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
                        // Avoid cycles and redundant checks.
                        if visited_blocks.insert(jump_block_id.index()) {
                            block_stack.push(jump_block_id);
                        }
                        if !conditional {
                            // The next instructions are unreachable.
                            break;
                        }
                    }
                    InstructionKind::Return => {
                        if let Some(NodeOrToken::Node(node)) = &instruction.node {
                            if let Some(return_stmt) = JsReturnStatement::cast_ref(node) {
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
        invalid_returns.into_boxed_slice()
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
                    "This "<Emphasis>"return"</Emphasis>" should return a value because it is located in a "<Emphasis>"getter"</Emphasis>"."
                },
            ),
        };
        Some(diagnostic)
    }
}

#[derive(Debug)]
pub enum InvalidGetterReturn {
    /// No `return` statement.
    MissingReturn,
    // A `return` statement without argument.
    EmptyReturn(TextRange),
}
