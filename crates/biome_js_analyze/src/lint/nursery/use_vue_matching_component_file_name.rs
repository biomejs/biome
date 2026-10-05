use crate::JsRuleAction;
use crate::frameworks::vue::vue_call::{is_vue_api_reference, is_vue_compiler_macro_call};
use crate::frameworks::vue::vue_component::{
    AnyPotentialVueComponent, AnyVueComponent, VueComponentQuery, VueOptionsApiBasedComponent,
};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::static_value::StaticValue;
use biome_js_syntax::{
    AnyJsExpression, AnyJsObjectMember, JsCallExpression, JsSyntaxKind, JsSyntaxToken,
    inner_string_text, unescape_js_string,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TextRange};
use biome_rule_options::use_vue_matching_component_file_name::UseVueMatchingComponentFileNameOptions;

declare_lint_rule! {
    /// Enforce that the name of a Vue component matches its file name.
    ///
    /// When a component's name differs from the name of the file that defines it, finding the
    /// component's source from its name, or the other way around, becomes harder.
    ///
    /// The names must be identical, including uppercase and lowercase letters: a component named
    /// `my-component` doesn't match the file `MyComponent.vue`.
    ///
    /// The rule checks every component whose name is written as a string, including when a file
    /// defines several components. It reads the component's name from:
    ///
    /// - the `name` option of `export default { ... }` in a `.vue`, `.jsx`, or `.tsx` file;
    /// - the `name` option passed to `defineComponent()`, `createApp()`, or `defineOptions()`;
    /// - the name passed to `app.component()` together with the component's definition.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,file=MyComponent.vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   name: "MyButton"
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,file=MyComponent.vue,expect_diagnostic
    /// <script setup>
    /// defineOptions({
    ///   name: "my-component"
    /// });
    /// </script>
    /// ```
    ///
    /// ```js,file=MyComponent.js,expect_diagnostic
    /// app.component("MyButton", {
    ///   template: "<div />"
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue,file=MyComponent.vue
    /// <script>
    /// export default {
    ///   name: "MyComponent"
    /// };
    /// </script>
    /// ```
    ///
    pub UseVueMatchingComponentFileName {
        version: "next",
        name: "useVueMatchingComponentFileName",
        language: "js",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("match-component-file-name").inspired()],
        fix_kind: FixKind::Unsafe,
    }
}

/// A component name that differs from the file name.
pub struct MismatchedComponentName {
    /// The range of the expression that holds the name.
    range: TextRange,
    value: StaticValue,
}

impl Rule for UseVueMatchingComponentFileName {
    type Query = VueComponentQuery;
    type State = MismatchedComponentName;
    type Signals = Box<[Self::State]>;
    type Options = UseVueMatchingComponentFileNameOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(file_name) = ctx.file_path().file_stem() else {
            return Box::default();
        };

        let source_type = ctx.source_type::<JsFileSource>();
        let mut name_expressions = Vec::new();
        match ctx.query() {
            AnyPotentialVueComponent::JsModule(_) => {}
            AnyPotentialVueComponent::JsExportDefaultExpressionClause(clause) => {
                let definition = if source_type.as_embedding_kind().is_vue() {
                    let Some(AnyVueComponent::OptionsApi(component)) =
                        AnyVueComponent::from_potential_component(
                            ctx.query(),
                            ctx.model(),
                            source_type,
                        )
                    else {
                        return Box::default();
                    };
                    component.definition_expression()
                } else if source_type.is_jsx() {
                    clause.expression().ok()
                } else {
                    None
                };
                name_expressions.extend(definition.as_ref().and_then(name_option));
            }
            AnyPotentialVueComponent::JsCallExpression(call) => {
                if let Some(component) =
                    AnyVueComponent::from_potential_component(ctx.query(), ctx.model(), source_type)
                {
                    let definition = match component {
                        AnyVueComponent::DefineComponent(component) => {
                            component.definition_expression()
                        }
                        AnyVueComponent::CreateApp(component) => component.definition_expression(),
                        AnyVueComponent::OptionsApi(_) | AnyVueComponent::Setup(_) => None,
                    };
                    name_expressions.extend(definition.as_ref().and_then(name_option));
                } else if source_type.as_embedding_kind().is_vue_setup()
                    && is_vue_compiler_macro_call(call, ctx.model(), "defineOptions")
                {
                    name_expressions.extend(first_argument(call).as_ref().and_then(name_option));
                } else if let Some((name, definition)) = component_registration(call, ctx) {
                    name_expressions.push(name);
                    if definition.as_js_object_expression().is_some() {
                        name_expressions.extend(name_option(&definition));
                    }
                }
            }
        }

        name_expressions
            .into_iter()
            .filter_map(|expression| {
                let value = static_string(&expression)?;
                let is_matching = match &value {
                    StaticValue::String(token) => {
                        &*unescape_js_string(inner_string_text(token)) == file_name
                    }
                    _ => false,
                };
                (!is_matching).then(|| MismatchedComponentName {
                    range: expression.range(),
                    value,
                })
            })
            .collect()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let file_name = ctx.file_path().file_stem()?;
        let component_name = state.value.text();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "The component name "<Emphasis>"\""{component_name}"\""</Emphasis>" doesn't match the file name "<Emphasis>"\""{file_name}"\""</Emphasis>"."
                },
            )
            .note(markup! {
                "When a component and its file have different names, it's harder to find where the component is defined."
            })
            .note(markup! {
                "Rename the component or the file so that both names are the same."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let file_name = ctx.file_path().file_stem()?;
        let StaticValue::String(token) = &state.value else {
            return None;
        };
        let new_text = match token.kind() {
            JsSyntaxKind::JS_STRING_LITERAL => {
                let quote = token.text_trimmed().chars().next()?;
                if file_name.contains([quote, '\\']) {
                    return None;
                }
                format!("{quote}{file_name}{quote}")
            }
            JsSyntaxKind::TEMPLATE_CHUNK => {
                if file_name.contains(['`', '\\']) || file_name.contains("${") {
                    return None;
                }
                file_name.to_string()
            }
            _ => return None,
        };
        let new_token = JsSyntaxToken::new_detached(token.kind(), &new_text, [], []);
        let mut mutation = ctx.root().begin();
        mutation.replace_token_transfer_trivia(token.clone(), new_token);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Rename the component to "<Emphasis>"\""{file_name}"\""</Emphasis>"." }
                .to_owned(),
            mutation,
        ))
    }
}

/// Returns the value of the `name` option in a component definition such as `{ name: "Foo" }`.
fn name_option(definition: &AnyJsExpression) -> Option<AnyJsExpression> {
    let definition = definition.inner_expression()?;
    definition
        .as_js_object_expression()?
        .members()
        .iter()
        .flatten()
        .find_map(|member| {
            let AnyJsObjectMember::JsPropertyObjectMember(property) = member else {
                return None;
            };
            if property.name().ok()?.name()? != "name" {
                return None;
            }
            property.value().ok()
        })
}

fn first_argument(call: &JsCallExpression) -> Option<AnyJsExpression> {
    let arguments = call.arguments().ok()?;
    arguments
        .args()
        .first()?
        .ok()?
        .as_any_js_expression()
        .cloned()
}

/// Matches a call such as `app.component("Foo", { ... })` that registers a component defined in
/// place, and returns the registered name and the definition.
///
/// A call that registers a component defined elsewhere, such as `app.component("Foo", Foo)`, is
/// ignored because the component doesn't belong to this file.
fn component_registration(
    call: &JsCallExpression,
    ctx: &RuleContext<UseVueMatchingComponentFileName>,
) -> Option<(AnyJsExpression, AnyJsExpression)> {
    let callee = call.callee().ok()?.inner_expression()?;
    let member = callee.as_js_static_member_expression()?;
    if member
        .member()
        .ok()?
        .as_js_name()?
        .value_token()
        .ok()?
        .text_trimmed()
        != "component"
    {
        return None;
    }
    member
        .object()
        .ok()?
        .inner_expression()?
        .as_js_identifier_expression()?;

    let arguments = call.arguments().ok()?;
    if arguments.args().len() != 2 {
        return None;
    }
    let [Some(name), Some(definition)] = arguments.get_arguments_by_index([0, 1]) else {
        return None;
    };
    let name = name.as_any_js_expression()?.clone();
    let definition = definition.as_any_js_expression()?.inner_expression()?;
    let is_defined_in_place = match &definition {
        AnyJsExpression::JsObjectExpression(_) => true,
        AnyJsExpression::JsCallExpression(definition_call) => definition_call
            .callee()
            .ok()
            .and_then(|callee| callee.inner_expression())
            .is_some_and(|callee| is_vue_api_reference(&callee, ctx.model(), "defineComponent")),
        _ => false,
    };
    is_defined_in_place.then_some((name, definition))
}

/// Returns the value of a string literal or of a template literal without substitutions.
fn static_string(expression: &AnyJsExpression) -> Option<StaticValue> {
    let expression = expression.clone().omit_parentheses();
    match &expression {
        AnyJsExpression::AnyJsLiteralExpression(literal)
            if literal.as_js_string_literal_expression().is_some() =>
        {
            expression.as_static_value()
        }
        AnyJsExpression::JsTemplateExpression(template) if template.tag().is_none() => {
            expression.as_static_value()
        }
        _ => None,
    }
}
