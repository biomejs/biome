use std::borrow::Cow;

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{
    AnyAstroDirective, AnyHtmlAttribute, HtmlOpeningElement, T, element_ext::AnyHtmlTagElement,
};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::no_astro_unsafe_inline_scripts::NoAstroUnsafeInlineScriptsOptions;
use biome_string_case::StrLikeExtension;

const DEFAULT_NON_EXECUTING_TYPES: [&str; 2] = ["application/json", "application/ld+json"];

declare_lint_rule! {
    /// Disallows inline Astro scripts unless an explicitly allowed safety condition applies.
    ///
    /// Scripts with a `src` attribute are always allowed. Spread attributes are ignored because they may provide `src` or another allowed attribute at runtime.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic
    /// <script>alert("unsafe")</script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro
    /// <script src="/app.js"></script>
    /// <script type="application/json">{"enabled": true}</script>
    /// <script {...attributes}>alert("unknown")</script>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `allowDefineVars`
    ///
    /// Allows scripts with Astro's `define:vars` directive. Defaults to `false`.
    ///
    /// ```json,options
    /// { "options": { "allowDefineVars": true } }
    /// ```
    ///
    /// ```astro,use_options
    /// <script define:vars={{ value }}>console.log(value)</script>
    /// ```
    ///
    /// ### `allowModuleScripts`
    ///
    /// Allows scripts whose normalized `type` is `module`. Defaults to `false`.
    ///
    /// ```json,options
    /// { "options": { "allowModuleScripts": true } }
    /// ```
    ///
    /// ```astro,use_options
    /// <script type="module">console.log("module")</script>
    /// ```
    ///
    /// ### `allowNonExecutingTypes`
    ///
    /// Lists normalized non-executing MIME types. It defaults to `application/json` and `application/ld+json`; an explicit empty array allows none.
    ///
    /// ```json,options
    /// { "options": { "allowNonExecutingTypes": ["text/plain"] } }
    /// ```
    ///
    /// ```astro,use_options
    /// <script type=" TEXT/PLAIN ; charset=utf-8 ">data</script>
    /// ```
    ///
    /// ### `allowNonce`
    ///
    /// Allows scripts with a `nonce` attribute. Defaults to `false`.
    ///
    /// ```json,options
    /// { "options": { "allowNonce": true } }
    /// ```
    ///
    /// ```astro,use_options
    /// <script nonce={nonce}>console.log("trusted")</script>
    /// ```
    pub NoAstroUnsafeInlineScripts {
        version: "next",
        name: "noAstroUnsafeInlineScripts",
        language: "html",
        sources: &[RuleSource::EslintAstro("no-unsafe-inline-scripts").inspired()],
        recommended: false,
        domains: &[RuleDomain::Astro],
    }
}

impl Rule for NoAstroUnsafeInlineScripts {
    type Query = Ast<HtmlOpeningElement>;
    type State = biome_rowan::TextRange;
    type Signals = Option<Self::State>;
    type Options = NoAstroUnsafeInlineScriptsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_astro() {
            return None;
        }

        let opening = ctx.query();
        if AnyHtmlTagElement::from(opening.clone()).tag_name_kind() != Some(T![script]) {
            return None;
        }

        let attributes = opening.attributes();
        if attributes.find_attribute_by_name("src").is_some()
            || attributes
                .iter()
                .any(|attribute| matches!(attribute, AnyHtmlAttribute::HtmlSpreadAttribute(_)))
        {
            return None;
        }

        let options = ctx.options();
        if options.allow_define_vars.unwrap_or(false)
            && attributes.iter().any(|attribute| {
                let AnyHtmlAttribute::AnyAstroDirective(
                    AnyAstroDirective::AstroDefineDirective(directive),
                ) = attribute
                else {
                    return false;
                };
                directive
                    .value()
                    .and_then(|value| value.name())
                    .and_then(|name| name.value_token())
                    .is_ok_and(|name| name.text_trimmed() == "vars")
            })
        {
            return None;
        }
        if options.allow_nonce.unwrap_or(false)
            && attributes.find_attribute_by_name("nonce").is_some()
        {
            return None;
        }

        if let Some(script_type) = attributes
            .find_attribute_by_name("type")
            .and_then(|attribute| attribute.as_html_attribute().cloned())
            .and_then(|attribute| attribute.as_static_value())
        {
            let decoded_type = htmlize::unescape(script_type.text());
            let normalized_type = normalize_script_type(&decoded_type);
            if options.allow_module_scripts.unwrap_or(false) && normalized_type == "module" {
                return None;
            }

            let is_allowed = options.allow_non_executing_types.as_deref().map_or_else(
                || DEFAULT_NON_EXECUTING_TYPES.contains(&normalized_type.as_ref()),
                |allowed_types| {
                    allowed_types
                        .iter()
                        .any(|allowed| normalize_script_type(allowed) == normalized_type)
                },
            );
            if is_allowed {
                return None;
            }
        }

        Some(opening.range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Avoid inline "<Emphasis>"scripts"</Emphasis>" without a src attribute."
                },
            )
            .note(markup! {
                "An inline "<Emphasis>"script"</Emphasis>" is harder to restrict with a Content Security Policy and cannot be managed as a separate script resource."
            })
            .note(markup! {
                "Move the script to a separate file and reference it with a "<Emphasis>"src"</Emphasis>" attribute, or configure a narrowly scoped allowance."
            }),
        )
    }
}

fn normalize_script_type(value: &str) -> Cow<'_, str> {
    value
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase_cow()
}
