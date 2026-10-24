use crate::services::control_flow::{AnyJsControlFlowRoot, JsControlFlowGraph, control_flow_graph};
use crate::services::vue::VueComponent;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_control_flow::{ExceptionHandlerKind, InstructionKind, builder::ROOT_BLOCK_ID};
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsObjectMember, AnyJsObjectMemberName,
    JsReturnStatement,
};
use biome_rowan::{AstNode, NodeOrToken};
use biome_rule_options::use_vue_render_return::UseVueRenderReturnOptions;
use roaring::RoaringBitmap;

declare_lint_rule! {
    /// Require the `render` function of a Vue component to return a value.
    ///
    /// A `render` function describes what a component displays, and Vue uses the value it
    /// returns. When the function ends without a `return` statement, or uses `return;` without
    /// a value, the component renders nothing.
    ///
    /// The rule reports a `render` function when at least one way through the function
    /// ends without returning a value.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// import { h } from "vue";
    ///
    /// export default {
    ///   render() {
    ///     if (this.visible) {
    ///       return h("div", "hello");
    ///     }
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   render: function () {
    ///     return;
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// import { h } from "vue";
    ///
    /// export default {
    ///   render() {
    ///     if (this.visible) {
    ///       return h("div", "hello");
    ///     }
    ///     return null;
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    pub UseVueRenderReturn {
        version: "next",
        name: "useVueRenderReturn",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("require-render-return").inspired()],
    }
}

impl Rule for UseVueRenderReturn {
    type Query = VueComponent;
    /// The name of the component's `render` option.
    type State = AnyJsObjectMemberName;
    type Signals = Option<Self::State>;
    type Options = UseVueRenderReturnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let range = ctx.query().render_range()?;
        let member = ctx
            .root()
            .syntax()
            .covering_element(range)
            .ancestors()
            .find_map(AnyJsObjectMember::cast)?;
        let (name, function) = match member {
            AnyJsObjectMember::JsMethodObjectMember(method) => (
                method.name().ok()?,
                AnyJsControlFlowRoot::JsMethodObjectMember(method),
            ),
            AnyJsObjectMember::JsPropertyObjectMember(property) => {
                let function = match property.value().ok()?.inner_expression()? {
                    AnyJsExpression::JsFunctionExpression(function) => {
                        AnyJsControlFlowRoot::cast(function.into_syntax())?
                    }
                    AnyJsExpression::JsArrowFunctionExpression(function) => {
                        // An arrow function with an expression body always returns it.
                        if let Ok(AnyJsFunctionBody::AnyJsExpression(_)) = function.body() {
                            return None;
                        }
                        AnyJsControlFlowRoot::cast(function.into_syntax())?
                    }
                    _ => return None,
                };
                (property.name().ok()?, function)
            }
            _ => return None,
        };
        let cfg = control_flow_graph(ctx, &function)?;
        if returns_value_on_every_path(&cfg) {
            None
        } else {
            Some(name)
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, name: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                name.range(),
                markup! {
                    "This "<Emphasis>"render"</Emphasis>" function doesn't always return a value."
                },
            )
            .note(markup! {
                "Vue displays what the "<Emphasis>"render"</Emphasis>" function returns. When the function ends without returning a value, the component displays nothing."
            })
            .note(markup! {
                "Return a value at the end of every path through the function. Return "<Emphasis>"null"</Emphasis>" if the component should display nothing."
            }),
        )
    }
}

/// Returns `true` when every path through the function ends with a `return`
/// statement that has a value, or with a thrown error.
fn returns_value_on_every_path(cfg: &JsControlFlowGraph) -> bool {
    let mut block_stack = vec![ROOT_BLOCK_ID];
    let mut visited_blocks = RoaringBitmap::new();
    visited_blocks.insert(ROOT_BLOCK_ID.index());
    while let Some(block_id) = block_stack.pop() {
        let block = cfg.get(block_id);
        for handler in block.exception_handlers.iter() {
            // `finally` blocks are already part of the paths that go through them.
            if matches!(handler.kind, ExceptionHandlerKind::Catch)
                && visited_blocks.insert(handler.target.index())
            {
                block_stack.push(handler.target);
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
                    match &instruction.node {
                        // A `throw` statement also ends the path, and needs no value.
                        Some(NodeOrToken::Node(node)) => {
                            if JsReturnStatement::cast_ref(node)
                                .is_some_and(|statement| statement.argument().is_none())
                            {
                                return false;
                            }
                        }
                        // The path reaches the end of the function.
                        _ => return false,
                    }
                    break;
                }
            }
        }
    }
    true
}
