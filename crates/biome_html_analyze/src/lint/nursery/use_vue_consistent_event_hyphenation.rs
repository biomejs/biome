use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{
    AnyVueDirective, AnyVueDirectiveArgument, HtmlAttributeList, HtmlSyntaxToken,
    VueStaticArgument, element_ext::AnyHtmlTagElement,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_vue_consistent_event_hyphenation::{
    EventHyphenation, UseVueConsistentEventHyphenationOptions,
};

use biome_string_case::Case;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Enforce a consistent hyphenation style for event names in `v-on` directives on custom components.
    ///
    /// Vue converts event names between camelCase and kebab-case automatically, so
    /// `@custom-event` and `@customEvent` both listen to an event emitted as `customEvent`.
    /// Mixing both styles makes templates harder to read and search. The
    /// [Vue documentation](https://vuejs.org/guide/components/events.html#emitting-and-listening-to-events)
    /// recommends kebab-cased event listeners in templates.
    ///
    /// The rule only checks components, such as `<MyComponent>`, `<my-component>` and
    /// `<Foo.Bar>`. Native elements such as `<div>` are ignored, even when they have an `is`
    /// attribute.
    /// Dynamic event names, such as `@[event]`, are ignored as well.
    ///
    /// The fix is unsafe: Vue doesn't convert event names on custom elements (web components)
    /// declared with `isCustomElement`, so renaming their event listeners can stop them from
    /// receiving events.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent @customEvent="handleEvent" />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent v-on:update:modelValue="handleEvent" />
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <MyComponent @custom-event="handleEvent" />
    ///   <MyComponent v-on:update:model-value="handleEvent" />
    ///   <div @customEvent="handleEvent" />
    /// </template>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `hyphenation`
    ///
    /// Whether event names must be hyphenated (`"always"`) or must not contain hyphens (`"never"`).
    ///
    /// Default: `"always"`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "hyphenation": "never"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///   <MyComponent @custom-event="handleEvent" />
    /// </template>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <MyComponent @customEvent="handleEvent" />
    /// </template>
    /// ```
    ///
    /// ### `ignore`
    ///
    /// A list of event names that aren't checked.
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignore": ["customEvent"]
    ///   }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <MyComponent @customEvent="handleEvent" />
    /// </template>
    /// ```
    ///
    /// ### `ignoreTags`
    ///
    /// A list of tag names whose events aren't checked. Tag names are matched exactly.
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignoreTags": ["MyComponent"]
    ///   }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <MyComponent @customEvent="handleEvent" />
    /// </template>
    /// ```
    ///
    pub UseVueConsistentEventHyphenation {
        version: "2.6.0",
        name: "useVueConsistentEventHyphenation",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("v-on-event-hyphenation").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseVueConsistentEventHyphenation {
    type Query = Ast<AnyVueDirective>;
    type State = VueStaticArgument;
    type Signals = Option<Self::State>;
    type Options = UseVueConsistentEventHyphenationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let options = ctx.options();
        let argument = event_name_argument(ctx.query())?;
        let name = argument.name_token().ok()?;
        let name = name.text_trimmed();

        if follows_style(name, options.hyphenation())
            || options.ignore().iter().any(|ignored| ignored.as_ref() == name)
        {
            return None;
        }

        let element = ctx
            .query()
            .parent::<HtmlAttributeList>()?
            .parent::<AnyHtmlTagElement>()?;
        if !element.is_custom_component() {
            return None;
        }
        // Member names such as `<Foo.Bar>` have no single tag name token, so they can't be ignored.
        if let Some(tag_name) = element.name().ok()?.token_text_trimmed()
            && options
                .ignore_tags()
                .iter()
                .any(|ignored| ignored.as_ref() == tag_name.text())
        {
            return None;
        }

        Some(argument)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = state.name_token().ok()?;
        let name = name.text_trimmed();
        let diagnostic = match ctx.options().hyphenation() {
            EventHyphenation::Always => RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "The event name "<Emphasis>{name}</Emphasis>" isn't hyphenated."
                },
            )
            .note(markup! {
                "Vue matches event listeners regardless of their casing, so mixing camelCase and kebab-case event names makes templates inconsistent."
            })
            .note(markup! {
                "This project requires hyphenated (kebab-case) event names."
            }),
            EventHyphenation::Never => RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "The event name "<Emphasis>{name}</Emphasis>" contains hyphens."
                },
            )
            .note(markup! {
                "Vue matches event listeners regardless of their casing, so mixing camelCase and kebab-case event names makes templates inconsistent."
            })
            .note(markup! {
                "This project requires event names without hyphens, such as camelCase."
            }),
        };
        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let old_token = state.name_token().ok()?;
        let name = old_token.text_trimmed();

        // Underscores can't be converted without guessing the intended word boundaries.
        if name.contains('_') {
            return None;
        }

        let case = match ctx.options().hyphenation() {
            EventHyphenation::Always => Case::Kebab,
            EventHyphenation::Never => Case::Camel,
        };
        let suggested = convert_event_name(name, case);

        let new_token = HtmlSyntaxToken::new_detached(old_token.kind(), &suggested, [], []);
        let mut mutation = ctx.root().begin();
        mutation.replace_token_transfer_trivia(old_token, new_token);

        Some(biome_analyze::RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Rename the event to "<Emphasis>{suggested}</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns the static event name argument of a `v-on` directive, such as `click` in
/// `@click` or `v-on:click`.
fn event_name_argument(directive: &AnyVueDirective) -> Option<VueStaticArgument> {
    let argument = match directive {
        AnyVueDirective::VueDirective(directive) if directive.is_event_listener() => {
            directive.arg()?.arg()?
        }
        AnyVueDirective::VueVOnShorthandDirective(directive) => directive.arg().ok()?,
        _ => return None,
    };
    match argument {
        AnyVueDirectiveArgument::VueStaticArgument(argument) => Some(argument),
        _ => None,
    }
}

fn follows_style(name: &str, hyphenation: EventHyphenation) -> bool {
    match hyphenation {
        EventHyphenation::Always => !name.chars().any(char::is_uppercase),
        EventHyphenation::Never => !name.contains('-'),
    }
}

/// Converts each `:`-separated segment of an event name to `case`, so `update:modelValue`
/// becomes `update:model-value`.
///
/// `Case::convert` treats `:` as a word separator, so converting the whole name at once would
/// rename the `update:` namespace and produce `update-model-value`.
fn convert_event_name(name: &str, case: Case) -> String {
    // Kebab-casing inserts a hyphen at each word boundary, so the result can be longer than
    // `name`. The extra 4 bytes fit the hyphens of typical event names like
    // `update:modelValue` without reallocating; longer results still grow as needed.
    let mut result = String::with_capacity(name.len() + 4);
    for (index, segment) in name.split(':').enumerate() {
        if index > 0 {
            result.push(':');
        }
        result.push_str(&case.convert(segment));
    }
    result
}
