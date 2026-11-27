use crate::services::control_flow::{AnyJsControlFlowRoot, control_flow_graph};
use crate::utils::function_return::{InvalidReturn, invalid_returns};
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsFunction, JsConstructorClassMember, JsFunctionBody, JsLanguage, JsReturnStatement,
};
use biome_rowan::{AstNode, SyntaxKindSet, TextRange, WalkEvent};
use biome_rule_options::no_mixed_returns::NoMixedReturnsOptions;

declare_lint_rule! {
    /// Require `return` statements in a function to either always or never return a value.
    ///
    /// When only some `return` statements in a function return a value, code that calls the
    /// function can get `undefined` when it doesn't expect it.
    ///
    /// This only looks at syntax, not types. Typescript has a built in check
    /// [`noImplicitReturns`](https://www.typescriptlang.org/tsconfig/#noImplicitReturns),
    /// which is recommended if you want type information to be taken into account.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function doSomething(condition) {
    ///     if (condition) {
    ///         return true;
    ///     } else {
    ///         return;
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function doSomethingElse(condition) {
    ///     if (condition) {
    ///         return true;
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function doSomething(condition) {
    ///     if (condition) {
    ///         return true;
    ///     } else {
    ///         return false;
    ///     }
    /// }
    ///
    /// function doSomethingElse(condition) {
    ///     if (condition) {
    ///         return true;
    ///     }
    ///     return undefined;
    /// }
    ///
    /// function Foo() {
    ///     if (!(this instanceof Foo)) {
    ///         return new Foo();
    ///     }
    ///     this.a = 0;
    /// }
    /// ```
    ///
    pub NoMixedReturns {
        version: "next",
        name: "noMixedReturns",
        language: "js",
        sources: &[
            RuleSource::Eslint("consistent-return").same(),
            RuleSource::EslintTypeScript("consistent-return").inspired(),
        ],
        recommended: false,
    }
}

impl Rule for NoMixedReturns {
    type Query = Ast<JsFunctionBody>;
    type State = NoMixedReturnsState;
    type Signals = Box<[Self::State]>;
    type Options = NoMixedReturnsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let body = ctx.query();
        let mut first_return = None;
        let mut signals = Vec::new();

        let mut events = body.syntax().preorder_of_kinds(RETURN_SEARCH_KINDS);
        while let Some(event) = events.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            // Nested functions have their own `return` statements.
            events.skip_subtree();
            let Some(statement) = JsReturnStatement::cast(node) else {
                continue;
            };
            let has_value = statement.argument().is_some();
            match first_return {
                None => first_return = Some((statement.range(), has_value)),
                Some((first, first_has_value)) if first_has_value != has_value => {
                    let statement = statement.range();
                    signals.push(if first_has_value {
                        NoMixedReturnsState::MissingValue { statement, first }
                    } else {
                        NoMixedReturnsState::UnexpectedValue { statement, first }
                    });
                }
                Some(_) => {}
            }
        }

        if let Some((first, true)) = first_return
            && !is_constructor(body)
            && let Some(function) = body.parent::<AnyJsControlFlowRoot>()
            && let Some(cfg) = control_flow_graph(ctx, &function)
            && invalid_returns(&cfg)
                .iter()
                .any(|invalid_return| matches!(invalid_return, InvalidReturn::MissingReturn))
        {
            signals.push(NoMixedReturnsState::MissingReturnAtEnd { first });
        }

        signals.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (range, first) = match state {
            NoMixedReturnsState::MissingValue { statement, first }
            | NoMixedReturnsState::UnexpectedValue { statement, first } => (*statement, *first),
            NoMixedReturnsState::MissingReturnAtEnd { first } => {
                (function_head_range(ctx.query())?, *first)
            }
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                match state {
                    NoMixedReturnsState::MissingValue { .. } => markup! {
                        "This "<Emphasis>"return"</Emphasis>" doesn't return a value, but another "<Emphasis>"return"</Emphasis>" in this function does."
                    },
                    NoMixedReturnsState::UnexpectedValue { .. } => markup! {
                        "This "<Emphasis>"return"</Emphasis>" returns a value, but another "<Emphasis>"return"</Emphasis>" in this function doesn't."
                    },
                    NoMixedReturnsState::MissingReturnAtEnd { .. } => markup! {
                        "This function can reach its end without returning a value, but it returns a value elsewhere."
                    },
                },
            )
            .detail(
                first,
                if matches!(state, NoMixedReturnsState::UnexpectedValue { .. }) {
                    markup! { "This "<Emphasis>"return"</Emphasis>" doesn't return a value." }
                } else {
                    markup! { "This "<Emphasis>"return"</Emphasis>" returns a value." }
                },
            )
            .note(markup! {
                "A function that returns a value on some paths gives "<Emphasis>"undefined"</Emphasis>" on the others, which is easy to overlook."
            })
            .note(if matches!(state, NoMixedReturnsState::MissingReturnAtEnd { .. }) {
                markup! {
                    "Add a "<Emphasis>"return"</Emphasis>" with a value at the end of the function. If the function should give "<Emphasis>"undefined"</Emphasis>", write "<Emphasis>"return undefined"</Emphasis>"."
                }
            } else {
                markup! {
                    "Make every "<Emphasis>"return"</Emphasis>" in this function return a value, or none of them. If the function should give "<Emphasis>"undefined"</Emphasis>", write "<Emphasis>"return undefined"</Emphasis>"."
                }
            }),
        )
    }
}

/// A `return` statement, or the end of a function body, that doesn't match the first `return`
/// statement of the function.
pub enum NoMixedReturnsState {
    /// A `return` statement without a value, in a function whose first `return` statement has a
    /// value.
    MissingValue {
        statement: TextRange,
        first: TextRange,
    },
    /// A `return` statement with a value, in a function whose first `return` statement has no
    /// value.
    UnexpectedValue {
        statement: TextRange,
        first: TextRange,
    },
    /// The end of the function body can be reached, and the first `return` statement of the
    /// function has a value.
    MissingReturnAtEnd { first: TextRange },
}

/// Returns whether the function of `body` is a class constructor, or is named like a constructor,
/// such as `function Foo() {}`.
fn is_constructor(body: &JsFunctionBody) -> bool {
    let Some(parent) = body.syntax().parent() else {
        return false;
    };
    if JsConstructorClassMember::can_cast(parent.kind()) {
        return true;
    }
    let name = match AnyJsFunction::cast(parent) {
        Some(AnyJsFunction::JsFunctionDeclaration(declaration)) => declaration.id().ok(),
        Some(AnyJsFunction::JsFunctionExportDefaultDeclaration(declaration)) => declaration.id(),
        Some(AnyJsFunction::JsFunctionExpression(expression)) => expression.id(),
        Some(AnyJsFunction::JsArrowFunctionExpression(_)) | None => None,
    };
    name.and_then(|name| name.as_js_identifier_binding()?.name_token().ok())
        .and_then(|name| name.text_trimmed().chars().next())
        .is_some_and(char::is_uppercase)
}

/// Returns the range from the start of the function to the end of the token preceding its body,
/// so the diagnostic doesn't span the whole function.
fn function_head_range(body: &JsFunctionBody) -> Option<TextRange> {
    let function = body.syntax().parent()?;
    let last_head_token = body.syntax().first_token()?.prev_token()?;
    Some(TextRange::new(
        function.text_trimmed_range().start(),
        last_head_token.text_trimmed_range().end(),
    ))
}

/// The nodes the search for `return` statements stops at: `return` statements, and the functions
/// whose `return` statements don't belong to the searched function.
const RETURN_SEARCH_KINDS: SyntaxKindSet<JsLanguage> =
    JsReturnStatement::KIND_SET.union(AnyJsControlFlowRoot::KIND_SET);
