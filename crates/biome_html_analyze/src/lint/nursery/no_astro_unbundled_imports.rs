use crate::services::embedded::EmbeddedService;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyAstroDirective, AnyHtmlAttribute, HtmlAttributeInitializerClause, HtmlElement,
    HtmlOpeningElement, ScriptType,
};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_astro_unbundled_imports::NoAstroUnbundledImportsOptions;

declare_lint_rule! {
    /// Disallow static imports in Astro scripts that are neither bundled nor modules.
    ///
    /// Astro bundles a `<script>` with no attributes and emits it as a module.
    /// Adding `is:inline`, `define:vars`, or another attribute disables this processing.
    /// Without `type="module"`, the browser treats the resulting script as a classic
    /// script, where static import declarations are syntax errors.
    /// Type-only imports are also reported because unbundled scripts are not compiled.
    ///
    /// Move imports and the code that uses them to a script that Astro bundles.
    /// When using `define:vars`, pass server values through HTML data attributes so
    /// the bundled script can read them.
    ///
    /// This rule checks scripts with no `type` or with a JavaScript MIME type
    /// supported by Biome. It skips scripts with `src`, spread attributes, `hoist`,
    /// `set:html`, `set:text`, incomplete attributes, or an unknown or dynamic `type`.
    /// It also skips `is:raw` scripts, whose contents Biome leaves unparsed.
    /// Frontmatter and dynamic `import()` expressions are not reported.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic
    /// <script define:vars={{ message: "Hello" }}>
    ///   import { greet } from "./greet.js";
    ///   greet(message);
    /// </script>
    /// ```
    ///
    /// ```astro,expect_diagnostic
    /// <script is:inline>
    ///   import "./setup.js";
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro
    /// <script>
    ///   import { greet } from "./greet.js";
    ///   greet("Hello");
    /// </script>
    /// <script is:inline type="module">
    ///   import { greet } from "/greet.js";
    ///   greet("Hello");
    /// </script>
    /// <script is:inline>
    ///   import("/setup.js");
    /// </script>
    /// ```
    pub NoAstroUnbundledImports {
        version: "next",
        name: "noAstroUnbundledImports",
        language: "html",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Astro],
    }
}

pub struct RuleState {
    import: TextRange,
    attribute: TextRange,
}

impl Rule for NoAstroUnbundledImports {
    type Query = Ast<HtmlElement>;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = NoAstroUnbundledImportsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_astro() {
            return Box::default();
        }
        let element = ctx.query();
        if !element.is_script_tag() {
            return Box::default();
        }
        let Ok(opening) = element.opening_element() else {
            return Box::default();
        };
        let Some(attribute) = unbundling_attribute(&opening) else {
            return Box::default();
        };
        let Ok(closing) = element.closing_element() else {
            return Box::default();
        };
        let Some(service) = ctx.get_service::<EmbeddedService>() else {
            return Box::default();
        };
        let body = TextRange::new(opening.range().end(), closing.range().start());
        service
            .static_imports(body)
            .iter()
            .map(|import| RuleState {
                import: *import,
                attribute,
            })
            .collect()
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        Some(ctx.query().range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.import,
            markup! { "This static import is in an unbundled, non-module script." },
        )
        .detail(state.attribute, markup! {
            "This attribute prevents Astro from bundling the script."
        })
        .note(markup! {
            "Browsers cannot use static imports in classic scripts. Astro leaves this script uncompiled."
        })
        .note(markup! {
            "Move the import and the code that uses it to a "<Emphasis>"<script>"</Emphasis>" with no attributes so Astro can bundle it."
        }))
    }
}

fn unbundling_attribute(opening: &HtmlOpeningElement) -> Option<TextRange> {
    let mut cause = None;
    for attribute in opening.attributes() {
        if attribute.syntax().has_skipped_descendants() {
            return None;
        }
        match &attribute {
            AnyHtmlAttribute::HtmlAttribute(attribute) => {
                let name = attribute.name().ok()?.value_token().ok()?;
                let name = name.text_trimmed();
                if name.eq_ignore_ascii_case("src") || name.eq_ignore_ascii_case("hoist") {
                    return None;
                }
                if let Some(initializer) = attribute.initializer() {
                    check_initializer(&initializer)?;
                }
                if name.eq_ignore_ascii_case("type") {
                    let value = attribute.initializer()?.value().ok()?;
                    let value = value.as_html_string()?.inner_string_text().ok()?;
                    if ScriptType::from_type_value(value.text()) != ScriptType::Classic {
                        return None;
                    }
                }
            }
            AnyHtmlAttribute::AnyAstroDirective(directive) => {
                let value = match directive {
                    AnyAstroDirective::AstroSetDirective(_) => return None,
                    AnyAstroDirective::AstroIsDirective(directive) => directive.value().ok()?,
                    AnyAstroDirective::AstroDefineDirective(directive) => directive.value().ok()?,
                    AnyAstroDirective::AstroClassDirective(directive) => directive.value().ok()?,
                    AnyAstroDirective::AstroClientDirective(directive) => directive.value().ok()?,
                    AnyAstroDirective::AstroServerDirective(directive) => directive.value().ok()?,
                };
                value.colon_token_token().ok()?;
                value.name().ok()?.value_token().ok()?;
                if let Some(initializer) = value.initializer() {
                    check_initializer(&initializer)?;
                }
            }
            _ => return None,
        }
        cause.get_or_insert(attribute.range());
    }
    cause
}

fn check_initializer(initializer: &HtmlAttributeInitializerClause) -> Option<()> {
    let value = initializer.value().ok()?;
    if let Some(expression) = value.as_html_attribute_single_text_expression() {
        expression.r_curly_token().ok()?;
        let token = expression.expression().ok()?.html_literal_token().ok()?;
        if token.text_trimmed().is_empty() {
            return None;
        }
    }
    Some(())
}
