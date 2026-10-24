use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsMemberExpression, AnyPossibleGlobalIdentifier, global_identifier};
use biome_languages::JsFileSource;
use biome_rowan::AstNode;
use biome_rule_options::no_astro_deprecated_resolve::NoAstroDeprecatedResolveOptions;

declare_lint_rule! {
    /// Disallow the deprecated `Astro.resolve()` API.
    ///
    /// `Astro.resolve()` was used to get URLs for files such as images and stylesheets.
    /// It no longer works, because Astro now processes these files when it builds the site.
    ///
    /// Import the file with an `import` statement instead, or put it in the `public/` directory and refer to it with a path that starts with `/`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic
    /// <img src={Astro.resolve("../images/penguin.png")} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro
    /// ---
    /// import penguin from "../images/penguin.png";
    /// ---
    /// <img src={penguin.src} />
    /// <img src="/images/penguin.png" />
    /// ```
    ///
    /// ## References
    ///
    /// - [Astro migration guide: Deprecated `Astro.resolve()`](https://docs.astro.build/en/guides/upgrade-to/v1/#deprecated-astroresolve)
    pub NoAstroDeprecatedResolve {
        version: "next",
        name: "noAstroDeprecatedResolve",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Astro],
        sources: &[RuleSource::EslintAstro("no-deprecated-astro-resolve").same()],
    }
}

impl Rule for NoAstroDeprecatedResolve {
    type Query = Semantic<AnyJsMemberExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoAstroDeprecatedResolveOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_astro()
        {
            return None;
        }

        let member_expression = ctx.query();
        if member_expression.member_name()?.text() != "resolve" {
            return None;
        }

        let object = member_expression.object().ok()?.omit_parentheses();
        let object = AnyPossibleGlobalIdentifier::cast(object.into_syntax())?;
        let (reference, name) = global_identifier(&object)?;
        if name.text() != "Astro" {
            return None;
        }

        ctx.model().binding(&reference).is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    <Emphasis>"Astro.resolve()"</Emphasis>" is deprecated."
                },
            )
            .note(markup! {
                "Astro now processes files such as images and stylesheets when it builds the site, so "<Emphasis>"Astro.resolve()"</Emphasis>" no longer returns URLs that work."
            })
            .note(markup! {
                "Import the file with an "<Emphasis>"import"</Emphasis>" statement, or put it in the "<Emphasis>"public/"</Emphasis>" directory and refer to it with a path that starts with "<Emphasis>"/"</Emphasis>"."
            }),
        )
    }
}
