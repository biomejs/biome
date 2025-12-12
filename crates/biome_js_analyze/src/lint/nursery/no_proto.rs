use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsAssignment, AnyJsMemberExpression};
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
        sources: &[RuleSource::Eslint("no-proto").same()],
    }
}

declare_node_union! {
    pub NoProtoQuery = AnyJsAssignment | AnyJsMemberExpression
}

impl Rule for NoProto {
    type Query = Ast<NoProtoQuery>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoProtoOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            NoProtoQuery::AnyJsAssignment(assignment) => match assignment {
                AnyJsAssignment::JsComputedMemberAssignment(assignment) => {
                    if assignment
                        .member()
                        .ok()?
                        .to_trimmed_text()
                        .trim_matches(['\'', '"', '`'])
                        == "__proto__"
                    {
                        return Some(());
                    }
                }
                AnyJsAssignment::JsStaticMemberAssignment(assignment) => {
                    if assignment.member().ok()?.to_trimmed_text() == "__proto__" {
                        return Some(());
                    }
                }
                _ => {}
            },
            NoProtoQuery::AnyJsMemberExpression(expr) => match expr {
                AnyJsMemberExpression::JsComputedMemberExpression(expr) => {
                    if expr
                        .member()
                        .ok()?
                        .to_trimmed_text()
                        .trim_matches(['\'', '"', '`'])
                        == "__proto__"
                    {
                        return Some(());
                    }
                }
                AnyJsMemberExpression::JsStaticMemberExpression(expr) => {
                    if expr.member().ok()?.to_trimmed_text() == "__proto__" {
                        return Some(());
                    }
                }
            },
        }

        None
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
