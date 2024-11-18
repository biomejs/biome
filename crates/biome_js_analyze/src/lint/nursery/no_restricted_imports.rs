use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationDiagnostic,
};
use biome_js_syntax::{
    inner_string_text, AnyJsArrowFunctionParameters, AnyJsBindingPattern, AnyJsCombinedSpecifier,
    AnyJsExpression, AnyJsImportLike, AnyJsNamedImportSpecifier, AnyJsObjectBindingPatternMember,
    JsCallExpression, JsDefaultImportSpecifier, JsExportFromClause, JsExportNamedFromClause,
    JsExportNamedFromSpecifier, JsExportNamedFromSpecifierList, JsIdentifierBinding,
    JsImportBareClause, JsImportCallExpression, JsImportCombinedClause, JsImportDefaultClause,
    JsImportNamedClause, JsImportNamespaceClause, JsLanguage, JsModuleSource,
    JsNamedImportSpecifier, JsNamedImportSpecifiers, JsNamespaceImportSpecifier,
    JsObjectBindingPattern, JsObjectBindingPatternProperty,
    JsObjectBindingPatternShorthandProperty, JsShorthandNamedImportSpecifier,
    JsStaticMemberExpression, JsSyntaxKind, JsVariableDeclarator,
};
use biome_rowan::{AstNode, SyntaxNode, SyntaxNodeCast, SyntaxToken, TextRange};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

declare_lint_rule! {
    /// Disallow specified modules when loaded by import or require.
    ///
    /// ## Options
    ///
    /// ```json,options
    /// {
    ///     "noRestrictedImports": {
    ///         "options": {
    ///             "paths": {
    ///                 "lodash": "Using lodash is not encouraged",
    ///                 "underscore": "Using underscore is not encouraged",
    ///                 "import-foo": {
    ///                     "importNames": ["Bar"],
    ///                     "message": "Please use Bar from /import-bar/baz/ instead."
    ///                 },
    ///                 "import-bar": {
    ///                   "allowImportNames": ["Bar"],
    ///                   "message": "Please use only Bar from import-bar."
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ## Examples
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
    pub NoRestrictedImports {
        version: "1.6.0",
        name: "noRestrictedImports",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-restricted-imports"),
            RuleSource::EslintTypeScript("no-restricted-imports"),
        ],
        recommended: false,
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
    /// A list of names that should trigger the rule
    #[serde(skip_serializing_if = "FxHashMap::is_empty")]
    paths: FxHashMap<Box<str>, CustomRestrictedImport>,
}

/// Specifies why a specific import is allowed or disallowed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ImportRestrictionCause {
    /// Reason: The import source is forbidden or allowed.
    ImportSource,
    /// Reason: A set of forbidden import names has been defined via `importNames`.
    ImportNames,
    /// Reason: A set of allowed import names has been defined via `allowImportNames`.
    AllowImportNames,
}

/// Specifies whether a specific import is (dis)allowed, and why it is allowed/disallowed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ImportRestrictionStatus(bool, ImportRestrictionCause);

impl ImportRestrictionStatus {
    pub fn is_allowed(&self) -> bool {
        self.0
    }

    pub fn is_forbidden(&self) -> bool {
        !self.0
    }

    pub fn reason(&self) -> ImportRestrictionCause {
        self.1
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
pub struct CustomRestrictedImportOptions {
    message: String,
    import_names: Box<[Box<str>]>,
    allow_import_names: Box<[Box<str>]>,
}

impl CustomRestrictedImportOptions {
    pub fn has_import_name_patterns(&self) -> bool {
        !self.import_names.is_empty() || !self.allow_import_names.is_empty()
    }

    fn is_import_allowed(&self, imported_name: &str) -> ImportRestrictionStatus {
        if !self.allow_import_names.is_empty() {
            // Deny all imports except for the names specified in allow_import_names
            let is_allowed = self
                .allow_import_names
                .iter()
                .any(|name| &**name == imported_name);

            ImportRestrictionStatus(is_allowed, ImportRestrictionCause::AllowImportNames)
        } else if !self.import_names.is_empty() {
            // Allow all imports except for the names specified in import_names
            let is_forbidden = self
                .import_names
                .iter()
                .any(|name| &**name == imported_name);

            ImportRestrictionStatus(!is_forbidden, ImportRestrictionCause::ImportNames)
        } else {
            // Deny all imports from this module
            ImportRestrictionStatus(false, ImportRestrictionCause::ImportSource)
        }
    }

    fn get_message_for_restriction(
        &self,
        import_source: &str,
        imported_name: &str,
        reason: ImportRestrictionCause,
    ) -> String {
        if !self.message.is_empty() {
            self.message.clone()
        } else {
            match reason {
                ImportRestrictionCause::ImportSource => {
                    format!("Do not import '{import_source}'.")
                }
                ImportRestrictionCause::ImportNames | ImportRestrictionCause::AllowImportNames => {
                    format!("Do not import '{imported_name}' from '{import_source}'.")
                }
            }
        }
    }

    fn get_note_for_restriction(
        &self,
        import_source: &str,
        _imported_name: &str,
        reason: ImportRestrictionCause,
    ) -> Option<String> {
        match reason {
            ImportRestrictionCause::ImportSource | ImportRestrictionCause::ImportNames => None,
            ImportRestrictionCause::AllowImportNames => {
                let mut sorted = self.allow_import_names.to_vec();
                sorted.sort();
                let allowed_import_names = sorted
                    .into_iter()
                    .map(|name| {
                        if &**&name == RestrictedImportVisitor::BARE_IMPORT_ALIAS {
                            "side-effect import".into()
                        } else {
                            name
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                Some(format!("Only the following imports from '{import_source}' are allowed: {allowed_import_names}."))
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum CustomRestrictedImport {
    Plain(String),
    WithOptions(CustomRestrictedImportOptions),
}

impl From<CustomRestrictedImport> for CustomRestrictedImportOptions {
    fn from(options: CustomRestrictedImport) -> Self {
        match options {
            CustomRestrictedImport::Plain(message) => CustomRestrictedImportOptions {
                message,
                import_names: [].into(),
                allow_import_names: [].into(),
            },
            CustomRestrictedImport::WithOptions(options) => options,
        }
    }
}

impl Deserializable for CustomRestrictedImport {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            biome_deserialize::Deserializable::deserialize(value, name, diagnostics)
                .map(Self::Plain)
        } else {
            biome_deserialize::Deserializable::deserialize(value, name, diagnostics)
                .map(Self::WithOptions)
        }
    }
}

struct RestrictedImportVisitor<'a> {
    import_source: &'a str,
    restricted_import: CustomRestrictedImportOptions,
    results: Vec<RestrictedImportMessage>,
}

impl<'a> RestrictedImportVisitor<'a> {
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
                        return self.visit_namespace_binding(namespace_binding);
                    }
                    _ => {
                        // Use fallback instead
                    }
                },
                AnyJsBindingPattern::JsObjectBindingPattern(named_bindings) => {
                    // const { ... } = await import(...)
                    return self.visit_named_bindings(named_bindings);
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
            if member_name.as_js_name()?.text() != "then" {
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
                    .into_iter()
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
                                    .into_iter()
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
                            .into_iter()
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
            return Some(variable_declarator.id().ok()?);
        } else {
            return None;
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
                let sideeffect_import: JsImportBareClause = clause.cast()?;
                self.visit_sideeffect_import(sideeffect_import)
            }
            JsSyntaxKind::JS_IMPORT_COMBINED_CLAUSE => {
                let import_combined_clause: JsImportCombinedClause = clause.cast()?;
                if let Ok(default_specifier) = import_combined_clause.default_specifier() {
                    self.visit_default_import(default_specifier);
                }
                if let Ok(combined_specifier) = import_combined_clause.specifier() {
                    self.visit_combined_specifier(combined_specifier);
                }
                Some(())
            }
            JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE => {
                let import_named_clause: JsImportNamedClause = clause.cast()?;
                let import_specifiers = import_named_clause.named_specifiers().ok()?;
                self.visit_named_imports(import_specifiers)
            }
            JsSyntaxKind::JS_EXPORT_NAMED_FROM_CLAUSE => {
                let export_named_from_clause = clause.cast::<JsExportNamedFromClause>()?;
                let import_specifiers = export_named_from_clause.specifiers();
                self.visit_named_reexports(import_specifiers)
            }
            JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE => {
                let import_default_clause: JsImportDefaultClause = clause.cast()?;
                let default_specifier = import_default_clause.default_specifier().ok()?;
                self.visit_default_import(default_specifier)
            }
            JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE => {
                let import_namespace_clause: JsImportNamespaceClause = clause.cast()?;
                let namespace_specifier = import_namespace_clause.namespace_specifier().ok()?;
                self.visit_namespace_import(namespace_specifier)
            }
            JsSyntaxKind::JS_EXPORT_FROM_CLAUSE => {
                let reexport_namespace_clause: JsExportFromClause = clause.cast()?;
                self.visit_namespace_reexport(reexport_namespace_clause)
            }
            _ => None,
        }
    }

    fn visit_combined_specifier(
        &mut self,
        combined_specifier: AnyJsCombinedSpecifier,
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

    fn visit_named_imports(&mut self, named_imports: JsNamedImportSpecifiers) -> Option<()> {
        let import_specifiers = named_imports.specifiers();
        for import_specifier_maybe in import_specifiers.into_iter() {
            if let Some(import_specifier) = import_specifier_maybe.ok() {
                self.visit_named_or_shorthand_import(import_specifier);
            }
        }
        Some(())
    }

    fn visit_named_reexports(
        &mut self,
        named_reexports: JsExportNamedFromSpecifierList,
    ) -> Option<()> {
        for export_specifier_maybe in named_reexports.into_iter() {
            if let Some(export_specifier) = export_specifier_maybe.ok() {
                self.visit_named_or_shorthand_reexport(export_specifier);
            }
        }
        Some(())
    }

    fn visit_named_bindings(&mut self, named_imports: JsObjectBindingPattern) -> Option<()> {
        let import_bindings = named_imports.properties();
        for import_binding_maybe in import_bindings.into_iter() {
            if let Some(import_binding) = import_binding_maybe.ok() {
                self.visit_named_or_shorthand_binding(import_binding);
            }
        }
        Some(())
    }

    fn visit_named_or_shorthand_import(
        &mut self,
        import_specifier: AnyJsNamedImportSpecifier,
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
        import_binding: AnyJsObjectBindingPatternMember,
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
    fn visit_sideeffect_import(&mut self, sideeffect_import: JsImportBareClause) -> Option<()> {
        let source_token = sideeffect_import
            .source()
            .ok()?
            .as_js_module_source()?
            .value_token()
            .ok()?;
        self.visit_special_import_token(&source_token, Self::BARE_IMPORT_ALIAS)
    }

    /// Checks whether this import of the form `local_name` (as in `import local_name from 'source'`) is allowed.
    fn visit_default_import(&mut self, default_import: JsDefaultImportSpecifier) -> Option<()> {
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
        namespace_import: JsNamespaceImportSpecifier,
    ) -> Option<()> {
        self.visit_special_import_token(
            &namespace_import.star_token().ok()?,
            Self::NAMESPACE_IMPORT_ALIAS,
        )
    }

    /// Checks whether this namespace reexport of the form `export * from ...` is allowed.
    fn visit_namespace_reexport(&mut self, namespace_reexport: JsExportFromClause) -> Option<()> {
        self.visit_special_import_token(
            &namespace_reexport.star_token().ok()?,
            Self::NAMESPACE_IMPORT_ALIAS,
        )
    }

    /// Checks whether this import of the form `const local_name = import(...)` is allowed.
    fn visit_namespace_binding(&mut self, namespace_import: JsIdentifierBinding) -> Option<()> {
        return self
            .visit_special_import_node(&namespace_import.syntax(), Self::NAMESPACE_IMPORT_ALIAS);
    }

    /// Checks whether this import of the form `{ imported_name }` is allowed.
    fn visit_shorthand_import(
        &mut self,
        shorthand_import: JsShorthandNamedImportSpecifier,
    ) -> Option<()> {
        self.visit_imported_identifier(
            shorthand_import
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
        shorthand_import: JsObjectBindingPatternShorthandProperty,
    ) -> Option<()> {
        self.visit_imported_identifier(
            shorthand_import
                .identifier()
                .ok()?
                .as_js_identifier_binding()?
                .name_token()
                .ok()?,
        )
    }

    /// Checks whether this import of the form `{ imported_name as local_name }`
    /// (including `{ default as local_name }`) is allowed.
    fn visit_named_import(&mut self, named_import: JsNamedImportSpecifier) -> Option<()> {
        self.visit_imported_identifier(named_import.name().ok()?.value().ok()?)
    }

    /// Checks whether this import of the form `{ imported_name }` or `{ imported_name as exported_name }`
    /// (including `{ default as exported_name }`) is allowed.
    fn visit_named_or_shorthand_reexport(
        &mut self,
        named_reexport: JsExportNamedFromSpecifier,
    ) -> Option<()> {
        self.visit_imported_identifier(named_reexport.source_name().ok()?.value().ok()?)
    }

    /// Checks whether this import of the form `{ imported_name: local_name }`
    /// (including `{ default: local_name }` and `{ "imported name": local_name `) is allowed.
    fn visit_named_binding(&mut self, named_import: JsObjectBindingPatternProperty) -> Option<()> {
        self.visit_imported_identifier(
            named_import
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
    fn visit_imported_identifier(&mut self, name_token: SyntaxToken<JsLanguage>) -> Option<()> {
        // TODO: inner_string_text removes quotes but does not e.g. decode escape sequences.
        //       If the imported name uses e.g. Unicode escape sequences, this may cause
        //       problems because restricted_import.(allow_)import_names contains decoded
        //       strings, while inner_string_text(name_token) returns encoded strings.
        self.visit_special_import_token(&name_token, inner_string_text(&name_token).text())
    }

    /// Checks whether the import specified by `name_or_alias` is allowed.
    /// and records a diagnostic for `import_node.text_trimmed_range()` if not.
    fn visit_special_import_node(
        &mut self,
        import_node: &SyntaxNode<JsLanguage>,
        name_or_alias: &str,
    ) -> Option<()> {
        let status = self.restricted_import.is_import_allowed(name_or_alias);
        if status.is_allowed() {
            return None;
        }
        self.results.push(RestrictedImportMessage {
            location: import_node.text_trimmed_range(),
            message: self.restricted_import.get_message_for_restriction(
                self.import_source,
                name_or_alias,
                status.reason(),
            ),
            note: self.restricted_import.get_note_for_restriction(
                self.import_source,
                name_or_alias,
                status.reason(),
            ),
        });
        return Some(());
    }

    /// Checks whether the import specified by `name_or_alias` is allowed.
    /// and records a diagnostic for `import_token.text_trimmed_range()` if not.
    fn visit_special_import_token(
        &mut self,
        import_token: &SyntaxToken<JsLanguage>,
        name_or_alias: &str,
    ) -> Option<()> {
        let status = self.restricted_import.is_import_allowed(name_or_alias);
        if status.is_allowed() {
            return None;
        }
        self.results.push(RestrictedImportMessage {
            location: import_token.text_trimmed_range(),
            message: self.restricted_import.get_message_for_restriction(
                self.import_source,
                name_or_alias,
                status.reason(),
            ),
            note: self.restricted_import.get_note_for_restriction(
                self.import_source,
                name_or_alias,
                status.reason(),
            ),
        });
        return Some(());
    }
}

pub struct RestrictedImportMessage {
    pub location: TextRange,
    pub message: String,
    pub note: Option<String>,
}

impl Rule for NoRestrictedImports {
    type Query = Ast<AnyJsImportLike>;
    type State = RestrictedImportMessage;
    type Signals = Vec<Self::State>;
    type Options = Box<RestrictedImportsOptions>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.is_in_ts_module_declaration() {
            return [].into();
        }
        let Some(module_name) = node.module_name_token() else {
            return vec![];
        };
        let import_source_text = inner_string_text(&module_name);
        let import_source = import_source_text.text();

        let Some(restricted_import_settings) = ctx.options().paths.get(import_source) else {
            return vec![];
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
                        note: None,
                    }]
                } else {
                    // Check (and possibly report) each imported name individually
                    let mut visitor = RestrictedImportVisitor {
                        import_source,
                        restricted_import,
                        results: Vec::new(),
                    };
                    visitor.visit_import(module_source_node);
                    visitor.results
                }
            }
            AnyJsImportLike::JsImportCallExpression(import_call) => {
                // TODO: We have to parse the context of the import() call to determine
                // which exports are being used/whether this should be considered a
                // namespace import, a sideeffect import (the two of which may
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
                        note: None,
                    }]
                } else {
                    // Check (and possibly report) each imported name individually
                    let mut visitor = RestrictedImportVisitor {
                        import_source,
                        restricted_import,
                        results: Vec::new(),
                    };
                    visitor.visit_import_call(import_call);
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
                        note: None,
                    }]
                } else {
                    vec![]
                }
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, occurence: &Self::State) -> Option<RuleDiagnostic> {
        let mut rule_diagnostic = RuleDiagnostic::new(
            rule_category!(),
            occurence.location,
            markup! {
                {occurence.message}
            },
        );
        if let Some(note) = &occurence.note {
            rule_diagnostic = rule_diagnostic.note(note);
        }
        Some(rule_diagnostic)
    }
}
