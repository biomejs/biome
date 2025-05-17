use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
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
use biome_rowan::{AstNode, AstSeparatedList, SyntaxNode, SyntaxNodeCast, SyntaxToken, TextRange};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use regex::RegexBuilder;
use rustc_hash::FxHashMap;
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
    ///         },
    ///         "patterns": ["utils/*", "!utils/foo"]
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
    /// ```js,expect_diagnostic,use_options
    /// import "utils/bar";
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
    /// ```js,use_options
    /// import "utils/foo";
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
    ///        "patterns": ["utils/*", "!utils/foo"]
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
    ///
    /// ### `patterns`
    ///
    /// This option allows you to specify multiple modules to restrict using gitignore-style patterns or regular expressions.
    ///
    /// ```json,options
    /// {
    ///    "options": {
    ///         "patterns": ["utils/*", "!utils/foo"]
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// import foo from "utils/foo";
    /// import bar from "utils/bar";
    /// ```
    ///
    /// ### `group`
    ///
    /// The patterns array can also include objects. The group property is used to specify the gitignore-style patterns for restricting modules and the message property is used to specify a custom message.
    /// **`group` cannot be used in combination with `regex`.**
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
    /// ### `regex`
    ///
    /// The regex property is used to specify the regex patterns for restricting modules.
    /// **`regex` cannot be used in combination with `group`.**
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "patterns": [{
    ///             "regex": "^import-foo/\\d{4}-\\d{2}-\\d{2}$",
    ///             "message": "import-foo/ + particular date format is deprecated."
    ///         }]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// import { foo } from 'import-foo/2025-05-17';
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// import { foo } from 'import-foo/foo';
    /// ```
    ///
    /// ### `caseSensitive`
    ///
    /// This is a boolean option and sets the patterns specified in the group or regex properties to be case-sensitive when true. Default is false.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "patterns": [{
    ///             "group": ["import-foo/prefix[A-Z]*"],
    ///             "caseSensitive": true
    ///         }]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// import { Foo } from 'import-foo/prefixFoo';
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// import { Foo } from 'import-foo/prefixfoo';
    /// ```
    ///
    /// ### `importNames`
    ///
    /// You can also specify importNames within objects inside the patterns array. In this case, the specified names apply only to the associated group or regex property.
    /// **`importNames` cannot be used in combination with `importNamePattern` , `allowImportNames` and `allowImportNamePattern`.**
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "patterns": [{
    ///             "group": ["utils/*"],
    ///             "importNames": ["isEmpty"],
    ///             "message": "Use 'isEmpty' from lodash instead."
    ///         }]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// import { isEmpty } from 'utils/foo';
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// import { Foo } from 'utils/foo';
    /// ```
    ///
    /// ### `allowImportNames`
    ///
    /// You can also specify allowImportNames within objects inside the patterns array. In this case, the specified names apply only to the associated group or regex property.
    /// **`allowImportNames` cannot be used in combination with `importNames`, `importNamePattern` and `allowImportNamePattern`.**
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "patterns": [{
    ///             "group": ["utils/*"],
    ///             "allowImportNames": ["isEmpty"],
    ///             "message": "Please use only 'isEmpty' from utils."
    ///         }]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// import { Foo } from 'utils/foo';
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// import { isEmpty } from 'utils/foo';
    /// ```
    ///
    /// ### `importNamePattern`
    ///
    /// This option allows you to use regex patterns to restrict import names.
    /// **`importNamePattern` cannot be used in combination with `importNames` , `allowImportNames` and `allowImportNamePattern`.**
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "patterns": [{
    ///             "group": ["import-foo/*"],
    ///             "importNamePattern": "^foo"
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
    /// import { Foo } from 'import-foo/foo';
    /// ```
    ///
    /// ### `allowImportNamePattern`
    ///
    /// Inverse of importNamePattern, this option allows imports that matches the specified regex pattern.
    /// **`allowImportNamePattern` cannot be used in combination with `importNames` , `importNamePattern` and `allowImportNames`.**
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "patterns": [{
    ///             "group": ["import-foo/*"],
    ///             "allowImportNamePattern": "^foo"
    ///         }]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// import { Foo } from 'import-foo/foo';
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// import { foo } from 'import-foo/foo';
    /// ```
    pub NoRestrictedImports {
        version: "1.6.0",
        name: "noRestrictedImports",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-restricted-imports"),
            RuleSource::EslintTypeScript("no-restricted-imports")
        ],
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for NoRestrictedImports {
    type Query = Ast<AnyJsImportLike>;
    type State = RestrictedImportMessage;
    type Signals = Vec<Self::State>;
    type Options = Box<RestrictedImportsOptions>;

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

        if let Some(paths) = options.paths.get(import_source) {
            let path_options: PathOptions = paths.clone().into();
            path_options.check_import_restrictions(node, &module_name, import_source)
        } else if let Some(patterns) = &options.patterns {
            check_patterns_import_restrictions(node, patterns, &module_name, import_source)
        } else {
            return vec![];
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
pub struct RestrictedImportsOptions {
    /// A list of import paths that should trigger the rule.
    #[serde(skip_serializing_if = "FxHashMap::is_empty")]
    paths: FxHashMap<Box<str>, Paths>,

    /// A list of gitignore-style patterns or regular expressions that should trigger the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    patterns: Option<Box<[Patterns]>>,
}

/// Specifies why a specific import is allowed or disallowed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cause {
    /// Reason: The import source is forbidden or allowed.
    ImportSource,
    /// Reason: A set of forbidden import names has been defined via `importNames`.
    ImportNames,
    /// Reason: A set of allowed import names has been defined via `allowImportNames`.
    AllowImportNames,
}

/// Specifies whether a specific import is (dis)allowed, and why it is allowed/disallowed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Restriction {
    allowed: bool,
    cause: Cause,
}

impl Restriction {
    const fn allowed(cause: Cause) -> Self {
        Self {
            allowed: true,
            cause,
        }
    }
    const fn forbidden(cause: Cause) -> Self {
        Self {
            allowed: false,
            cause,
        }
    }
    fn is_allowed(self) -> bool {
        self.allowed
    }
    fn is_forbidden(self) -> bool {
        !self.allowed
    }
}

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
pub struct PathOptions {
    /// The message to display when this module is imported.
    #[serde(skip_serializing_if = "str::is_empty")]
    message: Box<str>,

    /// Names of the exported members that should not be used.
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    import_names: Box<[Box<str>]>,

    /// Names of the exported members that allowed to be not be used.
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    allow_import_names: Box<[Box<str>]>,
}

impl PathOptions {
    fn has_import_name_constraints(&self) -> bool {
        !self.import_names.is_empty() || !self.allow_import_names.is_empty()
    }

    fn check_restriction(&self, imported_name: &str) -> Restriction {
        // Deny all imports except for the names specified in allow_import_names
        if !self.allow_import_names.is_empty() {
            if self
                .allow_import_names
                .iter()
                .any(|n| n.as_ref() == imported_name)
            {
                Restriction::allowed(Cause::AllowImportNames)
            } else {
                Restriction::forbidden(Cause::AllowImportNames)
            }
        // Allow all imports except for the names specified in import_names
        } else if !self.import_names.is_empty() {
            if self
                .import_names
                .iter()
                .any(|n| n.as_ref() == imported_name)
            {
                Restriction::forbidden(Cause::ImportNames)
            } else {
                Restriction::allowed(Cause::ImportNames)
            }
        } else {
            // Deny all imports from this module
            Restriction::forbidden(Cause::ImportSource)
        }
    }

    fn check_import_restrictions(
        &self,
        node: &AnyJsImportLike,
        module_name: &SyntaxToken<JsLanguage>,
        import_source: &str,
    ) -> Vec<RestrictedImportMessage> {
        match node {
            AnyJsImportLike::JsModuleSource(module_source_node) => {
                if !self.has_import_name_constraints() {
                    // All imports disallowed, add diagnostic to the import source
                    vec![RestrictedImportMessage {
                        location: module_name.text_trimmed_range(),
                        message: self.message(import_source, "", Cause::ImportSource),
                        import_source: import_source.to_string(),
                        allowed_import_names: [].into(),
                    }]
                } else {
                    // Check (and possibly report) each imported name individually
                    let mut visitor = RestrictedImportVisitor {
                        import_source,
                        options: Options::PathOptions(self),
                        results: vec![],
                    };
                    visitor.visit_import(module_source_node);
                    visitor.results
                }
            }
            AnyJsImportLike::JsImportCallExpression(import_call) => {
                // TODO: We have to parse the context of the import() call to determine
                // which exports are being used/whether this should be considered a
                // namespace import, a side-effect import (the two of which may
                // be difficult to distinguish) or a collection of named imports.
                if !self.has_import_name_constraints() {
                    // All imports disallowed, add diagnostic to the import source
                    vec![RestrictedImportMessage {
                        location: module_name.text_trimmed_range(),
                        message: self.message(import_source, "", Cause::ImportSource),
                        import_source: import_source.to_string(),
                        allowed_import_names: [].into(),
                    }]
                } else {
                    // Check (and possibly report) each imported name individually
                    let mut visitor = RestrictedImportVisitor {
                        import_source,
                        options: Options::PathOptions(self),
                        results: vec![],
                    };
                    visitor.visit_import_call(import_call);
                    visitor.results
                }
            }
            AnyJsImportLike::JsCallExpression(_expression) => {
                let restriction =
                    self.check_restriction(RestrictedImportVisitor::DEFAULT_IMPORT_ALIAS);

                if restriction.is_forbidden() {
                    // require() calls can only import the default import, so
                    // there are no individual import names to check or report on.
                    vec![RestrictedImportMessage {
                        location: module_name.text_trimmed_range(),
                        message: self.message(import_source, "", Cause::ImportSource),
                        import_source: import_source.to_string(),
                        allowed_import_names: [].into(),
                    }]
                } else {
                    vec![]
                }
            }
        }
    }

    fn message(&self, import_source: &str, imported_name: &str, cause: Cause) -> String {
        if !self.message.is_empty() {
            return self.message.to_string();
        }
        default_message(import_source, imported_name, cause)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum Paths {
    /// The message to display when this module is imported.
    Plain(Box<str>),
    /// Additional options to configure the message and allowed/disallowed import names.
    WithOptions(PathOptions),
}

impl From<Paths> for PathOptions {
    fn from(paths: Paths) -> Self {
        match paths {
            Paths::Plain(message) => Self {
                message,
                import_names: [].into(),
                allow_import_names: [].into(),
            },
            Paths::WithOptions(path_options) => path_options,
        }
    }
}

impl Deserializable for Paths {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            biome_deserialize::Deserializable::deserialize(ctx, value, name).map(Self::Plain)
        } else {
            biome_deserialize::Deserializable::deserialize(ctx, value, name).map(Self::WithOptions)
        }
    }
}

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
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct PatternOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<Box<[Box<str>]>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<Box<str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Box<str>>,

    case_sensitive: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    import_names: Option<Box<[Box<str>]>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    allow_import_names: Option<Box<[Box<str>]>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    import_name_pattern: Option<Box<str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    allow_import_name_pattern: Option<Box<str>>,
}

impl PatternOptions {
    fn has_import_name_constraints(&self) -> bool {
        self.import_names.as_ref().is_some_and(|v| !v.is_empty())
            || self
                .allow_import_names
                .as_ref()
                .is_some_and(|v| !v.is_empty())
            || self.import_name_pattern.as_ref().is_some()
            || self.allow_import_name_pattern.as_ref().is_some()
    }

    fn check_restriction(&self, imported_name: &str) -> Restriction {
        // allow_import_names
        if let Some(allow_import_names) = &self.allow_import_names {
            if allow_import_names
                .iter()
                .any(|n| n.as_ref() == imported_name)
            {
                Restriction::allowed(Cause::AllowImportNames)
            } else {
                Restriction::forbidden(Cause::AllowImportNames)
            }

        // import_names
        } else if let Some(import_names) = &self.import_names {
            if import_names.iter().any(|n| n.as_ref() == imported_name) {
                Restriction::forbidden(Cause::ImportNames)
            } else {
                Restriction::allowed(Cause::ImportNames)
            }

        // import_name_pattern
        } else if let Some(import_name_pattern) = &self.import_name_pattern {
            let re = RegexBuilder::new(import_name_pattern.as_ref())
                .build()
                .unwrap();
            if re.is_match(imported_name) {
                Restriction::forbidden(Cause::ImportNames)
            } else {
                Restriction::allowed(Cause::ImportNames)
            }

        // allow_import_name_pattern
        } else if let Some(allow_import_name_pattern) = &self.allow_import_name_pattern {
            if RegexBuilder::new(allow_import_name_pattern)
                .build()
                .unwrap()
                .is_match(imported_name)
            {
                Restriction::allowed(Cause::AllowImportNames)
            } else {
                Restriction::forbidden(Cause::AllowImportNames)
            }
        } else {
            Restriction::forbidden(Cause::ImportSource)
        }
    }

    fn message(&self, import_source: &str, imported_name: &str, cause: Cause) -> String {
        if self.message.is_some() {
            return self.message.as_ref().map(|msg| msg.to_string()).unwrap();
        }
        default_message(import_source, imported_name, cause)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Patterns {
    Simple(Box<str>),
    WithOptions(PatternOptions),
}

impl Deserializable for Patterns {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            biome_deserialize::Deserializable::deserialize(ctx, value, name).map(Self::Simple)
        } else {
            biome_deserialize::Deserializable::deserialize(ctx, value, name).map(Self::WithOptions)
        }
    }
}

#[cfg(feature = "schemars")]
impl schemars::JsonSchema for Patterns {
    fn schema_name() -> String {
        "Patterns".into()
    }

    fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::*;

        let simple = Schema::Object(SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            metadata: Some(Box::new(Metadata {
                description: Some("A simple gitignore-style pattern string.".into()),
                ..Default::default()
            })),
            ..Default::default()
        });

        let mut with_options = generator.subschema_for::<PatternOptions>();
        // add description
        if let schemars::schema::Schema::Object(mut obj) = with_options {
            obj.metadata = Some(Box::new(schemars::schema::Metadata {
            description: Some(
                "Additional options to configure the message and allowed/disallowed import names and modules."
                    .into(),
            ),
            ..Default::default()
        }));
            with_options = schemars::schema::Schema::Object(obj);
        }

        Schema::Object(SchemaObject {
            subschemas: Some(Box::new(SubschemaValidation {
                one_of: Some(vec![simple, with_options]),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

#[cfg(feature = "schemars")]
impl schemars::JsonSchema for PatternOptions {
    fn schema_name() -> String {
        "PatternOptions".into()
    }

    fn json_schema(_generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::Map;
        use schemars::schema::*;

        fn string_schema(desc: &str) -> Schema {
            Schema::Object(SchemaObject {
                instance_type: Some(InstanceType::String.into()),
                metadata: Some(Box::new(Metadata {
                    description: Some(desc.into()),
                    ..Default::default()
                })),
                ..Default::default()
            })
        }
        fn array_string_schema(desc: &str) -> Schema {
            Schema::Object(SchemaObject {
                instance_type: Some(InstanceType::Array.into()),
                array: Some(Box::new(ArrayValidation {
                    items: Some(SingleOrVec::Single(Box::new(Schema::Object(
                        SchemaObject {
                            instance_type: Some(InstanceType::String.into()),
                            ..Default::default()
                        },
                    )))),
                    min_items: Some(1),
                    ..Default::default()
                })),
                metadata: Some(Box::new(Metadata {
                    description: Some(desc.into()),
                    ..Default::default()
                })),
                ..Default::default()
            })
        }

        fn base_props() -> Map<String, Schema> {
            let mut m = Map::new();
            m.insert(
                "message".into(),
                string_schema("A custom message for diagnostics related to this pattern."),
            );
            m.insert(
                "caseSensitive".into(),
                Schema::Object(SchemaObject {
                    instance_type: Some(InstanceType::Boolean.into()),
                    metadata: Some(Box::new(Metadata {
                        description: Some(
                            "Whether the patterns are case-sensitive. Defaults to `false`.".into(),
                        ),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
            );
            m
        }

        let mut variants = Vec::<Schema>::new();
        let group_variants = ["group", "regex"];
        let import_variants = [
            "",
            "importNames",
            "allowImportNames",
            "importNamePattern",
            "allowImportNamePattern",
        ];
        for &g in &group_variants {
            for &i in &import_variants {
                let mut props = base_props();

                if g == "group" {
                    props.insert(
                        "group".into(),
                        array_string_schema(
                            "An array of gitignore-style patterns. Cannot be used with regex.",
                        ),
                    );
                } else if g == "regex" {
                    props.insert(
                        "regex".into(),
                        string_schema(
                            "A regular expression pattern string. Cannot be used with group.",
                        ),
                    );
                }

                match i {
                    "importNames" => {
                        props.insert("importNames".into(), array_string_schema("An array of specific import names to forbid within the matched modules. Cannot be used with importNamePattern, allowImportNames and allowImportNamePattern."));
                    }
                    "allowImportNames" => {
                        props.insert("allowImportNames".into(), array_string_schema("An array of specific import names to allow within the matched modules. Cannot be used with importNames, importNamePattern and allowImportNamePattern."));
                    }
                    "importNamePattern" => {
                        props.insert("importNamePattern".into(), string_schema("A regex pattern for import names to forbid within the matched modules. Cannot be used with importNames, allowImportNames and allowImportNamePattern."));
                    }
                    "allowImportNamePattern" => {
                        props.insert("allowImportNamePattern".into(), string_schema("A regex pattern for import names to allow within the matched modules. Cannot be used with importNames, importNamePattern and allowImportNames."));
                    }
                    _ => {}
                }

                let mut obj = ObjectValidation {
                    properties: props,
                    additional_properties: Some(Box::new(Schema::Bool(false))),
                    ..Default::default()
                };
                if g == "group" {
                    obj.required.insert("group".into());
                } else if g == "regex" {
                    obj.required.insert("regex".into());
                }
                if !i.is_empty() {
                    obj.required.insert(i.into());
                }

                variants.push(Schema::Object(SchemaObject {
                    instance_type: Some(InstanceType::Object.into()),
                    object: Some(Box::new(obj)),
                    ..Default::default()
                }));
            }
        }

        Schema::Object(SchemaObject {
            subschemas: Some(Box::new(SubschemaValidation {
                one_of: Some(variants),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

fn check_patterns_import_restrictions(
    node: &AnyJsImportLike,
    patterns: &[Patterns],
    module_name: &SyntaxToken<JsLanguage>,
    import_source: &str,
) -> Vec<RestrictedImportMessage> {
    let mut builder_for_simple = GitignoreBuilder::new("");
    let mut last_matched_options: Option<&PatternOptions> = None;

    for pattern in patterns {
        match pattern {
            Patterns::Simple(glob) => {
                builder_for_simple.add_line(None, glob).unwrap();
            }
            Patterns::WithOptions(pattern_options) => {
                if match_pattern_options(import_source, pattern_options) {
                    last_matched_options = Some(pattern_options);
                }
            }
        }
    }

    let gitignore_for_simple: Gitignore = builder_for_simple
        // The default case-sensitive option is false.
        // The Simple pattern has no case-sensitive option.
        .case_insensitive(true)
        .unwrap()
        .build()
        .unwrap();

    if gitignore_for_simple
        .matched(import_source, false)
        .is_ignore()
    {
        return vec![RestrictedImportMessage::simple(module_name, import_source)];
    }

    if let Some(pattern_options) = last_matched_options {
        return handle_pattern_options(node, pattern_options, module_name, import_source);
    }

    vec![]
}

fn match_pattern_options(import_source: &str, pattern_options: &PatternOptions) -> bool {
    if let Some(group) = &pattern_options.group {
        let mut builder = GitignoreBuilder::new("");
        for pattern in group.iter() {
            builder.add_line(None, pattern).unwrap();
        }
        return builder
            .case_insensitive(!pattern_options.case_sensitive)
            .unwrap()
            .build()
            .unwrap()
            .matched(import_source, false)
            .is_ignore();
    }
    if let Some(regex) = &pattern_options.regex {
        return RegexBuilder::new(regex)
            .case_insensitive(!pattern_options.case_sensitive)
            .build()
            .unwrap()
            .is_match(import_source);
    }
    false
}

fn handle_pattern_options(
    node: &AnyJsImportLike,
    pattern_options: &PatternOptions,
    module_name: &SyntaxToken<JsLanguage>,
    import_source: &str,
) -> Vec<RestrictedImportMessage> {
    if !pattern_options.has_import_name_constraints() {
        return vec![RestrictedImportMessage::with_additional_message(
            module_name,
            import_source,
            pattern_options.message.clone(),
        )];
    }

    let import_ctx = match node {
        AnyJsImportLike::JsModuleSource(module_source) => {
            Some(ImportCtx::JsModuleSource(module_source))
        }
        AnyJsImportLike::JsImportCallExpression(import_call) => {
            Some(ImportCtx::JsImportCallExpression(import_call))
        }
        AnyJsImportLike::JsCallExpression(expression) => {
            Some(ImportCtx::JsCallExpression(expression))
        }
    };

    let mut visitor = RestrictedImportVisitor {
        import_source,
        options: Options::PatternOptions(pattern_options),
        results: vec![],
    };

    match import_ctx {
        Some(ImportCtx::JsModuleSource(module_source)) => {
            visitor.visit_import(module_source);
            visitor.results
        }
        Some(ImportCtx::JsImportCallExpression(import_call)) => {
            visitor.visit_import_call(import_call);
            visitor.results
        }
        Some(ImportCtx::JsCallExpression(_)) => {
            vec![RestrictedImportMessage::with_additional_message(
                module_name,
                import_source,
                pattern_options.message.clone(),
            )]
        }
        None => {
            vec![RestrictedImportMessage::with_additional_message(
                module_name,
                import_source,
                pattern_options.message.clone(),
            )]
        }
    }
}

fn default_message(import_source: &str, imported_name: &str, cause: Cause) -> String {
    match cause {
        Cause::ImportSource => format!("Do not import '{import_source}'."),
        Cause::ImportNames | Cause::AllowImportNames => {
            if imported_name == RestrictedImportVisitor::BARE_IMPORT_ALIAS {
                format!("Do not import '{import_source}' through a side-effect import.")
            } else {
                format!("Do not import '{imported_name}' from '{import_source}'.")
            }
        }
    }
}

pub enum ImportCtx<'a> {
    JsModuleSource(&'a JsModuleSource),
    JsImportCallExpression(&'a JsImportCallExpression),
    JsCallExpression(&'a JsCallExpression),
}

pub enum Options<'a> {
    PathOptions(&'a PathOptions),
    PatternOptions(&'a PatternOptions),
}

struct RestrictedImportVisitor<'a> {
    import_source: &'a str,
    options: Options<'a>,
    results: Vec<RestrictedImportMessage>,
}

impl RestrictedImportVisitor<'_> {
    pub const BARE_IMPORT_ALIAS: &'static str = "";
    pub const NAMESPACE_IMPORT_ALIAS: &'static str = "*";
    pub const DEFAULT_IMPORT_ALIAS: &'static str = "default";

    /// Analyze the context of an `import(...)` call to find the imported names,
    /// then validate that each of the names is allowed to be imported.
    pub fn visit_import_call(&mut self, import_call: &JsImportCallExpression) -> Option<()> {
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
        if let Some(bindings) = Self::get_context_for_import_call(import_call) {
            match bindings {
                AnyJsBindingPattern::AnyJsBinding(namespace_binding) => match namespace_binding {
                    // const ... = import(...)
                    biome_js_syntax::AnyJsBinding::JsIdentifierBinding(namespace_binding) => {
                        // const namespaceImport = import(...)
                        return self.visit_namespace_binding(&namespace_binding);
                    }
                    _ => {
                        // Use fallback instead
                    }
                },
                AnyJsBindingPattern::JsObjectBindingPattern(named_bindings) => {
                    // const { ... } = await import(...)
                    return self.visit_named_bindings(&named_bindings);
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
        self.visit_special_import_node(import_call.syntax(), Self::NAMESPACE_IMPORT_ALIAS)
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
    pub fn visit_import(&mut self, module_source_node: &JsModuleSource) -> Option<()> {
        // Only certain imports are allowed/disallowed, add diagnostic to each disallowed import
        let clause = module_source_node.syntax().parent()?;
        match clause.kind() {
            JsSyntaxKind::JS_IMPORT_BARE_CLAUSE => {
                let side_effect_import: JsImportBareClause = clause.cast()?;
                self.visit_side_effect_import(&side_effect_import)
            }
            JsSyntaxKind::JS_IMPORT_COMBINED_CLAUSE => {
                let import_combined_clause: JsImportCombinedClause = clause.cast()?;
                if let Ok(default_specifier) = import_combined_clause.default_specifier() {
                    self.visit_default_import(&default_specifier);
                }
                if let Ok(combined_specifier) = import_combined_clause.specifier() {
                    self.visit_combined_specifier(&combined_specifier);
                }
                Some(())
            }
            JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE => {
                let import_named_clause: JsImportNamedClause = clause.cast()?;
                let import_specifiers = import_named_clause.named_specifiers().ok()?;
                self.visit_named_imports(&import_specifiers)
            }
            JsSyntaxKind::JS_EXPORT_NAMED_FROM_CLAUSE => {
                let export_named_from_clause = clause.cast::<JsExportNamedFromClause>()?;
                let import_specifiers = export_named_from_clause.specifiers();
                self.visit_named_reexports(&import_specifiers)
            }
            JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE => {
                let import_default_clause: JsImportDefaultClause = clause.cast()?;
                let default_specifier = import_default_clause.default_specifier().ok()?;
                self.visit_default_import(&default_specifier)
            }
            JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE => {
                let import_namespace_clause: JsImportNamespaceClause = clause.cast()?;
                let namespace_specifier = import_namespace_clause.namespace_specifier().ok()?;
                self.visit_namespace_import(&namespace_specifier)
            }
            JsSyntaxKind::JS_EXPORT_FROM_CLAUSE => {
                let reexport_namespace_clause: JsExportFromClause = clause.cast()?;
                self.visit_namespace_reexport(&reexport_namespace_clause)
            }
            _ => None,
        }
    }

    fn visit_combined_specifier(
        &mut self,
        combined_specifier: &AnyJsCombinedSpecifier,
    ) -> Option<()> {
        match combined_specifier {
            AnyJsCombinedSpecifier::JsNamedImportSpecifiers(named_imports) => {
                self.visit_named_imports(named_imports)
            }
            AnyJsCombinedSpecifier::JsNamespaceImportSpecifier(namespace_import) => {
                self.visit_namespace_import(namespace_import)
            }
        }
    }

    fn visit_named_imports(&mut self, named_imports: &JsNamedImportSpecifiers) -> Option<()> {
        let import_specifiers = named_imports.specifiers();
        for import_specifier in import_specifiers.iter().flatten() {
            self.visit_named_or_shorthand_import(&import_specifier);
        }
        Some(())
    }

    fn visit_named_reexports(
        &mut self,
        named_reexports: &JsExportNamedFromSpecifierList,
    ) -> Option<()> {
        for export_specifier in named_reexports.iter().flatten() {
            self.visit_named_or_shorthand_reexport(&export_specifier);
        }
        Some(())
    }

    fn visit_named_bindings(&mut self, named_imports: &JsObjectBindingPattern) -> Option<()> {
        let import_bindings = named_imports.properties();
        for import_binding in import_bindings.iter().flatten() {
            self.visit_named_or_shorthand_binding(&import_binding);
        }
        Some(())
    }

    fn visit_named_or_shorthand_import(
        &mut self,
        import_specifier: &AnyJsNamedImportSpecifier,
    ) -> Option<()> {
        match import_specifier {
            AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(shorthand_import) => {
                self.visit_shorthand_import(shorthand_import)
            }
            AnyJsNamedImportSpecifier::JsNamedImportSpecifier(named_import) => {
                self.visit_named_import(named_import)
            }
            AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => None,
        }
    }

    fn visit_named_or_shorthand_binding(
        &mut self,
        import_binding: &AnyJsObjectBindingPatternMember,
    ) -> Option<()> {
        match import_binding {
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                shorthand_import,
            ) => self.visit_shorthand_binding(shorthand_import),
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(named_import) => {
                self.visit_named_binding(named_import)
            }
            _ => None,
        }
    }

    /// Checks whether this bare import of the form `import from 'source'` is allowed.
    fn visit_side_effect_import(&mut self, bare_import: &JsImportBareClause) -> Option<()> {
        let source_token = bare_import
            .source()
            .ok()?
            .as_js_module_source()?
            .value_token()
            .ok()?;
        self.visit_special_import_token(&source_token, Self::BARE_IMPORT_ALIAS)
    }

    /// Checks whether this import of the form `local_name` (as in `import local_name from 'source'`) is allowed.
    fn visit_default_import(&mut self, default_import: &JsDefaultImportSpecifier) -> Option<()> {
        let local_name = default_import
            .local_name()
            .ok()?
            .as_js_identifier_binding()?
            .name_token()
            .ok()?;
        self.visit_special_import_token(&local_name, Self::DEFAULT_IMPORT_ALIAS)
    }

    /// Checks whether this import of the form `* as local_name` is allowed.
    fn visit_namespace_import(
        &mut self,
        namespace_import: &JsNamespaceImportSpecifier,
    ) -> Option<()> {
        self.visit_special_import_token(
            &namespace_import.star_token().ok()?,
            Self::NAMESPACE_IMPORT_ALIAS,
        )
    }

    /// Checks whether this namespace reexport of the form `export * from ...` is allowed.
    fn visit_namespace_reexport(&mut self, namespace_reexport: &JsExportFromClause) -> Option<()> {
        self.visit_special_import_token(
            &namespace_reexport.star_token().ok()?,
            Self::NAMESPACE_IMPORT_ALIAS,
        )
    }

    /// Checks whether this import of the form `const local_name = import(...)` is allowed.
    fn visit_namespace_binding(&mut self, namespace_import: &JsIdentifierBinding) -> Option<()> {
        self.visit_special_import_node(namespace_import.syntax(), Self::NAMESPACE_IMPORT_ALIAS)
    }

    /// Checks whether this import of the form `{ imported_name }` is allowed.
    fn visit_shorthand_import(
        &mut self,
        shorthand_import: &JsShorthandNamedImportSpecifier,
    ) -> Option<()> {
        self.visit_imported_identifier(
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
        &mut self,
        shorthand_import: &JsObjectBindingPatternShorthandProperty,
    ) -> Option<()> {
        self.visit_imported_identifier(
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
    fn visit_named_import(&mut self, named_import: &JsNamedImportSpecifier) -> Option<()> {
        self.visit_imported_identifier(&named_import.name().ok()?.value().ok()?)
    }

    /// Checks whether this import of the form `{ imported_name }` or `{ imported_name as exported_name }`
    /// (including `{ default as exported_name }`) is allowed.
    fn visit_named_or_shorthand_reexport(
        &mut self,
        named_reexport: &JsExportNamedFromSpecifier,
    ) -> Option<()> {
        self.visit_imported_identifier(&named_reexport.source_name().ok()?.value().ok()?)
    }

    /// Checks whether this import of the form `{ imported_name: local_name }`
    /// (including `{ default: local_name }` and `{ "imported name": local_name `) is allowed.
    fn visit_named_binding(&mut self, named_import: &JsObjectBindingPatternProperty) -> Option<()> {
        self.visit_imported_identifier(
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
    fn visit_imported_identifier(&mut self, name_token: &SyntaxToken<JsLanguage>) -> Option<()> {
        // TODO: inner_string_text removes quotes but does not e.g. decode escape sequences.
        //       If the imported name uses e.g. Unicode escape sequences, this may cause
        //       problems because path_options.(allow_)import_names contains decoded
        //       strings, while inner_string_text(name_token) returns encoded strings.
        self.visit_special_import_token(name_token, inner_string_text(name_token).text())
    }

    /// Checks whether the import specified by `name_or_alias` is allowed.
    /// and records a diagnostic for `import_node.text_trimmed_range()` if not.
    fn visit_special_import_node(
        &mut self,
        import_node: &SyntaxNode<JsLanguage>,
        name_or_alias: &str,
    ) -> Option<()> {
        match self.options {
            Options::PathOptions(path_options) => {
                let restriction = path_options.check_restriction(name_or_alias);
                if restriction.is_allowed() {
                    return None;
                }
                self.results.push(RestrictedImportMessage {
                    location: import_node.text_trimmed_range(),
                    message: path_options.message(
                        self.import_source,
                        name_or_alias,
                        restriction.cause,
                    ),
                    import_source: self.import_source.to_string(),
                    allowed_import_names: path_options.allow_import_names.clone(),
                });
                Some(())
            }
            Options::PatternOptions(pattern_options) => {
                let restriction = pattern_options.check_restriction(name_or_alias);
                if restriction.is_allowed() {
                    return None;
                }
                let allow_import_names: Box<[Box<str>]> = pattern_options
                    .allow_import_names
                    .as_ref()
                    .map_or_else(|| Vec::new().into_boxed_slice(), |names| names.clone());
                self.results.push(RestrictedImportMessage {
                    location: import_node.text_trimmed_range(),
                    message: pattern_options.message(
                        self.import_source,
                        name_or_alias,
                        restriction.cause,
                    ),
                    import_source: self.import_source.to_string(),
                    allowed_import_names: allow_import_names,
                });
                Some(())
            }
        }
    }

    /// Checks whether the import specified by `name_or_alias` is allowed.
    /// and records a diagnostic for `import_token.text_trimmed_range()` if not.
    fn visit_special_import_token(
        &mut self,
        import_token: &SyntaxToken<JsLanguage>,
        name_or_alias: &str,
    ) -> Option<()> {
        match self.options {
            Options::PathOptions(path_options) => {
                let restriction = path_options.check_restriction(name_or_alias);
                if restriction.is_allowed() {
                    return None;
                }
                self.results.push(RestrictedImportMessage {
                    location: import_token.text_trimmed_range(),
                    message: path_options.message(
                        self.import_source,
                        name_or_alias,
                        restriction.cause,
                    ),
                    import_source: self.import_source.to_string(),
                    allowed_import_names: path_options.allow_import_names.clone(),
                });
                Some(())
            }
            Options::PatternOptions(pattern_options) => {
                let restriction = pattern_options.check_restriction(name_or_alias);
                if restriction.is_allowed() {
                    return None;
                }

                let allow_import_names: Box<[Box<str>]> = pattern_options
                    .allow_import_names
                    .as_ref()
                    .map_or_else(|| Vec::new().into_boxed_slice(), |names| names.clone());

                self.results.push(RestrictedImportMessage {
                    location: import_token.text_trimmed_range(),
                    message: pattern_options.message(
                        self.import_source,
                        name_or_alias,
                        restriction.cause,
                    ),
                    import_source: self.import_source.to_string(),
                    allowed_import_names: allow_import_names,
                });

                Some(())
            }
        }
    }
}

pub struct RestrictedImportMessage {
    location: TextRange,
    message: String,
    import_source: String,
    allowed_import_names: Box<[Box<str>]>,
}

impl RestrictedImportMessage {
    fn simple(token: &SyntaxToken<JsLanguage>, import_source: &str) -> Self {
        Self::with_additional_message(token, import_source, None)
    }

    fn with_additional_message(
        token: &SyntaxToken<JsLanguage>,
        import_source: &str,
        additional_message: Option<Box<str>>,
    ) -> Self {
        let base_message = format!("'{}' import is restricted by a pattern.", import_source);
        let message = match additional_message {
            Some(additional_message) => format!("{base_message} {additional_message}"),
            None => base_message,
        };
        Self {
            location: token.text_trimmed_range(),
            message,
            import_source: import_source.to_string(),
            allowed_import_names: [].into(),
        }
    }
}
