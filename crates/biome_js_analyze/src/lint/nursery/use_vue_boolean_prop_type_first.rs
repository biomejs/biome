use crate::JsRuleAction;
use crate::services::vue::{VueProp, node_at_range};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, JsArrayElementList, JsPropertyObjectMember,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::use_vue_boolean_prop_type_first::UseVueBooleanPropTypeFirstOptions;
use biome_vue_semantic::{PropForm, PropType};

declare_lint_rule! {
    /// Require `Boolean` to be the first type of Vue props that accept several types.
    ///
    /// A prop can accept several types by listing them in an array, such as
    /// `[Boolean, String]`. When a Boolean prop is written in a template without a value, as in
    /// `<MyComponent disabled />`, Vue sets it to `true`. However, if `String` comes before
    /// `Boolean` in the list, Vue sets the prop to an empty string instead. Writing `Boolean`
    /// first makes such a prop behave the same as a prop that only accepts `Boolean`.
    ///
    /// This rule checks props declared through the `props` option and `defineProps()`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   props: {
    ///     disabled: [String, Boolean],
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// defineProps({
    ///   disabled: { type: [String, Number, Boolean] },
    /// });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup>
    /// defineProps({
    ///   disabled: Boolean,
    ///   label: String,
    ///   loading: [Boolean, String],
    ///   size: { type: [Boolean, String, Number] },
    /// });
    /// </script>
    /// ```
    ///
    pub UseVueBooleanPropTypeFirst {
        version: "next",
        name: "useVueBooleanPropTypeFirst",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("prefer-prop-type-boolean-first").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseVueBooleanPropTypeFirst {
    type Query = VueProp;
    /// The `Boolean` element of the prop's type array.
    type State = AnyJsExpression;
    type Signals = Option<Self::State>;
    type Options = UseVueBooleanPropTypeFirstOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let prop = ctx.query();
        let types = prop.types();
        // A prop that doesn't accept `Boolean` together with another type has nothing to reorder.
        if !types.contains(PropType::Boolean) || types.exactly_one().is_some() {
            return None;
        }
        let member =
            node_at_range::<JsPropertyObjectMember>(&ctx.root(), prop.declaration_range())?;
        let value = member.value().ok()?.inner_expression()?;
        let types = match prop.form()? {
            PropForm::Constructor => value,
            PropForm::Options => {
                let AnyJsExpression::JsObjectExpression(options) = value else {
                    return None;
                };
                options
                    .find_member("type")?
                    .as_js_property_object_member()?
                    .value()
                    .ok()?
                    .inner_expression()?
            }
            PropForm::ArrayString | PropForm::TypeMember => return None,
        };
        let AnyJsExpression::JsArrayExpression(array) = types else {
            return None;
        };
        let (index, element) = array
            .elements()
            .iter()
            .enumerate()
            .find_map(|(index, element)| {
                let element = element.ok()?.as_any_js_expression()?.clone();
                is_global_boolean(&element, ctx.model()).then_some((index, element))
            })?;
        (index > 0).then_some(element)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, element: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                element.range(),
                markup! {
                    <Emphasis>"Boolean"</Emphasis>" isn't the first type of this prop."
                },
            )
            .note(markup! {
                "When "<Emphasis>"String"</Emphasis>" comes before "<Emphasis>"Boolean"</Emphasis>", a prop written in a template without a value, such as "<Emphasis>"<MyComponent disabled />"</Emphasis>", is set to an empty string instead of "<Emphasis>"true"</Emphasis>"."
            })
            .note(markup! {
                "Move "<Emphasis>"Boolean"</Emphasis>" to the start of the list of types."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, element: &Self::State) -> Option<JsRuleAction> {
        let list = JsArrayElementList::cast(element.syntax().parent()?)?;
        // A comment usually describes the type next to it. The elements move but the trivia of
        // each position stays, so a comment would end up next to another type.
        if list.syntax().has_comments_descendants() {
            return None;
        }
        let element = AnyJsArrayElement::AnyJsExpression(element.clone());
        let elements: Vec<_> = list.iter().collect::<Result<_, _>>().ok()?;
        let separators: Vec<_> = list.separators().collect::<Result<_, _>>().ok()?;
        // The elements move, but the whitespace around each position stays, so the layout of the
        // array is preserved.
        let reordered = std::iter::once(&element)
            .chain(elements.iter().filter(|other| **other != element))
            .zip(&elements)
            .map(|(moved, slot)| with_trivia_of(moved, slot))
            .collect::<Vec<_>>();
        let mut mutation = ctx.root().begin();
        mutation.replace_node(list, make::js_array_element_list(reordered, separators));
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Move "<Emphasis>"Boolean"</Emphasis>" to the start of the list." }
                .to_owned(),
            mutation,
        ))
    }
}

/// Returns `true` when `expression` is the global `Boolean`, not a variable of the same name.
fn is_global_boolean(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    let Some(AnyJsExpression::JsIdentifierExpression(identifier)) = expression.inner_expression()
    else {
        return false;
    };
    identifier.name().is_ok_and(|reference| {
        reference
            .value_token()
            .is_ok_and(|token| token.text_trimmed() == "Boolean")
            && model.binding(&reference).is_none()
    })
}

/// Returns `element` with the leading and trailing trivia of `slot`.
///
/// Returns `element` unchanged when it has no tokens, as an array hole.
fn with_trivia_of(element: &AnyJsArrayElement, slot: &AnyJsArrayElement) -> AnyJsArrayElement {
    let leading: Vec<_> = slot
        .syntax()
        .first_leading_trivia()
        .map(|trivia| trivia.pieces().collect())
        .unwrap_or_default();
    let trailing: Vec<_> = slot
        .syntax()
        .last_trailing_trivia()
        .map(|trivia| trivia.pieces().collect())
        .unwrap_or_default();
    element
        .clone()
        .with_leading_trivia_pieces(leading)
        .and_then(|element| element.with_trailing_trivia_pieces(trailing))
        .unwrap_or_else(|| element.clone())
}
