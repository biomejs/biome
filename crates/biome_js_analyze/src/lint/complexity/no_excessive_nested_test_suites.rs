use biome_analyze::{
    context::RuleContext, declare_lint_rule, AddVisitor, Phases, QueryMatch, Queryable, Rule,
    RuleDiagnostic, RuleDomain, RuleSource, RuleSourceKind, ServiceBag, Visitor, VisitorContext,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsCallExpression, JsLanguage, JsStaticMemberExpression};
use biome_rowan::{AstNode, Language, SyntaxNode, SyntaxNodeOptionExt, TextRange, WalkEvent};

declare_lint_rule! {
    /// This rule enforces a maximum depth to nested `describe()` in test files.
    ///
    /// To improve code clarity in your tests, the rule limits nested `describe` to 5.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// describe('foo', () => {
    ///   describe('bar', () => {
    ///     describe('baz', () => {
    ///       describe('qux', () => {
    ///         describe('quxx', () => {
    ///           describe('too many', () => {
    ///             it('should get something', () => {
    ///               expect(getSomething()).toBe('Something');
    ///             });
    ///           });
    ///         });
    ///       });
    ///     });
    ///   });
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// describe('foo', () => {
    ///   describe('bar', () => {
    ///     it('should get something', () => {
    ///       expect(getSomething()).toBe('Something');
    ///     });
    ///   });

    ///   describe('qux', () => {
    ///     it('should get something', () => {
    ///       expect(getSomething()).toBe('Something');
    ///     });
    ///   });
    /// });
    /// ```
    ///
    pub NoExcessiveNestedTestSuites {
        version: "1.6.0",
        name: "noExcessiveNestedTestSuites",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::EslintJest("max-nested-describe")],
        source_kind: RuleSourceKind::SameLogic,
        domains: &[RuleDomain::Test],
    }
}

impl Rule for NoExcessiveNestedTestSuites {
    type Query = NestedTest;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_: &RuleContext<Self>) -> Self::Signals {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Excessive `describe()` nesting detected."
                },
            )
                .note(markup! {
                "Excessive nesting of "<Emphasis>"describe()"</Emphasis>" calls can hinder test readability."
            })
                .note(markup! {
                "Consider refactoring and "<Emphasis>"reduce the level of nested describe"</Emphasis>" to improve code clarity."
            }),
        )
    }
}

struct NestedTestVisitor {
    max_count: u8,
    curr_count: u8,
}

impl Default for NestedTestVisitor {
    fn default() -> Self {
        Self {
            max_count: 5,
            curr_count: 0,
        }
    }
}

impl Visitor for NestedTestVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(node) => {
                if let Some(node) = JsCallExpression::cast_ref(node) {
                    if let Ok(callee) = node.callee() {
                        if callee.contains_describe_call() && !is_member(&node) {
                            self.curr_count += 1;
                            if self.curr_count == self.max_count + 1 {
                                ctx.match_query(NestedTest(node.clone()));
                            }
                        }
                    }
                }
            }
            WalkEvent::Leave(node) => {
                if let Some(node) = JsCallExpression::cast_ref(node) {
                    if let Ok(callee) = node.callee() {
                        if callee.contains_describe_call() && !is_member(&node) {
                            self.curr_count -= 1;
                        }
                    }
                }
            }
        }
    }
}

/// Determines whether or not the call expression is for a function that is a member of another object.
///
/// ```js
/// const foo = {
///   bar() {}
/// }
/// foo.bar(); // bar is a member of foo
/// ```
fn is_member(call: &JsCallExpression) -> bool {
    call.syntax()
        .parent()
        .kind()
        .is_some_and(JsStaticMemberExpression::can_cast)
        || call
            .callee()
            .map(|callee| callee.syntax().kind())
            .is_ok_and(JsStaticMemberExpression::can_cast)
}

// Declare a query match struct type containing a JavaScript function node
pub struct NestedTest(JsCallExpression);

impl QueryMatch for NestedTest {
    fn text_range(&self) -> TextRange {
        self.0.range()
    }
}

impl Queryable for NestedTest {
    // `Input` is the type that `ctx.match_query()` is called with in the visitor
    type Input = Self;
    type Language = JsLanguage;
    // `Output` if the type that `ctx.query()` will return in the rule
    type Output = JsCallExpression;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        // Register our custom visitor to run in the `Syntax` phase
        analyzer.add_visitor(Phases::Syntax, NestedTestVisitor::default);
    }

    // Extract the output object from the input type
    fn unwrap_match(_: &ServiceBag, query: &Self::Input) -> Self::Output {
        query.0.clone()
    }
}
