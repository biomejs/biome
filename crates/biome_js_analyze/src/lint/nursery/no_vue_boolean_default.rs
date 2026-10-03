use crate::frameworks::vue::vue_component::{
    AnyVueComponent, AnyVuePropDeclaration, VueComponent, VueComponentDeclarations,
    VueComponentQuery, VueDeclaration, VueDeclarationCollectionFilter, VueDeclarationName,
    VueDefinePropsCall,
};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsObjectBindingPatternMember, AnyJsObjectMember, AnyTsType,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::no_vue_boolean_default::NoVueBooleanDefaultOptions;
use enumflags2::make_bitflags;

declare_lint_rule! {
    /// Disallow default values for Boolean props in Vue components.
    ///
    /// Vue treats a Boolean prop like an HTML boolean attribute: it is `true` when present and
    /// `false` when absent. A default value breaks that convention. With `default: true`, the prop
    /// can only be turned off by explicitly binding `false`, and `default: false` restates what
    /// Vue already does.
    ///
    /// This rule checks props declared through the `props` option, `defineProps()`,
    /// `withDefaults()`, and destructured `defineProps()` with default values.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   props: {
    ///     disabled: {
    ///       type: Boolean,
    ///       default: true,
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// withDefaults(defineProps<{ disabled?: boolean }>(), {
    ///   disabled: false,
    /// });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// const { disabled = false } = defineProps<{ disabled?: boolean }>();
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   props: {
    ///     disabled: Boolean,
    ///     size: {
    ///       type: Number,
    ///       default: 1,
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script setup lang="ts">
    /// defineProps<{ disabled?: boolean }>();
    /// </script>
    /// ```
    ///
    pub NoVueBooleanDefault {
        version: "2.5.16",
        name: "noVueBooleanDefault",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-boolean-default").same()],
    }
}

impl Rule for NoVueBooleanDefault {
    type Query = VueComponentQuery;
    /// The range of a default value given to a Boolean prop.
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoVueBooleanDefaultOptions;

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

        let mut result = Vec::new();
        match component.kind() {
            AnyVueComponent::Setup(setup) => {
                for call in setup.define_props_calls() {
                    for declaration in call.declarations(model) {
                        let VueDeclaration::Prop(prop) = declaration else {
                            continue;
                        };
                        if !is_boolean_prop(&prop, model) {
                            continue;
                        }
                        result.extend(inline_default_range(&prop));
                        if let Some(name) = prop.declaration_name() {
                            result.extend(external_default_ranges(&call, name.text()));
                        }
                    }
                }
            }
            _ => {
                for declaration in
                    component.declarations(make_bitflags!(VueDeclarationCollectionFilter::Prop))
                {
                    if let VueDeclaration::Prop(prop) = declaration
                        && is_boolean_prop(&prop, model)
                    {
                        result.extend(inline_default_range(&prop));
                    }
                }
            }
        }
        result.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This Boolean prop has a default value."
                },
            )
            .note(markup! {
                "Vue already defaults an absent Boolean prop to "<Emphasis>"false"</Emphasis>". A default of "<Emphasis>"true"</Emphasis>" means the prop can only be turned off by explicitly passing "<Emphasis>"false"</Emphasis>"."
            })
            .note(markup! {
                "Remove the default value. If the prop should be enabled by default, invert its name and meaning instead, for example "<Emphasis>"disabled"</Emphasis>" instead of "<Emphasis>"enabled"</Emphasis>"."
            }),
        )
    }
}

/// Checks whether the prop is declared as `Boolean`, either at runtime (`foo: Boolean` or
/// `foo: { type: Boolean }`) or as a type (`foo: boolean`).
fn is_boolean_prop(prop: &AnyVuePropDeclaration, model: &SemanticModel) -> bool {
    match prop {
        AnyVuePropDeclaration::JsPropertyObjectMember(property) => {
            let Some(value) = property.value().ok().and_then(|value| value.inner_expression())
            else {
                return false;
            };
            if let AnyJsExpression::JsObjectExpression(object) = value {
                object.find_member("type")
                    .and_then(|member| member.as_js_property_object_member()?.value().ok())
                    .is_some_and(|value| is_boolean_constructor(&value, model))
            } else {
                is_boolean_constructor(&value, model)
            }
        }
        AnyVuePropDeclaration::TsPropertySignatureTypeMember(property) => property
            .type_annotation()
            .and_then(|annotation| annotation.ty().ok())
            .is_some_and(|ty| is_boolean_type(&ty)),
        // Props declared with the array syntax have no type.
        AnyVuePropDeclaration::JsStringLiteralExpression(_) => false,
    }
}

/// Checks whether the expression is a reference to the global `Boolean`.
fn is_boolean_constructor(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    expression
        .inner_expression()
        .as_ref()
        .and_then(|expression| expression.as_js_reference_identifier())
        .is_some_and(|reference| {
            reference.has_name("Boolean") && model.binding(&reference).is_none()
        })
}

/// Checks whether the type only allows `true` and `false`, so that Vue infers `Boolean` as its
/// runtime type.
fn is_boolean_type(ty: &AnyTsType) -> bool {
    match ty.clone().omit_parentheses() {
        AnyTsType::TsBooleanType(_) | AnyTsType::TsBooleanLiteralType(_) => true,
        AnyTsType::TsUnionType(union) => union
            .types()
            .iter()
            .all(|ty| ty.is_ok_and(|ty| is_boolean_type(&ty))),
        _ => false,
    }
}

/// Returns the range of the `default` given in the prop's options object, as in
/// `foo: { type: Boolean, default: true }`.
fn inline_default_range(prop: &AnyVuePropDeclaration) -> Option<TextRange> {
    let AnyVuePropDeclaration::JsPropertyObjectMember(property) = prop else {
        return None;
    };
    let value = property.value().ok()?.inner_expression()?;
    default_value_range(&value.as_js_object_expression()?.find_member("default")?)
}

/// Returns the ranges of the defaults given to the prop outside its declaration, via
/// `withDefaults()` or a destructuring default value.
fn external_default_ranges(call: &VueDefinePropsCall, name: &str) -> Vec<TextRange> {
    let mut result = Vec::new();
    if let Some(member) = call
        .with_defaults()
        .and_then(|defaults| defaults.find_member(name))
    {
        result.extend(default_value_range(&member));
    }
    if let Some(destructuring) = call.destructuring() {
        for property in destructuring.properties().iter().flatten() {
            let (property_name, init) = match &property {
                AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property) => (
                    property.member().ok().and_then(|member| member.name()),
                    property.init(),
                ),
                AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                    property,
                ) => (
                    property.identifier().ok().and_then(|identifier| {
                        Some(
                            identifier
                                .as_js_identifier_binding()?
                                .name_token()
                                .ok()?
                                .token_text_trimmed(),
                        )
                    }),
                    property.init(),
                ),
                _ => continue,
            };
            if property_name.is_some_and(|property_name| property_name.text() == name)
                && let Some(expression) = init.and_then(|init| init.expression().ok())
            {
                result.push(expression.range());
            }
        }
    }
    result
}

/// Returns the range of the value given by an object member, as in `default: true`,
/// `default() { return true; }`, or `{ default }`.
fn default_value_range(member: &AnyJsObjectMember) -> Option<TextRange> {
    match member {
        AnyJsObjectMember::JsPropertyObjectMember(property) => Some(property.value().ok()?.range()),
        AnyJsObjectMember::JsMethodObjectMember(_)
        | AnyJsObjectMember::JsShorthandPropertyObjectMember(_) => Some(member.range()),
        _ => None,
    }
}
