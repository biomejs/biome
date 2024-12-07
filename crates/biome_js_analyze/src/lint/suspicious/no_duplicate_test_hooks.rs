use biome_analyze::{
    context::RuleContext, declare_lint_rule, AddVisitor, Phases, QueryMatch, Queryable, Rule,
    RuleDiagnostic, RuleDomain, RuleSource, RuleSourceKind, ServiceBag, Visitor, VisitorContext,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsLanguage, TextRange};
use biome_rowan::{AstNode, Language, SyntaxNode, WalkEvent};

declare_lint_rule! {
    /// A `describe` block should not contain duplicate hooks.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// describe('foo', () => {
    ///   beforeEach(() => {
    ///     // some setup
    ///   });
    ///   beforeEach(() => {
    ///     // some setup
    ///   });
    ///   test('foo_test', () => {
    ///    // some test
    ///   });
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// describe('foo', () => {
    ///   beforeEach(() => {
    ///     // some setup
    ///   });
    ///   test('foo_test', () => {
    ///     afterAll(() => {
    ///       // some teardown
    ///     });
    ///    afterAll(() => {
    ///      // some teardown
    ///    });
    ///   });
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// describe('foo', () => {
    ///   beforeEach(() => {
    ///     // some setup
    ///   });
    ///   test('foo_test', () => {
    ///     // some test
    ///   });
    /// });
    /// ```
    ///
    pub NoDuplicateTestHooks {
        version: "1.6.0",
        name: "noDuplicateTestHooks",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::EslintJest("no-duplicate-hooks")],
        source_kind: RuleSourceKind::Inspired,
        domains: &[RuleDomain::Test],
    }
}

#[derive(Debug, Default)]
struct HooksContext {
    after: u8,
    after_all: u8,
    after_each: u8,
    before: u8,
    before_all: u8,
    before_each: u8,
}

impl HooksContext {
    fn add(&mut self, hook_name: &str) -> u8 {
        let counter = match hook_name {
            "after" => &mut self.after,
            "afterAll" => &mut self.after_all,
            "afterEach" => &mut self.after_each,
            "before" => &mut self.before,
            "beforeEach" => &mut self.before_each,
            "beforeAll" => &mut self.before_all,
            _ => return 0, // Should never happen
        };
        if *counter <= 1 {
            *counter += 1;
        }

        *counter
    }
}

#[derive(Default)]
struct DuplicateHooksVisitor {
    stack: Vec<HooksContext>,
}

impl Visitor for DuplicateHooksVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(node) => {
                let Some(node) = JsCallExpression::cast_ref(node) else {
                    return;
                };

                // When the visitor enters a function node, push a new entry on the stack
                if let Ok(callee) = node.callee() {
                    if callee.contains_a_test_pattern() == Ok(true) {
                        if let Some(function_name) = callee.get_callee_object_name() {
                            if function_name.text_trimmed() == "describe" {
                                self.stack.push(HooksContext::default());
                            }
                        }
                    }
                    // describe.each has a different syntax
                    else if let AnyJsExpression::JsCallExpression(call_expression) = callee {
                        if let Ok(callee) = call_expression.callee() {
                            if matches!(
                                callee.to_trimmed_string().as_str(),
                                "describe.each" | "describe.only.each" | "fdescribe.each"
                            ) {
                                self.stack.push(HooksContext::default());
                            }
                        }
                    }
                }

                if let Ok(AnyJsExpression::JsIdentifierExpression(identifier)) = node.callee() {
                    identifier
                        .name()
                        .and_then(|name| name.value_token())
                        .map_or((), |name| {
                            if let Some(hooks_context) = self.stack.last_mut() {
                                match name.text_trimmed() {
                                    "beforeEach" | "beforeAll" | "afterEach" | "afterAll"
                                    | "after" | "before" => {
                                        let counter =
                                            HooksContext::add(hooks_context, name.text_trimmed());
                                        if counter > 1 {
                                            ctx.match_query(DuplicateHooks(node.clone()));
                                        }
                                    }
                                    _ => {}
                                };
                            };
                        })
                }
            }
            WalkEvent::Leave(node) => {
                // When the visitor exits a function, if it matches the node of the top-most
                // entry of the stack and the `has_yield` flag is `false`, emit a query match
                if let Some(node) = JsCallExpression::cast_ref(node) {
                    if let Ok(callee) = node.callee() {
                        if callee.contains_a_test_pattern() == Ok(true) {
                            if let Some(function_name) = callee.get_callee_object_name() {
                                if function_name.text_trimmed() == "describe" {
                                    self.stack.pop();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// Declare a query match struct type containing a JavaScript function node
pub struct DuplicateHooks(JsCallExpression);

impl QueryMatch for DuplicateHooks {
    fn text_range(&self) -> TextRange {
        self.0.range()
    }
}

impl Queryable for DuplicateHooks {
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
        analyzer.add_visitor(Phases::Syntax, DuplicateHooksVisitor::default);
    }

    // Extract the output object from the input type
    fn unwrap_match(_: &ServiceBag, query: &Self::Input) -> Self::Output {
        query.0.clone()
    }
}

impl Rule for NoDuplicateTestHooks {
    type Query = DuplicateHooks;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_: &RuleContext<Self>) -> Self::Signals {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let callee = node.callee().ok()?;
        let node_name = callee.get_callee_object_name()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Disallow duplicate setup and teardown hooks."
                },
            )
            .note(markup! {
                "Disallow "<Emphasis>{node_name.text_trimmed()}</Emphasis>" duplicacy inside the describe function."
            }),
        )
    }
}
