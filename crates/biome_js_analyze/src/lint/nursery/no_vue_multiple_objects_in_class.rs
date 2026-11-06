use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsExpressionTemplateRoot};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::no_vue_multiple_objects_in_class::NoVueMultipleObjectsInClassOptions;

declare_lint_rule! {
    /// Disallow more than one object in an array passed to `:class`.
    ///
    /// In Vue, an object in a `:class` array adds each of its keys as a class when the
    /// matching value is truthy. Splitting these conditions over several objects in the same
    /// array makes it harder to see which classes depend on which conditions. Put them in a
    /// single object instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div :class="[{ active: isActive }, { disabled: isDisabled }]" />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-bind:class="['button', { active: isActive }, { disabled: isDisabled }]" />
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <div :class="[{ active: isActive, disabled: isDisabled }]" />
    ///   <div :class="['button', { active: isActive }]" />
    ///   <div :class="{ active: isActive, disabled: isDisabled }" />
    /// </template>
    /// ```
    ///
    pub NoVueMultipleObjectsInClass {
        version: "2.6.5",
        name: "noVueMultipleObjectsInClass",
        language: "js",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-multiple-objects-in-class").same()],
    }
}

impl Rule for NoVueMultipleObjectsInClass {
    type Query = Ast<JsExpressionTemplateRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueMultipleObjectsInClassOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let embedding_kind = ctx.source_type::<JsFileSource>().as_embedding_kind();
        if !embedding_kind.is_vue() || !embedding_kind.is_class_attribute() {
            return None;
        }

        let AnyJsExpression::JsArrayExpression(array) =
            ctx.query().expression()?.omit_parentheses()
        else {
            return None;
        };
        let object_count = array
            .elements()
            .iter()
            .flatten()
            .filter(|element| {
                element.as_any_js_expression().is_some_and(|expression| {
                    matches!(
                        expression.clone().omit_parentheses(),
                        AnyJsExpression::JsObjectExpression(_)
                    )
                })
            })
            .take(2)
            .count();
        (object_count > 1).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().expression()?.omit_parentheses().range(),
                markup! {
                    "This class array contains more than one object."
                },
            )
            .note(markup! {
                "Spreading conditional classes over several objects makes it harder to see which classes are applied and when."
            })
            .note(markup! {
                "Merge the objects into a single object."
            }),
        )
    }
}
