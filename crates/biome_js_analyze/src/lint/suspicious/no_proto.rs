use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsAssignment, AnyJsExpression, AnyJsMemberExpression};
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_proto::NoProtoOptions;

declare_lint_rule! {
    /// Disallow the use of the deprecated `__proto__` object property.
    ///
    /// [`Object.prototype.__proto__`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/proto)
    /// is a special accessor used to get or set the prototype of an object. \
    ///
    /// However, it has been **deprecated** since _ECMAScript 2009_, being much slower and much less reliable than its
    /// modern counterparts [`Object.getPrototypeOf()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/getPrototypeOf)
    /// and [`Object.setPrototypeOf()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/setPrototypeOf).
    ///
    /// Since it is a regular property on `Object.prototype`,
    /// `__proto__` **will not work** on `null`-prototype objects that do not extend from `Object.prototype`
    /// nor ones having created their own `__proto__` properties via `Object.defineProperty`.
    ///
    /// As such, this rule encourages the use of `Object.getPrototypeOf()` and `Object.setPrototypeOf()`
    /// in lieu of directly accessing `__proto__`.
    ///
    /// :::info
    /// Note that this does **not** check for the use of `__proto__` inside object literal definitions
    /// to set a newly created object's prototype, \
    /// which is standard practice and well-optimized in modern browsers.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// obj.__proto__ = a;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const b = obj.__proto__;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const a = Object.getPrototypeOf(obj);
    /// ```
    ///
    /// ```js
    /// Object.setPrototypeOf(obj, b);
    /// ```
    ///
    /// ```js
    /// // This sets `foo`'s prototype to `null` (similar to `Object.create`), and is
    /// // well-defined across browsers.
    /// const foo = {
    ///   __proto__: null,
    ///   a: 1,
    /// }
    /// ```
    pub NoProto {
        version: "2.3.8",
        name: "noProto",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
        sources: &[RuleSource::Eslint("no-proto").same()],
    }
}

declare_node_union! {
    pub AnyNoProtoQuery = AnyJsAssignment | AnyJsMemberExpression
}

impl Rule for NoProto {
    type Query = Ast<AnyNoProtoQuery>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoProtoOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let is_proto = match node {
            AnyNoProtoQuery::AnyJsAssignment(assignment) => match assignment {
                AnyJsAssignment::JsComputedMemberAssignment(assignment) => {
                    is_proto_member(&assignment.member().ok()?)
                }
                AnyJsAssignment::JsStaticMemberAssignment(assignment) => {
                    assignment
                        .member()
                        .ok()?
                        .as_js_name()?
                        .value_token()
                        .ok()?
                        .text_trimmed()
                        == "__proto__"
                }
                _ => false,
            },
            AnyNoProtoQuery::AnyJsMemberExpression(expr) => match expr {
                AnyJsMemberExpression::JsComputedMemberExpression(expr) => {
                    is_proto_member(&expr.member().ok()?)
                }
                AnyJsMemberExpression::JsStaticMemberExpression(expr) => {
                    expr.member()
                        .ok()?
                        .as_js_name()?
                        .value_token()
                        .ok()?
                        .text_trimmed()
                        == "__proto__"
                }
            },
        };

        is_proto.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Avoid use of the deprecated "<Emphasis>"__proto__"</Emphasis>" accessor."
            },
        )
            .note(markup! {
                <Emphasis>"Object.prototype.__proto__"</Emphasis>" is an outdated way to get or set an object's prototype,"
                "\nhaving been "<Emphasis>"deprecated in 2009"</Emphasis>" for being inefficient and unreliable."
            })
            .note(markup! {
                <Emphasis>"Object.getPrototypeOf()"</Emphasis>" and "<Emphasis>"Object.setPrototypeOf()"</Emphasis>" "
                "are modern alternatives that work on all objects and are more performant."
            })
        )
    }
}

fn is_proto_member(member: &AnyJsExpression) -> bool {
    let Some(member) = member.inner_expression() else {
        return false;
    };
    if let AnyJsExpression::JsTemplateExpression(template) = &member
        && template.tag().is_some()
    {
        return false;
    }

    member
        .as_static_value()
        .is_some_and(|value| value.as_string_constant() == Some("__proto__"))
}
