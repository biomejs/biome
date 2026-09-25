use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_html_syntax::static_value::StaticValue;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyHtmlAttributeInitializer, AnySvelteTemplateElement, HtmlAttribute,
    HtmlTextExpression, T,
};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::use_svelte_kit_resolve::UseSvelteKitResolveOptions;
use biome_unicode_table::{Dispatch, lookup_byte};

declare_lint_rule! {
    /// Require internal navigation in SvelteKit apps to use paths built with `resolve()`.
    ///
    /// SvelteKit's `resolve()` from `$app/paths` prefixes a route with the app's
    /// [base path](https://svelte.dev/docs/kit/configuration#paths) and checks it against the
    /// app's routes. A hand-written `href` breaks navigation when the app is served from a
    /// sub-path, and lets typos in routes go unnoticed.
    ///
    /// Links are accepted when their `href` is an absolute URL, such as `https://svelte.dev`
    /// or `mailto:hello@example.com`, a fragment such as `#top`, or when the `<a>` element has
    /// `rel="external"`.
    ///
    /// This rule only checks `href` values it can read statically. Expressions such as
    /// `href={url}` aren't checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// <a href="/foo">Click me!</a>
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <a href={"/foo"}>Click me!</a>
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <a href="/posts/{id}">Click me!</a>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// <a href={resolve("/foo")}>Click me!</a>
    /// <a href="https://svelte.dev">Click me!</a>
    /// <a href="/foo" rel="external">Click me!</a>
    /// <a href="#top">Click me!</a>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `ignoreLinks`
    ///
    /// Whether to ignore all `<a>` elements. Default: `false`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreLinks": true
    ///     }
    /// }
    /// ```
    ///
    /// ```svelte,use_options
    /// <a href="/foo">Click me!</a>
    /// ```
    ///
    /// ## References
    ///
    /// - [`resolve()` documentation](https://svelte.dev/docs/kit/$app-paths#resolve)
    pub UseSvelteKitResolve {
        version: "next",
        name: "useSvelteKitResolve",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("no-navigation-without-resolve").same()],
    }
}

impl Rule for UseSvelteKitResolve {
    type Query = Ast<AnyHtmlTagElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseSvelteKitResolveOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if ctx.options().ignore_links() || !ctx.source_type::<HtmlFileSource>().is_svelte() {
            return None;
        }

        let element = ctx.query();
        if element.tag_name_kind() != Some(T![a]) {
            return None;
        }

        let mut href = None;
        for attribute in element.attributes() {
            match attribute {
                AnyHtmlAttribute::HtmlAttribute(attribute) => {
                    let name = attribute.name().ok()?.value_token().ok()?;
                    let name = name.text_trimmed();
                    if name.eq_ignore_ascii_case("href") {
                        href = Some(attribute);
                    } else if name.eq_ignore_ascii_case("rel") && may_be_external(&attribute) {
                        return None;
                    }
                }
                // `{rel}` is dynamic, so it may contain `external`.
                AnyHtmlAttribute::HtmlAttributeSingleTextExpression(shorthand)
                    if shorthand
                        .expression()
                        .ok()
                        .and_then(|expression| expression.string_value())
                        .is_some_and(|name| name.eq_ignore_ascii_case("rel")) =>
                {
                    return None;
                }
                _ => {}
            }
        }

        let href = href?;
        let is_internal = match href.initializer()?.value().ok()? {
            AnyHtmlAttributeInitializer::HtmlString(string) => {
                is_internal_link(string.inner_string_text().ok()?.text())
            }
            AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(expression) => {
                is_internal_link_expression(&expression.expression().ok()?)
            }
            // Only the start of the value decides whether the link is internal.
            AnyHtmlAttributeInitializer::SvelteTemplateAttributeValue(value) => {
                match value.elements().first()? {
                    AnySvelteTemplateElement::SvelteTemplateChunkElement(chunk) => {
                        is_internal_link(chunk.html_template_chunk_token().ok()?.text_trimmed())
                    }
                    AnySvelteTemplateElement::HtmlAttributeSingleTextExpression(expression) => {
                        is_internal_link_expression(&expression.expression().ok()?)
                    }
                }
            }
            AnyHtmlAttributeInitializer::VueVForValue(_) => false,
        };

        is_internal.then(|| href.range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "This link's "<Emphasis>"href"</Emphasis>" isn't built with "<Emphasis>"resolve()"</Emphasis>"."
                },
            )
            .note(markup! {
                "Paths that aren't built with "<Emphasis>"resolve()"</Emphasis>" lack the app's base path, and SvelteKit can't check them against the app's routes."
            })
            .note(markup! {
                "Wrap the path in "<Emphasis>"resolve()"</Emphasis>" from "<Emphasis>"$app/paths"</Emphasis>". If the link leaves the app, add "<Emphasis>"rel=\"external\""</Emphasis>"."
            }),
        )
    }
}

/// Returns `true` if the `rel` attribute may contain the `external` link type.
fn may_be_external(rel: &HtmlAttribute) -> bool {
    let Some(Ok(value)) = rel.initializer().map(|initializer| initializer.value()) else {
        return false;
    };
    match value.as_static_value() {
        Some(value @ StaticValue::String(_)) => value
            .text()
            .split_ascii_whitespace()
            .any(|link_type| link_type == "external"),
        Some(_) => false,
        // A dynamic value may contain `external`.
        None => true,
    }
}

/// Returns `true` if the expression is a string literal with an internal path.
///
/// Other expressions aren't checked, since their value is unknown.
fn is_internal_link_expression(expression: &HtmlTextExpression) -> bool {
    match expression.as_static_value() {
        Some(value @ StaticValue::String(_)) => is_internal_link(value.text()),
        _ => false,
    }
}

/// Returns `true` if `href` is neither a fragment nor an absolute URL.
fn is_internal_link(href: &str) -> bool {
    !href.starts_with('#') && !has_url_scheme(href)
}

/// Returns `true` if `href` starts with a URL scheme, such as `https:` or `mailto:`.
fn has_url_scheme(href: &str) -> bool {
    let is_scheme_byte =
        |byte: u8| byte == b'+' || (byte != b'_' && lookup_byte(byte) == Dispatch::IDT);
    href.bytes()
        .find(|byte| !is_scheme_byte(*byte))
        .is_some_and(|byte| byte == b':')
}
