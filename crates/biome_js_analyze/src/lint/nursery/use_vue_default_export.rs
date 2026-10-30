use crate::services::vue::VueSemantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExportClause, AnyJsExportDefaultDeclaration, AnyJsModuleItem, JsModule};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, TextRange};
use biome_rule_options::use_vue_default_export::UseVueDefaultExportOptions;
use biome_vue_semantic::BlockKind;

declare_lint_rule! {
    /// Require the `<script>` block of a Vue component to have a default export.
    ///
    /// When a `.vue` file has no `<script setup>` block, Vue builds the component from the
    /// default export of its `<script>` block. If the block has no default export, the
    /// component can't use anything the block declares, such as data, props, or methods.
    ///
    /// A file with a `<script setup>` block is not checked, because Vue builds the component
    /// from that block instead.
    ///
    /// :::note
    /// This rule needs to read every block of a `.vue` file, which requires
    /// [`html.experimentalFullSupportEnabled`](https://biomejs.dev/reference/configuration/#htmlexperimentalfullsupportenabled)
    /// to be `true`. Without that setting, the rule reports nothing.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <script>
    /// const foo = "foo";
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <script>
    /// import { defineComponent } from "vue";
    ///
    /// export const component = defineComponent({});
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   data() {
    ///     return {
    ///       foo: "foo",
    ///     };
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script>
    /// export const foo = "foo";
    /// </script>
    ///
    /// <script setup>
    /// const bar = "bar";
    /// </script>
    /// ```
    ///
    pub UseVueDefaultExport {
        version: "next",
        name: "useVueDefaultExport",
        language: "js",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("require-default-export").same()],
    }
}

impl Rule for UseVueDefaultExport {
    type Query = VueSemantic<JsModule>;
    /// The range of a component that the block defines without exporting it as the default
    /// export, if there is one.
    type State = Option<TextRange>;
    type Signals = Option<Self::State>;
    type Options = UseVueDefaultExportOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source = ctx.source_type::<JsFileSource>();
        let embedding = source.as_embedding_kind();
        if !embedding.is_vue() || embedding.is_vue_setup() || !source.is_embedded_source() {
            return None;
        }
        // Without the rest of the file, a `<script setup>` block next to this one can't be
        // ruled out.
        let snippet = ctx.snippet()?;
        let model = ctx.model();
        if model
            .blocks()
            .any(|block| block.kind() == BlockKind::Script && block.is_setup())
        {
            return None;
        }

        let module = ctx.query();
        if module.items().is_empty() && module.directives().is_empty() {
            return None;
        }
        if module.items().iter().any(|item| is_default_export(&item)) {
            return None;
        }

        let component = model
            .components()
            .find(|component| component.snippet().as_ref() == Some(&snippet));
        Some(component.map(|component| component.range()))
    }

    fn diagnostic(ctx: &RuleContext<Self>, component: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                component.unwrap_or_else(|| ctx.query().range()),
                if component.is_some() {
                    markup! {
                        "This component is not the default export of the "<Emphasis>"<script>"</Emphasis>" block."
                    }
                } else {
                    markup! {
                        "This "<Emphasis>"<script>"</Emphasis>" block has no default export."
                    }
                },
            )
            .note(markup! {
                "Vue builds the component from the default export of the "<Emphasis>"<script>"</Emphasis>" block. Without one, the component can't use anything this block declares."
            })
            .note(markup! {
                "Export the component with "<Emphasis>"export default"</Emphasis>", or move the code to a "<Emphasis>"<script setup>"</Emphasis>" block."
            }),
        )
    }
}

/// Returns `true` when `item` exports a value as the default export, either with
/// `export default ...` or by exporting a name as `default`, as in `export { foo as default }`.
///
/// Type-only exports, such as `export type { Props as default }` or
/// `export default interface Props {}`, return `false`, because TypeScript removes them.
fn is_default_export(item: &AnyJsModuleItem) -> bool {
    let AnyJsModuleItem::JsExport(export) = item else {
        return false;
    };
    match export.export_clause() {
        Ok(AnyJsExportClause::JsExportDefaultExpressionClause(_)) => true,
        Ok(AnyJsExportClause::JsExportDefaultDeclarationClause(clause)) => matches!(
            clause.declaration(),
            Ok(AnyJsExportDefaultDeclaration::JsClassExportDefaultDeclaration(_)
                | AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(_))
        ),
        Ok(AnyJsExportClause::JsExportNamedClause(clause)) => {
            clause.type_token().is_none()
                && clause.specifiers().iter().flatten().any(|specifier| {
                    specifier
                        .as_js_export_named_specifier()
                        .filter(|specifier| specifier.type_token().is_none())
                        .and_then(|specifier| specifier.exported_name().ok())
                        .is_some_and(|name| name.is_default())
                })
        }
        Ok(AnyJsExportClause::JsExportNamedFromClause(clause)) => {
            clause.type_token().is_none()
                && clause.specifiers().iter().flatten().any(|specifier| {
                    specifier.type_token().is_none()
                        && match specifier.export_as() {
                            Some(export_as) => export_as
                                .exported_name()
                                .is_ok_and(|name| name.is_default()),
                            None => specifier
                                .source_name()
                                .is_ok_and(|name| name.is_default()),
                        }
                })
        }
        _ => false,
    }
}
