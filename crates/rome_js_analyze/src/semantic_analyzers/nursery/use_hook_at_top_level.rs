use super::use_exhaustive_dependencies::ReactExtensiveDependenciesOptions;
use crate::semantic_analyzers::nursery::use_exhaustive_dependencies::HooksOptions;
use crate::{react::hooks::react_hook_configuration, semantic_services::Semantic};
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use rome_js_semantic::CallsExtensions;
use rome_js_syntax::{AnyJsFunction, JsCallExpression, JsFunctionBody, JsSyntaxKind, TextRange};
use rome_rowan::AstNode;

declare_rule! {
    /// Enforce that all React hooks are being called from the Top Level
    /// component functions.
    ///
    /// To understand why this required see https://reactjs.org/docs/hooks-rules.html#only-call-hooks-at-the-top-level
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function Component1({ a }) {
    ///     if (a == 1) {
    ///         useEffect();
    ///     }
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// function Component1() {
    ///     useEffect();
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// Allows to specify custom hooks - from libraries or internal projects - that can be considered stable.
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options": {
    ///         "hooks": [
    ///             { "name": "useLocation", "closureIndex": 0, "dependenciesIndex": 1},
    ///             { "name": "useQuery", "closureIndex": 1, "dependenciesIndex": 0}
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// Given the previous example, your hooks be used like this:
    ///
    /// ```js
    /// function Foo() {
    ///     const location = useLocation(() => {}, []);
    ///     const query = useQuery([], () => {});
    /// }
    /// ```
    ///
    pub(crate) UseHookAtTopLevel {
        version: "1.0.0",
        name: "useHookAtTopLevel",
        recommended: false,
    }
}

pub enum Suggestion {
    None {
        hook_name_range: TextRange,
        path: Vec<TextRange>,
    },
}

// Verify if the call expression is at the top level
// of the component
fn enclosing_function_if_call_is_at_top_level(call: &JsCallExpression) -> Option<AnyJsFunction> {
    let next = call.syntax().ancestors().find(|x| {
        !matches!(
            x.kind(),
            JsSyntaxKind::JS_STATEMENT_LIST
                | JsSyntaxKind::JS_BLOCK_STATEMENT
                | JsSyntaxKind::JS_VARIABLE_STATEMENT
                | JsSyntaxKind::JS_EXPRESSION_STATEMENT
                | JsSyntaxKind::JS_RETURN_STATEMENT
                | JsSyntaxKind::JS_CALL_EXPRESSION
                | JsSyntaxKind::JS_CALL_ARGUMENT_LIST
                | JsSyntaxKind::JS_CALL_ARGUMENTS
                | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
                | JsSyntaxKind::JS_INITIALIZER_CLAUSE
                | JsSyntaxKind::JS_VARIABLE_DECLARATOR
                | JsSyntaxKind::JS_VARIABLE_DECLARATOR_LIST
                | JsSyntaxKind::JS_VARIABLE_DECLARATION
                | JsSyntaxKind::TS_AS_EXPRESSION
                | JsSyntaxKind::TS_SATISFIES_EXPRESSION
        )
    });

    next.and_then(JsFunctionBody::cast)
        .and_then(|body| body.parent::<AnyJsFunction>())
}

#[derive(Debug)]
pub struct CallPath {
    call: JsCallExpression,
    path: Vec<TextRange>,
}

impl Rule for UseHookAtTopLevel {
    type Query = Semantic<JsCallExpression>;
    type State = Suggestion;
    type Signals = Option<Self::State>;
    type Options = HooksOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let options = ctx.options();
        let options = ReactExtensiveDependenciesOptions::new(options.clone());

        let call = ctx.query();
        let hook_name_range = call.callee().ok()?.syntax().text_trimmed_range();
        if react_hook_configuration(call, &options.hooks_config).is_some() {
            let model = ctx.model();

            let root = CallPath {
                call: call.clone(),
                path: vec![],
            };
            let mut calls = vec![root];

            while let Some(CallPath { call, path }) = calls.pop() {
                let range = call.syntax().text_range();

                let mut path = path.clone();
                path.push(range);

                if let Some(enclosing_function) = enclosing_function_if_call_is_at_top_level(&call)
                {
                    if let Some(calls_iter) = enclosing_function.all_calls(model) {
                        for call in calls_iter {
                            calls.push(CallPath {
                                call: call.tree(),
                                path: path.clone(),
                            });
                        }
                    }
                } else {
                    return Some(Suggestion::None {
                        hook_name_range,
                        path,
                    });
                }
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, suggestion: &Self::State) -> Option<RuleDiagnostic> {
        match suggestion {
            Suggestion::None {
                hook_name_range,
                path,
            } => {
                let call_deep = path.len() - 1;

                let mut diag = if call_deep == 0 {
                    RuleDiagnostic::new(
                        rule_category!(),
                        hook_name_range,
                        markup! {
                            "This hook is being called conditionally, but all hooks must be called in the exact same order in every component render."
                        },
                    )
                } else {
                    RuleDiagnostic::new(
                        rule_category!(),
                        hook_name_range,
                        markup! {
                            "This hook is being called indirectly and conditionally, but all hooks must be called in the exact same order in every component render."
                        },
                    )
                };

                for (i, range) in path.iter().skip(1).enumerate() {
                    let msg = if i == 0 {
                        markup! {
                            "This is the call path until the hook."
                        }
                    } else {
                        markup! {}
                    };

                    diag = diag.detail(range, msg);
                }

                let diag = diag.note(
                    markup! {
                        "For React to preserve state between calls, hooks needs to be called unconditionally and always in the same order."
                    },
                ).note(
                    markup! {
                        "See https://reactjs.org/docs/hooks-rules.html#only-call-hooks-at-the-top-level"
                    },
                );
                Some(diag)
            }
        }
    }
}
