use rome_analyze::context::RuleContext;
use rome_analyze::{
    declare_rule, AddVisitor, Phases, QueryMatch, Queryable, Rule, RuleDiagnostic, ServiceBag,
    Visitor, VisitorContext,
};
use rome_console::markup;
use rome_js_syntax::{AnyFunctionLike, JsLanguage, JsYieldExpression, TextRange, WalkEvent};
use rome_rowan::{AstNode, AstNodeList, Language, SyntaxNode};

declare_rule! {
    /// Require generator functions to contain `yield`.
    ///
    /// This rule generates warnings for generator functions that do not have the `yield` keyword.
    ///
    /// Source: [require-await](https://eslint.org/docs/latest/rules/require-yield).
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
    pub(crate) UseYield {
        version: "1.0.0",
        name: "useYield",
        recommended: true,
    }
}

#[derive(Default)]
struct MissingYieldVisitor {
    /// Vector to hold a function node and a boolean indicating whether the function
    /// contains an `yield` expression or not.
    stack: Vec<(AnyFunctionLike, bool)>,
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
                    self.stack.push((node, false));
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
                if let Some(exit_node) = AnyFunctionLike::cast_ref(node) {
                    if let Some((enter_node, has_yield)) = self.stack.pop() {
                        if enter_node == exit_node && !has_yield {
                            ctx.match_query(MissingYield(enter_node));
                        }
                    }
                }
            }
        }
    }
}

pub(crate) struct MissingYield(AnyFunctionLike);

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

        // Don't emit diagnostic for non-generators or generators with an empty body
        if !query.is_generator() || query.statements()?.is_empty() {
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
