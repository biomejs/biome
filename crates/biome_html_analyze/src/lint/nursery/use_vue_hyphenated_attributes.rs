use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyHtmlTagName, HtmlAttributeList, HtmlOpeningElement, HtmlSelfClosingElement,
};
use biome_rowan::{AstNode, AstNodeList, SyntaxResult, TokenText, declare_node_union};
use biome_rule_options::use_vue_hyphenated_attributes::UseVueHyphenatedAttributesOptions;
use biome_string_case::Case;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Enforce hyphenated (kebab-case) attribute names in Vue templates.
    ///
    /// Vue style guide recommends using hyphenated attribute (and prop) names in templates to
    /// keep them consistent and distinguish them from JavaScript identifiers written in camelCase/PascalCase.
    ///
    /// This rule flags attributes that are detected as camelCase, PascalCase, CONSTANT_CASE, snake_case
    /// or that contain any uppercase ASCII letter. It uses Biome's internal `Case::identify` helper.
    ///
    /// Allowed:
    /// - kebab-case attributes (e.g. `data-test-id`)
    /// - pure lowercase single word attributes (e.g. `class`, `id`)
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
    /// ```
    ///
    /// ## Options
    ///
    /// The rule supports the following options:
    ///
    /// ### `ignore`
    ///
    /// A list of attribute names that should be ignored by the rule (they won't be required to be hyphenated).
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
    /// A list of tag names whose attributes should be skipped entirely.
    /// This is useful for third-party or internal components that deliberately expose non‑hyphenated prop names.
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

declare_node_union! {
    pub AnyTagWithAttributes = HtmlOpeningElement | HtmlSelfClosingElement
}

impl Rule for UseVueHyphenatedAttributes {
    type Query = Ast<AnyTagWithAttributes>;
    type State = AnyHtmlAttribute;
    type Signals = Box<[Self::State]>;
    type Options = UseVueHyphenatedAttributesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

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
            if !is_hyphenated(attr_name.text()) {
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
        let suggested = Case::Kebab.convert(name.text());

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
                    && let Ok(any_arg) = vue_arg.arg()
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
                && let Ok(any_arg) = vue_arg.arg()
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

impl AnyTagWithAttributes {
    fn name(&self) -> SyntaxResult<AnyHtmlTagName> {
        match self {
            Self::HtmlOpeningElement(node) => node.name(),
            Self::HtmlSelfClosingElement(node) => node.name(),
        }
    }

    fn attributes(&self) -> HtmlAttributeList {
        match self {
            Self::HtmlOpeningElement(node) => node.attributes(),
            Self::HtmlSelfClosingElement(node) => node.attributes(),
        }
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
            if let Ok(name) = directive.name_token().map(|name| name.token_text_trimmed())
                && !(name == "v-bind" || name == "v-model")
            {
                return None;
            }
            if let Some(vue_arg) = directive.arg()
                && let Ok(any_arg) = vue_arg.arg()
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
            if let Ok(any_arg) = vue_arg.arg()
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

fn is_hyphenated(name: &str) -> bool {
    // Treat pure lowercase and kebab-case as valid.
    matches!(Case::identify(name, true), Case::Kebab | Case::Lower)
}
