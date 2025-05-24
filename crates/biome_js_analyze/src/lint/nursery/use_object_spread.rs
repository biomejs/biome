use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, global_identifier};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};

declare_lint_rule! {
    /// Prefer object spread over `Object.assign()` when constructing new objects.
    ///
    /// Object spread syntax is more concise, more readable, and performs better
    /// than `Object.assign()` when creating a new object from existing objects.
    /// It also has better TypeScript integration.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Object.assign({}, foo);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Object.assign({}, { foo: 'bar' });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Object.assign({ foo: 'bar' }, baz);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Object.assign({}, baz, { foo: 'bar' });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// ({ ...foo });
    /// ```
    ///
    /// ```js
    /// ({ ...baz, foo: 'bar' });
    /// ```
    ///
    /// Modifying an existing object is allowed:
    /// ```js
    /// Object.assign(foo, { bar: baz });
    /// ```
    ///
    pub UseObjectSpread {
        version: "2.0.0",
        name: "useObjectSpread",
        language: "js",
        sources: &[
            RuleSource::Eslint("prefer-object-spread"),
        ],
        recommended: false,
    }
}

impl Rule for UseObjectSpread {
    type Query = Semantic<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let callee = node.callee().ok()?;
        let member_expr = callee.as_js_static_member_expression()?;

        let obj = member_expr.object().ok()?;
        let (reference, obj_name) = global_identifier(&obj)?;
        if obj_name.text() != "Object" || ctx.model().binding(&reference).is_some() {
            return None;
        }

        let method = member_expr.member().ok()?;
        if method.value_token().ok()?.text() != "assign" {
            return None;
        }

        let args = node.arguments().ok()?;
        let first_arg = args.args().first()?.ok()?;
        let expression = first_arg.as_any_js_expression()?;

        expression
            .as_js_object_expression()
            .and(Some(member_expr.range()))
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Object spread should be used instead of "<Emphasis>"Object.assign"</Emphasis>
                    " when constructing new objects."
                },
            )
            .note(markup! {
                "Replace "<Emphasis>"Object.assign({...}, <object>)"</Emphasis>
                " with "<Emphasis>"{ ...<object> }"</Emphasis>"."
            }),
        )
    }
}
