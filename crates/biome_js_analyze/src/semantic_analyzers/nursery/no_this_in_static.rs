use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, ActionCategory};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::{JsMethodClassMember, JsStaticMemberExpression, JsSuperExpression, JsThisExpression, JsClassDeclaration};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, AstNodeExt, BatchMutationExt};

use crate::JsRuleAction;

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
    type State = JsStaticMemberExpression;
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
            
            if  static_method.is_some() {
                    this_super_expression
                        .syntax()
                        .ancestors()
                        .find_map(JsStaticMemberExpression::cast)
            } else {
                None
            }
    }

    fn diagnostic(_: &RuleContext<Self>, reference: &Self::State) -> Option<RuleDiagnostic> {
        let class_name_str = reference
            .syntax()
            .ancestors()
            .find_map(JsClassDeclaration::cast)
            .and_then(|declaration| Some(declaration.id()))?
            .unwrap()
            .text();

        let called_method_str = reference.text();

        let recommendation_str = called_method_str
            .replace("this", &class_name_str)
            .replace("super", &class_name_str);

        Some(RuleDiagnostic::new(
            rule_category!(),
            reference.range(),
            markup! {
                "Instead of "<Emphasis>{called_method_str}"()"</Emphasis>" use "<Emphasis>{recommendation_str}"()"</Emphasis>"."
            },
        ))
    }

    // fn action(ctx: &RuleContext<Self>, reference: &Self::State) -> Option<JsRuleAction> {
    //     let class_name_str = reference
    //         .syntax()
    //         .ancestors()
    //         .find_map(JsClassDeclaration::cast)
    //         .and_then(|declaration| Some(declaration.id()))?
    //         .unwrap()
    //         .text();

    //     let called_method_str = reference.text();

    //     let recommendation_str = called_method_str
    //         .replace("this", &class_name_str)
    //         .replace("super", &class_name_str);

    //     let mutation = ctx.query().begin(); // TODO: How does this work :(

    //     Some(JsRuleAction {
    //         category: ActionCategory::QuickFix,
    //         applicability: Applicability::Always,
    //         message: markup! {
    //             "Replace "<Emphasis>{called_method_str}"()"</Emphasis>" with "<Emphasis>{recommendation_str}"()"</Emphasis>"."
    //         }.to_owned(),
    //         mutation,
    //     })
    // }
}
