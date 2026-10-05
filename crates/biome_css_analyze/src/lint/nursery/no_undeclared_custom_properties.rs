use crate::services::module_graph::CssModuleGraph;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::{CssFunction, property_syntax::custom_property_name_from_var_function};
use biome_languages::CssFileSource;
use biome_module_graph::{SymbolFromModuleInfo, css_property_definitions};
use biome_rowan::TextRange;
use biome_rule_options::no_undeclared_custom_properties::NoUndeclaredCustomPropertiesOptions;

declare_lint_rule! {
    /// Reports custom properties used with `var()` that have no visible declaration.
    ///
    /// This rule checks custom properties defined by declarations such as `--theme: blue`,
    /// registrations such as `@property --theme`, imported stylesheets, linked stylesheets, and
    /// `<style>` blocks in HTML-like files. Files that aren't imported by the project
    /// aren't analyzed.
    ///
    /// Locally scoped styles in Vue, Svelte, and Astro are visible only within their component.
    /// Global styles can provide custom properties to imported child components.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,file=invalid.css
    /// a {
    ///   color: var(--link-color);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css,file=valid.css
    /// :root {
    ///   --link-color: blue;
    /// }
    ///
    /// a {
    ///   color: var(--link-color);
    /// }
    /// ```
    ///
    pub NoUndeclaredCustomProperties {
        version: "2.5.11",
        name: "noUndeclaredCustomProperties",
        language: "css",
        recommended: false,
        domains: &[RuleDomain::Project],
    }
}

impl Rule for NoUndeclaredCustomProperties {
    type Query = CssModuleGraph<CssFunction>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoUndeclaredCustomPropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if ctx
            .source_type::<CssFileSource>()
            .as_embedding_kind()
            .is_html_style_attribute()
        {
            return None;
        }
        let property = custom_property_name_from_var_function(ctx.query())?;
        let db = ctx.db();
        let module = db.module_for_path(ctx.file_path())?;

        let is_defined = !css_property_definitions(
            db,
            SymbolFromModuleInfo::new(db, property.text_trimmed(), module),
        )
        .is_empty();
        (!is_defined).then(|| property.text_trimmed_range())
    }

    fn diagnostic(ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        let property = custom_property_name_from_var_function(ctx.query())?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *range,
                markup! { "The custom property " <Emphasis>{property.text_trimmed()}</Emphasis> " is not defined." },
            )
            .note(markup! {
                "An undeclared custom property can indicate a misspelled name or a missing declaration."
            })
            .note(markup! {
                "Define the custom property in an available stylesheet, rename it or remove it."
            }),
        )
    }
}
