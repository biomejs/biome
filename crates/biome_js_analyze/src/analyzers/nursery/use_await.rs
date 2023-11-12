use biome_analyze::{
    context::RuleContext, declare_rule, AddVisitor, Phases, QueryMatch, Queryable, Rule,
    RuleDiagnostic, ServiceBag, Visitor, VisitorContext,
};
use biome_console::markup;
use biome_js_syntax::{AnyFunctionLike, JsAwaitExpression, JsLanguage, TextRange, WalkEvent};
use biome_rowan::{AstNode, AstNodeList, Language, SyntaxNode, TextSize};

declare_rule! {
    /// Ensure `async` functions utilize `await`.
    ///
    /// This rule reports `async` functions that lack an `await` expression. As `async`
    /// functions return a promise, the use of `await` is often necessary to capture the
    /// resolved value and handle the asynchronous operation appropriately. Without `await`,
    /// the function operates synchronously and might not leverage the advantages of async
    /// functions.
    ///
    /// Source: [require-await](https://eslint.org/docs/latest/rules/require-await)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// async function fetchData() {
    /// // Missing `await` for the promise returned by `fetch`
    ///   return fetch('/data');
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// async function fetchData() {
    ///   const response = await fetch('/data');
    ///   const data = await response.json();
    ///   return data;
    /// }
    ///
    /// // This rule does not warn about non-async functions
    /// function processData() {
    ///   return compute(data);
    /// }
    ///
    /// // Nor does it warn about empty `async` functions
    /// async function noop() { }
    /// ```
    pub(crate) UseAwait {
        version: "1.3.0",
        name: "useAwait",
        recommended: true,
    }
}

#[derive(Default)]
struct MissingAwaitVisitor {
    /// Vector to hold a function node and a boolean indicating whether the function
    /// contains an `await` expression or not.
    stack: Vec<(TextSize, bool)>,
}

impl Visitor for MissingAwaitVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(node) => {
                // When the visitor enters an `async` function node, push a new entry on the stack
                if let Some(node) = AnyFunctionLike::cast_ref(node) {
                    if node.is_async() {
                        self.stack.push((node.range().start(), false));
                    }
                }

                // Replace JsYieldExpression with the appropriate check for `await`
                if JsAwaitExpression::can_cast(node.kind()) {
                    // When the visitor enters an `await` expression, set the
                    // `has_await` flag for the top entry on the stack to `true`
                    if let Some((_, has_await)) = self.stack.last_mut() {
                        *has_await = true;
                    }
                }
            }
            WalkEvent::Leave(node) => {
                // When the visitor exits a function, if it matches the node of the top-most
                // entry of the stack and the `has_await` flag is `false`, emit a query match
                if let Some(node) = AnyFunctionLike::cast_ref(node) {
                    if let Some((function_start_range, has_await)) = self.stack.pop() {
                        if function_start_range == node.range().start()
                            && !has_await
                            && node.is_async()
                        {
                            ctx.match_query(MissingAwait(node));
                        }
                    }
                }
            }
        }
    }
}

pub(crate) struct MissingAwait(AnyFunctionLike);

impl QueryMatch for MissingAwait {
    fn text_range(&self) -> TextRange {
        // Return the range of the function that's missing an `await` statement
        self.0.range()
    }
}

// Implement the Queryable for MissingAwait
impl Queryable for MissingAwait {
    type Input = Self;
    type Language = JsLanguage;
    type Output = AnyFunctionLike;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        // Register the MissingAwaitVisitor to run during the Syntax phase
        analyzer.add_visitor(Phases::Syntax, MissingAwaitVisitor::default);
    }

    fn unwrap_match(_: &ServiceBag, query: &Self::Input) -> Self::Output {
        // Extract the function that's missing an `await` statement from the input query match
        query.0.clone()
    }
}

impl Rule for UseAwait {
    type Query = MissingAwait;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        // Don't emit a diagnostic for `async` functions with an empty body
        if query.statements()?.is_empty() {
            return None;
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This async function lacks an "<Emphasis>"await"</Emphasis>" expression."
                },
            )
            .note(markup! {
                "Async functions should contain at least one 'await' expression. Functions with no 'await' may not need to be declared 'async'."
            }),
        )
    }
}
