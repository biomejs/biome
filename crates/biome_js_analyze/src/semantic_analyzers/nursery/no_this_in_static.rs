use crate::{
    analyzers::suspicious::no_duplicate_class_members::AnyClassMemberDefinition,
    semantic_services::Semantic,
};
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_semantic::{Reference, ReferencesExtensions};
use biome_js_syntax::{
    AnyJsClass, AnyJsClassMember, JsClassMemberList, JsIdentifierBinding, JsMethodClassMember,
};
use biome_rowan::AstNodeList;

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

impl Rule for NoThisInStatic {
    type Query = Semantic<JsMethodClassMember>; // HELP: For some reason this does not work, I would love to be able to only trigger the visit in a class method definition
                                                // type Query = Semantic<AnyJsClass>;
    type State = AnyClassMemberDefinition;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let class_method = ctx.query();
        let is_static = class_method
            .modifiers()
            .iter()
            .any(|modifier| modifier.as_js_static_modifier().is_some());

        if !is_static {
            return None;
        }

        let body_statements = class_method.body().ok()?.statements().iter();
        if body_statements.count() != 0 {
            return None;
        }

        // TODO: Now I need to check whether any of the statements in the body contain a `this` or `super` reference
        // If so, I need to report a diagnostic

        let mut references = Vec::new();
    }

    fn diagnostic(_: &RuleContext<Self>, reference: &Self::State) -> Option<RuleDiagnostic> {
        // Some(
        //     RuleDiagnostic::new(
        //         rule_category!(),
        //         reference.range(),
        //         markup! {
        //             "Variable is read here."
        //         },
        //     )
        //     .note(markup! {
        //         "This note will give you more information."
        //     }),
        // )
    }
}
