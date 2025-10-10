use crate::react::components::ReactComponentInfo;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsModuleItem, AnyJsStatement, JsModule, export_ext::AnyJsExported};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_component_export_only_modules::UseComponentExportOnlyModulesOptions;
use rustc_hash::FxHashMap;

declare_lint_rule! {
    /// Enforce declaring components only within modules that export React Components exclusively.
    ///
    /// This is necessary to enable the [`React Fast Refresh`] feature, which improves development efficiency.
    /// The determination of whether something is a component depends on naming conventions.
    /// Components should be written in [`PascalCase`] and regular functions in [`camelCase`].
    /// If the framework already has established conventions, consider optionally specifying exceptions.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// export const foo = () => {};
    /// export const Bar = () => <></>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// const Tab = () => {};
    /// export const tabs = [<Tab />, <Tab />];
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// const App = () => {}
    /// createRoot(document.getElementById("root")).render(<App />);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// export default function Foo() {
    ///     return <></>;
    /// }
    /// ```
    ///
    /// ```jsx
    /// const foo = () => {};
    /// export const Bar = () => <></>;
    /// ```
    ///
    /// ```jsx
    /// import { App } from "./App";
    /// createRoot(document.getElementById("root")).render(<App />);
    /// ```
    ///
    /// Functions that return standard React components are also permitted.
    ///
    /// ```jsx
    /// import { memo } from 'react';
    /// const Component = () => <></>
    /// export default memo(Component);
    /// ```
    ///
    /// ## Options
    ///
    /// ### `allowConstantExport`
    ///
    /// Some tools, such as [Vite], allow exporting constants along with components. By enabling the following, the rule will support the pattern.
    ///
    /// ```json,options
    /// {
    ///     "options":{
    ///         "allowConstantExport" : true
    ///     }
    /// }
    /// ```
    ///
    /// ### `allowExportNames`
    ///
    /// If you use a framework that handles [Hot Module Replacement(HMR)] of some specific exports, you can use this option to avoid warning for them.
    ///
    /// Example for [Remix](https://remix.run/docs/en/main/discussion/hot-module-replacement#supported-exports):
    /// ```json,options
    /// {
    ///     "options":{
    ///         "allowExportNames": ["json", "loader", "headers", "meta", "links", "scripts"]
    ///     }
    /// }
    /// ```
    ///
    /// [`meta` in Remix]: https://remix.run/docs/en/main/route/meta
    /// [Hot Module Replacement(HMR)]: https://remix.run/docs/en/main/discussion/hot-module-replacement
    /// [`React Fast Refresh`]: https://github.com/facebook/react/tree/main/packages/react-refresh
    /// [Remix]: https://remix.run/
    /// [Vite]: https://vitejs.dev/
    /// [`camelCase`]: https://en.wikipedia.org/wiki/Camel_case
    /// [`PascalCase`]: https://en.wikipedia.org/wiki/Camel_case
    pub UseComponentExportOnlyModules {
        version: "1.9.2",
        name: "useComponentExportOnlyModules",
        language: "jsx",
        sources: &[RuleSource::EslintReactRefresh("only-export-components").inspired()],
        domains: &[RuleDomain::React],
        recommended: false,
        severity: Severity::Warning,
    }
}
enum ErrorType {
    ExportedNonComponentWithComponent,
    UnexportedComponent,
    NoExport,
}

pub struct UseComponentExportOnlyModulesState {
    error: ErrorType,
    range: TextRange,
}

const JSX_FILE_EXT: [&str; 2] = [".jsx", ".tsx"];

impl Rule for UseComponentExportOnlyModules {
    type Query = Ast<JsModule>;
    type State = UseComponentExportOnlyModulesState;
    type Signals = Box<[Self::State]>;
    type Options = UseComponentExportOnlyModulesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if let Some(file_name) = ctx.file_path().file_name()
            && !JSX_FILE_EXT.iter().any(|ext| file_name.ends_with(ext))
        {
            return Vec::new().into_boxed_slice();
        }
        let root = ctx.query();
        let mut local_components: FxHashMap<Box<str>, TextRange> = FxHashMap::default();
        let mut exported_component_ids = Vec::new();
        let mut exported_non_component_ids = Vec::new();
        for item in root.items() {
            if let AnyJsModuleItem::AnyJsStatement(stmt) = item {
                // Explore unexported component declarations
                if let AnyJsStatement::JsVariableStatement(var_statement) = stmt {
                    if let Ok(var_declaration) = var_statement.declaration() {
                        for var_declarator in var_declaration.declarators().into_iter().flatten() {
                            if let Some(ReactComponentInfo {
                                name: Some(name), ..
                            }) = ReactComponentInfo::from_declaration(var_declarator.syntax())
                            {
                                local_components
                                    .insert(name.text_trimmed().into(), name.text_trimmed_range());
                            }
                        }
                    }
                } else if matches!(
                    stmt,
                    AnyJsStatement::JsFunctionDeclaration(_)
                        | AnyJsStatement::JsClassDeclaration(_)
                ) && let Some(ReactComponentInfo {
                    name: Some(name), ..
                }) = ReactComponentInfo::from_declaration(stmt.syntax())
                {
                    local_components.insert(name.text_trimmed().into(), name.text_trimmed_range());
                }
            } else if let AnyJsModuleItem::JsExport(export) = item {
                // Explore exported component declarations
                for exported_item in export.get_exported_items() {
                    if let Some(AnyJsExported::AnyTsType(_)) = exported_item.exported {
                        continue;
                    }
                    // Allow exporting specific names
                    if let Some(exported_item_id) = exported_item
                        .identifier
                        .as_ref()
                        .and_then(|x| x.name_token())
                        && ctx.options().allow_export_names.iter().any(|export_name| {
                            export_name.as_ref() == exported_item_id.text_trimmed()
                        })
                    {
                        continue;
                    }
                    // Allow exporting constants along with components
                    if ctx.options().allow_constant_export
                        && exported_item
                            .exported
                            .clone()
                            .is_some_and(|partof| match partof {
                                AnyJsExported::AnyJsExpression(expr) => {
                                    expr.is_literal_expression()
                                }
                                _ => false,
                            })
                    {
                        continue;
                    }

                    if let Some(AnyJsExported::AnyIdentifier(identifier)) =
                        exported_item.exported.as_ref()
                        && let Some(name) = identifier.name_token()
                        && local_components.contains_key(name.text_trimmed())
                    {
                        exported_component_ids.push(exported_item);
                        continue;
                    }

                    if ReactComponentInfo::from_exported_item(&exported_item).is_some() {
                        exported_component_ids.push(exported_item);
                    } else {
                        exported_non_component_ids.push(exported_item);
                    }
                }
            }
        }

        if !exported_component_ids.is_empty() {
            return exported_non_component_ids
                .iter()
                .filter_map(|id| {
                    let range = id.identifier.as_ref().map_or_else(
                        || id.exported.as_ref().map(|exported| exported.range()),
                        |identifier| Some(identifier.range()),
                    );
                    range.map(|range| UseComponentExportOnlyModulesState {
                        error: ErrorType::ExportedNonComponentWithComponent,
                        range,
                    })
                })
                .collect();
        }

        local_components
            .into_values()
            .map(|id| UseComponentExportOnlyModulesState {
                error: if exported_non_component_ids.is_empty() {
                    ErrorType::UnexportedComponent
                } else {
                    ErrorType::NoExport
                },
                range: id,
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (message, suggestion, error_item) = match state.error {
            ErrorType::ExportedNonComponentWithComponent => (
                "Exporting a non-component with components is not allowed.",
                "Consider separating non-component exports into a new file.",
                "a component",
            ),
            ErrorType::UnexportedComponent => (
                "Unexported components are not allowed.",
                "Consider separating component exports into a new file.",
                "not a component",
            ),
            ErrorType::NoExport => (
                "Components should be exported.",
                "Consider separating component exports into a new file.",
                "not a component",
            ),
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    {message}
                },
            )
            .note(markup! {
                <Hyperlink href="https://github.com/facebook/react/tree/main/packages/react-refresh">"Fast Refresh"</Hyperlink>" only works when a file only exports components."
            })
            .note(markup! {
                {suggestion}
            })
            .note(markup! {
                "If it is "{error_item}", it may not be following the variable naming conventions."
            }),
        )
    }
}
