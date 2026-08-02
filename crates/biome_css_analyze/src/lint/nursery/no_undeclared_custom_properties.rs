use crate::services::module_graph::CssModuleGraph;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::{CssFunction, decode_css_identifier, property_syntax::custom_property_name_from_var_function};
use biome_module_graph::{SymbolFromModuleInfo, css_property_definitions};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_undeclared_custom_properties::NoUndeclaredCustomPropertiesOptions;

declare_lint_rule! {
    /// Reports custom properties used with `var()` that have no visible definition.
    ///
    /// This rule checks custom properties defined by declarations such as `--theme: blue`,
    /// registrations such as `@property --theme`, imported stylesheets, linked stylesheets, and
    /// `<style>` blocks in HTML-like files.
    ///
    /// Locally scoped styles in Vue, Svelte, and Astro are visible only within their component.
    /// Global styles can provide custom properties to imported child components.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css
    /// a {
    ///   color: var(--link-color);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
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
        version: "next",
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
        let property = custom_property_name_from_var_function(ctx.query())?;
        let name = property.value_token().ok()?;
        let name = decode_css_identifier(name.text_trimmed());
        let db = ctx.db();
        let module = db.module_for_path(ctx.file_path())?;

        css_property_definitions(
            db,
            SymbolFromModuleInfo::new(db, name.as_ref(), module),
        )
        .is_empty()
        .then(|| property.range())
    }

    fn diagnostic(ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        let property = custom_property_name_from_var_function(ctx.query())?;
        let name = property.value_token().ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *range,
                markup! { "The custom property " <Emphasis>{name.text_trimmed()}</Emphasis> " is not defined." },
            )
            .note(markup! {
                "An undefined custom property uses its fallback when present; without one, the declaration is invalid."
            })
            .note(markup! {
                "Define the custom property in an available stylesheet or correct its name."
            }),
        )
    }
}
