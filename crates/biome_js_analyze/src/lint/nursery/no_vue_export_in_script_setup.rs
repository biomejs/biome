use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsDeclarationClause, AnyJsExportClause, JsExport, JsModule, JsModuleItemList,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::no_vue_export_in_script_setup::NoVueExportInScriptSetupOptions;

declare_lint_rule! {
    /// Disallow `export` in `<script setup>`.
    ///
    /// Everything declared at the top level of `<script setup>` is already available to the
    /// component's template, so there is nothing to export. Vue fails to compile a component
    /// whose `<script setup>` block exports a value.
    ///
    /// To share values with other modules, export them from a separate, regular `<script>` block
    /// in the same file. To expose values to a parent component, use `defineExpose()`.
    ///
    /// TypeScript type exports, such as `export type` and `export interface`, are allowed,
    /// because they are removed when the code is compiled. TypeScript's `export =` and
    /// `export as namespace` forms are also allowed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// export const message = "Hello";
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// export default {};
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// const count = 1;
    /// export { count };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script lang="ts">
    /// export const message = "Hello";
    /// </script>
    /// <script setup lang="ts">
    /// export type Size = "small" | "large";
    /// export interface Props {
    ///   size: Size;
    /// }
    ///
    /// const count = 1;
    /// defineExpose({ count });
    /// </script>
    /// ```
    ///
    pub NoVueExportInScriptSetup {
        version: "next",
        name: "noVueExportInScriptSetup",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-export-in-script-setup").same()],
    }
}

impl Rule for NoVueExportInScriptSetup {
    type Query = Ast<JsExport>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueExportInScriptSetupOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_vue_setup()
        {
            return None;
        }

        let export = ctx.query();
        // Exports inside a TypeScript namespace or module declaration are not module exports.
        let is_top_level = export
            .parent::<JsModuleItemList>()
            .and_then(|list| list.parent::<JsModule>())
            .is_some();
        if !is_top_level || is_allowed_export(&export.export_clause().ok()?) {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let export = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                diagnostic_range(export)?,
                markup! {
                    <Emphasis>"<script setup>"</Emphasis>" can't contain exports."
                },
            )
            .note(markup! {
                "Vue fails to compile a component when its "<Emphasis>"<script setup>"</Emphasis>" block exports a value. Top-level declarations are already available to the template."
            })
            .note(markup! {
                "Remove the "<Emphasis>"export"</Emphasis>". To share the value with other modules, move the export to a regular "<Emphasis>"<script>"</Emphasis>" block. To expose it to a parent component, use "<Emphasis>"defineExpose()"</Emphasis>"."
            }),
        )
    }
}

/// Returns `true` when Vue accepts `clause` in `<script setup>`.
///
/// Vue accepts exports of TypeScript types, which are removed at compile time, and the
/// TypeScript-specific `export =` and `export as namespace` forms. An empty `export {}` is not
/// type-only, and neither is `export default interface`.
fn is_allowed_export(clause: &AnyJsExportClause) -> bool {
    match clause {
        AnyJsExportClause::AnyJsDeclarationClause(declaration) => matches!(
            declaration,
            AnyJsDeclarationClause::TsTypeAliasDeclaration(_)
                | AnyJsDeclarationClause::TsInterfaceDeclaration(_)
        ),
        AnyJsExportClause::JsExportNamedClause(clause) => {
            clause.type_token().is_some() || {
                let mut specifiers = clause.specifiers().iter();
                specifiers.len() > 0
                    && specifiers.all(|specifier| {
                        specifier.is_ok_and(|specifier| specifier.type_token().is_some())
                    })
            }
        }
        AnyJsExportClause::JsExportFromClause(clause) => clause
            .source()
            .ok()
            .and_then(|source| source.as_js_module_source().cloned())
            .is_some_and(|source| source.imports_only_types()),
        AnyJsExportClause::JsExportNamedFromClause(clause) => clause
            .source()
            .ok()
            .and_then(|source| source.as_js_module_source().cloned())
            .is_some_and(|source| source.imports_only_types()),
        // `export declare ...` only describes values that exist elsewhere, so it produces no code.
        AnyJsExportClause::TsExportDeclareClause(_) => true,
        AnyJsExportClause::TsExportAsNamespaceClause(_)
        | AnyJsExportClause::TsExportAssignmentClause(_) => true,
        AnyJsExportClause::JsExportDefaultDeclarationClause(_)
        | AnyJsExportClause::JsExportDefaultExpressionClause(_) => false,
    }
}

/// Returns the range of the `export` keyword, followed by `default` when present, for exports
/// that wrap a declaration. Returns the range of the whole export otherwise.
///
/// Declarations such as functions and classes can span many lines, so only the keywords are
/// highlighted for them.
fn diagnostic_range(export: &JsExport) -> Option<TextRange> {
    let export_token = export.export_token().ok()?;
    match export.export_clause().ok()? {
        AnyJsExportClause::AnyJsDeclarationClause(_) => Some(export_token.text_trimmed_range()),
        AnyJsExportClause::JsExportDefaultDeclarationClause(clause) => Some(TextRange::new(
            export_token.text_trimmed_range().start(),
            clause.default_token().ok()?.text_trimmed_range().end(),
        )),
        _ => Some(export.range()),
    }
}
