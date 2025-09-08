use crate::organize_imports::import_groups::{ImportSourceCandidate, SourcesMatcher};
use crate::organize_imports::import_source::ImportSource;
use crate::restricted_regex::RestrictedRegex;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext, TextRange,
};
use biome_deserialize_macros::Deserializable;
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
use biome_rowan::{AstNode, AstSeparatedList, SyntaxNode, SyntaxNodeCast, SyntaxToken, TokenText};
use biome_string_case::comparable_token::ComparableToken;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoRestrictedImportsOptions {
    /// A list of import paths that should trigger the rule.
    #[serde(skip_serializing_if = "FxHashMap::is_empty")]
    pub paths: FxHashMap<Box<str>, Paths>,

    /// gitignore-style patterns that should trigger the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patterns: Option<Box<[Patterns]>>,
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
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct PathOptions {
    /// The message to display when this module is imported.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub message: Box<str>,

    /// Names of the exported members that should not be used.
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub import_names: Box<[Box<str>]>,

    /// Names of the exported members that allowed to be not be used.
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub allow_import_names: Box<[Box<str>]>,
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

    fn message(&self, import_source: &str, imported_name: &str, cause: Cause) -> String {
        if !self.message.is_empty() {
            return self.message.to_string();
        }
        default_message(import_source, imported_name, cause)
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum Patterns {
    WithOptions(PatternOptions),
}

impl From<Patterns> for PatternOptions {
    fn from(patterns: Patterns) -> Self {
        match patterns {
            Patterns::WithOptions(pattern_options) => pattern_options,
        }
    }
}

impl Deserializable for Patterns {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        biome_deserialize::Deserializable::deserialize(ctx, value, name).map(Self::WithOptions)
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
pub struct PatternOptions {
    /// An array of gitignore-style patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<SourcesMatcher>,

    /// A custom message for diagnostics related to this pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Box<str>>,

    /// A regex pattern for import names to forbid within the matched modules.
    #[serde(skip_serializing_if = "Option::is_none")]
    import_name_pattern: Option<RestrictedRegex>,

    /// If true, the matched patterns in the importNamePattern will be allowed. Defaults to `false`.
    invert_import_name_pattern: bool,
}

impl PatternOptions {
    fn has_import_name_constraints(&self) -> bool {
        self.import_name_pattern.as_ref().is_some()
    }

    fn check_restriction(&self, imported_name: &str) -> Restriction {
        match &self.import_name_pattern {
            Some(pattern) => {
                // The imported name is forbidden if the match result and the inversion flag are different.
                // - don't invert (false) + match (true) => forbidden
                // - invert (true) + no match (false) => forbidden
                let is_forbidden =
                    pattern.is_match(imported_name) != self.invert_import_name_pattern;

                if is_forbidden {
                    Restriction::forbidden(Cause::ImportNames)
                } else {
                    Restriction::allowed(Cause::ImportNames)
                }
            }
            None => Restriction::allowed(Cause::ImportNames),
        }
    }

    pub fn check_import_restrictions(
        &self,
        node: &AnyJsImportLike,
        module_name: &SyntaxToken<JsLanguage>,
        import_source_text: &TokenText,
    ) -> Vec<RestrictedImportMessage> {
        if let Some(group) = &self.group {
            let source = ImportSource::from(ComparableToken {
                token: import_source_text.clone(),
            });
            let candidate = ImportSourceCandidate::new(&source);
            if group.is_match(&candidate) {
                return check_import_restrictions(
                    self,
                    node,
                    module_name,
                    import_source_text.text(),
                );
            }
        }
        vec![]
    }

    fn message(&self, import_source: &str, imported_name: &str, cause: Cause) -> String {
        if let Some(ref msg) = self.message {
            return msg.to_string();
        }
        default_message(import_source, imported_name, cause)
    }
}

/// Specifies whether a specific import is (dis)allowed, and why it is allowed/disallowed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Restriction {
    pub allowed: bool,
    pub cause: Cause,
}

impl Restriction {
    pub const fn allowed(cause: Cause) -> Self {
        Self {
            allowed: true,
            cause,
        }
    }
    pub const fn forbidden(cause: Cause) -> Self {
        Self {
            allowed: false,
            cause,
        }
    }
    pub fn is_allowed(self) -> bool {
        self.allowed
    }
    pub fn is_forbidden(self) -> bool {
        !self.allowed
    }
}

/// Specifies why a specific import is allowed or disallowed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cause {
    /// Reason: The import source is forbidden or allowed.
    ImportSource,
    /// Reason: A set of forbidden import names has been defined via `importNames`.
    ImportNames,
    /// Reason: A set of allowed import names has been defined via `allowImportNames`.
    AllowImportNames,
}

pub enum Options<'a> {
    PathOptions(&'a PathOptions),
    PatternOptions(&'a PatternOptions),
}

pub trait ImportRestrictions {
    fn check_restriction(&self, imported_name: &str) -> Restriction;
    fn has_import_name_constraints(&self) -> bool;
    fn message(&self, import_source: &str, imported_name: &str, cause: Cause) -> String;
    fn options(&self) -> Options<'_>;
}

impl ImportRestrictions for PathOptions {
    fn check_restriction(&self, imported_name: &str) -> Restriction {
        self.check_restriction(imported_name)
    }
    fn has_import_name_constraints(&self) -> bool {
        self.has_import_name_constraints()
    }
    fn message(&self, import_source: &str, imported_name: &str, cause: Cause) -> String {
        self.message(import_source, imported_name, cause)
    }
    fn options(&self) -> Options<'_> {
        Options::PathOptions(self)
    }
}

impl ImportRestrictions for PatternOptions {
    fn check_restriction(&self, imported_name: &str) -> Restriction {
        self.check_restriction(imported_name)
    }
    fn has_import_name_constraints(&self) -> bool {
        self.has_import_name_constraints()
    }
    fn message(&self, import_source: &str, imported_name: &str, cause: Cause) -> String {
        self.message(import_source, imported_name, cause)
    }
    fn options(&self) -> Options<'_> {
        Options::PatternOptions(self)
    }
}

pub struct RestrictedImportVisitor<'a> {
    pub import_source: &'a str,
    pub options: Options<'a>,
    pub results: Vec<RestrictedImportMessage>,
}

impl RestrictedImportVisitor<'_> {
    pub const BARE_IMPORT_ALIAS: &'static str = "";
    pub const NAMESPACE_IMPORT_ALIAS: &'static str = "*";
    pub const DEFAULT_IMPORT_ALIAS: &'static str = "default";
}

pub struct RestrictedImportMessage {
    pub location: TextRange,
    pub message: String,
    pub import_source: String,
    pub allowed_import_names: Box<[Box<str>]>,
}

impl RestrictedImportMessage {
    pub fn new(
        token: TextRange,
        import_source: &str,
        message: String,
        allowed_import_names: Box<[Box<str>]>,
    ) -> Self {
        let allowed_names: Box<[Box<str>]> = if allowed_import_names.is_empty() {
            [].into()
        } else {
            allowed_import_names
        };
        Self {
            location: token,
            message,
            import_source: import_source.to_string(),
            allowed_import_names: allowed_names,
        }
    }
}

pub fn check_import_restrictions<I: ImportRestrictions>(
    import_restriction: &I,
    node: &AnyJsImportLike,
    module_name: &SyntaxToken<JsLanguage>,
    import_source: &str,
) -> Vec<RestrictedImportMessage> {
    match node {
        AnyJsImportLike::JsModuleSource(module_source_node) => {
            if !import_restriction.has_import_name_constraints() {
                // All imports disallowed, add diagnostic to the import source
                vec![RestrictedImportMessage::new(
                    module_name.text_trimmed_range(),
                    import_source,
                    import_restriction.message(import_source, "", Cause::ImportSource),
                    [].into(),
                )]
            } else {
                // Check (and possibly report) each imported name individually
                let mut visitor = RestrictedImportVisitor {
                    import_source,
                    options: import_restriction.options(),
                    results: vec![],
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
            if !import_restriction.has_import_name_constraints() {
                // All imports disallowed, add diagnostic to the import source
                vec![RestrictedImportMessage::new(
                    module_name.text_trimmed_range(),
                    import_source,
                    import_restriction.message(import_source, "", Cause::ImportSource),
                    [].into(),
                )]
            } else {
                // Check (and possibly report) each imported name individually
                let mut visitor = RestrictedImportVisitor {
                    import_source,
                    options: import_restriction.options(),
                    results: vec![],
                };
                visit_import_call(&mut visitor, import_call);
                visitor.results
            }
        }

        AnyJsImportLike::JsCallExpression(_expression) => {
            let restriction =
                import_restriction.check_restriction(RestrictedImportVisitor::DEFAULT_IMPORT_ALIAS);

            if restriction.is_forbidden() {
                // require() calls can only import the default import, so
                // there are no individual import names to check or report on.
                vec![RestrictedImportMessage::new(
                    module_name.text_trimmed_range(),
                    import_source,
                    import_restriction.message(import_source, "", Cause::ImportSource),
                    [].into(),
                )]
            } else {
                vec![]
            }
        }
    }
}

/// Analyze the context of an `import(...)` call to find the imported names,
/// then validate that each of the names is allowed to be imported.
fn visit_import_call(
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
                // so discard the additional information and use fallback instead.
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
fn visit_import(
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
    //       problems because path_options.(allow_)import_names contains decoded
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
    match visitor.options {
        Options::PathOptions(path_options) => {
            let restriction = path_options.check_restriction(name_or_alias);
            if restriction.is_allowed() {
                return None;
            }
            visitor.results.push(RestrictedImportMessage::new(
                import_node.text_trimmed_range(),
                visitor.import_source,
                path_options.message(visitor.import_source, name_or_alias, restriction.cause),
                path_options.allow_import_names.clone(),
            ));
            Some(())
        }
        Options::PatternOptions(pattern_options) => {
            let restriction = pattern_options.check_restriction(name_or_alias);
            if restriction.is_allowed() {
                return None;
            }
            visitor.results.push(RestrictedImportMessage::new(
                import_node.text_trimmed_range(),
                visitor.import_source,
                pattern_options.message(visitor.import_source, name_or_alias, restriction.cause),
                Vec::new().into_boxed_slice(),
            ));
            Some(())
        }
    }
}

/// Checks whether the import specified by `name_or_alias` is allowed.
/// and records a diagnostic for `import_token.text_trimmed_range()` if not.
fn visit_special_import_token(
    visitor: &mut RestrictedImportVisitor,
    import_token: &SyntaxToken<JsLanguage>,
    name_or_alias: &str,
) -> Option<()> {
    match visitor.options {
        Options::PathOptions(path_options) => {
            let restriction = path_options.check_restriction(name_or_alias);
            if restriction.is_allowed() {
                return None;
            }
            visitor.results.push(RestrictedImportMessage::new(
                import_token.text_trimmed_range(),
                visitor.import_source,
                path_options.message(visitor.import_source, name_or_alias, restriction.cause),
                path_options.allow_import_names.clone(),
            ));
            Some(())
        }
        Options::PatternOptions(pattern_options) => {
            let restriction = pattern_options.check_restriction(name_or_alias);
            if restriction.is_allowed() {
                return None;
            }
            visitor.results.push(RestrictedImportMessage::new(
                import_token.text_trimmed_range(),
                visitor.import_source,
                pattern_options.message(visitor.import_source, name_or_alias, restriction.cause),
                Vec::new().into_boxed_slice(),
            ));
            Some(())
        }
    }
}
