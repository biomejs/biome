use crate::ControlFlowGraph;
use crate::utils::function_return::{InvalidReturn, invalid_returns};
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsGetterClassMember, JsGetterObjectMember};
use biome_rowan::AstNode;
use biome_rule_options::use_getter_return::UseGetterReturnOptions;

declare_lint_rule! {
    /// Enforce `get` methods to always return a value.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class Person {
    ///     get firstName() {}
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const obj = {
    ///     get firstName() {
    ///         return;
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Option {
    ///     get value() {
    ///         if (this.hasValue) {
    ///             log();
    ///         } else {
    ///             return null;
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class Person {
    ///     get firstName() {
    ///         return this.fullname.split(" ")[0];
    ///     }
    /// }
    /// ```
    ///
    /// ```js
    /// const obj = {
    ///     get firstName() {
    ///         return this.fullname.split(" ")[0];
    ///     }
    /// }
    /// ```
    ///
    pub UseGetterReturn {
        version: "1.0.0",
        name: "useGetterReturn",
        language: "js",
        sources: &[RuleSource::Eslint("getter-return").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseGetterReturn {
    type Query = ControlFlowGraph;
    type State = InvalidReturn;
    type Signals = Box<[Self::State]>;
    type Options = UseGetterReturnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let cfg = ctx.query();
        let node_kind = cfg.node.kind();
        if !JsGetterClassMember::can_cast(node_kind) && !JsGetterObjectMember::can_cast(node_kind) {
            // The node is not a getter.
            return Box::default();
        }
        invalid_returns(cfg)
    }

    fn diagnostic(ctx: &RuleContext<Self>, invalid_return: &Self::State) -> Option<RuleDiagnostic> {
        let cfg = ctx.query();
        let diagnostic = match invalid_return {
            InvalidReturn::MissingReturn => {
                let getter_range = cfg.node.text_trimmed_range();
                RuleDiagnostic::new(
                    rule_category!(),
                    getter_range,
                    markup! {
                        "This "<Emphasis>"getter"</Emphasis>" should "<Emphasis>"return"</Emphasis>" a value."
                    },
                )
                .note(markup! {
                    "A getter without a "<Emphasis>"return"</Emphasis>" statement always evaluates to "<Emphasis>"undefined"</Emphasis>", so reading the property does not give the value the code intends."
                })
                .note(markup! {
                    "Add a "<Emphasis>"return"</Emphasis>" statement that returns the value of the property."
                })
            }
            InvalidReturn::EmptyReturn(return_stmt_range) => RuleDiagnostic::new(
                rule_category!(),
                return_stmt_range,
                markup! {
                    "This "<Emphasis>"return"</Emphasis>" should return a value because it is located in a "<Emphasis>"getter"</Emphasis>"."
                },
            )
            .note(markup! {
                "A "<Emphasis>"return"</Emphasis>" without a value makes the getter evaluate to "<Emphasis>"undefined"</Emphasis>", so reading the property does not give the value the code intends."
            })
            .note(markup! {
                "Return the value of the property from this statement."
            }),
        };
        Some(diagnostic)
    }
}
