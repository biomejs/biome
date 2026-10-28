use crate::frameworks::vue::vue_instance::{
    is_in_component_definition_object, is_this_or_direct_const_alias,
};
use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsMemberExpression, JsAssignmentExpression,
    JsCallExpression, JsIdentifierExpression, JsVariableDeclarator,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::no_vue_multiple_slot_args::NoVueMultipleSlotArgsOptions;

declare_lint_rule! {
    /// Disallow passing more than one argument to a Vue slot.
    ///
    /// In a render function, `this.$slots.name(...)` renders the content that the parent component provides for the slot called `name`.
    /// When the parent writes that content in a template, it receives only the first argument as its slot props, so any other arguments are lost.
    /// Pass a single object that holds all the values instead.
    ///
    /// Spreading an array into a slot call, as in `this.$slots.default(...items)`, is also reported, because each item becomes a separate argument.
    ///
    /// The rule checks calls on `this.$slots`, or on a `const` variable that holds `this`, inside components defined with `export default { ... }` in a `.vue` file, `defineComponent()`, or `createApp()`.
    /// It also checks calls through a variable that holds a slot, such as `const item = this.$slots.item`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   render() {
    ///     return this.$slots.default(item, index);
    ///   }
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   render() {
    ///     return this.$slots.default(...items);
    ///   }
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   render() {
    ///     this.$slots.header();
    ///     this.$slots.footer(item);
    ///     return this.$slots.default({ item, index });
    ///   }
    /// };
    /// </script>
    /// ```
    pub NoVueMultipleSlotArgs {
        version: "next",
        name: "noVueMultipleSlotArgs",
        language: "js",
        sources: &[RuleSource::EslintVueJs("no-multiple-slot-args").same()],
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
    }
}

/// How a slot call passes its arguments incorrectly.
#[derive(Clone, Copy, Debug)]
pub enum SlotArgumentProblem {
    /// The call has two or more arguments.
    MultipleArguments,
    /// The first argument is a spread, such as `...items`.
    SpreadArgument,
}

impl Rule for NoVueMultipleSlotArgs {
    type Query = Semantic<JsCallExpression>;
    type State = SlotArgumentProblem;
    type Signals = Box<[Self::State]>;
    type Options = NoVueMultipleSlotArgsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let Ok(arguments) = call.arguments() else {
            return Box::default();
        };
        let arguments = arguments.args();

        let has_multiple_arguments = arguments.len() > 1;
        let has_spread_argument =
            matches!(arguments.first(), Some(Ok(AnyJsCallArgument::JsSpread(_))));
        if !(has_multiple_arguments || has_spread_argument) || !is_slot_call(ctx, call) {
            return Box::default();
        }

        let mut problems = Vec::with_capacity(2);
        if has_multiple_arguments {
            problems.push(SlotArgumentProblem::MultipleArguments);
        }
        if has_spread_argument {
            problems.push(SlotArgumentProblem::SpreadArgument);
        }
        problems.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, problem: &Self::State) -> Option<RuleDiagnostic> {
        let arguments = ctx.query().arguments().ok()?.args();
        let diagnostic = match problem {
            SlotArgumentProblem::MultipleArguments => {
                let second = arguments.iter().nth(1)?.ok()?;
                let last = arguments.last()?.ok()?;
                let range = TextRange::new(
                    second.syntax().text_trimmed_range().start(),
                    last.syntax().text_trimmed_range().end(),
                );
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "This slot is called with more than one argument."
                    },
                )
                .note(markup! {
                    "Slot content written in a template receives only the first argument, so the other arguments are lost."
                })
            }
            SlotArgumentProblem::SpreadArgument => RuleDiagnostic::new(
                rule_category!(),
                arguments.first()?.ok()?.range(),
                markup! {
                    "This slot is called with a spread argument."
                },
            )
            .note(markup! {
                "Spreading passes each item as a separate argument, but slot content written in a template receives only the first one."
            }),
        };
        Some(diagnostic.note(markup! {
            "Pass a single object that holds all the values, for example "<Emphasis>"this.$slots.default({ item, index })"</Emphasis>"."
        }))
    }
}

/// Returns `true` if `call` calls a slot, either directly as in
/// `this.$slots.name()`, or through a variable that holds one.
fn is_slot_call(ctx: &RuleContext<NoVueMultipleSlotArgs>, call: &JsCallExpression) -> bool {
    let Some(callee) = call.callee().ok().and_then(|callee| callee.inner_expression()) else {
        return false;
    };
    match callee {
        AnyJsExpression::JsIdentifierExpression(identifier) => {
            is_slot_variable(ctx, &identifier)
        }
        callee => is_slot_member(ctx, &callee),
    }
}

/// Returns `true` if `expression` reads a member of `this.$slots`, such as
/// `this.$slots.default` or `vm.$slots["header"]`.
///
/// Parentheses, non-null assertions, and type assertions around the member
/// and around `this.$slots` are ignored.
fn is_slot_member(ctx: &RuleContext<NoVueMultipleSlotArgs>, expression: &AnyJsExpression) -> bool {
    let Some(slot) = expression
        .inner_expression()
        .and_then(|expression| AnyJsMemberExpression::cast(expression.into_syntax()))
    else {
        return false;
    };
    let Some(slots) = slot
        .object()
        .ok()
        .and_then(|object| object.inner_expression())
        .and_then(|object| AnyJsMemberExpression::cast(object.into_syntax()))
    else {
        return false;
    };
    slots
        .member_name()
        .is_some_and(|name| name.text() == "$slots")
        && slots
            .object()
            .is_ok_and(|object| is_this_or_direct_const_alias(object, ctx.model()))
        && is_in_component_definition_object(
            &slots,
            ctx.model(),
            ctx.source_type::<JsFileSource>(),
            ctx.file_path(),
        )
}

/// Returns `true` if the variable `identifier` refers to is initialized with,
/// or assigned, a member of `this.$slots`.
fn is_slot_variable(
    ctx: &RuleContext<NoVueMultipleSlotArgs>,
    identifier: &JsIdentifierExpression,
) -> bool {
    let Some(binding) = identifier
        .name()
        .ok()
        .and_then(|reference| ctx.model().binding(&reference))
    else {
        return false;
    };

    let initialized_with_slot = binding
        .syntax()
        .parent()
        .and_then(JsVariableDeclarator::cast)
        .and_then(|declarator| declarator.initializer())
        .and_then(|initializer| initializer.expression().ok())
        .is_some_and(|initializer| is_slot_member(ctx, &initializer));
    if initialized_with_slot {
        return true;
    }

    binding.all_writes().any(|write| {
        write
            .syntax()
            .parent()
            .and_then(JsAssignmentExpression::cast)
            .and_then(|assignment| assignment.right().ok())
            .is_some_and(|value| is_slot_member(ctx, &value))
    })
}
