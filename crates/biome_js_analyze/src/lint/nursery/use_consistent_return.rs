use crate::ControlFlowGraph;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_control_flow::{ExceptionHandlerKind, InstructionKind, builder::ROOT_BLOCK_ID};
use biome_js_syntax::{
    AnyJsFunction, JsConstructorClassMember, JsGetterClassMember, JsGetterObjectMember,
    JsMethodClassMember, JsMethodObjectMember, JsReturnStatement, JsSetterClassMember,
    JsSetterObjectMember, JsSyntaxKind,
};
use biome_rowan::{AstNode, NodeOrToken, TextRange};
use biome_rule_options::use_consistent_return::UseConsistentReturnOptions;
use roaring::RoaringBitmap;

declare_lint_rule! {
    /// Require `return` statements to either always or never specify values.
    ///
    /// Unifying the return behavior of a function makes its contract obvious:
    /// callers know whether a meaningful value comes back. A function that
    /// returns a value on one path but nothing (an explicit `return;` or by
    /// falling off the end) on another is usually a mistake.
    ///
    /// This rule reports a bare `return;` — or a code path that finishes without
    /// returning — when the same function returns a value elsewhere.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function foo(bar) {
    ///     if (bar) {
    ///         return true;
    ///     }
    ///     return;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo(bar) {
    ///     if (bar) {
    ///         return true;
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function foo(bar) {
    ///     if (bar) {
    ///         return true;
    ///     }
    ///     return false;
    /// }
    /// ```
    ///
    /// ```js
    /// function foo(bar) {
    ///     if (bar) {
    ///         return;
    ///     }
    /// }
    /// ```
    ///
    pub UseConsistentReturn {
        version: "next",
        name: "useConsistentReturn",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("consistent-return").same()],
    }
}

#[derive(Debug)]
pub enum InconsistentReturn {
    /// A bare `return;` in a function that returns a value elsewhere.
    MissingValue(TextRange),
    /// A code path completes without returning, in a function that returns a
    /// value elsewhere. Reported at the function.
    ImplicitReturn(TextRange),
}

impl Rule for UseConsistentReturn {
    type Query = ControlFlowGraph;
    type State = InconsistentReturn;
    type Signals = Box<[Self::State]>;
    type Options = UseConsistentReturnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let cfg = ctx.query();
        if !is_function_like(cfg.node.kind()) {
            return Box::new([]);
        }

        let mut has_value_return = false;
        let mut bare_returns = Vec::new();
        let mut has_implicit_return = false;

        // Walk every reachable block, following jumps and catch handlers,
        // exactly like `useGetterReturn`. Unreachable tail instructions after a
        // `return`/unconditional jump are skipped, so an implicit end-of-function
        // exit only counts when a real code path reaches it.
        let mut block_stack = vec![ROOT_BLOCK_ID];
        let mut visited_blocks = RoaringBitmap::new();
        visited_blocks.insert(ROOT_BLOCK_ID.index());
        while let Some(block_id) = block_stack.pop() {
            let block = cfg.get(block_id);
            for exception_handler in block.exception_handlers.iter() {
                if matches!(exception_handler.kind, ExceptionHandlerKind::Catch)
                    && visited_blocks.insert(exception_handler.target.index())
                {
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
                        if visited_blocks.insert(jump_block_id.index()) {
                            block_stack.push(jump_block_id);
                        }
                        if !conditional {
                            break;
                        }
                    }
                    InstructionKind::Return => {
                        if let Some(NodeOrToken::Node(node)) = &instruction.node {
                            if let Some(return_stmt) = JsReturnStatement::cast_ref(node) {
                                if return_stmt.argument().is_some() {
                                    has_value_return = true;
                                } else {
                                    bare_returns.push(return_stmt.range());
                                }
                            }
                        } else {
                            // Implicit `return` synthesized at the end of the function.
                            has_implicit_return = true;
                        }
                        break;
                    }
                }
            }
        }

        // Only a value-returning function is held to consistency; a function
        // that never returns a value is fine with bare returns / implicit exits.
        if !has_value_return {
            return Box::new([]);
        }

        let mut signals: Vec<InconsistentReturn> = bare_returns
            .into_iter()
            .map(InconsistentReturn::MissingValue)
            .collect();
        if has_implicit_return {
            signals.push(InconsistentReturn::ImplicitReturn(
                cfg.node.text_trimmed_range(),
            ));
        }
        signals.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state {
            InconsistentReturn::MissingValue(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This "<Emphasis>"return"</Emphasis>" does not return a value."
                },
            )
            .note(markup! {
                "This function returns a value on another code path, so its returns are inconsistent."
            })
            .note(markup! {
                "Return a value here, or remove the values from the other returns."
            }),
            InconsistentReturn::ImplicitReturn(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This function can finish without returning a value."
                },
            )
            .note(markup! {
                "It returns a value on another code path, so completing without a value is inconsistent."
            })
            .note(markup! {
                "Return a value on every code path, or on none of them."
            }),
        };
        Some(diagnostic)
    }
}

/// Whether the control-flow-graph root is a function-like node (as opposed to a
/// module or a class static-initialization block, where a `return` value has no
/// meaning).
fn is_function_like(kind: JsSyntaxKind) -> bool {
    AnyJsFunction::can_cast(kind)
        || JsMethodClassMember::can_cast(kind)
        || JsMethodObjectMember::can_cast(kind)
        || JsGetterClassMember::can_cast(kind)
        || JsGetterObjectMember::can_cast(kind)
        || JsSetterClassMember::can_cast(kind)
        || JsSetterObjectMember::can_cast(kind)
        || JsConstructorClassMember::can_cast(kind)
}
