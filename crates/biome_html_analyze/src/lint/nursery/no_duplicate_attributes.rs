use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyHtmlAttribute, AnyVueDirective, HtmlAttributeList};
use biome_rowan::{AstNode, AstNodeList, TextRange, TokenText};
use biome_rule_options::no_duplicate_attributes::NoDuplicateAttributesOptions;
use std::collections::HashSet;

declare_lint_rule! {
    /// Disallow duplication of attributes.
    ///
    /// According to the HTML specification, each attribute name must be unique within a single element.
    /// Duplicate attributes are invalid and can lead to unexpected behavior in browsers.
    ///
    /// ## Vue templates
    ///
    /// For Vue templates (`.vue` files), this rule also considers the following directives as
    /// aliases of their arguments:
    ///
    /// - `v-bind:foo` and `:foo` are handled as the attribute `foo`.
    ///
    /// Vue class/style bindings are ignored. For example, `class` and `:class` may co-exist.
    ///
    /// Event handlers are ignored. For example, `@click` and `v-on:click` are not considered
    /// attributes by this rule.
    ///
    /// Dynamic arguments such as `:[foo]` or `v-bind:[foo]` are ignored.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div foo="a" foo="b"></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div foo :foo="bar" />
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div foo="a" bar="b"></div>
    /// ```
    ///
    pub NoDuplicateAttributes {
        version: "next",
        name: "noDuplicateAttributes",
        language: "html",
        recommended: true,
        sources: &[
            RuleSource::HtmlEslint("no-duplicate-attrs").same(),
            RuleSource::EslintVueJs("no-duplicate-attributes").same()
        ],
    }
}

pub struct State {
    range: TextRange,
    name: TokenText,
    /// Range of the first occurrence of the attribute.
    original_range: TextRange,
}

impl Rule for NoDuplicateAttributes {
    type Query = Ast<HtmlAttributeList>;
    type State = State;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateAttributesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut seen = HashSet::<(TokenText, TextRange)>::new();
        let mut violations = Vec::new();

        for attribute in node.iter() {
            let Some(key) = attribute_key(&attribute) else {
                continue;
            };

            if let Some((_, original_range)) = seen.iter().find(|(tt, _)| tt == &key.0) {
                violations.push(State {
                    range: attribute.range(),
                    name: key.0.clone(),
                    original_range: *original_range,
                });
            } else {
                seen.insert(key);
            }
        }

        violations.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = state.name.text();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "Duplicate attribute '"<Emphasis>{name}</Emphasis>"'."
                },
            )
            .detail(state.original_range, "This is the first occurrence of the attribute.")
            .note("Each attribute name must be unique within a single element. Duplicate attributes are invalid and can lead to unexpected browser behavior.").note(
                markup! {
                    "Consider removing or renaming the duplicate '"<Emphasis>{name}</Emphasis>"' attribute."
                },
            ),
        )
    }
}

fn attribute_key(attribute: &AnyHtmlAttribute) -> Option<(TokenText, TextRange)> {
    // Plain HTML attribute (eg. `foo`)
    if let Some(html_attr) = attribute.as_html_attribute()
        && let Ok(name) = html_attr.name()
        && let Ok(token) = name.value_token()
    {
        return Some((token.token_text_trimmed(), token.text_trimmed_range()));
    }

    // Vue directives (`.vue` files only)
    let vue = attribute.as_any_vue_directive()?;

    match vue {
        // Longhand directive: v-bind:foo
        AnyVueDirective::VueDirective(directive) => {
            let name_token = directive.name_token().ok()?;
            let name = name_token.text_trimmed();
            if name != "v-bind" {
                return None;
            }

            let argument = directive.arg()?;
            let argument = argument.arg().ok()?;
            let static_argument = argument.as_vue_static_argument()?;
            let name_token = static_argument.name_token().ok()?;

            let key = name_token.token_text_trimmed();
            if key.text() == "class" || key.text() == "style" {
                return None;
            }

            Some((key, name_token.text_trimmed_range()))
        }

        // Shorthand bind: :foo
        AnyVueDirective::VueVBindShorthandDirective(directive) => {
            let argument = directive.arg().ok()?;
            let argument = argument.arg().ok()?;
            let static_argument = argument.as_vue_static_argument()?;
            let name_token = static_argument.name_token().ok()?;

            let key = name_token.token_text_trimmed();
            if key.text() == "class" || key.text() == "style" {
                return None;
            }

            Some((key, name_token.text_trimmed_range()))
        }

        // Ignore all v-on and shorthand @event handlers.
        AnyVueDirective::VueVOnShorthandDirective(_) => None,

        _ => None,
    }
}
