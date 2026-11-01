use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, JsCallExpression};
use biome_rowan::{AstNode, WalkEvent};
use biome_rule_options::no_duplicate_test_hooks::NoDuplicateTestHooksOptions;

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

impl Rule for NoDuplicateTestHooks {
    type Query = Semantic<JsCallExpression>;
    type State = JsCallExpression;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateTestHooksOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let describe = ctx.query();
        if !is_test_describe_call(describe) {
            return Box::default();
        }
        let model = ctx.model();
        let mut hooks = HooksContext::default();
        let mut duplicates = Vec::new();
        let mut preorder = describe.syntax().preorder();
        while let Some(event) = preorder.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            let Some(call) = JsCallExpression::cast(node) else {
                continue;
            };
            if call == *describe {
                continue;
            }
            // Hooks of a nested `describe` are counted by its own run.
            if is_test_describe_call(&call) {
                preorder.skip_subtree();
                continue;
            }
            let Ok(AnyJsExpression::JsIdentifierExpression(identifier)) = call.callee() else {
                continue;
            };
            let Ok(reference) = identifier.name() else {
                continue;
            };
            let Ok(name) = reference.value_token() else {
                continue;
            };
            if !matches!(
                name.text_trimmed(),
                "beforeEach" | "beforeAll" | "afterEach" | "afterAll" | "after" | "before"
            ) {
                continue;
            }
            // A binding that is not an import is a local `before` etc., not a test hook.
            if model
                .binding(&reference)
                .is_some_and(|binding| !binding.is_imported())
            {
                continue;
            }
            if hooks.add(name.text_trimmed()) > 1 {
                duplicates.push(call);
            }
        }
        duplicates.into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let callee = node.callee().ok()?;
        let node_name = callee.get_callee_object_name()?;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
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
