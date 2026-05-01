use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, RuleSuppressions, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsFunction, JsCallArgumentList, JsCallArguments, JsCallExpression, JsLanguage,
};
use biome_rowan::AstNode;
use biome_rule_options::no_excessive_nested_callbacks::NoExcessiveNestedCallbacksOptions;

declare_lint_rule! {
    /// Enforce a maximum depth that callbacks can be nested.
    ///
    /// Deeply nested callbacks make asynchronous control flow difficult to read and follow.
    /// This rule reports callback functions nested beyond the configured limit.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// foo1(function () {
    ///     foo2(function () {
    ///         foo3(function () {
    ///             foo4(function () {
    ///                 foo5(function () {
    ///                     foo6(function () {});
    ///                 });
    ///             });
    ///         });
    ///     });
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// foo1(handleFoo1);
    ///
    /// function handleFoo1() {
    ///     foo2(handleFoo2);
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### max
    ///
    /// The maximum callback nesting depth allowed (default: 5).
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "max": 3
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    /// ```js,use_options,expect_diagnostic
    /// foo1(function () {
    ///     foo2(function () {
    ///         foo3(function () {
    ///             foo4(function () {});
    ///         });
    ///     });
    /// });
    /// ```
    ///
    /// #### Valid
    /// ```js,use_options
    /// foo1(function () {
    ///     foo2(function () {
    ///         foo3(function () {});
    ///     });
    /// });
    /// ```
    ///
    pub NoExcessiveNestedCallbacks {
        version: "next",
        name: "noExcessiveNestedCallbacks",
        language: "js",
        sources: &[RuleSource::Eslint("max-nested-callbacks").same()],
        severity: Severity::Warning,
        recommended: false,
    }
}

#[derive(Debug)]
pub struct CallbackDepth(usize);

impl Rule for NoExcessiveNestedCallbacks {
    type Query = Ast<AnyJsFunction>;
    type State = CallbackDepth;
    type Signals = Option<Self::State>;
    type Options = NoExcessiveNestedCallbacksOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let function = ctx.query();

        if !is_callback(function) {
            return None;
        }

        let depth = function
            .syntax()
            .ancestors()
            .filter_map(AnyJsFunction::cast)
            .filter(is_callback)
            .count();

        if depth > ctx.options().max() as usize {
            Some(CallbackDepth(depth))
        } else {
            None
        }
    }

    fn suppressed_nodes(
        ctx: &RuleContext<Self>,
        _state: &Self::State,
        suppressions: &mut RuleSuppressions<JsLanguage>,
    ) {
        // we only want to flag the first function that exceeds the nesting limit, to avoid overwhelming users with diagnostics. Suppress all nested callbacks within the flagged function to prevent additional diagnostics for them.
        let function = ctx.query();

        suppressions.suppress_node(function.syntax().clone());

        for nested_function in function
            .syntax()
            .descendants()
            .filter_map(AnyJsFunction::cast)
        {
            if is_callback(&nested_function) {
                suppressions.suppress_node(nested_function.syntax().clone());
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let function = ctx.query();
        let max = ctx.options().max();
        let depth = state.0;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                function.range(),
                markup! {
                    "This callback is nested too deeply."
                },
            )
            .note(markup! {
                "Callbacks nested "{depth}" levels deep are harder to read and maintain. The configured maximum is "{max}"."
            })
            .note(markup! {
                "Extract some callbacks into named functions to reduce nesting."
            })
        )
    }
}

fn is_callback(function: &AnyJsFunction) -> bool {
    if !matches!(
        function,
        AnyJsFunction::JsFunctionExpression(_) | AnyJsFunction::JsArrowFunctionExpression(_)
    ) {
        return false;
    }

    let Some(argument_list) = function
        .syntax()
        .parent()
        .and_then(JsCallArgumentList::cast)
    else {
        return false;
    };

    argument_list
        .parent::<JsCallArguments>()
        .and_then(|arguments| arguments.parent::<JsCallExpression>())
        .is_some()
}
