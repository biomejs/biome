use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::Binding;
use biome_js_syntax::{
    AnyJsExpression, AnyJsImportLike, JsCallExpression, JsObjectBindingPattern,
    JsVariableDeclarator, binding_ext::AnyJsBindingDeclaration, inner_string_text,
};
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::no_test_hooks::NoTestHooksOptions;

use crate::{
    frameworks::{named_framework_lib_export, unit_tests::LifecycleHook},
    services::semantic::Semantic,
};

declare_lint_rule! {
    /// Disallow setup and teardown hooks in tests.
    ///
    /// Test frameworks provide hooks, which are functions that run code before or after tests:
    /// `beforeAll`, `beforeEach`, `afterEach` and `afterAll`, and the `before` and `after` hooks of `node:test` and Mocha.
    ///
    /// Hooks usually prepare variables that several tests share.
    /// To understand one test, the reader has to find every hook that runs around it.
    /// A test can also pass or fail depending on what an earlier test left in the shared variables.
    /// Calling the setup code inside each test keeps the test complete on its own.
    ///
    /// The rule reports a hook that is called as a function, such as `beforeEach()`, when the hook is
    /// a global function or when the file loads it from `@jest/globals`, `bun:test`, `mocha`, `node:test` or `vitest`.
    /// A hook that the file loads under another name is reported too.
    /// A function that has the name of a hook but comes from anywhere else, such as a function
    /// that the file declares, is not reported.
    ///
    /// The rule also reports `beforeAll`, `beforeEach`, `afterEach` and `afterAll` when they are called
    /// on `test`, `it`, `describe` or `suite`, such as `test.beforeEach()`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// describe("cart", () => {
    ///     let cart;
    ///
    ///     beforeEach(() => {
    ///         cart = createCart();
    ///     });
    ///
    ///     it("starts empty", () => {
    ///         expect(cart.items).toHaveLength(0);
    ///     });
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test.afterAll(async () => {
    ///     await server.close();
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// describe("cart", () => {
    ///     it("starts empty", () => {
    ///         const cart = createCart();
    ///         expect(cart.items).toHaveLength(0);
    ///     });
    /// });
    ///
    /// // This function is declared in the file, so it isn't a hook.
    /// function before(message) {
    ///     console.log(message);
    /// }
    /// before("starting");
    /// ```
    ///
    pub NoTestHooks {
        version: "next",
        name: "noTestHooks",
        language: "js",
        recommended: false,
        sources: &[
            RuleSource::EslintJest("no-hooks").same(),
            RuleSource::EslintVitest("no-hooks").same(),
            RuleSource::EslintPlaywright("no-hooks").same(),
        ],
        domains: &[RuleDomain::Test],
    }
}

impl Rule for NoTestHooks {
    type Query = Semantic<JsCallExpression>;
    type State = LifecycleHook;
    type Signals = Option<Self::State>;
    type Options = NoTestHooksOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let callee = call.callee().ok()?.omit_parentheses();
        let binding = callee
            .as_js_identifier_expression()
            .and_then(|identifier| ctx.model().binding(&identifier.name().ok()?));
        match binding {
            Some(binding) => loaded_hook(&binding),
            None => LifecycleHook::from_call_expression(call),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, hook: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().callee().ok()?.range(),
                markup! {
                    "The "<Emphasis>{hook}</Emphasis>" hook is not allowed."
                },
            )
            .note(markup! {
                "Hooks run code away from the tests that depend on it, and they encourage tests to share variables. This makes a test harder to understand on its own, and lets one test change the result of another."
            })
            .note(markup! {
                "Move this code into the tests that need it, for example by calling a helper function at the start of each test."
            }),
        )
    }
}

/// Returns the hook that `binding` refers to, if the file loads `binding` from
/// one of the [`HOOK_MODULES`] with a named `import` or by destructuring a
/// `require()` call.
///
/// The hook is identified by the name it has in the module, so a hook that is
/// loaded under another name is still recognized:
/// `import { beforeEach as setup } from "vitest"` makes `setup` refer to
/// `beforeEach`.
fn loaded_hook(binding: &Binding) -> Option<LifecycleHook> {
    if let Some(name) = named_framework_lib_export(binding, HOOK_MODULES) {
        return LifecycleHook::from_name(inner_string_text(&name).text());
    }

    let declaration = binding.tree().declaration()?;
    let name = match &declaration {
        AnyJsBindingDeclaration::JsObjectBindingPatternProperty(property) => {
            property.member().ok()?.name()?
        }
        AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(property) => property
            .identifier()
            .ok()?
            .as_js_identifier_binding()?
            .name_token()
            .ok()?
            .token_text_trimmed(),
        _ => return None,
    };
    let module = required_module(&declaration)?;

    if !HOOK_MODULES.contains(&module.text()) {
        return None;
    }
    LifecycleHook::from_name(name.text())
}

/// Returns the name of the module that `property` is taken from, if
/// `property` belongs to the object pattern that directly destructures a
/// `require()` call, as `before` does in
/// `const { before } = require("node:test")`.
fn required_module(property: &AnyJsBindingDeclaration) -> Option<TokenText> {
    let declarator = property
        .syntax()
        .grand_parent()
        .and_then(JsObjectBindingPattern::cast)?
        .parent::<JsVariableDeclarator>()?;
    let AnyJsExpression::JsCallExpression(call) =
        declarator.initializer()?.expression().ok()?.omit_parentheses()
    else {
        return None;
    };

    AnyJsImportLike::JsCallExpression(call).inner_string_text()
}

/// The modules that export hooks as functions.
const HOOK_MODULES: &[&str] = &["@jest/globals", "bun:test", "mocha", "node:test", "vitest"];
