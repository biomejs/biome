use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_factory::make;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyHtmlAttributeInitializer, AnyVueDirective, AnyVueDirectiveArgument,
    HtmlAttribute, HtmlRoot, HtmlSyntaxKind, HtmlSyntaxToken, T, VueDirectiveArgument,
    element_ext::AnyHtmlTagElement,
};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TextRange};
use biome_rule_options::use_vue_consistent_boolean_prop_style::{
    BooleanPropStyle, UseVueConsistentBooleanPropStyleOptions,
};

use crate::HtmlRuleAction;
use crate::vue::is_vue_component;

declare_lint_rule! {
    /// Enforce a consistent style for component props set to `true`.
    ///
    /// In Vue, a `Boolean` prop written without a value receives `true`. This means
    /// `<MyComponent disabled />` passes the same value as `<MyComponent :disabled="true" />`, and
    /// the shorthand form is easier to read.
    ///
    /// This rule only checks components. On native HTML elements, an attribute without a value
    /// isn't always the same as `true`: for example, `<div data-active>` sets `data-active` to an
    /// empty string.
    ///
    /// :::caution
    /// In Vue, the shorthand form passes `true` only to props declared with the `Boolean` type.
    /// It passes an empty string instead when the prop isn't declared as `Boolean`, when its type
    /// lists `String` before `Boolean`, such as `type: [String, Boolean]`, or when the component
    /// doesn't declare the prop and Vue adds it to the component's root element as an HTML
    /// attribute. Check how the component declares the prop before applying the fix. To catch
    /// props that list another type before `Boolean`, use
    /// [`useVueBooleanPropTypeFirst`](https://biomejs.dev/linter/rules/use-vue-boolean-prop-type-first/).
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent :disabled="true" />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent v-bind:disabled="true" />
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <MyComponent disabled />
    ///   <MyComponent :disabled="false" />
    ///   <MyComponent :disabled="isDisabled" />
    ///   <MyComponent disabled="true" />
    ///   <input :disabled="true" />
    /// </template>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `style`
    ///
    /// Default: `"shorthand"`
    ///
    /// How a prop set to `true` should be written:
    ///
    /// - `"shorthand"`: write only the name of the prop, such as `disabled`.
    /// - `"longhand"`: write the value `true` explicitly, such as `:disabled="true"`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "style": "longhand"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///   <MyComponent disabled />
    /// </template>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <MyComponent :disabled="true" />
    ///   <MyComponent v-bind:disabled="true" />
    /// </template>
    /// ```
    ///
    /// ### `except`
    ///
    /// Default: `[]`
    ///
    /// A list of prop names that must use the opposite of [`style`](#style). Names must match
    /// exactly, as written in the template. With the default `style`, the following configuration
    /// requires `value` to be written in the longhand form:
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "except": ["value"]
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///   <MyComponent value />
    /// </template>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <MyComponent :value="true" disabled />
    /// </template>
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want a consistent style for `v-bind` in Vue, see [`useVueConsistentVBindStyle`](https://biomejs.dev/linter/rules/use-vue-consistent-v-bind-style/).
    /// - If you want `Boolean` to be listed first in the type of a prop, see [`useVueBooleanPropTypeFirst`](https://biomejs.dev/linter/rules/use-vue-boolean-prop-type-first/).
    ///
    pub UseVueConsistentBooleanPropStyle {
        version: "next",
        name: "useVueConsistentBooleanPropStyle",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("prefer-true-attribute-shorthand").same()],
        fix_kind: FixKind::Unsafe,
    }
}

/// A component prop that passes `true`, in the form it's written.
pub enum TrueAttribute {
    /// A prop without a value, such as `disabled`.
    Shorthand(HtmlAttribute),
    /// A prop with the value `true`, such as `:disabled="true"` or `v-bind:disabled="true"`.
    Longhand(AnyVueDirective),
}

impl Rule for UseVueConsistentBooleanPropStyle {
    type Query = Ast<AnyHtmlTagElement>;
    type State = TrueAttribute;
    type Signals = Box<[Self::State]>;
    type Options = UseVueConsistentBooleanPropStyleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        if !ctx.source_type::<HtmlFileSource>().is_vue()
            || is_top_level(element)
            || !is_vue_component(element)
        {
            return Box::default();
        }

        let options = ctx.options();
        let prefers_shorthand = options.style() == BooleanPropStyle::Shorthand;
        element
            .attributes()
            .iter()
            .filter_map(|attribute| {
                let true_attribute = TrueAttribute::from_attribute(attribute)?;
                let name = true_attribute.name()?;
                let is_excepted = options
                    .except()
                    .iter()
                    .any(|except| except.as_ref() == name.text_trimmed());
                let expects_shorthand = prefers_shorthand != is_excepted;
                let is_shorthand = matches!(true_attribute, TrueAttribute::Shorthand(_));
                (is_shorthand != expects_shorthand).then_some(true_attribute)
            })
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = state.name()?;
        let name = name.text_trimmed();
        let is_longhand = matches!(state, TrueAttribute::Longhand(_));
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.range(),
            markup! {
                "The prop "<Emphasis>{name}</Emphasis>{
                    if is_longhand {
                        markup! { " has the value "<Emphasis>"true"</Emphasis>", which can be omitted." }
                    } else {
                        markup! { " is written without a value." }
                    }
                }
            },
        )
        .note(if is_longhand {
            markup! {
                "A prop written without a value receives "<Emphasis>"true"</Emphasis>", so the shorthand form is shorter and passes the same value."
            }
        } else {
            markup! {
                "This project requires writing the value "<Emphasis>"true"</Emphasis>" explicitly, which makes it clear that the prop receives a boolean."
            }
        })
        .note(if is_longhand {
            markup! {
                "The shorthand form passes "<Emphasis>"true"</Emphasis>" only if the component declares the prop as "<Emphasis>"Boolean"</Emphasis>" and doesn't list "<Emphasis>"String"</Emphasis>" before "<Emphasis>"Boolean"</Emphasis>". Otherwise, it passes an empty string. Check how the component declares the prop before you remove the value."
            }
        } else {
            markup! {
                "A prop without a value receives "<Emphasis>"true"</Emphasis>" only if the component declares it as "<Emphasis>"Boolean"</Emphasis>" and doesn't list "<Emphasis>"String"</Emphasis>" before "<Emphasis>"Boolean"</Emphasis>". Otherwise, it receives an empty string. Check how the component declares the prop before you add the value."
            }
        });
        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let mut mutation = ctx.root().begin();
        let message = match state {
            TrueAttribute::Longhand(directive) => {
                let name = state.name()?;
                let shorthand = make::html_attribute(make::html_attribute_name(literal(
                    name.text_trimmed(),
                )))
                .build();
                mutation.replace_node(
                    AnyHtmlAttribute::AnyVueDirective(directive.clone()),
                    AnyHtmlAttribute::from(shorthand),
                );
                markup! { "Remove the value." }
            }
            TrueAttribute::Shorthand(attribute) => {
                let name = attribute.name().ok()?.value_token().ok()?;
                let argument = make::vue_directive_argument(make::token(T![:]))
                    .with_arg(AnyVueDirectiveArgument::VueStaticArgument(
                        make::vue_static_argument(literal(name.text_trimmed())),
                    ))
                    .build();
                let initializer = make::html_attribute_initializer_clause(
                    make::token(T![=]),
                    AnyHtmlAttributeInitializer::HtmlString(make::html_string(
                        make::html_string_literal("true"),
                    )),
                );
                let longhand = AnyVueDirective::from(
                    make::vue_v_bind_shorthand_directive(argument, make::vue_modifier_list([]))
                        .with_initializer(initializer)
                        .build(),
                );
                mutation.replace_node(
                    AnyHtmlAttribute::from(attribute.clone()),
                    AnyHtmlAttribute::AnyVueDirective(longhand),
                );
                markup! { "Write the value "<Emphasis>"true"</Emphasis>" explicitly." }
            }
        };
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message,
            mutation,
        ))
    }
}

impl TrueAttribute {
    /// Returns the attribute if it's a prop that passes `true`, in a form that this rule can
    /// rewrite.
    ///
    /// Vue bindings with modifiers, such as `:disabled.prop="true"`, return `None`, because the
    /// shorthand form can't keep the modifiers.
    fn from_attribute(attribute: AnyHtmlAttribute) -> Option<Self> {
        match attribute {
            // Vue reads a plain attribute value as a string, so only an attribute without a value
            // can pass `true`.
            AnyHtmlAttribute::HtmlAttribute(attribute) => attribute
                .initializer()
                .is_none()
                .then_some(Self::Shorthand(attribute)),
            AnyHtmlAttribute::AnyVueDirective(directive) => {
                let (modifiers, initializer) = match &directive {
                    AnyVueDirective::VueVBindShorthandDirective(directive) => {
                        (directive.modifiers(), directive.initializer())
                    }
                    AnyVueDirective::VueDirective(directive) if directive.is_binding() => {
                        (directive.modifiers(), directive.initializer())
                    }
                    _ => return None,
                };
                if !modifiers.is_empty() {
                    return None;
                }
                let AnyHtmlAttributeInitializer::HtmlString(value) = initializer?.value().ok()?
                else {
                    return None;
                };
                let value = value.inner_string_text().ok()?;
                is_true_literal(value.text()).then_some(Self::Longhand(directive))
            }
            _ => None,
        }
    }

    fn range(&self) -> TextRange {
        match self {
            Self::Shorthand(attribute) => attribute.range(),
            Self::Longhand(attribute) => attribute.range(),
        }
    }

    /// Returns the token with the name of the prop, such as `disabled` in `:disabled="true"`.
    ///
    /// Returns `None` for Vue bindings with a dynamic name, such as `:[name]="true"`.
    fn name(&self) -> Option<HtmlSyntaxToken> {
        match self {
            Self::Shorthand(attribute) => attribute.name().ok()?.value_token().ok(),
            Self::Longhand(directive) => {
                let argument = match directive {
                    AnyVueDirective::VueVBindShorthandDirective(directive) => {
                        directive.arg().ok()?
                    }
                    AnyVueDirective::VueDirective(directive) => directive.arg()?,
                    _ => return None,
                };
                vue_static_argument_name(&argument)
            }
        }
    }
}

/// Returns `true` if the element is at the top level of the file.
///
/// In a Vue file, the top-level elements are blocks such as `<template>`, `<script>`, or custom
/// blocks like `<i18n>`, so their attributes aren't props.
fn is_top_level(element: &AnyHtmlTagElement) -> bool {
    let element = match element {
        AnyHtmlTagElement::HtmlOpeningElement(element) => element.syntax().parent(),
        AnyHtmlTagElement::HtmlSelfClosingElement(element) => Some(element.syntax().clone()),
    };
    element
        .and_then(|element| element.grand_parent())
        .is_some_and(|parent| HtmlRoot::can_cast(parent.kind()))
}

fn vue_static_argument_name(argument: &VueDirectiveArgument) -> Option<HtmlSyntaxToken> {
    argument
        .arg()?
        .as_vue_static_argument()?
        .name_token()
        .ok()
}

/// Returns `true` if the source text of a JavaScript expression is the literal `true`.
fn is_true_literal(expression: &str) -> bool {
    expression.trim() == "true"
}

fn literal(text: &str) -> HtmlSyntaxToken {
    HtmlSyntaxToken::new_detached(HtmlSyntaxKind::HTML_LITERAL, text, [], [])
}
