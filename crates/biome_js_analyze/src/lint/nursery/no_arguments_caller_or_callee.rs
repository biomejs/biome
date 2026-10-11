use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsStaticMemberAssignment, JsStaticMemberExpression, unescape_js_identifier};
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_arguments_caller_or_callee::NoArgumentsCallerOrCalleeOptions;

declare_lint_rule! {
    /// Disallow the use of `arguments.caller` and `arguments.callee`.
    ///
    /// These deprecated properties prevent JavaScript optimizations and are forbidden in strict mode.
    /// Refer to a function by name for recursion, and pass the caller explicitly if you need to access it.
    ///
    /// This rule checks dot access, including optional chaining.
    /// It also reports these properties on local variables named `arguments`.
    /// Computed access such as `arguments["callee"]` is not checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function factorial(n) {
    ///     return n <= 1 ? 1 : n * arguments.callee(n - 1);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function getCaller() {
    ///     return arguments.caller;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function factorial(n) {
    ///     return n <= 1 ? 1 : n * factorial(n - 1);
    /// }
    /// ```
    ///
    /// ## See Also
    ///
    /// - To disallow the use of `arguments` entirely, see [`noArguments`](https://biomejs.dev/linter/rules/no-arguments/).
    ///
    pub NoArgumentsCallerOrCallee {
        version: "next",
        name: "noArgumentsCallerOrCallee",
        language: "js",
        sources: &[RuleSource::Eslint("no-caller").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

declare_node_union! {
    pub AnyJsCallerMemberLike = JsStaticMemberExpression | JsStaticMemberAssignment
}

impl Rule for NoArgumentsCallerOrCallee {
    type Query = Ast<AnyJsCallerMemberLike>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoArgumentsCallerOrCalleeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let (object, member) = match ctx.query() {
            AnyJsCallerMemberLike::JsStaticMemberExpression(member) => {
                (member.object().ok()?, member.member().ok()?)
            }
            AnyJsCallerMemberLike::JsStaticMemberAssignment(member) => {
                (member.object().ok()?, member.member().ok()?)
            }
        };
        let member = member.as_js_name()?.value_token().ok()?;
        match unescape_js_identifier(member.text_trimmed()).as_ref() {
            "caller" | "callee" => (),
            _ => return None,
        }

        let object = object.omit_parentheses();
        let reference = object.as_js_identifier_expression()?.name().ok()?;
        let identifier = reference.value_token().ok()?;
        if unescape_js_identifier(identifier.text_trimmed()) != "arguments" {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let member = match ctx.query() {
            AnyJsCallerMemberLike::JsStaticMemberExpression(member) => member.member().ok()?,
            AnyJsCallerMemberLike::JsStaticMemberAssignment(member) => member.member().ok()?,
        };
        let member = member.as_js_name()?.value_token().ok()?;
        let property = unescape_js_identifier(member.text_trimmed());
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Do not use "<Emphasis>"arguments."{property.as_ref()}</Emphasis>"."
                },
            )
            .note(markup! {
                "These deprecated properties are forbidden in strict mode and prevent JavaScript optimizations."
            })
            .note(markup! {
                "Refer to the function by name for recursion, and pass the caller explicitly if you need to access it."
            }),
        )
    }
}
