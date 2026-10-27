use crate::HtmlRuleAction;
use crate::services::vue::VueSemantic;
use biome_analyze::{
    FixKind, Rule, RuleAction, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_factory::make;
use biome_html_syntax::{
    AnyVueDirective, AnyVueDirectiveArgument, HtmlSyntaxKind, HtmlSyntaxToken, T,
    element_ext::AnyHtmlTagElement,
};
use biome_languages::HtmlFileSource;
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};
use biome_rule_options::use_vue_v_model::UseVueVModelOptions;
use biome_vue_semantic::{Element, RootShape, Snippet};

declare_lint_rule! {
    /// Enforce `v-model` instead of a prop binding paired with an `update:` listener that writes the value back.
    ///
    /// A component that supports [`v-model`](https://vuejs.org/guide/components/v-model.html)
    /// receives a prop, and emits an `update:` event with the new value when it changes. Binding
    /// the prop and writing the new value back by hand, as in
    /// `:title="title" @update:title="title = $event"`, does the same thing as
    /// `v-model:title="title"`. The `v-model` form is shorter, and the prop and its listener can't
    /// get out of sync.
    ///
    /// For the `modelValue` prop, the rule suggests the shorter `v-model="value"` form.
    ///
    /// The rule only checks components. It reports a pair when the listener does nothing but
    /// assign the new value to the bound expression, written either as `value = $event` or as
    /// `(newValue) => value = newValue`. Bindings and listeners with modifiers are ignored.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///   <MyComponent :modelValue="text" @update:modelValue="text = $event" />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///   <MyComponent :title="title" @update:title="title = $event" />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///   <MyComponent v-bind:title="title" v-on:update:title="(value) => title = value" />
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue,ignore
    /// <template>
    ///   <MyComponent v-model="text" />
    ///   <MyComponent v-model:title="title" />
    ///   <MyComponent :title="title" @update:title="otherTitle = $event" />
    ///   <MyComponent :title="title" @update:title="setTitle($event)" />
    ///   <MyComponent :title="title" @update:title="title = $event; save()" />
    /// </template>
    /// ```
    ///
    pub UseVueVModel {
        version: "next",
        name: "useVueVModel",
        language: "html",
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("prefer-v-model").same()],
        fix_kind: FixKind::Unsafe,
    }
}

/// A prop binding and the `update:` listener that only writes the new value back to it.
pub struct RedundantPair {
    /// The `:prop` or `v-bind:prop` directive.
    binding: AnyVueDirective,
    /// The `@update:prop` or `v-on:update:prop` directive.
    listener: AnyVueDirective,
}

impl Rule for UseVueVModel {
    type Query = VueSemantic<AnyHtmlTagElement>;
    type State = RedundantPair;
    type Signals = Box<[Self::State]>;
    type Options = UseVueVModelOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        if !ctx.source_type::<HtmlFileSource>().is_vue() || !is_component(element) {
            return Box::default();
        }

        let mut bindings = Vec::new();
        let mut listeners = Vec::new();
        for attribute in element.attributes().iter() {
            let Some(directive) = attribute.as_any_vue_directive() else {
                continue;
            };
            if let Some(prop) = bound_prop(directive) {
                bindings.push((directive.clone(), prop));
            } else if let Some(prop) = updated_prop(directive) {
                listeners.push((directive.clone(), prop));
            }
        }
        if bindings.is_empty() || listeners.is_empty() {
            return Box::default();
        }

        let Some(model_element) = ctx.model().element_at(element.range()) else {
            return Box::default();
        };
        bindings
            .into_iter()
            .filter_map(|(binding, prop)| {
                // Only the first listener for the prop is compared.
                let (listener, _) = listeners
                    .iter()
                    .find(|(_, updated)| is_same_prop(prop.text_trimmed(), updated.text_trimmed()))?;
                let bound = value_snippet(&model_element, &binding)?;
                let handler = value_snippet(&model_element, listener)?;
                let writes_back = *handler.root()
                    == RootShape::Assignment {
                        target: bound.fingerprint(),
                        from_event: true,
                    };
                writes_back.then(|| RedundantPair {
                    binding,
                    listener: listener.clone(),
                })
            })
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let prop = bound_prop(&state.binding)?;
        let v_model = VModel(prop.text_trimmed());
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.binding.range(),
                markup! {
                    "The "<Emphasis>{prop.text_trimmed()}</Emphasis>" prop is bound and updated by hand, which "<Emphasis>{v_model}</Emphasis>" does for you."
                },
            )
            .detail(
                state.listener.range(),
                markup! {
                    "This listener only writes the new value back to the bound expression."
                },
            )
            .note(markup! {
                "Replace both with "<Emphasis>{v_model}</Emphasis>", which is shorter and keeps the binding and the listener from getting out of sync."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let prop = bound_prop(&state.binding)?;
        let initializer = match &state.binding {
            AnyVueDirective::VueDirective(directive) => directive.initializer(),
            AnyVueDirective::VueVBindShorthandDirective(directive) => directive.initializer(),
            _ => None,
        }?;

        let mut builder = make::vue_directive(make::ident("v-model"), make::vue_modifier_list([]))
            .with_initializer(initializer);
        if !is_same_prop(prop.text_trimmed(), "modelValue") {
            // A new `:` token, because the one of a `:prop` shorthand holds the space before the
            // directive, which would end up between `v-model` and `:prop`.
            builder = builder.with_arg(
                make::vue_directive_argument(make::token(T![:]))
                    .with_arg(AnyVueDirectiveArgument::VueStaticArgument(
                        make::vue_static_argument(prop.clone()),
                    ))
                    .build(),
            );
        }

        let v_model = AnyVueDirective::VueDirective(builder.build());
        let binding = state.binding.syntax();
        let listener = state.listener.syntax();
        let mut mutation = ctx.root().begin();
        // The space before an attribute is trailing trivia of the token before it. When the
        // listener ends the tag, as in `@update:prop="value = $event">`, removing the listener
        // alone would leave that space before the `>`.
        let previous = listener.first_token()?.prev_token()?;
        let ends_tag = listener
            .last_trailing_trivia()
            .is_none_or(|trivia| trivia.is_empty());
        if ends_tag && binding.last_token().as_ref() == Some(&previous) {
            let leading = binding.first_token()?.leading_trivia();
            mutation.replace_node_discard_trivia(
                state.binding.clone(),
                v_model
                    .with_leading_trivia_pieces(leading.pieces())?
                    .trim_trailing_trivia()?,
            );
        } else {
            if ends_tag {
                mutation.replace_token_discard_trivia(previous.clone(), previous.trim_trailing_trivia());
            }
            mutation.replace_node(state.binding.clone(), v_model);
        }
        mutation.remove_node(state.listener.clone());
        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace them with "<Emphasis>{VModel(prop.text_trimmed())}</Emphasis>"." },
            mutation,
        ))
    }
}

/// Returns `true` if Vue renders the element as a component instead of a native element.
///
/// Tags that aren't known HTML or SVG elements, such as `<card>`, `<component>` or `<MyComponent>`,
/// are components. A native element becomes a component when its `is` attribute starts with
/// `vue:`, as in `<input is="vue:my-input">`.
fn is_component(element: &AnyHtmlTagElement) -> bool {
    element
        .tag_name_kind()
        .is_none_or(|kind| COMPONENT_TAG_NAMES.contains(kind))
        || element
            .find_attribute_by_name("is")
            .and_then(|attribute| attribute.as_html_attribute()?.initializer()?.value().ok())
            .and_then(|value| value.as_static_value())
            .is_some_and(|value| value.text().starts_with("vue:"))
}

/// Tag name kinds that Vue resolves to a component: `<component>` and tags that aren't known HTML
/// or SVG elements.
const COMPONENT_TAG_NAMES: TokenSet<HtmlSyntaxKind> = token_set![
    HtmlSyntaxKind::COMPONENT_KW,
    HtmlSyntaxKind::HTML_UNKNOWN_TAG
];

/// Returns the prop name token of a `:prop="value"` or `v-bind:prop="value"` directive without
/// modifiers.
fn bound_prop(directive: &AnyVueDirective) -> Option<HtmlSyntaxToken> {
    let (argument, modifiers, has_value) = match directive {
        AnyVueDirective::VueDirective(directive) if directive.is_binding() => (
            directive.arg()?,
            directive.modifiers(),
            directive.initializer().is_some(),
        ),
        AnyVueDirective::VueVBindShorthandDirective(directive) => (
            directive.arg().ok()?,
            directive.modifiers(),
            directive.initializer().is_some(),
        ),
        _ => return None,
    };
    if !modifiers.is_empty() || !has_value {
        return None;
    }
    argument.arg()?.as_vue_static_argument()?.name_token().ok()
}

/// Returns the event name token of an `@update:prop="handler"` or `v-on:update:prop="handler"`
/// directive without modifiers.
fn updated_prop(directive: &AnyVueDirective) -> Option<HtmlSyntaxToken> {
    let (argument, modifiers, has_value) = match directive {
        AnyVueDirective::VueDirective(directive) if directive.is_event_listener() => (
            directive.arg()?.arg()?,
            directive.modifiers(),
            directive.initializer().is_some(),
        ),
        AnyVueDirective::VueVOnShorthandDirective(directive) => (
            directive.arg().ok()?,
            directive.modifiers(),
            directive.initializer().is_some(),
        ),
        _ => return None,
    };
    if !modifiers.is_empty() || !has_value {
        return None;
    }
    let token = argument.as_vue_static_argument()?.name_token().ok()?;
    token.text_trimmed().starts_with("update:").then_some(token)
}

/// Returns `true` if `prop` names the same prop as `other`, which is either a prop name or an
/// `update:` event name.
///
/// Vue converts kebab-case prop and event names to camelCase, so `model-value` and
/// `update:modelValue` name the same prop.
fn is_same_prop(prop: &str, other: &str) -> bool {
    let other = other.strip_prefix("update:").unwrap_or(other);
    camelized(prop).eq(camelized(other))
}

/// Returns the characters of `name` with each `-` removed and the letter after it uppercased, as
/// Vue's `camelize` does.
fn camelized(name: &str) -> impl Iterator<Item = char> + '_ {
    let mut chars = name.chars().peekable();
    std::iter::from_fn(move || {
        let char = chars.next()?;
        if char == '-'
            && let Some(next) = chars.next_if(|next| next.is_ascii_alphanumeric() || *next == '_')
        {
            return Some(next.to_ascii_uppercase());
        }
        Some(char)
    })
}

/// Returns the snippet holding the value of `directive`.
fn value_snippet(element: &Element, directive: &AnyVueDirective) -> Option<Snippet> {
    let range = directive.range();
    element
        .attrs()
        .find(|attr| attr.range() == range)?
        .value_snippet()
}

/// Writes the `v-model` directive that replaces the binding of `prop`: `v-model` for `modelValue`,
/// and `v-model:prop` for other props.
struct VModel<'a>(&'a str);

impl biome_console::fmt::Display for VModel<'_> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        if is_same_prop(self.0, "modelValue") {
            fmt.write_str("v-model")
        } else {
            fmt.write_str("v-model:")?;
            fmt.write_str(self.0)
        }
    }
}
