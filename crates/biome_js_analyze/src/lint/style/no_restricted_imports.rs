use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBindingPattern, AnyJsCombinedSpecifier, AnyJsExpression,
    AnyJsImportLike, AnyJsNamedImportSpecifier, AnyJsObjectBindingPatternMember, JsCallExpression,
    JsDefaultImportSpecifier, JsExportFromClause, JsExportNamedFromClause,
    JsExportNamedFromSpecifier, JsExportNamedFromSpecifierList, JsIdentifierBinding,
    JsImportBareClause, JsImportCallExpression, JsImportCombinedClause, JsImportDefaultClause,
    JsImportNamedClause, JsImportNamespaceClause, JsLanguage, JsModuleSource,
    JsNamedImportSpecifier, JsNamedImportSpecifiers, JsNamespaceImportSpecifier,
    JsObjectBindingPattern, JsObjectBindingPatternProperty,
    JsObjectBindingPatternShorthandProperty, JsShorthandNamedImportSpecifier,
    JsStaticMemberExpression, JsSyntaxKind, JsVariableDeclarator, inner_string_text,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxNode, SyntaxNodeCast, SyntaxToken};
use biome_rule_options::no_restricted_imports::{
    CustomRestrictedImportOptions, ImportRestrictionCause, NoRestrictedImportsOptions,
    RestrictedImportMessage, RestrictedImportVisitor,
};
use serde::{Deserialize, Serialize};

declare_lint_rule! {
    /// Disallow specified modules when loaded by import or require.
    ///
    /// ## Examples
    ///
    /// ```json
    /// {
    ///     "noRestrictedImports": {
    ///         "options": {
    ///             "paths": {
    ///                 "lodash": "Using lodash is not encouraged",
    ///                 "underscore": "Using underscore is not encouraged"
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "paths": {
    ///             "lodash": "Using lodash is not encouraged.",
    ///             "underscore": "",
    ///             "import-foo": { "importNames": ["Bar"] },
    ///             "import-bar": { "allowImportNames": ["Bar"] }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// import "lodash";
    /// import "allowed-import";
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// const underscore = await import("underscore");
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// const lodash = require("lodash");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,use_options
    /// import "allowed-import";
    /// const myImport = await import("allowed-import");
    /// const myImport = require("allowed-import");
    /// ```
    ///
    /// ## Supported Import Syntaxes
    ///
    /// The rule tries to parse the context of the import to see if only one or more
    /// of the allowed import names have been imported from a given module.
    ///
    /// All of the following import syntaxes are supported:
    ///
    /// ### Static `import` (and re-`export`) declarations
    ///
    /// Normal static [ESM `import` declarations](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import) are supported:
    ///
    /// ```js
    /// // Static `import` declaration:
    /// // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import
    ///
    /// import "sideeffect-import";
    /// import * as alias1 from "namespace-import";
    /// import { export1, export2 as alias2, "string-name" as alias3, default as defaultExport /* … */ } from "named-import";
    /// import defaultExport from "default-import";
    /// import defaultExport, * as alias5 from "default+namespace-import";
    /// import defaultExport, { export1 /* … */ } from "default+named-import";
    ///
    /// export * from "namespace-import";
    /// export { export1, export2 as alias2, "string-name" as alias3, default as defaultExport /* … */ } from "named-import";
    /// ```
    ///
    /// The TypeScript-specific [type-only imports](https://www.typescriptlang.org/docs/handbook/modules/reference.html#type-only-imports-and-exports) are also supported:
    ///
    /// ```ts
    /// // TypeScript-specific type-only `import` declaration:
    /// // https://www.typescriptlang.org/docs/handbook/modules/reference.html#type-only-imports-and-exports
    ///
    /// import { type export1, type export2 as alias2, type "string-name" as alias3, type default as defaultExport /* … */ } from "named-import";
    /// import type { export1, export2 as alias2, "string-name" as alias3, default as defaultExport /* … */ } from "named-import";
    /// import type defaultExport from "default-import";
    /// ```
    ///
    /// ### Dynamic `import()` calls
    ///
    /// Dynamic [ESM `import()` calls](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/import) are also supported.
    /// Because the import is performed at runtime, it is not always possible to determine which import names are being used.
    /// Nevertheless, the rule tries to detect the following common usage patterns where the set of imported names is determined statically:
    ///
    /// ```js
    /// // Dynamic `import()` calls:
    /// // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/import
    ///
    /// import('sideeffect-import');
    /// await import('sideeffect-import');
    ///
    /// // ...using await + destructuring-assignment:
    /// const alias1 = await import('namespace-import');
    /// const { default: defaultExport } = await import('default-import')
    /// const { export1, export2: alias2, "string-name": alias3, default: defaultExport /* … */ } = await import("named-import");
    ///
    /// // ...using then() with arrow-function + destructuring parameters:
    /// import('namespace-import').then(alias1 => { /* … */ });
    /// import('namespace-import').then((alias1) => { /* … */ });
    /// import('default-import').then(({ default: defaultExport }) => { /* … */ });
    /// import('named-import').then(({ export1, export2: alias2, "string-name": alias3, default: defaultExport /* … */ }) => { /* … */ });
    ///
    /// // ...using then() with function + destructuring parameters:
    /// import('namespace-import').then(function(alias1) { /* … */ });
    /// import('default-import').then(function({ default: defaultExport }) { /* … */ });
    /// import('named-import').then(function({ export1, export2: alias2, "string-name": alias3, default: defaultExport /* … */ }) { /* … */ });
    ///
    /// // Standalone `import('...')` calls that appear in some other
    /// // unrecognized context will be treated as a namespace import,
    /// // because the return value of `import('...')` is a namespace object:
    ///
    /// myFunction(...args, import("namespace-import"), ...args)
    /// ```
    ///
    /// ### Dynamic `require()` calls
    ///
    /// NodeJS-style `require()` calls are also supported.
    /// Due to the way `require()` works, these are always treated as default imports.
    ///
    /// ```js
    /// // Dynamic `require()` call
    /// const defaultExport = require('default-import');
    /// ```
    ///
    /// ## Options
    ///
    /// ```json
    /// {
    ///     "noRestrictedImports": {
    ///         "options": {
    ///             "paths": {
    ///                 "lodash": "Using lodash is not encouraged",
    ///                 "underscore": "Using underscore is not encouraged"
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// Use the options to specify the import paths and/or specific import names within them that you want to restrict in your source code.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "paths": {
    ///             "lodash": "Using lodash is not encouraged",
    ///             "underscore": "Using underscore is not encouraged",
    ///             "import-foo": {
    ///                 "importNames": ["Bar"],
    ///                 "message": "Please use Bar from /import-bar/baz/ instead."
    ///             },
    ///             "import-bar": {
    ///               "allowImportNames": ["Bar"],
    ///               "message": "Please use only Bar from import-bar."
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ### `paths`
    ///
    /// An object that lists the import paths that are either wholly or partially restricted.
    ///
    /// The keys of the object are the import paths to restrict, and the values can be:
    /// - A string with a custom message to show in the diagnostic when any
    /// - An object with additional options, as explained [below](#pathsimportimportnames).
    ///
    /// In the example below, we restrict the two paths `services-deprecated` and `constants`, with two particular messages.
    /// Importing `services-deprecated` will emit the message `Use services instead.`.
    /// Importing `constants` will emit the message `This file will be deleted soon.`:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "paths": {
    ///             "services-deprecated": {
    ///                 "message": "Use services instead."
    ///             },
    ///	            "constants": "This file will be deleted soon."
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// import * as namespaceAlias from 'services-deprecated';
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// import { export1 } from 'constants';
    /// ```
    ///
    /// ### `paths.<import>.message`
    ///
    /// Specifies the message to be shown when the restricted import is used.
    ///
    /// A default message will be generated if `message` is empty or not specified:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "paths": {
    ///             "import-foo": { }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// import { export1 } 'import-foo';
    /// ```
    ///
    /// ### `paths.<import>.importNames`
    ///
    /// Specifies the array of import names that should be explicitly forbidden.
    /// The following import name specifiers are supported:
    ///
    /// - **Named import:** `"someIdentifier"` (`import { someIdentifier } from 'named-import'`)
    /// - **Default import:** `"default"` (`import defaultExport from 'default-import'`)
    /// - **Namespace import:** `"*"` (`import * as alias1 from 'namespace-import'`)
    /// - **Side effect/Bare import:** `""` (`import "sideeffect-import"`)
    ///
    /// **Only one of `importNames` and `allowImportNames` must be specified.**
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "paths": {
    ///             "import-foo": {
    ///                 "importNames": ["Bar"],
    ///                 "message": "Please use Bar from /import-bar/baz/ instead."
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// import { Bar } from 'import-foo';
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// import { Foo } from 'import-foo';
    /// ```
    ///
    /// ### `paths.<import>.allowImportNames`
    ///
    /// Specifies the set of import names that should be explicitly allowed.
    /// See `importNames` for the set of supported import name specifiers.
    ///
    /// **Only one of `importNames` and `allowImportNames` must be specified.**
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "paths": {
    ///             "import-bar": {
    ///               "allowImportNames": ["Bar"]
    ///             },
    ///             "restrictPackagePrivate": "all"
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// import { Baz } from 'import-bar';
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// import { Bar } from 'import-bar';
    /// ```
    pub NoRestrictedImports {
        version: "1.6.0",
        name: "noRestrictedImports",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-restricted-imports").same(),
            RuleSource::EslintTypeScript("no-restricted-imports").same(),
        ],
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for NoRestrictedImports {
    type Query = Ast<AnyJsImportLike>;
    type State = RestrictedImportMessage;
    type Signals = Vec<Self::State>;
    type Options = NoRestrictedImportsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.is_in_ts_module_declaration() {
            return Vec::new();
        }
        let Some(module_name) = node.module_name_token() else {
            return Vec::new();
        };
        let import_source_text = inner_string_text(&module_name);
        let import_source = import_source_text.text();

        let Some(restricted_import_settings) = ctx.options().paths.get(import_source) else {
            return Vec::new();
        };
        let restricted_import: CustomRestrictedImportOptions =
            restricted_import_settings.clone().into();

        match node {
            AnyJsImportLike::JsModuleSource(module_source_node) => {
                if !restricted_import.has_import_name_patterns() {
                    // All imports disallowed, add diagnostic to the import source
                    vec![RestrictedImportMessage {
                        location: module_name.text_trimmed_range(),
                        message: restricted_import.get_message_for_restriction(
                            import_source,
                            "",
                            ImportRestrictionCause::ImportSource,
                        ),
                        import_source: import_source.to_string(),
                        allowed_import_names: [].into(),
                    }]
                } else {
                    // Check (and possibly report) each imported name individually
                    let mut visitor = RestrictedImportVisitor {
                        import_source,
                        restricted_import,
                        results: Vec::new(),
                    };
                    visit_import(&mut visitor, module_source_node);
                    visitor.results
                }
            }
            AnyJsImportLike::JsImportCallExpression(import_call) => {
                // TODO: We have to parse the context of the import() call to determine
                // which exports are being used/whether this should be considered a
                // namespace import, a side-effect import (the two of which may
                // be difficult to distinguish) or a collection of named imports.
                if !restricted_import.has_import_name_patterns() {
                    // All imports disallowed, add diagnostic to the import source
                    vec![RestrictedImportMessage {
                        location: module_name.text_trimmed_range(),
                        message: restricted_import.get_message_for_restriction(
                            import_source,
                            "",
                            ImportRestrictionCause::ImportSource,
                        ),
                        import_source: import_source.to_string(),
                        allowed_import_names: [].into(),
                    }]
                } else {
                    // Check (and possibly report) each imported name individually
                    let mut visitor = RestrictedImportVisitor {
                        import_source,
                        restricted_import,
                        results: Vec::new(),
                    };
                    visit_import_call(&mut visitor, import_call);
                    visitor.results
                }
            }
            AnyJsImportLike::JsCallExpression(_expression) => {
                let status = restricted_import
                    .is_import_allowed(RestrictedImportVisitor::DEFAULT_IMPORT_ALIAS);

                if status.is_forbidden() {
                    // require() calls can only import the default import, so
                    // there are no individual import names to check or report on.
                    vec![RestrictedImportMessage {
                        location: module_name.text_trimmed_range(),
                        message: restricted_import.get_message_for_restriction(
                            import_source,
                            "",
                            ImportRestrictionCause::ImportSource,
                        ),
                        import_source: import_source.to_string(),
                        allowed_import_names: [].into(),
                    }]
                } else {
                    vec![]
                }
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let RestrictedImportMessage {
            import_source,
            allowed_import_names,
            location,
            message,
        } = state;

        let mut rule_diagnostic = RuleDiagnostic::new(
            rule_category!(),
            location,
            markup! {
                {message}
            },
        );
        if !allowed_import_names.is_empty() {
            let mut sorted = allowed_import_names.to_vec();
            sorted.sort();
            let allowed_import_names = sorted.into_iter().map(|name| {
                if &*name == RestrictedImportVisitor::BARE_IMPORT_ALIAS {
                    "Side-effect only import".into()
                } else {
                    name
                }
            });

            rule_diagnostic = rule_diagnostic.footer_list(
                        markup! { "Only the following imports from "<Emphasis>"'"{import_source}"'"</Emphasis>" are allowed:" },
                        allowed_import_names,
                    );
        }
        Some(rule_diagnostic)
    }
}

/// Options for the rule `noRestrictedImports`.
#[derive(
    Clone,
    Debug,
    Default,
    Deserialize,
    biome_deserialize_macros::Deserializable,
    Eq,
    PartialEq,
    Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct RestrictedImportsOptions {}

/// Analyze the context of an `import(...)` call to find the imported names,
/// then validate that each of the names is allowed to be imported.
pub fn visit_import_call(
    visitor: &mut RestrictedImportVisitor,
    import_call: &JsImportCallExpression,
) -> Option<()> {
    // An import() call can appear by itself, but might also appear within
    // the following contexts, where we can infer more details about what is
    // being imported, and thus better target our emitted diagnostics:
    //
    //     import("imported-module")
    //     import("imported-module").then((namespaceImport) => /* ... */)
    //     import("imported-module").then(({ import1, import2: localName2 }) => /* ... */)
    //     import("imported-module").then(function(namespaceImport) { /* ... */ })
    //     import("imported-module").then(function({ import1, import2: localName2 }) { /* ... */ })
    //     const namespaceImport = await import("imported-module")
    //     const { default: localName1, import1, import2: localName2, "import3": localName3 } = await import("imported-module")
    //
    // To make this diagnostic a bit tolerant to other errors in the source code,
    // we also allow the "await" keyword to be missing, and just act as if it was
    // there in that case. We also try to ignore parentheses and thus treat "(expr)"
    // the same as "expr".
    //
    // Given the import_call node, we navigate up the parent chain to see
    // whether we are in one of the mentioned contexts:
    if let Some(bindings) = get_context_for_import_call(import_call) {
        match bindings {
            AnyJsBindingPattern::AnyJsBinding(namespace_binding) => match namespace_binding {
                // const ... = import(...)
                biome_js_syntax::AnyJsBinding::JsIdentifierBinding(namespace_binding) => {
                    // const namespaceImport = import(...)
                    return visit_namespace_binding(visitor, &namespace_binding);
                }
                _ => {
                    // Use fallback instead
                }
            },
            AnyJsBindingPattern::JsObjectBindingPattern(named_bindings) => {
                // const { ... } = await import(...)
                return visit_named_bindings(visitor, &named_bindings);
            }
            AnyJsBindingPattern::JsArrayBindingPattern(_) => {
                // const [ ... ] = await import(...)
                //
                // Array binding patterns do not really make sense for an import,
                // so discard the additonal information and use fallback instead.
            }
        }
    };

    // We failed to find any additional context, and are therefore
    // restricted to analyzing "import(...)" as a namespace import,
    // because that what is returned by "import(...)".
    //
    // The diagnostic will be associated with "import('module-name')"
    // instead of just "'module_name'" to indicate that not the
    // imported module itself is forbidden, but the ways in which
    // it can be imported are restricted.
    visit_special_import_node(
        visitor,
        import_call.syntax(),
        RestrictedImportVisitor::NAMESPACE_IMPORT_ALIAS,
    )
}

fn get_context_for_import_call(
    import_call: &JsImportCallExpression,
) -> Option<AnyJsBindingPattern> {
    let mut current = import_call.syntax().parent()?;

    while current.kind() == JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION {
        // #1: const { ... } = (await **(import(""))**)
        // #2: **(import(""))**.then(...)
        current = current.parent()?;
    }

    if current.kind() == JsSyntaxKind::JS_AWAIT_EXPRESSION {
        // #1: const { ... } = (**await (import(""))**)
        current = current.parent()?;

        while current.kind() == JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION {
            // #1: const { ... } = **(await (import("")))**
            current = current.parent()?;
        }
    } else if current.kind() == JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION {
        // #2: **(import("")).then**(...)
        let static_member_expr = current.cast::<JsStaticMemberExpression>()?;
        let member_name = static_member_expr.member().ok()?;
        if member_name.as_js_name()?.syntax().text_trimmed() != "then" {
            return None;
        }
        current = static_member_expr.syntax().parent()?;

        if current.kind() == JsSyntaxKind::JS_CALL_EXPRESSION {
            // #2: **(import("")).then(...)**
            let then_call_expr = current.cast::<JsCallExpression>()?;
            let then_call_arg = then_call_expr
                .arguments()
                .ok()?
                .args()
                .iter()
                .next()?
                .ok()?
                .as_any_js_expression()?
                .clone()
                .omit_parentheses();

            return match then_call_arg {
                // then(... => ...)
                AnyJsExpression::JsArrowFunctionExpression(arrow_expr) => {
                    match arrow_expr.parameters().ok()? {
                        // then(arg => ...)
                        AnyJsArrowFunctionParameters::AnyJsBinding(binding) => {
                            Some(AnyJsBindingPattern::AnyJsBinding(binding))
                        }
                        // then ({ ... } => ...)
                        AnyJsArrowFunctionParameters::JsParameters(parameters) => Some(
                            parameters
                                .items()
                                .iter()
                                .next()?
                                .ok()?
                                .as_any_js_formal_parameter()?
                                .as_js_formal_parameter()?
                                .binding()
                                .ok()?,
                        ),
                    }
                }
                // then(function(...) { ... })
                AnyJsExpression::JsFunctionExpression(function_expr) => Some(
                    function_expr
                        .parameters()
                        .ok()?
                        .items()
                        .iter()
                        .next()?
                        .ok()?
                        .as_any_js_formal_parameter()?
                        .as_js_formal_parameter()?
                        .binding()
                        .ok()?,
                ),
                _ => None,
            };
        }
    }

    // #1: const { ... } = **(await (import("")))**
    if current.kind() == JsSyntaxKind::JS_INITIALIZER_CLAUSE {
        // #1: const { ... } **= (await (import("")))**
        current = current.parent()?;
    } else {
        return None;
    }

    if current.kind() == JsSyntaxKind::JS_VARIABLE_DECLARATOR {
        // #1: const **{ ... } = (await (import("")))**
        let variable_declarator = current.cast::<JsVariableDeclarator>()?;

        // #1: const **{ ... }** = (await (import("")))
        variable_declarator.id().ok()
    } else {
        None
    }
}

/// Analyze a static `import ... from ...` or `export ... from ...`declaration
/// (including all the different variants of `import` and `export`) to find the names
/// that are being imported, then validate that each of the names is allowed to be imported.
pub fn visit_import(
    visitor: &mut RestrictedImportVisitor,
    module_source_node: &JsModuleSource,
) -> Option<()> {
    // Only certain imports are allowed/disallowed, add diagnostic to each disallowed import
    let clause = module_source_node.syntax().parent()?;
    match clause.kind() {
        JsSyntaxKind::JS_IMPORT_BARE_CLAUSE => {
            let side_effect_import: JsImportBareClause = clause.cast()?;
            visit_side_effect_import(visitor, &side_effect_import)
        }
        JsSyntaxKind::JS_IMPORT_COMBINED_CLAUSE => {
            let import_combined_clause: JsImportCombinedClause = clause.cast()?;
            if let Ok(default_specifier) = import_combined_clause.default_specifier() {
                visit_default_import(visitor, &default_specifier);
            }
            if let Ok(combined_specifier) = import_combined_clause.specifier() {
                visit_combined_specifier(visitor, &combined_specifier);
            }
            Some(())
        }
        JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE => {
            let import_named_clause: JsImportNamedClause = clause.cast()?;
            let import_specifiers = import_named_clause.named_specifiers().ok()?;
            visit_named_imports(visitor, &import_specifiers)
        }
        JsSyntaxKind::JS_EXPORT_NAMED_FROM_CLAUSE => {
            let export_named_from_clause = clause.cast::<JsExportNamedFromClause>()?;
            let import_specifiers = export_named_from_clause.specifiers();
            visit_named_reexports(visitor, &import_specifiers)
        }
        JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE => {
            let import_default_clause: JsImportDefaultClause = clause.cast()?;
            let default_specifier = import_default_clause.default_specifier().ok()?;
            visit_default_import(visitor, &default_specifier)
        }
        JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE => {
            let import_namespace_clause: JsImportNamespaceClause = clause.cast()?;
            let namespace_specifier = import_namespace_clause.namespace_specifier().ok()?;
            visit_namespace_import(visitor, &namespace_specifier)
        }
        JsSyntaxKind::JS_EXPORT_FROM_CLAUSE => {
            let reexport_namespace_clause: JsExportFromClause = clause.cast()?;
            visit_namespace_reexport(visitor, &reexport_namespace_clause)
        }
        _ => None,
    }
}

fn visit_combined_specifier(
    visitor: &mut RestrictedImportVisitor,
    combined_specifier: &AnyJsCombinedSpecifier,
) -> Option<()> {
    match combined_specifier {
        AnyJsCombinedSpecifier::JsNamedImportSpecifiers(named_imports) => {
            visit_named_imports(visitor, named_imports)
        }
        AnyJsCombinedSpecifier::JsNamespaceImportSpecifier(namespace_import) => {
            visit_namespace_import(visitor, namespace_import)
        }
    }
}

fn visit_named_imports(
    visitor: &mut RestrictedImportVisitor,
    named_imports: &JsNamedImportSpecifiers,
) -> Option<()> {
    let import_specifiers = named_imports.specifiers();
    for import_specifier in import_specifiers.iter().flatten() {
        visit_named_or_shorthand_import(visitor, &import_specifier);
    }
    Some(())
}

fn visit_named_reexports(
    visitor: &mut RestrictedImportVisitor,
    named_reexports: &JsExportNamedFromSpecifierList,
) -> Option<()> {
    for export_specifier in named_reexports.iter().flatten() {
        visit_named_or_shorthand_reexport(visitor, &export_specifier);
    }
    Some(())
}

fn visit_named_bindings(
    visitor: &mut RestrictedImportVisitor,
    named_imports: &JsObjectBindingPattern,
) -> Option<()> {
    let import_bindings = named_imports.properties();
    for import_binding in import_bindings.iter().flatten() {
        visit_named_or_shorthand_binding(visitor, &import_binding);
    }
    Some(())
}

fn visit_named_or_shorthand_import(
    visitor: &mut RestrictedImportVisitor,
    import_specifier: &AnyJsNamedImportSpecifier,
) -> Option<()> {
    match import_specifier {
        AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(shorthand_import) => {
            visit_shorthand_import(visitor, shorthand_import)
        }
        AnyJsNamedImportSpecifier::JsNamedImportSpecifier(named_import) => {
            visit_named_import(visitor, named_import)
        }
        AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => None,
    }
}

fn visit_named_or_shorthand_binding(
    visitor: &mut RestrictedImportVisitor,
    import_binding: &AnyJsObjectBindingPatternMember,
) -> Option<()> {
    match import_binding {
        AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
            shorthand_import,
        ) => visit_shorthand_binding(visitor, shorthand_import),
        AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(named_import) => {
            visit_named_binding(visitor, named_import)
        }
        _ => None,
    }
}

/// Checks whether this bare import of the form `import from 'source'` is allowed.
fn visit_side_effect_import(
    visitor: &mut RestrictedImportVisitor,
    bare_import: &JsImportBareClause,
) -> Option<()> {
    let source_token = bare_import
        .source()
        .ok()?
        .as_js_module_source()?
        .value_token()
        .ok()?;
    visit_special_import_token(
        visitor,
        &source_token,
        RestrictedImportVisitor::BARE_IMPORT_ALIAS,
    )
}

/// Checks whether this import of the form `local_name` (as in `import local_name from 'source'`) is allowed.
fn visit_default_import(
    visitor: &mut RestrictedImportVisitor,
    default_import: &JsDefaultImportSpecifier,
) -> Option<()> {
    let local_name = default_import
        .local_name()
        .ok()?
        .as_js_identifier_binding()?
        .name_token()
        .ok()?;
    visit_special_import_token(
        visitor,
        &local_name,
        RestrictedImportVisitor::DEFAULT_IMPORT_ALIAS,
    )
}

/// Checks whether this import of the form `* as local_name` is allowed.
fn visit_namespace_import(
    visitor: &mut RestrictedImportVisitor,
    namespace_import: &JsNamespaceImportSpecifier,
) -> Option<()> {
    visit_special_import_token(
        visitor,
        &namespace_import.star_token().ok()?,
        RestrictedImportVisitor::NAMESPACE_IMPORT_ALIAS,
    )
}

/// Checks whether this namespace reexport of the form `export * from ...` is allowed.
fn visit_namespace_reexport(
    visitor: &mut RestrictedImportVisitor,
    namespace_reexport: &JsExportFromClause,
) -> Option<()> {
    visit_special_import_token(
        visitor,
        &namespace_reexport.star_token().ok()?,
        RestrictedImportVisitor::NAMESPACE_IMPORT_ALIAS,
    )
}

/// Checks whether this import of the form `const local_name = import(...)` is allowed.
fn visit_namespace_binding(
    visitor: &mut RestrictedImportVisitor,
    namespace_import: &JsIdentifierBinding,
) -> Option<()> {
    visit_special_import_node(
        visitor,
        namespace_import.syntax(),
        RestrictedImportVisitor::NAMESPACE_IMPORT_ALIAS,
    )
}

/// Checks whether this import of the form `{ imported_name }` is allowed.
fn visit_shorthand_import(
    visitor: &mut RestrictedImportVisitor,
    shorthand_import: &JsShorthandNamedImportSpecifier,
) -> Option<()> {
    visit_imported_identifier(
        visitor,
        &shorthand_import
            .local_name()
            .ok()?
            .as_js_identifier_binding()?
            .name_token()
            .ok()?,
    )
}

/// Checks whether this import of the form `{ imported_name }` is allowed.
fn visit_shorthand_binding(
    visitor: &mut RestrictedImportVisitor,
    shorthand_import: &JsObjectBindingPatternShorthandProperty,
) -> Option<()> {
    visit_imported_identifier(
        visitor,
        &shorthand_import
            .identifier()
            .ok()?
            .as_js_identifier_binding()?
            .name_token()
            .ok()?,
    )
}

/// Checks whether this import of the form `{ imported_name as local_name }`
/// (including `{ default as local_name }`) is allowed.
fn visit_named_import(
    visitor: &mut RestrictedImportVisitor,
    named_import: &JsNamedImportSpecifier,
) -> Option<()> {
    visit_imported_identifier(visitor, &named_import.name().ok()?.value().ok()?)
}

/// Checks whether this import of the form `{ imported_name }` or `{ imported_name as exported_name }`
/// (including `{ default as exported_name }`) is allowed.
fn visit_named_or_shorthand_reexport(
    visitor: &mut RestrictedImportVisitor,
    named_reexport: &JsExportNamedFromSpecifier,
) -> Option<()> {
    visit_imported_identifier(visitor, &named_reexport.source_name().ok()?.value().ok()?)
}

/// Checks whether this import of the form `{ imported_name: local_name }`
/// (including `{ default: local_name }` and `{ "imported name": local_name `) is allowed.
fn visit_named_binding(
    visitor: &mut RestrictedImportVisitor,
    named_import: &JsObjectBindingPatternProperty,
) -> Option<()> {
    visit_imported_identifier(
        visitor,
        &named_import
            .member()
            .ok()?
            .as_js_literal_member_name()?
            .value()
            .ok()?,
    )
}

/// Checks whether the import specified by `name_token` is allowed,
/// and records a diagnostic for `name_token.text_trimmed_range()` if not.
///
/// `name_token` can be either a string literal or an identifier.
fn visit_imported_identifier(
    visitor: &mut RestrictedImportVisitor,
    name_token: &SyntaxToken<JsLanguage>,
) -> Option<()> {
    // TODO: inner_string_text removes quotes but does not e.g. decode escape sequences.
    //       If the imported name uses e.g. Unicode escape sequences, this may cause
    //       problems because restricted_import.(allow_)import_names contains decoded
    //       strings, while inner_string_text(name_token) returns encoded strings.
    visit_special_import_token(visitor, name_token, inner_string_text(name_token).text())
}

/// Checks whether the import specified by `name_or_alias` is allowed.
/// and records a diagnostic for `import_node.text_trimmed_range()` if not.
fn visit_special_import_node(
    visitor: &mut RestrictedImportVisitor,
    import_node: &SyntaxNode<JsLanguage>,
    name_or_alias: &str,
) -> Option<()> {
    let status = visitor.restricted_import.is_import_allowed(name_or_alias);
    if status.is_allowed() {
        return None;
    }
    visitor.results.push(RestrictedImportMessage {
        location: import_node.text_trimmed_range(),
        message: visitor.restricted_import.get_message_for_restriction(
            visitor.import_source,
            name_or_alias,
            status.reason(),
        ),
        import_source: visitor.import_source.to_string(),
        allowed_import_names: visitor.restricted_import.allow_import_names.clone(),
    });
    Some(())
}

/// Checks whether the import specified by `name_or_alias` is allowed.
/// and records a diagnostic for `import_token.text_trimmed_range()` if not.
fn visit_special_import_token(
    visitor: &mut RestrictedImportVisitor,
    import_token: &SyntaxToken<JsLanguage>,
    name_or_alias: &str,
) -> Option<()> {
    let status = visitor.restricted_import.is_import_allowed(name_or_alias);
    if status.is_allowed() {
        return None;
    }
    visitor.results.push(RestrictedImportMessage {
        location: import_token.text_trimmed_range(),
        message: visitor.restricted_import.get_message_for_restriction(
            visitor.import_source,
            name_or_alias,
            status.reason(),
        ),
        import_source: visitor.import_source.to_string(),
        allowed_import_names: visitor.restricted_import.allow_import_names.clone(),
    });
    Some(())
}
