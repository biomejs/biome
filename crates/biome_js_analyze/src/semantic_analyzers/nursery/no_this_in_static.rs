use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{
    JsCallExpression, JsClassDeclaration, JsMethodClassMember, JsSuperExpression, JsThisExpression,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList};

declare_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding ESLint rule (if any):
    ///
    /// Source: https://eslint.org/docs/latest/rules/rule-name
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// var a = 1;
    /// ```
    ///
    pub(crate) NoThisInStatic {
        version: "next",
        name: "noThisInStatic",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) JsThisSuperExpression = JsSuperExpression | JsThisExpression
}

impl Rule for NoThisInStatic {
    type Query = Ast<JsThisSuperExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let this_super_expression = ctx.query();

        let static_method = this_super_expression
            .syntax()
            .ancestors()
            .find_map(JsMethodClassMember::cast)
            .filter(|member| {
                member
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some())
            });

        if static_method.is_some() {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let this_super_expression = ctx.query();

        let call_expression = this_super_expression
            .syntax()
            .ancestors()
            .find_map(JsCallExpression::cast)?;

        let class_name_str = this_super_expression
            .syntax()
            .ancestors()
            .find_map(JsClassDeclaration::cast)
            .and_then(|declaration| Some(declaration.id()))?
            .ok()?
            .text();

        let called_method_str = call_expression.text();

        let recommendation_str = called_method_str
            .replace("this", &class_name_str)
            .replace("super", &class_name_str);

        Some(RuleDiagnostic::new(
            rule_category!(),
            call_expression.range(),
            markup! {
                "Instead of "<Emphasis>{called_method_str}</Emphasis>" use "<Emphasis>{recommendation_str}</Emphasis>"."
            },
        ))
    }
}
