use biome_analyze::context::RuleContext;
use biome_analyze::{
    declare_lint_rule, AddVisitor, Phases, QueryMatch, Queryable, Rule, RuleDiagnostic, RuleSource,
    ServiceBag, Visitor, VisitorContext,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyFunctionLike, JsLanguage, JsYieldExpression, TextRange, WalkEvent};
use biome_rowan::{AstNode, AstNodeList, Language, SyntaxNode, TextSize};

declare_lint_rule! {
    /// Require generator functions to contain `yield`.
    ///
    /// This rule generates warnings for generator functions that do not have the `yield` keyword.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function* foo() {
    ///   return 10;
    /// }
    /// ```
    ///
    /// ### Valid
    /// ```js
    /// function* foo() {
    ///   yield 5;
    ///   return 10;
    /// }
    ///
    /// function foo() {
    ///   return 10;
    /// }
    ///
    /// // This rule does not warn on empty generator functions.
    /// function* foo() { }
    /// ```
    pub UseYield {
        version: "1.0.0",
        name: "useYield",
        language: "js",
        sources: &[RuleSource::Eslint("require-yield")],
        recommended: true,
        severity: Severity::Error,
    }
}

#[derive(Default)]
struct MissingYieldVisitor {
    /// Vector to hold a function node and a boolean indicating whether the function
    /// contains an `yield` expression or not.
    stack: Vec<(TextSize, bool)>,
}

impl Visitor for MissingYieldVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(node) => {
                // When the visitor enters a function node, push a new entry on the stack
                if let Some(node) = AnyFunctionLike::cast_ref(node) {
                    if node.is_generator() {
                        self.stack.push((node.range().start(), false));
                    }
                }

                if JsYieldExpression::can_cast(node.kind()) {
                    // When the visitor enters a `yield` expression, set the
                    // `has_yield` flag for the top entry on the stack to `true`
                    if let Some((_, has_yield)) = self.stack.last_mut() {
                        *has_yield = true;
                    }
                }
            }
            WalkEvent::Leave(node) => {
                // When the visitor exits a function, if it matches the node of the top-most
                // entry of the stack and the `has_yield` flag is `false`, emit a query match
                if let Some(node) = AnyFunctionLike::cast_ref(node) {
                    if let Some((function_start_range, has_yield)) = self.stack.pop() {
                        if function_start_range == node.range().start()
                            && !has_yield
                            && node.is_generator()
                        {
                            ctx.match_query(MissingYield(node));
                        }
                    }
                }
            }
        }
    }
}

pub struct MissingYield(AnyFunctionLike);

impl QueryMatch for MissingYield {
    fn text_range(&self) -> TextRange {
        self.0.range()
    }
}

impl Queryable for MissingYield {
    type Input = Self;
    type Language = JsLanguage;
    type Output = AnyFunctionLike;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, MissingYieldVisitor::default);
    }

    fn unwrap_match(_: &ServiceBag, query: &Self::Input) -> Self::Output {
        query.0.clone()
    }
}

impl Rule for UseYield {
    type Query = MissingYield;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        // Don't emit diagnostic for generators with an empty body
        if query.statements()?.is_empty() {
            return None;
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {"This generator function doesn't contain "<Emphasis>"yield"</Emphasis>"."},
        ))
    }
}
