use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsImportLike, inner_string_text};
use biome_rule_options::no_restricted_imports::{
    NoRestrictedImportsOptions, PathOptions, Patterns, RestrictedImportMessage,
    RestrictedImportVisitor, check_import_restrictions,
};

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
    ///         },
    ///         "patterns": [{
    ///             "group": ["import-foo/*", "!import-foo/bar"]
    ///         }]
    ///     }
    /// }
    /// ```
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// import "lodash";
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// import "underscore";
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// import { Bar } from "import-foo";
    /// ```
    ///
    ///
    /// ```js,expect_diagnostic,use_options
    /// const underscore = await import("underscore");
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// const lodash = require("lodash");
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// import foo from 'import-foo/foo';
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
    /// ```js,use_options
    /// import foo from 'import-foo';
    /// import bar from 'import-foo/bar';
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
    /// Use the options to specify import paths and/or patterns, including specific import names, that you want to restrict in your source code.
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
    /// ```json,options
    /// {
    ///     "options": {
    ///         "patterns": [{
    ///             "group": ["import-foo/*", "!import-foo/bar"]
    ///         }]
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
    /// import { export1 } from 'import-foo';
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
    ///
    /// ### `patterns`
    /// **Since `v2.2.0`**
    ///
    /// This option allows you to specify multiple modules to restrict using gitignore-style patterns.
    ///
    /// ### `group`
    ///
    /// The patterns array can also include objects. The group property is used to specify the gitignore-style patterns for restricting modules and the message property is used to specify a custom message.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "patterns": [{
    ///             "group": ["import-foo/*", "!import-foo/bar"],
    ///             "message": "import-foo is deprecated, except the modules in import-foo/bar."
    ///         }]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// import foo from 'import-foo/foo';
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// import foo from 'import-foo';
    /// import bar from 'import-foo/bar';
    /// ```
    ///
    /// ### `importNamePattern`
    /// **Since `v2.2.0`**
    ///
    /// This option allows you to use regex patterns to restrict import names.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "patterns": [{
    ///             "group": ["import-foo/*"],
    ///             "importNamePattern": "[xyz]"
    ///         }]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// import { x } from 'import-foo/foo';
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// import { foo } from 'import-foo/foo';
    /// ```
    ///
    /// ### `invertImportNamePattern`
    /// **Since `v2.2.0`**
    ///
    /// If true, the matched patterns in the importNamePattern will be allowed
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "patterns": [{
    ///             "group": ["import-foo/*"],
    ///             "importNamePattern": "[xyz]",
    ///             "invertImportNamePattern": true
    ///         }]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// import { foo } from 'import-foo/foo';
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// import { x } from 'import-foo/foo';
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
            return vec![];
        }
        let Some(module_name) = node.module_name_token() else {
            return vec![];
        };
        let import_source_text = inner_string_text(&module_name);
        let import_source = import_source_text.text();
        let options = ctx.options();

        let mut results: Vec<RestrictedImportMessage> = vec![];

        if let Some(paths) = options.paths.get(import_source) {
            let path_options: PathOptions = paths.clone().into();
            results.extend(check_import_restrictions(
                &path_options,
                node,
                &module_name,
                import_source,
            ))
        }

        if let Some(patterns) = &options.patterns {
            for pattern in patterns {
                match pattern {
                    Patterns::WithOptions(pattern_options) => {
                        results.extend(pattern_options.check_import_restrictions(
                            node,
                            &module_name,
                            &import_source_text,
                        ));
                    }
                }
            }
        }

        results
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
