use biome_analyze::{
    context::RuleContext, declare_rule, AddVisitor, Phases, QueryMatch, Queryable, Rule,
    RuleDiagnostic, ServiceBag, Visitor,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsRoot, JsCallExpression, JsExport, JsLanguage};
use biome_rowan::{AstNode, Language, TextRange, WalkEvent};

declare_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding ESLint rule (if any):
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var a = 1;
    /// ```
    ///
    pub(crate) NoExportsInTest {
        version: "next",
        name: "noExportsInTest",
        recommended: true,
    }
}

#[derive(Default)]

struct ExportClauseInTestVisitor {
    has_test: bool,
    exports: Vec<JsExport>,
}

impl Visitor for ExportClauseInTestVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &biome_rowan::WalkEvent<biome_rowan::SyntaxNode<Self::Language>>,
        mut ctx: biome_analyze::VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(node) => {
                // TODO(ah-yu): CommonJs export
                if let Some(export) = JsExport::cast_ref(node) {
                    self.exports.push(export)
                }

                if !self.has_test {
                    if let Some(call_expr) = JsCallExpression::cast_ref(node) {
                        self.has_test = call_expr.is_test_call_expression().ok().unwrap_or(false);
                    }
                }
            }
            WalkEvent::Leave(node) => {
                if let Some(_) = AnyJsRoot::cast_ref(node) {
                    if self.has_test {
                        for export in self.exports.iter() {
                            ctx.match_query(ExportClauseInTest(export.clone()));
                        }
                    }
                }
            }
        }
    }
}

pub(crate) struct ExportClauseInTest(JsExport);

impl QueryMatch for ExportClauseInTest {
    fn text_range(&self) -> TextRange {
        self.0.range()
    }
}

impl Queryable for ExportClauseInTest {
    type Input = Self;
    type Language = JsLanguage;
    type Output = JsExport;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, ExportClauseInTestVisitor::default);
    }

    fn unwrap_match(_: &ServiceBag, query: &Self::Input) -> Self::Output {
        query.0.clone()
    }
}

impl Rule for NoExportsInTest {
    type Query = ExportClauseInTest;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_: &RuleContext<Self>) -> Self::Signals {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Don't export in a test file."
            },
        ))
    }
}
