use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsBinding, AnyJsBindingPattern, AnyJsExportClause,
    AnyJsExportNamedSpecifier, AnyJsObjectBindingPatternMember, JsExport, JsExportFromClause,
    JsExportNamedClause, JsExportNamedFromClause, JsSyntaxToken, JsVariableDeclaration,
    inner_string_text, unescape_js_string,
};
use biome_languages::JsFileSource;
use biome_rowan::TextRange;
use biome_rule_options::no_astro_prerender_export_outside_pages::NoAstroPrerenderExportOutsidePagesOptions;
use camino::Utf8Path;

declare_lint_rule! {
    /// Reports `prerender` exports in Astro files outside a `pages` directory.
    ///
    /// Astro only uses `export const prerender = true` and `export const prerender = false`
    /// in pages and endpoints. The same exports in components and other files have no effect.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// In `src/components/Card.astro`:
    ///
    /// ```astro,expect_diagnostic,ignore
    /// ---
    /// export const prerender = true;
    /// ---
    /// ```
    ///
    /// ### Valid
    ///
    /// In `src/pages/index.astro`:
    ///
    /// ```astro,ignore
    /// ---
    /// export const prerender = true;
    /// ---
    /// ```
    ///
    /// ## References
    ///
    /// - [Astro on-demand rendering](https://docs.astro.build/en/guides/on-demand-rendering/)
    pub NoAstroPrerenderExportOutsidePages {
        version: "next",
        name: "noAstroPrerenderExportOutsidePages",
        language: "js",
        sources: &[RuleSource::EslintAstro("no-prerender-export-outside-pages").inspired()],
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Astro],
    }
}

impl Rule for NoAstroPrerenderExportOutsidePages {
    type Query = Ast<JsExport>;
    type State = TextRange;
    type Signals = Vec<Self::State>;
    type Options = NoAstroPrerenderExportOutsidePagesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_astro_frontmatter()
            || is_in_pages_directory(ctx.file_path())
        {
            return Vec::new();
        }

        ctx.query()
            .export_clause()
            .map(PrerenderExportCollector::collect)
            .unwrap_or_default()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The "<Emphasis>"prerender"</Emphasis>" export has no effect on Astro rendering outside a pages directory."
                },
            )
            .note(markup! {
                "Astro only applies "<Emphasis>"prerender"</Emphasis>" to page routes and endpoints."
            })
            .note(markup! {
                "Move this export to the relevant route or remove it."
            }),
        )
    }
}

struct PrerenderExportCollector {
    ranges: Vec<TextRange>,
}

impl PrerenderExportCollector {
    fn collect(export_clause: AnyJsExportClause) -> Vec<TextRange> {
        let mut collector = Self { ranges: Vec::new() };

        match export_clause {
            AnyJsExportClause::AnyJsDeclarationClause(declaration_clause) => {
                if let Some(variable_clause) =
                    declaration_clause.as_js_variable_declaration_clause()
                    && let Ok(declaration) = variable_clause.declaration()
                {
                    collector.collect_variable_declaration(&declaration);
                }
            }
            AnyJsExportClause::JsExportFromClause(clause) => {
                collector.collect_export_from_clause(&clause);
            }
            AnyJsExportClause::JsExportNamedClause(clause) => {
                collector.collect_export_named_clause(&clause);
            }
            AnyJsExportClause::JsExportNamedFromClause(clause) => {
                collector.collect_export_named_from_clause(&clause);
            }
            AnyJsExportClause::JsExportDefaultDeclarationClause(_)
            | AnyJsExportClause::JsExportDefaultExpressionClause(_)
            | AnyJsExportClause::TsExportAsNamespaceClause(_)
            | AnyJsExportClause::TsExportAssignmentClause(_)
            | AnyJsExportClause::TsExportDeclareClause(_) => {}
        }

        collector.ranges
    }

    fn collect_variable_declaration(&mut self, declaration: &JsVariableDeclaration) {
        for declarator in declaration.declarators().into_iter().filter_map(Result::ok) {
            let Ok(binding_pattern) = declarator.id() else {
                continue;
            };

            self.collect_binding_pattern(binding_pattern);
        }
    }

    fn collect_binding_pattern(&mut self, binding_pattern: AnyJsBindingPattern) {
        let mut pending = vec![binding_pattern];

        while let Some(binding_pattern) = pending.pop() {
            // Nested bindings are added in reverse source order because `pending` is LIFO.
            match binding_pattern {
                AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(binding)) => {
                    if let Ok(name_token) = binding.name_token() {
                        self.collect_name(&name_token);
                    }
                }
                AnyJsBindingPattern::AnyJsBinding(_) => {}
                AnyJsBindingPattern::JsArrayBindingPattern(pattern) => {
                    pending.extend(
                        pattern
                            .elements()
                            .into_iter()
                            .rev()
                            .filter_map(Result::ok)
                            .filter_map(|element| match element {
                                AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(
                                    element,
                                ) => element.pattern().ok(),
                                AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(
                                    element,
                                ) => element.pattern().ok(),
                                AnyJsArrayBindingPatternElement::JsArrayHole(_) => None,
                            }),
                    );
                }
                AnyJsBindingPattern::JsObjectBindingPattern(pattern) => {
                    pending.extend(
                        pattern
                            .properties()
                            .into_iter()
                            .rev()
                            .filter_map(Result::ok)
                            .filter_map(|property| match property {
                                AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(
                                    property,
                                ) => property.pattern().ok(),
                                AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(
                                    rest,
                                ) => rest.binding().ok().map(AnyJsBindingPattern::AnyJsBinding),
                                AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                                    property,
                                ) => property
                                    .identifier()
                                    .ok()
                                    .map(AnyJsBindingPattern::AnyJsBinding),
                                AnyJsObjectBindingPatternMember::JsBogusBinding(_)
                                | AnyJsObjectBindingPatternMember::JsMetavariable(_) => None,
                            }),
                    );
                }
            }
        }
    }

    fn collect_export_named_clause(&mut self, clause: &JsExportNamedClause) {
        if clause.type_token().is_some() {
            return;
        }

        for specifier in clause.specifiers().into_iter().filter_map(Result::ok) {
            if specifier.type_token().is_some() {
                continue;
            }

            let name_token = match specifier {
                AnyJsExportNamedSpecifier::JsExportNamedShorthandSpecifier(specifier) => {
                    specifier.name().and_then(|name| name.value_token())
                }
                AnyJsExportNamedSpecifier::JsExportNamedSpecifier(specifier) => specifier
                    .exported_name()
                    .and_then(|name| name.value()),
            };

            if let Ok(name_token) = name_token {
                self.collect_name(&name_token);
            }
        }
    }

    fn collect_export_named_from_clause(&mut self, clause: &JsExportNamedFromClause) {
        if clause.type_token().is_some() {
            return;
        }

        for specifier in clause.specifiers().into_iter().filter_map(Result::ok) {
            if specifier.type_token().is_some() {
                continue;
            }

            let exported_name = if let Some(export_as) = specifier.export_as() {
                export_as.exported_name()
            } else {
                specifier.source_name()
            };

            if let Ok(exported_name) = exported_name
                && let Ok(name_token) = exported_name.value()
            {
                self.collect_name(&name_token);
            }
        }
    }

    fn collect_export_from_clause(&mut self, clause: &JsExportFromClause) {
        if clause.type_token().is_some() {
            return;
        }

        if let Some(export_as) = clause.export_as()
            && let Ok(exported_name) = export_as.exported_name()
            && let Ok(name_token) = exported_name.value()
        {
            self.collect_name(&name_token);
        }
    }

    fn collect_name(&mut self, name_token: &JsSyntaxToken) {
        let name = inner_string_text(name_token);
        if name
            .as_bytes()
            .windows(3)
            .any(|window| matches!(window, [b'\\', b'0', digit] if digit.is_ascii_digit()))
        {
            return;
        }

        let name = unescape_js_string(name);
        if name.text() == "prerender" {
            self.ranges.push(name_token.text_trimmed_range());
        }
    }
}

fn is_in_pages_directory(path: &Utf8Path) -> bool {
    path.components()
        .any(|component| component.as_str() == "pages")
}

#[cfg(test)]
mod tests {
    use super::is_in_pages_directory;
    use camino::Utf8Path;

    #[test]
    fn detects_pages_path_component() {
        assert!(is_in_pages_directory(Utf8Path::new(
            "src/pages/index.astro"
        )));
        assert!(is_in_pages_directory(Utf8Path::new(
            "app/routes/pages/index.astro"
        )));
        assert!(is_in_pages_directory(Utf8Path::new(
            "/workspace/custom/pages/index.astro"
        )));
        assert!(!is_in_pages_directory(Utf8Path::new(
            "src/pages-demo/index.astro"
        )));
        assert!(!is_in_pages_directory(Utf8Path::new(
            "src/components/pages.astro"
        )));
    }
}
