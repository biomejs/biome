use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{JsMethodClassMember, JsSuperExpression, JsThisExpression, AnyJsClass};
use biome_rowan::{declare_node_union, AstNode, AstNodeList};

use crate::analyzers::suspicious::no_duplicate_class_members::AnyClassMemberDefinition;

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
    type State = AnyClassMemberDefinition;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let this_or_super_expression = ctx.query();

        let static_method = this_or_super_expression
            .syntax()
            .ancestors()
            .filter_map(|ancestor| {
                JsMethodClassMember::cast(ancestor).filter(|member| {
                    member
                        .modifiers()
                        .iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some())
                })
            })
            .map(|member| AnyClassMemberDefinition::from(member)) // Add this line
            .collect();

        static_method
    }

    fn diagnostic(_: &RuleContext<Self>, reference: &Self::State) -> Option<RuleDiagnostic> {
        let class_name = reference
            .syntax()
            .ancestors()
            .find_map(|ancestor| AnyJsClass::cast(ancestor))
            .and_then(|class| class.as_js_class_declaration().map(|declaration| declaration.text()))
            .unwrap_or_else(|| String::from("A"));

        let function_used = reference .syntax()
            .descendants()
            .find_map(|descendant| JsMethodClassMember::cast(descendant))
            .map(|method| {
                let whole_expression = method.syntax().to_string();
                whole_expression
            })
            .unwrap_or_else(|| String::from("unknown"));


        Some(
            RuleDiagnostic::new(
                rule_category!(),
                reference.range(),
                markup! {
                    "Instead of "<Emphasis>{function_used}"()"</Emphasis>" use "<Emphasis>{class_name}</Emphasis>".something()."
                },
            )
        )
    }
}
