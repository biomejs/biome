use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsReturnStatement, JsSetterClassMember, JsSetterObjectMember};
use biome_rowan::{declare_node_union, AstNode};

use crate::services::control_flow::AnyJsControlFlowRoot;

declare_lint_rule! {
    /// Disallow returning a value from a setter
    ///
    /// While returning a value from a setter does not produce an error, the returned value is being ignored. Therefore, returning a value from a setter is either unnecessary or a possible error.
    ///
    /// Only returning without a value is allowed, as it’s a control flow statement.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class A {
    ///     set foo(x) {
    ///         return x;
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const b = {
    ///     set foo(x) {
    ///         return x;
    ///     },
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const c = {
    ///     set foo(x) {
    ///         if (x) {
    ///             return x;
    ///         }
    ///     },
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // early-return
    /// class A {
    ///     set foo(x) {
    ///         if (x) {
    ///             return;
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ```js
    /// // not a setter
    /// class B {
    ///   set(x) {
    ///     return x;
    ///   }
    /// }
    /// ```
    pub NoSetterReturn {
        version: "1.0.0",
        name: "noSetterReturn",
        language: "js",
        sources: &[RuleSource::Eslint("no-setter-return")],
        recommended: true,
        severity: Severity::Error,
    }
}

declare_node_union! {
    pub JsSetterMember = JsSetterClassMember | JsSetterObjectMember
}

impl Rule for NoSetterReturn {
    type Query = Ast<JsReturnStatement>;
    type State = JsSetterMember;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let ret = ctx.query();
        // Do not take arg-less returns into account
        let _arg = ret.argument()?;
        let setter = ret
            .syntax()
            .ancestors()
            .find(|x| AnyJsControlFlowRoot::can_cast(x.kind()))
            .and_then(JsSetterMember::cast);
        setter
    }

    fn diagnostic(ctx: &RuleContext<Self>, setter: &Self::State) -> Option<RuleDiagnostic> {
        let ret = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ret.range(),
                markup! {
                    "The setter should not "<Emphasis>"return"</Emphasis>" a value."
                },
            )
            .detail(setter.range(), "The setter is here:")
            .note("Returning a value from a setter is ignored."),
        )
    }
}
