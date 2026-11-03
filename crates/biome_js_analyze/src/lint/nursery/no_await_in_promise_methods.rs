use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, AnyJsStatement, JsArrayElementList,
    JsArrowFunctionExpression, JsAwaitExpression, JsCallArgumentList, JsCallArguments,
    JsCallExpression, JsFunctionBody, JsLanguage, JsParenthesizedExpression, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxKindSet};
use biome_rule_options::no_await_in_promise_methods::NoAwaitInPromiseMethodsOptions;

use crate::{JsRuleAction, services::semantic::Semantic};

declare_lint_rule! {
    /// Disallow using `await` inside the array passed to `Promise.all()`, `Promise.allSettled()`, `Promise.any()`, or `Promise.race()`.
    ///
    /// These methods wait for several promises at the same time.
    /// Using `await` while building the array pauses the code until that promise finishes.
    /// The rest of the array, and the method call itself, can only run after that,
    /// so the promises no longer run at the same time.
    ///
    /// If an element needs an awaited value, move the `await` into an async function and call it right away,
    /// or use `.then()`. Both create a promise without pausing the code.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Promise.all([await promise, anotherPromise]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Promise.race([condition ? await promise : fallbackPromise, anotherPromise]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Promise.all([(await fetch(url)).json(), anotherPromise]);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// Promise.all([promise, anotherPromise]);
    /// Promise.allSettled([promise.then((value) => value.foo), anotherPromise]);
    /// Promise.any([(async () => (await fetch(url)).json())(), anotherPromise]);
    /// ```
    ///
    pub NoAwaitInPromiseMethods {
        version: "next",
        name: "noAwaitInPromiseMethods",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-await-in-promise-methods").same()],
        recommended: true,
        severity: Severity::Warning,
        // Unsafe because removing `await` changes when the promise is created and what value ends up in the array.
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoAwaitInPromiseMethods {
    type Query = Semantic<JsAwaitExpression>;
    type State = NoAwaitInPromiseMethodsState;
    type Signals = Option<Self::State>;
    type Options = NoAwaitInPromiseMethodsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut element = node.syntax().clone();
        for ancestor in node.syntax().ancestors().skip(1) {
            if WALK_BOUNDARY_KINDS.matches(ancestor.kind()) {
                return None;
            }
            if let Some(list) = JsArrayElementList::cast_ref(&ancestor)
                && let Some(method) = promise_method_of_array(ctx.model(), &list)
            {
                let is_array_element = AnyJsExpression::cast(element)
                    .is_some_and(|element| element.omit_parentheses().syntax() == node.syntax());
                return Some(NoAwaitInPromiseMethodsState {
                    method,
                    is_array_element,
                });
            }
            element = ancestor;
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let method = state.method;
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "This "<Emphasis>"await"</Emphasis>" delays the call to "<Emphasis>"Promise."{method}"()"</Emphasis>"."
            },
        )
        .note(markup! {
            "The rest of the array and the call to "<Emphasis>"Promise."{method}"()"</Emphasis>" have to wait until this promise finishes, so the promises don't run at the same time."
        });
        Some(if state.is_array_element {
            diagnostic
        } else {
            diagnostic.note(markup! {
                "Move the "<Emphasis>"await"</Emphasis>" into an async function that you call right away, or use "<Emphasis>".then()"</Emphasis>" instead."
            })
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        if !state.is_array_element {
            return None;
        }
        let node = ctx.query();
        let await_token = node.await_token().ok()?;
        let argument = node.argument().ok()?;
        // Keeps the comments around `await`, but not the space between `await` and its argument.
        let leading_trivia: Vec<_> = await_token
            .leading_trivia()
            .pieces()
            .chain(
                await_token
                    .trailing_trivia()
                    .pieces()
                    .chain(argument.syntax().first_leading_trivia()?.pieces())
                    .skip_while(|piece| piece.is_whitespace() || piece.is_newline()),
            )
            .collect();
        let new_argument = argument.clone().with_leading_trivia_pieces(leading_trivia)?;

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(AnyJsExpression::from(node.clone()), new_argument);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"await"</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

pub struct NoAwaitInPromiseMethodsState {
    /// The `Promise` method whose array contains the `await`.
    method: PromiseMethod,
    /// Whether the `await` expression is itself an element of the array, ignoring parentheses.
    is_array_element: bool,
}

/// Returns the `Promise` method that receives the array of `list` as its only argument.
fn promise_method_of_array(
    model: &SemanticModel,
    list: &JsArrayElementList,
) -> Option<PromiseMethod> {
    let argument = list
        .syntax()
        .ancestors()
        .skip(2)
        .find(|node| !JsParenthesizedExpression::can_cast(node.kind()))?;
    let arguments = JsCallArgumentList::cast(argument)?;
    if arguments.len() != 1 {
        return None;
    }
    let call = arguments
        .parent::<JsCallArguments>()?
        .parent::<JsCallExpression>()?;
    if call.is_optional() {
        return None;
    }
    let callee = AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())?;
    if callee.is_optional_chain() {
        return None;
    }
    let method = PromiseMethod::from_name(callee.member_name()?.text())?;
    let object = callee
        .object()
        .ok()?
        .omit_parentheses()
        .as_any_global_identifier_expression()?;
    let (reference, name) = global_identifier(&object)?;
    (name.text() == "Promise" && model.binding(&reference).is_none()).then_some(method)
}

#[derive(Clone, Copy)]
enum PromiseMethod {
    All,
    AllSettled,
    Any,
    Race,
}

impl PromiseMethod {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "all" => Some(Self::All),
            "allSettled" => Some(Self::AllSettled),
            "any" => Some(Self::Any),
            "race" => Some(Self::Race),
            _ => None,
        }
    }
}

impl biome_console::fmt::Display for PromiseMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::All => "all",
            Self::AllSettled => "allSettled",
            Self::Any => "any",
            Self::Race => "race",
        })
    }
}

/// The nodes that stop the search for an enclosing `Promise` method call:
/// function bodies, whose code runs later; outer `await` expressions, which
/// are reported instead; and statements, which an array element can only
/// contain inside a function body.
const WALK_BOUNDARY_KINDS: SyntaxKindSet<JsLanguage> = JsFunctionBody::KIND_SET
    .union(JsArrowFunctionExpression::KIND_SET)
    .union(JsAwaitExpression::KIND_SET)
    .union(AnyJsStatement::KIND_SET);
