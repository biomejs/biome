use crate::JsRuleAction;
use crate::services::vue::VueProp;
use biome_vue_semantic::AnyVuePropNode;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBooleanLiteralExpression, JsInitializerClause,
    JsPropertyObjectMember, JsSyntaxNode, T, TsMethodSignatureTypeMember,
    TsPropertySignatureTypeMember,
};
use biome_rowan::{AstNode, BatchMutationExt, TextRange, declare_node_union};
use biome_rule_options::no_vue_required_prop_with_default::NoVueRequiredPropWithDefaultOptions;
use biome_vue_semantic::{Prop, Tri};

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
        version: "2.6.0",
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
    type Query = VueProp;
    type State = RequiredPropWithDefault;
    type Signals = Option<Self::State>;
    type Options = NoVueRequiredPropWithDefaultOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let prop = ctx.query();
        if prop.required() != Tri::Yes {
            return None;
        }
        let vue_model = ctx.vue_model();
        let default = vue_model.node(&prop.defaults().next()?)?;
        let required = match vue_model.node(prop)? {
            AnyVuePropNode::JsPropertyObjectMember(member) => {
                let options = member.value().ok()?.inner_expression()?;
                let required = options.as_js_object_expression()?.find_member("required")?;
                let required = required.as_js_property_object_member()?;
                required_true_literal(required)?;
                AnyRequiredProp::from(required.clone())
            }
            // Computed names are ignored, because they may not be resolvable.
            AnyVuePropNode::TsPropertySignatureTypeMember(member)
                if member.name().ok()?.as_js_literal_member_name().is_some() =>
            {
                member.into()
            }
            AnyVuePropNode::TsMethodSignatureTypeMember(member)
                if member.name().ok()?.as_js_literal_member_name().is_some() =>
            {
                member.into()
            }
            _ => return None,
        };
        Some(RequiredPropWithDefault {
            prop: prop.clone(),
            required,
            default_range: default_declaration(default).text_trimmed_range(),
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.required.range(),
                markup! {
                    "The prop "<Emphasis>{state.prop.name()}</Emphasis>" is required, but it also has a default value."
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
        match &state.required {
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
    prop: Prop,
    /// The node that makes the prop required.
    required: AnyRequiredProp,
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

/// Returns the node that gives a prop its default value: the `default: value` or `name: value`
/// member that holds the value, or the `{ name = value }` destructuring property.
fn default_declaration(value: JsSyntaxNode) -> JsSyntaxNode {
    let Some(parent) = value.parent() else {
        return value;
    };
    if JsPropertyObjectMember::can_cast(parent.kind()) {
        parent
    } else if JsInitializerClause::can_cast(parent.kind()) {
        parent.parent().unwrap_or(parent)
    } else {
        value
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
