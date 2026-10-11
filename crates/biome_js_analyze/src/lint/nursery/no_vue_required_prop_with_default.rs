use crate::JsRuleAction;
use crate::frameworks::vue::vue_component::{
    AnyVueComponent, AnyVuePropDeclaration, VueComponent, VueComponentDeclarations,
    VueComponentQuery, VueDeclaration, VueDeclarationCollectionFilter, VueDefinePropsCall,
    resolve_props_type_members,
};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsObjectBindingPatternMember,
    AnyJsObjectMemberName, AnyTsTypeMember, JsBooleanLiteralExpression, JsPropertyObjectMember,
    T, TsMethodSignatureTypeMember, TsPropertySignatureTypeMember,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TextRange, TokenText, declare_node_union};
use biome_rule_options::no_vue_required_prop_with_default::NoVueRequiredPropWithDefaultOptions;
use enumflags2::make_bitflags;

declare_lint_rule! {
    /// Disallow props that are both required and have a default value.
    ///
    /// A prop with a default value can be omitted, because Vue falls back to the default value.
    /// Marking such a prop as required is contradictory: parent components are expected to always
    /// pass a required prop, so its default value should never be needed.
    ///
    /// This rule checks props declared with the `props` option, with `defineProps()`, and with
    /// type-based `defineProps<T>()` whose defaults are provided through `withDefaults()` or
    /// props destructuring.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// const props = withDefaults(
    ///   defineProps<{
    ///     name: string;
    ///   }>(),
    ///   {
    ///     name: "Foo",
    ///   },
    /// );
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// const { name = "Foo" } = defineProps<{
    ///   name: string;
    /// }>();
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   props: {
    ///     name: {
    ///       required: true,
    ///       default: "Foo",
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup lang="ts">
    /// const props = withDefaults(
    ///   defineProps<{
    ///     name?: string;
    ///     age: number;
    ///   }>(),
    ///   {
    ///     name: "Foo",
    ///   },
    /// );
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script setup>
    /// const props = defineProps({
    ///   name: {
    ///     required: false,
    ///     default: "Foo",
    ///   },
    ///   age: {
    ///     required: true,
    ///   },
    /// });
    /// </script>
    /// ```
    ///
    pub NoVueRequiredPropWithDefault {
        version: "next",
        name: "noVueRequiredPropWithDefault",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-required-prop-with-default").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoVueRequiredPropWithDefault {
    type Query = VueComponentQuery;
    type State = RequiredPropWithDefault;
    type Signals = Box<[Self::State]>;
    type Options = NoVueRequiredPropWithDefaultOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let Some(component) = VueComponent::from_potential_component(
            ctx.query(),
            model,
            ctx.source_type::<JsFileSource>(),
            ctx.file_path(),
        ) else {
            return Box::default();
        };

        let mut signals = Vec::new();
        match component.kind() {
            AnyVueComponent::Setup(setup) => {
                for call in setup.define_props_calls() {
                    for declaration in call.declarations(model) {
                        if let VueDeclaration::Prop(AnyVuePropDeclaration::JsPropertyObjectMember(
                            prop,
                        )) = declaration
                        {
                            signals.extend(check_runtime_prop(&prop, Some(&call)));
                        }
                    }
                    check_type_props(&call, model, &mut signals);
                }
            }
            _ => {
                for declaration in
                    component.declarations(make_bitflags!(VueDeclarationCollectionFilter::Prop))
                {
                    if let VueDeclaration::Prop(AnyVuePropDeclaration::JsPropertyObjectMember(
                        prop,
                    )) = declaration
                    {
                        signals.extend(check_runtime_prop(&prop, None));
                    }
                }
            }
        }
        signals.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.prop.range(),
                markup! {
                    "The prop "<Emphasis>{state.name.text()}</Emphasis>" is required, but it also has a default value."
                },
            )
            .detail(
                state.default_range,
                markup! { "The default value is declared here." },
            )
            .note(markup! {
                "Parent components are expected to always pass a required prop, so its default value should never be needed."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        match &state.prop {
            AnyRequiredProp::JsPropertyObjectMember(required) => {
                let value_token = required_true_literal(required)?.value_token().ok()?;
                mutation.replace_token_transfer_trivia(value_token, make::token(T![false]));
            }
            AnyRequiredProp::TsPropertySignatureTypeMember(member) => {
                mutation.replace_node(
                    member.clone(),
                    member
                        .clone()
                        .with_optional_token(Some(make::token(T![?]))),
                );
            }
            AnyRequiredProp::TsMethodSignatureTypeMember(member) => {
                mutation.replace_node(
                    member.clone(),
                    member
                        .clone()
                        .with_optional_token(Some(make::token(T![?]))),
                );
            }
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Make the prop optional." }.to_owned(),
            mutation,
        ))
    }
}

pub struct RequiredPropWithDefault {
    /// The node that makes the prop required.
    prop: AnyRequiredProp,
    name: TokenText,
    default_range: TextRange,
}

declare_node_union! {
    /// Either the `required: true` member of a runtime prop declaration, or a non-optional member
    /// of a type-based prop declaration.
    pub AnyRequiredProp =
        JsPropertyObjectMember
        | TsPropertySignatureTypeMember
        | TsMethodSignatureTypeMember
}

/// Checks the props declared with a type argument, as in `defineProps<{ name: string }>()`,
/// whose defaults come from `withDefaults()` or props destructuring.
fn check_type_props(
    call: &VueDefinePropsCall,
    model: &SemanticModel,
    signals: &mut Vec<RequiredPropWithDefault>,
) {
    if call.with_defaults().is_none() && call.destructuring().is_none() {
        return;
    }
    let Some(type_arguments) = call.define_props_call().type_arguments() else {
        return;
    };
    let Some(Ok(props_type)) = type_arguments.ts_type_argument_list().first() else {
        return;
    };
    let Some(members) = resolve_props_type_members(&props_type, model) else {
        return;
    };
    for member in members {
        let (prop, name) = match member {
            AnyTsTypeMember::TsPropertySignatureTypeMember(member)
                if member.optional_token().is_none() =>
            {
                let name = member.name().ok();
                (AnyRequiredProp::from(member), name)
            }
            AnyTsTypeMember::TsMethodSignatureTypeMember(member)
                if member.optional_token().is_none() =>
            {
                let name = member.name().ok();
                (AnyRequiredProp::from(member), name)
            }
            _ => continue,
        };
        // Computed names are ignored, because they may not be resolvable.
        let Some(name @ AnyJsObjectMemberName::JsLiteralMemberName(_)) = name else {
            continue;
        };
        let Some(name) = name.name() else {
            continue;
        };
        if let Some(default_range) = external_default_range(call, name.text()) {
            signals.push(RequiredPropWithDefault {
                prop,
                name,
                default_range,
            });
        }
    }
}

/// Checks a runtime prop declaration such as `name: { required: true, default: "Foo" }`.
///
/// In `<script setup>`, the default value may also come from `withDefaults()` or props
/// destructuring of `call`.
fn check_runtime_prop(
    prop: &JsPropertyObjectMember,
    call: Option<&VueDefinePropsCall>,
) -> Option<RequiredPropWithDefault> {
    let name = prop.name().ok()?.name()?;
    let AnyJsExpression::JsObjectExpression(options) = prop.value().ok()?.inner_expression()?
    else {
        return None;
    };

    let required = options.find_member("required")?;
    let required = required.as_js_property_object_member()?;
    required_true_literal(required)?;
    let default_range = options
        .find_member("default")
        .map(|member| member.range())
        .or_else(|| external_default_range(call?, name.text()))?;
    Some(RequiredPropWithDefault {
        prop: AnyRequiredProp::from(required.clone()),
        name,
        default_range,
    })
}

/// Returns the range of the default value given to the prop outside its declaration, through
/// `withDefaults()` or props destructuring.
fn external_default_range(call: &VueDefinePropsCall, name: &str) -> Option<TextRange> {
    if let Some(member) = call
        .with_defaults()
        .and_then(|defaults| defaults.find_member(name))
    {
        return Some(member.range());
    }
    call.destructuring()?
        .properties()
        .iter()
        .flatten()
        .find(|property| destructured_default_name(property).is_some_and(|n| n.text() == name))
        .map(|property| property.range())
}

/// Returns the name of a destructured prop that has a default value, as in
/// `const { name = "Foo" } = defineProps()`.
fn destructured_default_name(property: &AnyJsObjectBindingPatternMember) -> Option<TokenText> {
    match property {
        AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property) => {
            property.init()?;
            property.member().ok()?.name()
        }
        AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(property) => {
            property.init()?;
            Some(
                property
                    .identifier()
                    .ok()?
                    .as_js_identifier_binding()?
                    .name_token()
                    .ok()?
                    .token_text_trimmed(),
            )
        }
        _ => None,
    }
}

/// Returns the `true` literal of a `required: true` member.
fn required_true_literal(
    required: &JsPropertyObjectMember,
) -> Option<JsBooleanLiteralExpression> {
    let AnyJsExpression::AnyJsLiteralExpression(
        AnyJsLiteralExpression::JsBooleanLiteralExpression(literal),
    ) = required.value().ok()?.omit_parentheses()
    else {
        return None;
    };
    (literal.value_token().ok()?.kind() == T![true]).then_some(literal)
}
