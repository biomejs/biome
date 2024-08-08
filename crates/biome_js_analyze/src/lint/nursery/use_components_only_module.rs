use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{
    export_ext::{AnyJsExported, ExportedItem},
    AnyJsBindingPattern, AnyJsExpression, AnyJsModuleItem, AnyJsStatement, JsModule,
};
use biome_rowan::{AstNode, TextRange};
use biome_string_case::Case;
use serde::{Deserialize, Serialize};

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
    /// Some tools, such as Vite, allow exporting constants along with components. By enabling the following, the rule will support the pattern.
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options":{
    ///         "allowConstantExport" : true
    ///     }
    /// }
    /// ```
    ///
    /// ### `allowExportNames`
    ///
    /// If you use a framework that handles [Hot Mudule Replacement(HMR)] of some specific exports, you can use this option to avoid warning for them.
    ///
    /// Example for [Remix](https://remix.run/docs/en/main/discussion/hot-module-replacement#supported-exports):
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options":{
    ///         "allowExportNames": ["json", "loader", "headers", "meta", "links", "scripts"]
    ///     }
    /// }
    /// ```
    ///
    /// ### checkJS
    ///
    /// If you are using JSX within .js files, you can apply the following to enable it for .js files as well. However, this is generally not recommended.
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options":{
    ///         "checkJS" : true
    ///     }
    /// }
    /// ```
    ///
    /// [Hot Mudule Replacement(HMR)]: https://remix.run/docs/en/main/discussion/hot-module-replacement
    /// [`React Fast Refresh`]: https://github.com/facebook/react/tree/main/packages/react-refresh
    /// [`camelCase`]: https://en.wikipedia.org/wiki/Camel_case
    /// [`PascalCase`]: https://en.wikipedia.org/wiki/Camel_case
    pub UseComponentsOnlyModule {
        version: "next",
        name: "useComponentsOnlyModule",
        language: "jsx",
        sources: &[RuleSource::EslintReactRefresh("only-export-components")],
        source_kind: RuleSourceKind::Inspired,
        recommended: false,
    }
}

#[derive(Debug, Clone, Deserialize, Deserializable, Eq, PartialEq, Serialize, Default)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UseComponentsOnlyModuleOptions {
    #[serde(default)]
    allow_constant_export: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    allow_export_names: Vec<String>,
    #[serde(default)]
    check_js: bool,
}

enum ErrorType {
    ExportedNonComponentWithComponent,
    UnexportedComponent,
    NoExport,
}

pub struct UseComponentsOnlyModuleState {
    error: ErrorType,
    range: TextRange,
}

const JSX_FILE_EXT: [&str; 2] = [".jsx", ".tsx"];

impl Rule for UseComponentsOnlyModule {
    type Query = Ast<JsModule>;
    type State = UseComponentsOnlyModuleState;
    type Signals = Vec<Self::State>;
    type Options = UseComponentsOnlyModuleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let file_name = ctx.file_path().file_name().and_then(|x| x.to_str());
        if let Some(file_name2) = file_name {
            if !ctx.options().check_js && !JSX_FILE_EXT.iter().any(|ext| file_name2.ends_with(ext))
            {
                return vec![];
            }
        }
        let root = ctx.query();
        let mut local_declaration_ids = Vec::new();
        let mut exported_component_ids = Vec::new();
        let mut exported_non_component_ids = Vec::new();
        for item in root.items() {
            if let AnyJsModuleItem::AnyJsStatement(stmt) = item {
                // Explore unexported component declarations
                if let AnyJsStatement::JsVariableStatement(vstmt) = stmt {
                    if let Ok(vdec) = vstmt.declaration() {
                        for vdeclator in vdec.declarators().into_iter().flatten() {
                            if let Ok(id) = vdeclator.id() {
                                local_declaration_ids.push(id)
                            }
                        }
                    }
                } else if let AnyJsStatement::JsFunctionDeclaration(func) = stmt {
                    if let Ok(id) = func.id() {
                        local_declaration_ids.push(AnyJsBindingPattern::AnyJsBinding(id));
                    }
                }
            } else if let AnyJsModuleItem::JsExport(export) = item {
                // Explore exported component declarations
                for exported_item in export.get_exported_items() {
                    if let Some(AnyJsExported::AnyTsType(_)) = exported_item.exported {
                        continue;
                    }
                    // Allow exporting specific names
                    if let Some(exported_item_id) = &exported_item.identifier {
                        if ctx
                            .options()
                            .allow_export_names
                            .contains(&exported_item_id.text())
                        {
                            continue;
                        }
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
                    if is_exported_react_component(&exported_item) {
                        exported_component_ids.push(exported_item);
                    } else {
                        exported_non_component_ids.push(exported_item);
                    }
                }
            }
        }

        let local_component_ids = local_declaration_ids.iter().filter_map(|id| {
            if Case::identify(&id.text(), false) == Case::Pascal {
                Some(id.range())
            } else {
                None
            }
        });

        if !exported_component_ids.is_empty() {
            return exported_non_component_ids
                .iter()
                .filter_map(|id| {
                    id.identifier
                        .as_ref()
                        .map(|identifier| UseComponentsOnlyModuleState {
                            error: ErrorType::ExportedNonComponentWithComponent,
                            range: identifier.range(),
                        })
                })
                .collect::<Vec<UseComponentsOnlyModuleState>>();
        }

        local_component_ids
            .map(|id| UseComponentsOnlyModuleState {
                error: if exported_non_component_ids.is_empty() {
                    ErrorType::UnexportedComponent
                } else {
                    ErrorType::NoExport
                },
                range: id,
            })
            .collect::<Vec<UseComponentsOnlyModuleState>>()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state.error {
            ErrorType::ExportedNonComponentWithComponent => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    state.range,
                    markup! {
                        "Exporting a non-component with components is not allowed."
                    },
                )
                .note(markup! {
                    <Hyperlink href="https://github.com/facebook/react/tree/main/packages/react-refresh">"Fast Refresh"</Hyperlink>" only works when a file only exports components."
                })
                .note(markup! {
                    "Consider separating non-component exports into a new file."
                })
                .note(markup! {
                    "If it is a component, it may not be following the variable naming conventions."
                }),
            ),
            ErrorType::UnexportedComponent => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    state.range,
                    markup! {
                        "Unexported components are not allowed."
                    },
                )
                .note(markup! {
                    <Hyperlink href="https://github.com/facebook/react/tree/main/packages/react-refresh">"Fast Refresh"</Hyperlink>" only works when a file only exports components."
                })
                .note(markup! {
                    "Consider separating components into a new file."
                })
                .note(markup! {
                    "If it is not a component, it may not be following the variable naming conventions."
                })
            ),
            ErrorType::NoExport => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    state.range,
                    markup! {
                        "Components should be exported."
                    },
                )
                .note(markup! {
                    <Hyperlink href="https://github.com/facebook/react/tree/main/packages/react-refresh">"Fast Refresh"</Hyperlink>" only works when a file only exports components."
                })
                .note(markup! {
                    "Consider separating component exports into a new file."
                })
                .note(markup! {
                    "If it is not a component, it may not be following the variable naming conventions."
                }),
            )
        }
    }
}

// Function that returns a standard React component
const REACT_HOOKS: [&str; 2] = ["memo", "forwardRef"];

fn is_exported_react_component(any_exported_item: &ExportedItem) -> bool {
    if let Some(AnyJsExported::AnyJsExpression(AnyJsExpression::JsCallExpression(f))) =
        any_exported_item.exported.clone()
    {
        if let Ok(AnyJsExpression::JsIdentifierExpression(fn_name)) = f.callee() {
            if REACT_HOOKS.contains(&fn_name.text().as_str()) {
                return true;
            }
        }
    }
    let Some(exported_item_id) = any_exported_item.identifier.clone() else {
        return false;
    };
    Case::identify(&exported_item_id.text(), false) == Case::Pascal
        && match any_exported_item.exported.clone() {
            Some(exported) => !matches!(exported, AnyJsExported::TsEnumDeclaration(_)),
            None => true,
        }
}
