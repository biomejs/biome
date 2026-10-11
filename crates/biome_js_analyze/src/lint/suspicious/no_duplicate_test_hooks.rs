use crate::services::semantic::{SemanticModelBuilderVisitor, SemanticServices};
use biome_analyze::{
    AddVisitor, Phases, QueryMatch, Queryable, Rule, RuleDiagnostic, RuleDomain, RuleSource,
    ServiceBag, Visitor, VisitorContext, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsExpression, JsCallExpression, JsLanguage, JsSyntaxKind, TextRange, binding_ext::AnyJsBindingDeclaration,
    unescape_js_identifier,
};
use biome_rowan::{AstNode, Language, SyntaxNode, WalkEvent};
use biome_rule_options::no_duplicate_test_hooks::NoDuplicateTestHooksOptions;
use rustc_hash::FxHashMap;

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
        sources: &[
            RuleSource::EslintJest("no-duplicate-hooks").inspired(),
            RuleSource::EslintVitest("no-duplicate-hooks").inspired(),
        ],
        domains: &[RuleDomain::Test],
    }
}

// Declare a query match struct type containing a JavaScript function node
pub struct DuplicateHooks(JsCallExpression);

impl Rule for NoDuplicateTestHooks {
    type Query = DuplicateHooks;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoDuplicateTestHooksOptions;

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
                    "Duplicate "<Emphasis>{node_name.text_trimmed()}</Emphasis>" hook found."
                },
            )
            .note(markup! {
                "Remove this duplicate hook or consolidate the logic into a single hook."
            }),
        )
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
    local_functions: FxHashMap<TextRange, bool>,
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
                if Self::is_test_describe_call(&node) {
                    self.stack.push(HooksContext::default());
                }

                if let Ok(AnyJsExpression::JsIdentifierExpression(identifier)) = node.callee() {
                    let Ok(reference) = identifier.name() else {
                        return;
                    };
                    let Ok(name) = reference.value_token() else {
                        return;
                    };
                    if !matches!(
                        name.text_trimmed(),
                        "beforeEach" | "beforeAll" | "afterEach" | "afterAll" | "after" | "before"
                    ) || self.stack.is_empty()
                    {
                        return;
                    }
                    let Some(model) = ctx.services.get_service::<SemanticModel>() else {
                        return;
                    };
                    if model.binding(&reference).is_some_and(|binding| {
                        *self
                            .local_functions
                            .entry(binding.range())
                            .or_insert_with(|| Self::is_local_function(&binding, model))
                    })
                    {
                        return;
                    }
                    if let Some(hooks_context) = self.stack.last_mut() {
                        let counter = HooksContext::add(hooks_context, name.text_trimmed());
                        if counter > 1 {
                            ctx.match_query(DuplicateHooks(node.clone()));
                        }
                    }
                }
            }
            WalkEvent::Leave(node) => {
                // When the visitor exits a function, if it matches the node of the top-most
                // entry of the stack and the `has_yield` flag is `false`, emit a query match
                if let Some(node) = JsCallExpression::cast_ref(node)
                    && Self::is_test_describe_call(&node)
                {
                    self.stack.pop();
                }
            }
        }
    }
}

impl DuplicateHooksVisitor {
    fn is_local_function(binding: &Binding, model: &SemanticModel) -> bool {
        if binding.all_writes().next().is_some() {
            return false;
        }
        let Ok(name) = binding.tree().name_token() else {
            return false;
        };
        let name = unescape_js_identifier(name.text_trimmed());
        let scope = model
            .scope_hoisted_to(&binding.syntax())
            .unwrap_or_else(|| binding.scope());
        if scope.syntax().kind() == JsSyntaxKind::JS_FUNCTION_BODY
            && scope
                .parent()
                .and_then(|parent| parent.get_binding(name.as_ref()))
                .is_some_and(|other| {
                    other.declaration_kind().declares_value()
                        && !matches!(
                            other.tree().declaration(),
                            Some(AnyJsBindingDeclaration::JsFunctionExpression(_))
                        )
                })
        {
            return false;
        }
        if scope.bindings().any(|other| {
            other.range() != binding.range()
                && other.declaration_kind().declares_value()
                && other
                    .tree()
                    .name_token()
                    .is_ok_and(|token| unescape_js_identifier(token.text_trimmed()) == name)
        }) {
            return false;
        }
        match binding.tree().declaration() {
            Some(
                AnyJsBindingDeclaration::JsFunctionDeclaration(_)
                | AnyJsBindingDeclaration::JsFunctionExpression(_)
                | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_),
            ) => true,
            Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) => declarator
                .initializer()
                .and_then(|initializer| initializer.expression().ok())
                .and_then(|expression| expression.inner_expression())
                .is_some_and(|expression| {
                    matches!(
                        expression,
                        AnyJsExpression::JsArrowFunctionExpression(_)
                            | AnyJsExpression::JsFunctionExpression(_)
                    )
                }),
            _ => false,
        }
    }

    /// Determines if a [call expression] is a `describe.` call by checking:
    /// 1. The node must be a test call expression
    ///    e.g. `it.only`, `describe.skip`, `test`
    /// 2. The test call must be a `describe.` call
    ///    first section = `describe` | `fdescribe` | `xdescribe` e.g. `describe.only`, `describe.skip`
    ///    or second section = `describe` e.g. `test.describe`
    ///
    /// [call expression]: crate::JsCallExpression
    fn is_test_describe_call(node: &JsCallExpression) -> bool {
        if let Ok(callee) = node.callee()
            && node.is_test_call_expression() == Ok(true)
        {
            let text = callee.to_trimmed_text();
            let mut split = text.split('.');

            let first = split.next();
            let second = split.next();

            if matches!(first, Some("describe" | "fdescribe" | "xdescribe"))
                || second == Some("describe")
            {
                return true;
            }
        }

        false
    }
}

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
    type Services = SemanticServices;

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor);
        analyzer.add_visitor(Phases::Semantic, DuplicateHooksVisitor::default);
    }

    // Extract the output object from the input type
    fn unwrap_match(_: &ServiceBag, query: &Self::Input) -> Self::Output {
        query.0.clone()
    }
}
