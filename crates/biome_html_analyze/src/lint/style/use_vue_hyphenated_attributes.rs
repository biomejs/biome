use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{
    AnyHtmlAttribute, SVG_EXCLUSIVE_TAG_NAMES, element_ext::AnyHtmlTagElement,
};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNodeList, TokenText};
use biome_rule_options::use_vue_hyphenated_attributes::UseVueHyphenatedAttributesOptions;
use biome_string_case::Case;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Disallow uppercase letters in Vue template attribute names.
    ///
    /// Vue style guide recommends using hyphenated attribute (and prop) names in templates to
    /// keep them consistent and distinguish them from JavaScript identifiers written in camelCase/PascalCase.
    ///
    /// Like the upstream ESLint rule, this rule flags attribute names that contain uppercase letters.
    /// It doesn't require exact kebab-case, so punctuation such as colons and underscores is allowed.
    ///
    /// Allowed:
    /// - names without uppercase letters (e.g. `data-test-id`, `pt:header:id`, `some_attr`)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <div fooBar="x"></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <MyComp :someProp="x" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <div data-test-id="x"></div>
    /// <div class="foo"></div>
    /// <MyComp :some-prop="x" />
    /// <MyComp pt:header:data-test-id="x" />
    /// ```
    ///
    /// ## Options
    ///
    /// The rule supports the following options:
    ///
    /// ### `ignore`
    ///
    /// A list of attribute names that should be exempt from the uppercase-letter check.
    /// Use this when you have a fixed set of camelCase / PascalCase prop names you intentionally allow.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignore": ["someProp", "fooBar"]
    ///   }
    /// }
    /// ```
    ///
    /// #### Valid (using `ignore`)
    ///
    /// ```vue,use_options
    /// <div fooBar="x"></div>
    /// ```
    ///
    /// ### `ignoreTags`
    ///
    /// A list of tag names whose attributes should be exempt from the uppercase-letter check.
    /// This is useful for third-party or internal components that deliberately expose camelCase or PascalCase prop names.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignoreTags": ["MyComp", "AnotherWidget"]
    ///   }
    /// }
    /// ```
    ///
    /// #### Valid (using `ignoreTags`)
    ///
    /// ```vue,use_options
    /// <MyComp :someProp="x" />
    /// ```
    ///
    pub UseVueHyphenatedAttributes {
        version: "2.3.6",
        name: "useVueHyphenatedAttributes",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("attribute-hyphenation").same()],
        // marked as unsafe until we feel comfortable making it safe
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseVueHyphenatedAttributes {
    type Query = Ast<AnyHtmlTagElement>;
    type State = AnyHtmlAttribute;
    type Signals = Box<[Self::State]>;
    type Options = UseVueHyphenatedAttributesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let source_type = ctx.source_type::<HtmlFileSource>();

        if !source_type.is_vue() {
            return Box::new([]);
        }

        if is_svg_element(node) {
            return Box::new([]);
        }

        if let Some(ignore_tags) = ctx.options().ignore_tags.as_ref()
            && let Ok(tag_name) = node.name()
            && let Some(tag_name_text) = tag_name.token_text_trimmed()
            && ignore_tags.contains(tag_name_text.as_ref())
        {
            return Box::new([]);
        }

        let mut violations = Vec::new();
        for attribute in node.attributes().iter() {
            let Some(attr_name) = extract_attribute_name(&attribute) else {
                continue;
            };
            if let Some(ignore) = ctx.options().ignore.as_ref()
                && ignore.contains(attr_name.text())
            {
                continue;
            }
            if contains_uppercase(attr_name.text()) {
                violations.push(attribute.clone());
            }
        }

        violations.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = extract_attribute_name(state)?;
        let name = name.text();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "Attribute "<Emphasis>{name}</Emphasis>" should be hyphenated (kebab-case)."
                },
            )
            .note(markup! {
                "The Vue style guide recommends using hyphenated attribute (and prop) names in templates to keep them consistent."
            })
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let name = extract_attribute_name(state)?;
        let name = name.text();

        // In Vue, colons can be meaningful parts of plain attribute names, such as PrimeVue
        // pass-through attributes. `Case::Kebab.convert` would replace them with hyphens, so
        // don't offer a fix that could change the attribute's meaning.
        if name.contains(':') {
            return None;
        }

        let suggested = Case::Kebab.convert(name);

        // Start a batch mutation
        let mut mutation = biome_rowan::BatchMutationExt::begin(ctx.root());

        let mut applied = false;

        // Plain HTML attribute name
        if let Some(html_attr) = state.as_html_attribute() {
            if let Ok(attr_name) = html_attr.name()
                && let Ok(old_token) = attr_name.value_token()
            {
                let new_token = biome_html_syntax::HtmlSyntaxToken::new_detached(
                    old_token.kind(),
                    &suggested,
                    [],
                    [],
                );
                mutation.replace_token_transfer_trivia(old_token, new_token);
                applied = true;
            }
        } else if let Some(vue) = state.as_any_vue_directive() {
            // v-directive with static argument: v-bind:foo
            if let Some(directive) = vue.as_vue_directive() {
                if let Some(vue_arg) = directive.arg()
                    && let Some(any_arg) = vue_arg.arg()
                    && let Some(static_arg) = any_arg.as_vue_static_argument()
                    && let Ok(old_token) = static_arg.name_token()
                {
                    let new_token = biome_html_syntax::HtmlSyntaxToken::new_detached(
                        old_token.kind(),
                        &suggested,
                        [],
                        [],
                    );
                    mutation.replace_token_transfer_trivia(old_token, new_token);
                    applied = true;
                }
            // v-bind shorthand: :foo
            } else if let Some(shorthand_bind) = vue.as_vue_v_bind_shorthand_directive()
                && let Ok(vue_arg) = shorthand_bind.arg()
                && let Some(any_arg) = vue_arg.arg()
                && let Some(static_arg) = any_arg.as_vue_static_argument()
                && let Ok(old_token) = static_arg.name_token()
            {
                let new_token = biome_html_syntax::HtmlSyntaxToken::new_detached(
                    old_token.kind(),
                    &suggested,
                    [],
                    [],
                );
                mutation.replace_token_transfer_trivia(old_token, new_token);
                applied = true;
            }
        }

        if !applied {
            return None;
        }

        Some(biome_analyze::RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Rename the attribute to "<Emphasis>{suggested}</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

fn extract_attribute_name(attr: &AnyHtmlAttribute) -> Option<TokenText> {
    // Plain HTML attribute
    if let Some(html_attr) = attr.as_html_attribute()
        && let Ok(name) = html_attr.name()
        && let Ok(token) = name.value_token()
    {
        return Some(token.token_text_trimmed());
    }

    // Vue directives (normal form)
    if let Some(vue) = attr.as_any_vue_directive() {
        if let Some(directive) = vue.as_vue_directive() {
            // only apply to v-bind or v-model directives
            if !(directive.is_binding() || directive.is_two_way_binding()) {
                return None;
            }
            if let Some(vue_arg) = directive.arg()
                && let Some(any_arg) = vue_arg.arg()
                && let Some(static_arg) = any_arg.as_vue_static_argument()
                && let Ok(name_token) = static_arg.name_token()
            {
                return Some(name_token.token_text_trimmed());
            }
            return None;
        }

        // v-bind shorthand (:foo)
        if let Some(shorthand_bind) = vue.as_vue_v_bind_shorthand_directive()
            && let Ok(vue_arg) = shorthand_bind.arg()
        {
            if let Some(any_arg) = vue_arg.arg()
                && let Some(static_arg) = any_arg.as_vue_static_argument()
                && let Ok(name_token) = static_arg.name_token()
            {
                return Some(name_token.token_text_trimmed());
            }
            return None;
        }
    }
    None
}

fn contains_uppercase(name: &str) -> bool {
    // Don't use `Case::identify`: it classifies names by their exact case shape, while the
    // upstream rule's default "always" mode only checks for uppercase letters. Punctuation is
    // therefore neutral rather than evidence that the rule should report an attribute.
    name.chars().any(char::is_uppercase)
}

fn is_svg_element(element: &AnyHtmlTagElement) -> bool {
    element
        .tag_name_kind()
        .is_some_and(|kind| SVG_EXCLUSIVE_TAG_NAMES.contains(kind))
}
