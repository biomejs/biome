use biome_analyze::{
    context::RuleContext, declare_rule, AddVisitor, Phases, QueryMatch, Queryable, Rule,
    RuleDiagnostic, ServiceBag, Visitor, VisitorContext,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsLanguage, JsSyntaxToken, TextRange};
use biome_rowan::{AstNode, Language, SyntaxNode, WalkEvent};

declare_rule! {
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
    pub(crate) NoDuplicateTestHooks {
        version: "next",
        name: "noDuplicateTestHooks",
        recommended: false,
    }
}

#[derive(Debug, Default)]
struct HooksContext {
    before_all: usize,
    before_each: usize,
    after_all: usize,
    after_each: usize,
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
                // When the visitor enters a function node, push a new entry on the stack
                if let Some(node) = JsCallExpression::cast_ref(node) {
                    if let Some(callee) = node.callee().ok() {
                        if let Some(_) = callee.contains_a_test_pattern().ok() {
                            if let Some(function_name) = get_function_name(&callee) {
                                if function_name.text_trimmed() == "describe" {
                                    self.stack.push(HooksContext::default());
                                }
                            }
                        }
                    }
                }

                if let Some(node) = JsCallExpression::cast_ref(node) {
                    if let Some(callee) = node.callee().ok() {
                        match callee {
                            AnyJsExpression::JsIdentifierExpression(identifier) => identifier
                                .name()
                                .and_then(|name| name.value_token())
                                .map_or((), |name| {
                                    if let Some(hooks_context) = self.stack.last_mut() {
                                        match name.text_trimmed() {
                                            "beforeEach" | "beforeAll" | "afterEach"
                                            | "afterAll" => {
                                                let counter = match name.text_trimmed() {
                                                    "beforeEach" => &mut hooks_context.before_each,
                                                    "beforeAll" => &mut hooks_context.before_all,
                                                    "afterEach" => &mut hooks_context.after_each,
                                                    "afterAll" => &mut hooks_context.after_all,
                                                    _ => {
                                                        unreachable!()
                                                    } // Should never happen
                                                };
                                                *counter += 1;
                                                if *counter > 1 {
                                                    ctx.match_query(DuplicateHooks(node.clone()));
                                                }
                                            }
                                            _ => {}
                                        };
                                    }
                                }),
                            _ => {}
                        }
                    }
                }
            }
            WalkEvent::Leave(node) => {
                // When the visitor exits a function, if it matches the node of the top-most
                // entry of the stack and the `has_yield` flag is `false`, emit a query match
                if let Some(node) = JsCallExpression::cast_ref(node) {
                    if let Some(callee) = node.callee().ok() {
                        if let Some(_) = callee.contains_a_test_pattern().ok() {
                            if let Some(function_name) = get_function_name(&callee) {
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
pub(crate) struct DuplicateHooks(JsCallExpression);

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
        let node_name = get_function_name(&callee)?;
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

fn get_function_name(callee: &AnyJsExpression) -> Option<JsSyntaxToken> {
    match callee {
        AnyJsExpression::JsStaticMemberExpression(node) => {
            let member = node.object().ok()?;
            let member = member.as_js_identifier_expression()?.name().ok()?;
            member.value_token().ok()
        }
        AnyJsExpression::JsIdentifierExpression(node) => node.name().ok()?.value_token().ok(),
        _ => None,
    }
}
