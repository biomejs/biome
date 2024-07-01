use biome_analyze::{
    context::RuleContext, declare_lint_rule, AddVisitor, Phases, QueryMatch, Queryable, Rule,
    RuleDiagnostic, RuleSource, ServiceBag, Visitor, VisitorContext,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyFunctionLike, JsAwaitExpression, JsForOfStatement, JsLanguage, TextRange, WalkEvent,
};
use biome_rowan::{AstNode, AstNodeList, Language, SyntaxNode, TextSize};

declare_lint_rule! {
    /// Ensure `async` functions utilize `await`.
    ///
    /// This rule reports `async` functions that lack an `await` expression. As `async`
    /// functions return a promise, the use of `await` is often necessary to capture the
    /// resolved value and handle the asynchronous operation appropriately. Without `await`,
    /// the function operates synchronously and might not leverage the advantages of async
    /// functions.
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
    pub UseAwait {
        version: "1.4.0",
        name: "useAwait",
        language: "js",
        sources: &[
            RuleSource::Eslint("require-await"),
            RuleSource::EslintTypeScript("require-await"),
        ],
        recommended: false,
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
                if let Some(node) = AnyFunctionLike::cast_ref(node) {
                    if node.is_async() {
                        self.stack.push((node.range().start(), false));
                    }
                }
                if let Some((_, has_await)) = self.stack.last_mut() {
                    if JsAwaitExpression::can_cast(node.kind()) {
                        *has_await = true;
                    } else if let Some(for_of) = JsForOfStatement::cast_ref(node) {
                        *has_await = *has_await || for_of.await_token().is_some();
                    }
                }
            }
            WalkEvent::Leave(node) => {
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

pub struct MissingAwait(AnyFunctionLike);

impl QueryMatch for MissingAwait {
    fn text_range(&self) -> TextRange {
        self.0.range()
    }
}

impl Queryable for MissingAwait {
    type Input = Self;
    type Language = JsLanguage;
    type Output = AnyFunctionLike;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, MissingAwaitVisitor::default);
    }

    fn unwrap_match(_: &ServiceBag, query: &Self::Input) -> Self::Output {
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
                    "This "<Emphasis>"async"</Emphasis>" function lacks an "<Emphasis>"await"</Emphasis>" expression."
                },
            )
            .note(markup! {
                <Emphasis>"Async"</Emphasis>" functions without "<Emphasis>"await"</Emphasis>" expressions may not need to be declared "<Emphasis>"async"</Emphasis>"."
            }).detail(ctx.query().range(), markup! {
                "Remove this "<Emphasis>"async"</Emphasis>" modifier, or add an "<Emphasis>"await"</Emphasis>" expression in the function."
            }),
        )
    }
}
