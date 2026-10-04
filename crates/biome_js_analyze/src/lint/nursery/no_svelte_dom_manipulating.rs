use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::assign_ext::AnyJsMemberAssignment;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsExpression, AnyJsMemberExpression, JsLanguage,
    JsAssignmentExpression, JsCallExpression, JsIdentifierBinding, JsIdentifierExpression,
    JsParenthesizedAssignment, JsParenthesizedExpression, JsReferenceIdentifier,
    TsAsExpression, TsNonNullAssertionExpression, TsSatisfiesExpression,
    static_value::StaticValue,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, SyntaxKindSet, TextRange, declare_node_union};
use biome_rule_options::no_svelte_dom_manipulating::NoSvelteDomManipulatingOptions;

use crate::services::embedded::EmbeddedService;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow direct DOM manipulation of elements bound with `bind:this`.
    ///
    /// Svelte keeps track of the DOM it renders from the component's template. Changing that DOM
    /// directly, for example by removing an element or replacing its content, makes the actual
    /// DOM diverge from what the Svelte runtime expects. Later updates can then fail or produce
    /// the wrong output.
    ///
    /// This rule reports calls to DOM-manipulating methods, such as `remove()` or
    /// `appendChild()`, and assignments to content properties, such as `textContent` or
    /// `innerHTML`, on variables that a `bind:this` directive binds to an HTML element.
    /// Instead, update the state that the template renders from.
    ///
    /// Only variables bound with `bind:this` are checked. Elements received in other ways, for
    /// example as the `node` argument of a transition or action, aren't tracked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// let element;
    /// const remove = () => element.remove();
    /// </script>
    ///
    /// <div bind:this={element}>Foo</div>
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// let element;
    /// const update = () => (element.textContent = "Update!");
    /// </script>
    ///
    /// <div bind:this={element}>Foo</div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// <script>
    /// let element;
    /// let show = $state(true);
    /// const toggle = () => (show = !show);
    /// </script>
    ///
    /// {#if show}
    ///   <div bind:this={element}>Foo</div>
    /// {/if}
    /// ```
    ///
    pub NoSvelteDomManipulating {
        version: "next",
        name: "noSvelteDomManipulating",
        language: "js",
        domains: &[RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("no-dom-manipulating").same()],
        recommended: true,
    }
}

declare_node_union! {
    pub AnyElementVariable = JsIdentifierBinding | JsReferenceIdentifier
}

impl Rule for NoSvelteDomManipulating {
    type Query = Semantic<AnyElementVariable>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoSvelteDomManipulatingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source = ctx.source_type::<JsFileSource>();
        if !source.is_svelte_component() {
            return Box::default();
        }
        let Some(embedded) = ctx.get_service::<EmbeddedService>() else {
            return Box::default();
        };
        let model = ctx.model();

        match ctx.query() {
            // A variable declared in a `<script>` block: check all of its references.
            AnyElementVariable::JsIdentifierBinding(binding) => {
                if !source.is_embedded_source()
                    || !model.as_binding(binding).scope().is_global_scope()
                {
                    return Box::default();
                }
                let Ok(name) = binding.name_token() else {
                    return Box::default();
                };
                if !embedded.is_svelte_element_reference(name.token_text_trimmed()) {
                    return Box::default();
                }
                binding
                    .all_references(model)
                    .filter_map(|reference| JsReferenceIdentifier::cast(reference.syntax()))
                    .filter_map(|reference| dom_manipulation_range(&reference))
                    .collect()
            }
            // A template expression can't see the `<script>` bindings, so the variable is
            // unresolved there.
            AnyElementVariable::JsReferenceIdentifier(reference) => {
                if source.is_embedded_source() || !model.is_unresolved_reference(reference) {
                    return Box::default();
                }
                let Ok(name) = reference.value_token() else {
                    return Box::default();
                };
                if !embedded.is_svelte_element_reference(name.token_text_trimmed()) {
                    return Box::default();
                }
                dom_manipulation_range(reference)
                    .into_iter()
                    .collect()
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This code manipulates the DOM of an element that Svelte renders."
                },
            )
            .note(markup! {
                "Svelte expects the DOM to match the template. When the DOM is changed directly, the Svelte runtime can lose track of the elements it renders and update them incorrectly."
            })
            .note(markup! {
                "Update the state that the template renders from instead, and let Svelte update the DOM."
            }),
        )
    }
}

/// Methods that insert, move, replace, or remove DOM nodes.
///
/// Sorted so that it can be binary searched.
const DOM_MANIPULATING_METHODS: &[&str] = &[
    "after",
    "append",
    "appendChild",
    "before",
    "insertAdjacentElement",
    "insertAdjacentHTML",
    "insertAdjacentText",
    "insertBefore",
    "normalize",
    "prepend",
    "remove",
    "removeChild",
    "replaceChild",
    "replaceChildren",
    "replaceWith",
];

/// Properties that replace the content of an element when assigned.
const DOM_MANIPULATING_PROPERTIES: &[&str] = &[
    "innerHTML",
    "innerText",
    "outerHTML",
    "outerText",
    "textContent",
];

/// Returns the range of the member access when `reference` is the object of a
/// DOM-manipulating method call or content property assignment.
fn dom_manipulation_range(reference: &JsReferenceIdentifier) -> Option<TextRange> {
    let object = skip_wrapping_expressions(AnyJsExpression::JsIdentifierExpression(
        JsIdentifierExpression::cast(reference.syntax().parent()?)?,
    ));
    let member = object.syntax().parent()?;

    if let Some(member) = AnyJsMemberExpression::cast_ref(&member) {
        if member.object().ok()? != object {
            return None;
        }
        let name = member.member_name()?;
        let callee = skip_wrapping_expressions(AnyJsExpression::cast_ref(member.syntax())?);
        let call = JsCallExpression::cast(callee.syntax().parent()?)?;
        let is_callee = call.callee().is_ok_and(|expression| expression == callee);
        return (is_callee && is_dom_manipulating_method(name.text())).then(|| member.range());
    }

    let member = AnyJsMemberAssignment::cast(member)?;
    if member.object().ok()? != object {
        return None;
    }
    let name = member_assignment_name(&member)?;
    let target = skip_parenthesized_assignments(AnyJsAssignment::cast_ref(member.syntax())?);
    let assignment = JsAssignmentExpression::cast(target.syntax().parent()?)?;
    let is_target = assignment
        .left()
        .is_ok_and(|left| left.syntax() == target.syntax());
    (is_target && DOM_MANIPULATING_PROPERTIES.contains(&name.text())).then(|| member.range())
}

fn is_dom_manipulating_method(name: &str) -> bool {
    DOM_MANIPULATING_METHODS.binary_search(&name).is_ok()
}

fn member_assignment_name(member: &AnyJsMemberAssignment) -> Option<StaticValue> {
    match member {
        AnyJsMemberAssignment::JsStaticMemberAssignment(member) => Some(StaticValue::String(
            member.member().ok()?.as_js_name()?.value_token().ok()?,
        )),
        AnyJsMemberAssignment::JsComputedMemberAssignment(member) => {
            let value = member.member().ok()?.omit_parentheses().as_static_value()?;
            matches!(value, StaticValue::String(_)).then_some(value)
        }
    }
}

/// Expressions that evaluate to the value they wrap.
const WRAPPING_EXPRESSION_KINDS: SyntaxKindSet<JsLanguage> = JsParenthesizedExpression::KIND_SET
    .union(TsNonNullAssertionExpression::KIND_SET)
    .union(TsAsExpression::KIND_SET)
    .union(TsSatisfiesExpression::KIND_SET);

/// Returns the outermost expression made of `expression` wrapped in
/// parentheses and TypeScript assertions.
fn skip_wrapping_expressions(expression: AnyJsExpression) -> AnyJsExpression {
    expression
        .syntax()
        .ancestors()
        .skip(1)
        .take_while(|node| WRAPPING_EXPRESSION_KINDS.matches(node.kind()))
        .last()
        .and_then(AnyJsExpression::cast)
        .unwrap_or(expression)
}

/// Returns the outermost assignment made of `assignment` wrapped in parentheses.
fn skip_parenthesized_assignments(assignment: AnyJsAssignment) -> AnyJsAssignment {
    assignment
        .syntax()
        .ancestors()
        .skip(1)
        .take_while(|node| JsParenthesizedAssignment::can_cast(node.kind()))
        .last()
        .and_then(AnyJsAssignment::cast)
        .unwrap_or(assignment)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dom_manipulating_methods_are_sorted() {
        for items in DOM_MANIPULATING_METHODS.windows(2) {
            assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
        }
    }
}
