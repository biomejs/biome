use biome_analyze::context::RuleContext;
use biome_analyze::RuleSource;
use biome_analyze::{declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsConstructorClassMember, JsReturnStatement};
use biome_rowan::AstNode;

use crate::services::control_flow::AnyJsControlFlowRoot;

declare_lint_rule! {
    /// Disallow returning a value from a `constructor`.
    ///
    /// Returning a value from a `constructor` of a class is a possible error.
    /// Forbidding this pattern prevents errors resulting from unfamiliarity with JavaScript or a copy-paste error.
    ///
    /// Only returning without a value is allowed, as it’s a control flow statement.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class A {
    ///     constructor() {
    ///         return 0;
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class A {
    ///     constructor() {}
    /// }
    /// ```
    ///
    /// ```js
    /// class B {
    ///     constructor(x) {
    ///         return;
    ///     }
    /// }
    /// ```
    ///
    /// ## Using this rule in combination with the singleton pattern
    ///
    /// Some people implement the singleton pattern in JavaScript by returning
    /// an existing instance from the constructor, which would conflict with
    /// this rule.
    ///
    /// Instead, we advise to follow one of the suggestions described in this
    /// blog post: https://arendjr.nl/blog/2024/11/singletons-in-javascript/
    ///
    pub NoConstructorReturn {
        version: "1.0.0",
        name: "noConstructorReturn",
        language: "js",
        sources: &[RuleSource::Eslint("no-constructor-return")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoConstructorReturn {
    type Query = Ast<JsReturnStatement>;
    type State = JsConstructorClassMember;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let ret = ctx.query();
        // Do not take arg-less returns into account
        let _arg = ret.argument()?;
        let constructor = ret
            .syntax()
            .ancestors()
            .find(|x| AnyJsControlFlowRoot::can_cast(x.kind()))
            .and_then(JsConstructorClassMember::cast);
        constructor
    }

    fn diagnostic(ctx: &RuleContext<Self>, constructor: &Self::State) -> Option<RuleDiagnostic> {
        let ret = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            ret.range(),
            markup! {
                "The "<Emphasis>"constructor"</Emphasis>" should not "<Emphasis>"return"</Emphasis>" a value."
            },
        ).detail(
            constructor.range(),
            "The constructor is here:"
        ).note("Returning a value from a constructor may confuse users of the class."))
    }
}
