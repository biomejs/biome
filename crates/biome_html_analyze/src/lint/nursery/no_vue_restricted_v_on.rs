use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{
    AnyHtmlTagName, AnyVueDirective, AnyVueDirectiveArgument, HtmlAttributeInitializerClause,
    HtmlAttributeList, VueModifierList, element_ext::AnyHtmlTagElement,
};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::no_vue_restricted_v_on::{
    NoVueRestrictedVOnOptions, RestrictedVOnListener,
};

declare_lint_rule! {
    /// Disallow configured event listeners in `v-on` directives.
    ///
    /// Projects sometimes want to stop certain events from being listened to in templates. For
    /// example, a design system might ask developers to use a component's own `select` event
    /// instead of listening to `click` on it, or a project might want form submissions to go
    /// through a shared component instead of `@submit`.
    /// This rule reports `v-on` directives, including the `@` shorthand, that match one of the
    /// configured listeners. Without listeners configured, this rule doesn't do anything.
    ///
    /// Event names, modifiers, and element names are matched exactly, including their casing.
    /// Event names that are computed when the page runs, such as `@[eventName]`, are never
    /// reported, because the rule can't know their value.
    ///
    /// ## Examples
    ///
    /// The examples below use this configuration:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "listeners": [
    ///             { "event": "focusin" },
    ///             { "event": "submit", "message": "Use the onSave prop instead." }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///     <input @focusin="onFocus" />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///     <form v-on:submit="save"></form>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///     <input @focus="onFocus" />
    ///     <form @[eventName]="save"></form>
    /// </template>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `listeners`
    ///
    /// Default: `[]`
    ///
    /// A list of event listeners to disallow. A `v-on` directive is reported when it matches
    /// every field of an entry. Each entry accepts these fields:
    ///
    /// - `event`: the event name, such as `"click"` for `@click` and `v-on:click`. Leave it out
    ///   to disallow `v-on` used without an event name, such as `v-on="handlers"`.
    /// - `modifiers`: a list of modifiers, such as `["prevent"]`. The entry only applies when
    ///   the directive uses all of these modifiers. The directive can use other modifiers too.
    /// - `element`: the element or component name, such as `"MyButton"`. The entry only applies
    ///   to directives on that element.
    /// - `message`: an extra message to show in the diagnostic, such as an explanation or an
    ///   alternative to use.
    ///
    /// #### `event` left out
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "listeners": [{}]
    ///     }
    /// }
    /// ```
    ///
    /// ##### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///     <div v-on="handlers"></div>
    /// </template>
    /// ```
    ///
    /// ##### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///     <div @click="onClick"></div>
    /// </template>
    /// ```
    ///
    /// #### `modifiers`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "listeners": [{ "event": "click", "modifiers": ["prevent"] }]
    ///     }
    /// }
    /// ```
    ///
    /// ##### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///     <a @click.stop.prevent="onClick"></a>
    /// </template>
    /// ```
    ///
    /// ##### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///     <a @click="onClick"></a>
    ///     <a @click.stop="onClick"></a>
    /// </template>
    /// ```
    ///
    /// #### `element`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "listeners": [{ "event": "click", "element": "MyButton" }]
    ///     }
    /// }
    /// ```
    ///
    /// ##### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///     <MyButton @click="onClick" />
    /// </template>
    /// ```
    ///
    /// ##### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///     <button @click="onClick"></button>
    ///     <MyLink @click="onClick" />
    /// </template>
    /// ```
    ///
    pub NoVueRestrictedVOn {
        version: "2.6.5",
        name: "noVueRestrictedVOn",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-restricted-v-on").same()],
    }
}

impl Rule for NoVueRestrictedVOn {
    type Query = Ast<AnyVueDirective>;
    /// The index of the matching entry in the configured `listeners`.
    type State = usize;
    type Signals = Option<Self::State>;
    type Options = NoVueRestrictedVOnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let listeners = ctx.options().listeners();
        if listeners.is_empty() {
            return None;
        }
        let (argument, modifiers) = v_on_parts(ctx.query())?;
        let tag_name = ctx
            .query()
            .parent::<HtmlAttributeList>()
            .and_then(|attributes| attributes.parent::<AnyHtmlTagElement>())
            .and_then(|element| element.name().ok());

        listeners.iter().position(|listener| {
            event_matches(argument.as_ref(), listener)
                && modifiers_match(&modifiers, listener)
                && element_matches(tag_name.as_ref(), listener)
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, index: &Self::State) -> Option<RuleDiagnostic> {
        let directive = ctx.query();
        let listener = ctx.options().listeners().get(*index)?;
        let range = directive_key_range(directive)?;
        let key = directive
            .syntax()
            .text_trimmed()
            .slice(range - directive.syntax().text_trimmed_range().start());

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "The event listener "<Emphasis>{format_args!("{key}")}</Emphasis>" is restricted in this project."
            },
        );
        if let Some(message) = &listener.message {
            diagnostic = diagnostic.note(markup! { {message} });
        }
        Some(diagnostic.note(markup! {
            "Remove the event listener or use an allowed alternative. If this event listener should be allowed, update the "<Emphasis>"listeners"</Emphasis>" option."
        }))
    }
}

/// Returns the event name argument and the modifiers of a `v-on` directive, or `None` if the
/// directive isn't `v-on`.
///
/// The argument is `None` for `v-on` used without an event name, such as `v-on="handlers"`.
fn v_on_parts(
    directive: &AnyVueDirective,
) -> Option<(Option<AnyVueDirectiveArgument>, VueModifierList)> {
    match directive {
        AnyVueDirective::VueDirective(directive) if directive.is_event_listener() => Some((
            directive.arg().and_then(|argument| argument.arg()),
            directive.modifiers(),
        )),
        AnyVueDirective::VueVOnShorthandDirective(directive) => {
            Some((directive.arg().ok(), directive.modifiers()))
        }
        _ => None,
    }
}

/// Returns whether the event name of a directive is the `event` of `listener`.
///
/// Dynamic event names, such as `@[eventName]`, never match.
fn event_matches(
    argument: Option<&AnyVueDirectiveArgument>,
    listener: &RestrictedVOnListener,
) -> bool {
    match (argument, listener.event.as_deref()) {
        (None, None) => true,
        (Some(AnyVueDirectiveArgument::VueStaticArgument(argument)), Some(event)) => argument
            .name_token()
            .is_ok_and(|name| name.text_trimmed() == event),
        _ => false,
    }
}

/// Returns whether the directive uses every modifier of `listener`.
fn modifiers_match(modifiers: &VueModifierList, listener: &RestrictedVOnListener) -> bool {
    listener.modifiers.iter().all(|expected| {
        modifiers.iter().any(|modifier| {
            modifier
                .modifier_token()
                .is_ok_and(|token| token.text_trimmed() == expected.as_ref())
        })
    })
}

/// Returns whether the directive is on the `element` of `listener`, or `true` when
/// `listener` doesn't restrict the element.
fn element_matches(tag_name: Option<&AnyHtmlTagName>, listener: &RestrictedVOnListener) -> bool {
    let Some(element) = listener.element.as_deref() else {
        return true;
    };
    tag_name.is_some_and(|name| name.syntax().text_trimmed() == element)
}

/// Returns the range of the directive without its value: `@click.prevent` in
/// `@click.prevent="onClick"`.
fn directive_key_range(directive: &AnyVueDirective) -> Option<TextRange> {
    let range = directive.syntax().text_trimmed_range();
    let Some(initializer) = directive
        .syntax()
        .children()
        .find_map(HtmlAttributeInitializerClause::cast)
    else {
        return Some(range);
    };
    let key_end = initializer
        .syntax()
        .first_token()?
        .prev_token()?
        .text_trimmed_range()
        .end();
    Some(TextRange::new(range.start(), key_end))
}
