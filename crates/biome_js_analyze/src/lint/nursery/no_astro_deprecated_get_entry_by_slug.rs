use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsImportClause, JsExportNamedFromClause, JsExportNamedFromSpecifier,
    JsExportNamedFromSpecifierList, JsModuleSource, JsNamedImportSpecifier,
    JsNamedImportSpecifierList, JsNamedImportSpecifiers, JsShorthandNamedImportSpecifier,
};
use biome_rowan::{AstNode, TokenText, declare_node_union};
use biome_rule_options::no_astro_deprecated_get_entry_by_slug::NoAstroDeprecatedGetEntryBySlugOptions;

declare_lint_rule! {
    /// Disallow importing the deprecated `getEntryBySlug()` function from `astro:content`.
    ///
    /// Astro deprecated `getEntryBySlug()` in favor of `getEntry()`, and Astro 6 removed it.
    /// Calling it in Astro 6 throws an error.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { getEntryBySlug } from "astro:content";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { getEntry } from "astro:content";
    /// ```
    ///
    /// ## References
    ///
    /// - [Astro 6 upgrade guide](https://docs.astro.build/en/guides/upgrade-to/v6/)
    /// - [`getEntry()` API reference](https://docs.astro.build/en/reference/modules/astro-content/#getentry)
    ///
    pub NoAstroDeprecatedGetEntryBySlug {
        version: "next",
        name: "noAstroDeprecatedGetEntryBySlug",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Astro],
        sources: &[RuleSource::EslintAstro("no-deprecated-getentrybyslug").inspired()],
    }
}

declare_node_union! {
    pub AnyJsGetEntryBySlugSpecifier =
        JsNamedImportSpecifier
        | JsShorthandNamedImportSpecifier
        | JsExportNamedFromSpecifier
}

impl Rule for NoAstroDeprecatedGetEntryBySlug {
    type Query = Ast<AnyJsGetEntryBySlugSpecifier>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoAstroDeprecatedGetEntryBySlugOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let specifier = ctx.query();
        if specifier.imported_name()?.text() != "getEntryBySlug" {
            return None;
        }

        let source = specifier.module_source()?;
        (source.inner_string_text().ok()?.text() == "astro:content").then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    <Emphasis>"getEntryBySlug()"</Emphasis>" is deprecated."
                },
            )
            .note(markup! {
                "Astro 6 removed this function, and calling it throws an error."
            })
            .note(markup! {
                "Use "<Emphasis>"getEntry()"</Emphasis>" instead."
            }),
        )
    }
}

impl AnyJsGetEntryBySlugSpecifier {
    /// The name this specifier imports or re-exports, without quotes.
    fn imported_name(&self) -> Option<TokenText> {
        match self {
            Self::JsNamedImportSpecifier(specifier) => {
                specifier.name().ok()?.inner_string_text().ok()
            }
            Self::JsShorthandNamedImportSpecifier(specifier) => Some(
                specifier
                    .local_name()
                    .ok()?
                    .as_js_identifier_binding()?
                    .name_token()
                    .ok()?
                    .token_text_trimmed(),
            ),
            Self::JsExportNamedFromSpecifier(specifier) => {
                specifier.source_name().ok()?.inner_string_text().ok()
            }
        }
    }

    /// The module that the enclosing `import` or `export … from` statement reads from.
    fn module_source(&self) -> Option<JsModuleSource> {
        match self {
            Self::JsExportNamedFromSpecifier(specifier) => specifier
                .parent::<JsExportNamedFromSpecifierList>()?
                .parent::<JsExportNamedFromClause>()?
                .source()
                .ok()?
                .as_js_module_source()
                .cloned(),
            _ => self
                .parent::<JsNamedImportSpecifierList>()?
                .parent::<JsNamedImportSpecifiers>()?
                .parent::<AnyJsImportClause>()?
                .source()
                .ok(),
        }
    }
}
