use biome_analyze::{
    context::RuleContext, declare_rule, AddVisitor, Phases, QueryMatch, Queryable, Rule,
    RuleDiagnostic, RuleSource, RuleSourceKind, ServiceBag, Visitor,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsRoot, JsCallExpression, JsExport, JsLanguage};
use biome_rowan::{AstNode, Language, TextRange, WalkEvent};

declare_rule! {
    /// Disallow using `exports` in files containing tests
    ///
    /// This rule aims to eliminate duplicate runs of tests by exporting things from test files.
    /// If you import from a test file, then all the tests in that file will be run in each imported instance,
    /// so bottom line, don't export from a test, but instead move helper functions into a separate file when they need to be shared across tests.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// export function myHelper() {}
    /// describe('a test', () => {
    ///     expect(1).toBe(1);
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function myHelper() {}
    /// describe('a test', () => {
    ///     expect(1).toBe(1);
    /// });
    /// ```
    ///
    pub(crate) NoExportsInTest {
        version: "next",
        name: "noExportsInTest",
        recommended: true,
        source: RuleSource::EslintJest("no-export"),
        source_kind: RuleSourceKind::Inspired,
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
                            ctx.match_query(ExportInTest(export.clone()));
                        }
                    }
                }
            }
        }
    }
}

pub(crate) struct ExportInTest(JsExport);

impl QueryMatch for ExportInTest {
    fn text_range(&self) -> TextRange {
        self.0.range()
    }
}

impl Queryable for ExportInTest {
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
    type Query = ExportInTest;
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
                "Do not export from a test file."
            },
        ))
    }
}
