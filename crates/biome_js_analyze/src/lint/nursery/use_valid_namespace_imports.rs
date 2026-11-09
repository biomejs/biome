use crate::services::database::ProjectSemantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsCombinedSpecifier, AnyJsDeclarationClause, AnyJsExportClause,
    AnyJsExpression, AnyJsImportClause, AnyJsModuleItem, AnyJsNamedImportSpecifier,
    AnyJsObjectBindingPatternMember, AnyJsRoot, AnyJsxObjectName, AnyTsModuleName,
    JsComputedMemberAssignment, JsExportFromClause, JsIdentifierBinding, JsIdentifierExpression,
    JsInitializerClause, JsLanguage, JsModuleSource, JsObjectBindingPattern,
    JsObjectBindingPatternProperty, JsReferenceIdentifier, JsStaticMemberAssignment,
    JsStaticMemberExpression, JsSyntaxToken, JsVariableDeclarator, JsxClosingElement,
    JsxMemberName, JsxReferenceIdentifier,
};
use biome_js_type_info::ImportSymbol;
use biome_module_graph::{
    JsExportedSymbolLookup, JsImport, JsModuleInfo, JsOwnExport, ModuleDb, ModuleInfo,
    ModuleInfoKind, SymbolFromModuleInfo, find_js_exported_symbol,
};
use biome_rowan::{AstNode, AstSeparatedList, Direction, declare_node_union};
use biome_rule_options::use_valid_namespace_imports::UseValidNamespaceImportsOptions;
use rustc_hash::FxHashSet;

declare_lint_rule! {
    /// Require that names read from a namespace import are exported by the imported module.
    ///
    /// A namespace import, such as `import * as utils from "./utils.js"`, puts every export of a
    /// module into one object. Reading a name that the module doesn't export gives `undefined`,
    /// which usually means the name has a typo, or the export was renamed or removed.
    ///
    /// This rule reports:
    ///
    /// - reading a name from a namespace import that the imported module doesn't export,
    ///   including names read through destructuring, such as `const { name } = utils`, and
    ///   JSX elements, such as `<Components.Button />`;
    /// - assigning a value to a member of a namespace import, which throws an error at runtime
    ///   because namespace imports are read-only;
    /// - namespace imports and `export * as name from` statements of a module that has no exports.
    ///
    /// The rule also follows namespaces that a module exports, such as
    /// `export * as helpers from "./helpers.js"`, so `utils.helpers.missing` is checked as well.
    ///
    /// The rule only checks modules that use `import` or `export` statements. It skips CommonJS
    /// modules and TypeScript modules that use `export =`, because their exports can't be
    /// determined reliably.
    ///
    /// Note that if you use TypeScript, you probably don't need this rule, since TypeScript
    /// already reports these mistakes.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,file=utils.js
    /// export const add = (a, b) => a + b;
    /// ```
    ///
    /// ```js,expect_diagnostic,file=invalid-read.js
    /// import * as utils from "./utils.js";
    ///
    /// utils.subtract(2, 1);
    /// ```
    ///
    /// ```js,expect_diagnostic,file=invalid-destructuring.js
    /// import * as utils from "./utils.js";
    ///
    /// const { subtract } = utils;
    /// ```
    ///
    /// ```js,expect_diagnostic,file=invalid-assignment.js
    /// import * as utils from "./utils.js";
    ///
    /// utils.add = () => 0;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,file=utils.js
    /// export const add = (a, b) => a + b;
    /// ```
    ///
    /// ```js,file=valid.js
    /// import * as utils from "./utils.js";
    ///
    /// utils.add(2, 1);
    /// const { add } = utils;
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to report imports of names that a module doesn't export, see [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/).
    /// - If you want to report reading a namespace import with a computed key, such as `utils[name]`, see [`noDynamicNamespaceImportAccess`](https://biomejs.dev/linter/rules/no-dynamic-namespace-import-access/).
    pub UseValidNamespaceImports {
        version: "next",
        name: "useValidNamespaceImports",
        language: "js",
        sources: &[RuleSource::EslintImport("namespace").same()],
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Project],
    }
}

declare_node_union! {
    pub AnyJsNamespaceSource = AnyJsImportClause | JsExportFromClause
}

pub enum UseValidNamespaceImportsState {
    /// The imported module has no exports.
    EmptyModule,
    /// `member` is read from `namespace`, but the module behind `namespace`
    /// doesn't export it.
    MissingExport {
        member: JsSyntaxToken,
        namespace: NamespacePath,
    },
    /// A member of the namespace `namespace` is the target of an assignment.
    Assignment(AnyJsExpression),
}

impl Rule for UseValidNamespaceImports {
    type Query = ProjectSemantic<AnyJsNamespaceSource>;
    type State = UseValidNamespaceImportsState;
    type Signals = Box<[Self::State]>;
    type Options = UseValidNamespaceImportsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut signals = Vec::new();
        let Some(source) = namespace_source(ctx.query()) else {
            return signals.into_boxed_slice();
        };
        let Some(target) = resolve_module_source(ctx, &source) else {
            return signals.into_boxed_slice();
        };
        let db = ctx.db();

        match ctx.query() {
            AnyJsNamespaceSource::JsExportFromClause(clause) => {
                if clause.type_token().is_none()
                    && clause.export_as().is_some()
                    && as_es_module(db, target).is_some_and(has_no_exports)
                {
                    signals.push(UseValidNamespaceImportsState::EmptyModule);
                }
            }
            AnyJsNamespaceSource::AnyJsImportClause(clause) => {
                if clause.type_token().is_some() {
                    return signals.into_boxed_slice();
                }

                if let Some(binding) = namespace_import_binding(clause)
                    && let Some(info) = as_es_module(db, target)
                {
                    if has_no_exports(info) {
                        signals.push(UseValidNamespaceImportsState::EmptyModule);
                    }
                    check_references(ctx, &binding, target, &mut signals);
                }

                if let Some(binding) = clause
                    .default_specifiers()
                    .and_then(|specifier| specifier.local_name().ok())
                    .and_then(|binding| binding.as_js_identifier_binding().cloned())
                    && let Some(namespace) = find_exported_namespace(db, target, "default")
                {
                    check_references(ctx, &binding, namespace, &mut signals);
                }

                for specifier in clause
                    .named_specifiers()
                    .iter()
                    .flat_map(|specifiers| specifiers.specifiers().into_iter().flatten())
                {
                    if specifier.type_token().is_some() {
                        continue;
                    }
                    let Some(binding) = specifier
                        .local_name()
                        .and_then(|binding| binding.as_js_identifier_binding().cloned())
                    else {
                        continue;
                    };
                    let Some(imported_name) = imported_name(&specifier) else {
                        continue;
                    };
                    if let Some(namespace) = find_exported_namespace(db, target, &imported_name) {
                        check_references(ctx, &binding, namespace, &mut signals);
                    }
                }
            }
        }

        signals.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            UseValidNamespaceImportsState::EmptyModule => {
                let source = namespace_source(ctx.query())?;
                let specifier = source.inner_string_text().ok()?;
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        source.range(),
                        markup! {
                            "The module "<Emphasis>{specifier.text()}</Emphasis>" has no exports."
                        },
                    )
                    .note(markup! {
                        "A namespace import of a module without exports is an empty object, so there is nothing to read from it."
                    })
                    .note(markup! {
                        "Add exports to the module, or import it only for its side effects with "<Emphasis>"import \""{specifier.text()}"\""</Emphasis>"."
                    }),
                )
            }
            UseValidNamespaceImportsState::MissingExport { member, namespace } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    member.text_trimmed_range(),
                    markup! {
                        "The namespace "<Emphasis>{namespace}</Emphasis>" has no export named "<Emphasis>{member.text_trimmed()}</Emphasis>"."
                    },
                )
                .note(markup! {
                    "Reading a name that a module doesn't export gives "<Emphasis>"undefined"</Emphasis>"."
                })
                .note(markup! {
                    "Check the spelling of the name, or export it from the module."
                }),
            ),
            UseValidNamespaceImportsState::Assignment(namespace) => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    namespace.syntax().parent()?.text_trimmed_range(),
                    markup! {
                        "The members of the namespace "<Emphasis>{NamespacePath::Expression(namespace.clone())}</Emphasis>" can't be reassigned."
                    },
                )
                .note(markup! {
                    "Namespace imports are read-only, so this assignment throws an error at runtime."
                })
                .note(markup! {
                    "To change a value that a module exports, export a function from that module that changes it."
                }),
            ),
        }
    }
}

/// The source text of the namespace that a member is read from.
pub enum NamespacePath {
    /// The object of a member expression, such as `utils.helpers` in `utils.helpers.add`.
    Expression(AnyJsExpression),
    /// The object of a JSX member name, such as `Components` in `<Components.Button />`.
    Jsx(AnyJsxObjectName),
    /// An object pattern that destructures a namespace, such as `{ add }` in
    /// `const { add } = utils`.
    Pattern(JsObjectBindingPattern),
}

impl biome_console::fmt::Display for NamespacePath {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self {
            Self::Expression(expression) => write_tokens(fmt, expression),
            Self::Jsx(name) => write_tokens(fmt, name),
            Self::Pattern(pattern) => {
                // The path is the declarator's initializer followed by the
                // property keys of the enclosing patterns, from the outermost
                // to the innermost one. Each key is found by walking up from
                // `pattern` again, which avoids collecting them first.
                let depth = enclosing_pattern_properties(pattern).count();
                let initializer = pattern
                    .syntax()
                    .ancestors()
                    .find_map(JsVariableDeclarator::cast)
                    .and_then(|declarator| declarator.initializer())
                    .and_then(|initializer| initializer.expression().ok());
                if let Some(initializer) = initializer {
                    write_tokens(fmt, &initializer)?;
                }
                for level in (0..depth).rev() {
                    if let Some(key) = enclosing_pattern_properties(pattern)
                        .nth(level)
                        .and_then(|property| property.member().ok())
                    {
                        fmt.write_str(".")?;
                        write_tokens(fmt, &key)?;
                    }
                }
                Ok(())
            }
        }
    }
}

/// Returns the module source of an import or export clause.
fn namespace_source(node: &AnyJsNamespaceSource) -> Option<JsModuleSource> {
    match node {
        AnyJsNamespaceSource::AnyJsImportClause(clause) => clause.source().ok(),
        AnyJsNamespaceSource::JsExportFromClause(clause) => {
            clause.source().ok()?.as_js_module_source().cloned()
        }
    }
}

/// Returns the module that `source` refers to, if it can be resolved.
fn resolve_module_source(
    ctx: &RuleContext<UseValidNamespaceImports>,
    source: &JsModuleSource,
) -> Option<ModuleInfo> {
    let owner = ctx.module_info_for_path(ctx.file_path())?;
    let ModuleInfoKind::Js(owner_info) = owner.kind(ctx.db()) else {
        return None;
    };
    let specifier = source.inner_string_text().ok()?;
    let import_path = owner_info.import_paths.get(specifier.text())?;
    let path = import_path
        .resolve_js(ctx.db(), owner)
        .path()
        .as_deref()
        .ok()?;
    ctx.module_info_for_path(path)
}

/// Returns the local name of `import * as name`, if the clause has one.
fn namespace_import_binding(clause: &AnyJsImportClause) -> Option<JsIdentifierBinding> {
    let specifier = match clause {
        AnyJsImportClause::JsImportNamespaceClause(clause) => clause.namespace_specifier().ok()?,
        AnyJsImportClause::JsImportCombinedClause(clause) => match clause.specifier().ok()? {
            AnyJsCombinedSpecifier::JsNamespaceImportSpecifier(specifier) => specifier,
            AnyJsCombinedSpecifier::JsNamedImportSpecifiers(_) => return None,
        },
        _ => return None,
    };
    specifier
        .local_name()
        .ok()?
        .as_js_identifier_binding()
        .cloned()
}

/// Returns the name that `specifier` imports, without quotes.
fn imported_name(specifier: &AnyJsNamedImportSpecifier) -> Option<biome_rowan::TokenText> {
    match specifier {
        AnyJsNamedImportSpecifier::JsNamedImportSpecifier(specifier) => {
            specifier.name().ok()?.inner_string_text().ok()
        }
        AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(_) => {
            Some(specifier.imported_name()?.token_text_trimmed())
        }
        AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => None,
    }
}

/// Returns the information of `module` if it is a JavaScript module whose
/// exports can be determined.
///
/// A module qualifies if it has at least one `import` or `export` statement and
/// no export that the module graph doesn't track. Modules without `import` or
/// `export` statements may be CommonJS modules, whose exports aren't tracked.
fn as_es_module(db: &dyn ModuleDb, module: ModuleInfo) -> Option<&JsModuleInfo> {
    let ModuleInfoKind::Js(info) = module.kind(db) else {
        return None;
    };
    let AnyJsRoot::JsModule(root) = info.semantic_model.root() else {
        return None;
    };
    let mut has_module_syntax = false;
    for item in root.items() {
        match item {
            AnyJsModuleItem::JsImport(_) => has_module_syntax = true,
            AnyJsModuleItem::JsExport(export) => {
                if export
                    .export_clause()
                    .is_ok_and(|clause| is_untracked_export(&clause))
                {
                    return None;
                }
                has_module_syntax = true;
            }
            _ => {}
        }
    }
    has_module_syntax.then_some(info)
}

/// Returns whether `clause` exports names that the module graph doesn't
/// record: `export =`, `export import Name = ...`, and
/// `export namespace A.B {}`.
fn is_untracked_export(clause: &AnyJsExportClause) -> bool {
    let declaration = match clause {
        AnyJsExportClause::TsExportAssignmentClause(_) => return true,
        AnyJsExportClause::AnyJsDeclarationClause(declaration) => declaration.clone(),
        AnyJsExportClause::TsExportDeclareClause(clause) => match clause.declaration() {
            Ok(declaration) => declaration,
            Err(_) => return false,
        },
        _ => return false,
    };
    match declaration {
        AnyJsDeclarationClause::TsImportEqualsDeclaration(_) => true,
        AnyJsDeclarationClause::TsModuleDeclaration(declaration) => declaration
            .name()
            .is_ok_and(|name| matches!(name, AnyTsModuleName::TsQualifiedModuleName(_))),
        _ => false,
    }
}

fn has_no_exports(info: &JsModuleInfo) -> bool {
    info.exports.is_empty() && info.blanket_reexports.is_empty()
}

/// Returns the module whose namespace `name` refers to, when `module` exports
/// `name` as a namespace.
///
/// Returns `None` when the export doesn't exist, isn't a namespace, or when the
/// module behind the namespace isn't an ES module.
fn find_exported_namespace(
    db: &dyn ModuleDb,
    module: ModuleInfo,
    name: &str,
) -> Option<ModuleInfo> {
    let JsExportedSymbolLookup::Found(export, owner) =
        find_js_exported_symbol(db, SymbolFromModuleInfo::new(db, name, module))
    else {
        return None;
    };
    follow_namespace_export(db, export, *owner)
}

/// Follows `export`, defined in `owner`, to the module whose namespace it
/// holds.
///
/// Namespaces can be exported with `export * as name from "..."`, or by
/// exporting an imported namespace, such as `import * as name from "..."`
/// followed by `export { name }`. An exported import of a single name is
/// followed to the module that defines that name.
fn follow_namespace_export<'db>(
    db: &'db dyn ModuleDb,
    mut export: &'db JsOwnExport,
    mut owner: ModuleInfo,
) -> Option<ModuleInfo> {
    let mut visited = FxHashSet::default();
    loop {
        let range = match export {
            JsOwnExport::Namespace(reexport) => {
                let namespace = resolve_import(db, &reexport.import, owner)?;
                as_es_module(db, namespace)?;
                return Some(namespace);
            }
            JsOwnExport::Type(_) => return None,
            JsOwnExport::Binding(range) => *range,
        };
        // Modules that import names from each other could otherwise make this
        // loop forever.
        if !visited.insert((owner, range)) {
            return None;
        }
        let ModuleInfoKind::Js(info) = owner.kind(db) else {
            return None;
        };
        let binding = info.semantic_model.as_binding_by_range(range)?;
        if !binding.declaration_kind().is_import_declaration() {
            return None;
        }
        let name = binding.tree().name_token().ok()?;
        let import = info.static_imports.get(name.text_trimmed())?;
        let module = resolve_import(db, import, owner)?;
        let lookup = match &import.symbol {
            ImportSymbol::All => {
                as_es_module(db, module)?;
                return Some(module);
            }
            ImportSymbol::Named(name) => {
                find_js_exported_symbol(db, SymbolFromModuleInfo::new(db, name.text(), module))
            }
            ImportSymbol::Default => {
                find_js_exported_symbol(db, SymbolFromModuleInfo::new(db, "default", module))
            }
        };
        let JsExportedSymbolLookup::Found(next_export, next_owner) = lookup else {
            return None;
        };
        export = next_export;
        owner = *next_owner;
    }
}

fn resolve_import(db: &dyn ModuleDb, import: &JsImport, owner: ModuleInfo) -> Option<ModuleInfo> {
    let path = import.resolve_js(db, owner).path().as_deref().ok()?;
    db.module_for_path(path)
}

/// Checks every read of `binding`, which holds the namespace of `module`.
fn check_references(
    ctx: &RuleContext<UseValidNamespaceImports>,
    binding: &JsIdentifierBinding,
    module: ModuleInfo,
    signals: &mut Vec<UseValidNamespaceImportsState>,
) {
    for reference in binding.all_reads(ctx.model()) {
        let node = reference.syntax();
        if let Some(expression) = JsReferenceIdentifier::cast_ref(&node)
            .and_then(|identifier| identifier.parent::<JsIdentifierExpression>())
        {
            check_expression(ctx.db(), expression.into(), module, signals);
        } else if let Some(identifier) = JsxReferenceIdentifier::cast_ref(&node) {
            check_jsx_name(ctx.db(), identifier.into(), module, signals);
        }
    }
}

/// Checks the member accesses, assignments, and destructuring applied to
/// `namespace`, which holds the namespace of `module`.
fn check_expression(
    db: &dyn ModuleDb,
    mut namespace: AnyJsExpression,
    mut module: ModuleInfo,
    signals: &mut Vec<UseValidNamespaceImportsState>,
) -> Option<()> {
    loop {
        namespace = namespace.outer_expression()?;
        let parent = namespace.syntax().parent()?;
        if let Some(member_expression) = JsStaticMemberExpression::cast_ref(&parent) {
            let member = member_expression.member().ok()?;
            let member = member.as_js_name()?.value_token().ok()?;
            match find_js_exported_symbol(
                db,
                SymbolFromModuleInfo::new(db, member.text_trimmed(), module),
            ) {
                JsExportedSymbolLookup::Found(export, owner) => {
                    module = follow_namespace_export(db, export, *owner)?;
                    namespace = member_expression.into();
                }
                JsExportedSymbolLookup::Missing => {
                    signals.push(UseValidNamespaceImportsState::MissingExport {
                        member,
                        namespace: NamespacePath::Expression(namespace),
                    });
                    return None;
                }
                JsExportedSymbolLookup::Unknown => return None,
            }
        } else if JsStaticMemberAssignment::can_cast(parent.kind())
            || JsComputedMemberAssignment::cast_ref(&parent)
                .and_then(|assignment| assignment.object().ok())
                .is_some_and(|object| object.syntax() == namespace.syntax())
        {
            signals.push(UseValidNamespaceImportsState::Assignment(namespace));
            return None;
        } else {
            let pattern = JsInitializerClause::cast(parent)?
                .parent::<JsVariableDeclarator>()?
                .id()
                .ok()?;
            if let AnyJsBindingPattern::JsObjectBindingPattern(pattern) = pattern {
                check_object_pattern(db, pattern, module, signals);
            }
            return None;
        }
    }
}

/// Checks the property keys of `pattern`, which destructures the namespace of
/// `module`, and of the patterns nested in it.
fn check_object_pattern(
    db: &dyn ModuleDb,
    pattern: JsObjectBindingPattern,
    module: ModuleInfo,
    signals: &mut Vec<UseValidNamespaceImportsState>,
) {
    let mut stack = vec![(pattern, module)];
    while let Some((pattern, module)) = stack.pop() {
        for property in pattern.properties().iter().flatten() {
            let (member, name, nested_pattern) = match property {
                AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                    property,
                ) => {
                    let Some(member) = property
                        .identifier()
                        .ok()
                        .and_then(|binding| binding.as_js_identifier_binding()?.name_token().ok())
                    else {
                        continue;
                    };
                    let name = member.token_text_trimmed();
                    (member, name, None)
                }
                AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property) => {
                    let Some(key) = property
                        .member()
                        .ok()
                        .and_then(|key| key.as_js_literal_member_name().cloned())
                    else {
                        continue;
                    };
                    let (Ok(member), Ok(name)) = (key.value(), key.name()) else {
                        continue;
                    };
                    let nested_pattern =
                        property.pattern().ok().and_then(|pattern| match pattern {
                            AnyJsBindingPattern::JsObjectBindingPattern(pattern) => Some(pattern),
                            _ => None,
                        });
                    (member, name, nested_pattern)
                }
                _ => continue,
            };
            match find_js_exported_symbol(db, SymbolFromModuleInfo::new(db, name.text(), module)) {
                JsExportedSymbolLookup::Found(export, owner) => {
                    if let Some(nested_pattern) = nested_pattern
                        && let Some(namespace) = follow_namespace_export(db, export, *owner)
                    {
                        stack.push((nested_pattern, namespace));
                    }
                }
                JsExportedSymbolLookup::Missing => {
                    signals.push(UseValidNamespaceImportsState::MissingExport {
                        member,
                        namespace: NamespacePath::Pattern(pattern.clone()),
                    });
                }
                JsExportedSymbolLookup::Unknown => {}
            }
        }
    }
}

/// Checks a JSX element name such as `<Components.Button />`, where `name`
/// holds the namespace of `module`.
fn check_jsx_name(
    db: &dyn ModuleDb,
    mut name: AnyJsxObjectName,
    mut module: ModuleInfo,
    signals: &mut Vec<UseValidNamespaceImportsState>,
) -> Option<()> {
    // The closing tag repeats the name of the opening tag, which is already
    // checked.
    if name
        .syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| !JsxMemberName::can_cast(ancestor.kind()))
        .is_some_and(|ancestor| JsxClosingElement::can_cast(ancestor.kind()))
    {
        return None;
    }
    loop {
        let member_name = name.parent::<JsxMemberName>()?;
        let member = member_name.member().ok()?.value_token().ok()?;
        match find_js_exported_symbol(
            db,
            SymbolFromModuleInfo::new(db, member.text_trimmed(), module),
        ) {
            JsExportedSymbolLookup::Found(export, owner) => {
                module = follow_namespace_export(db, export, *owner)?;
                name = member_name.into();
            }
            JsExportedSymbolLookup::Missing => {
                signals.push(UseValidNamespaceImportsState::MissingExport {
                    member,
                    namespace: NamespacePath::Jsx(name),
                });
                return None;
            }
            JsExportedSymbolLookup::Unknown => return None,
        }
    }
}

/// Returns the properties whose patterns enclose `pattern`, from the innermost
/// to the outermost one.
///
/// For `const { a: { b: { c } } } = ns`, the pattern `{ c }` is enclosed by the
/// properties `b: { c }` and `a: { b: { c } }`.
fn enclosing_pattern_properties(
    pattern: &JsObjectBindingPattern,
) -> impl Iterator<Item = JsObjectBindingPatternProperty> {
    std::iter::successors(
        pattern.parent::<JsObjectBindingPatternProperty>(),
        |property| {
            property
                .parent::<biome_js_syntax::JsObjectBindingPatternPropertyList>()?
                .parent::<JsObjectBindingPattern>()?
                .parent::<JsObjectBindingPatternProperty>()
        },
    )
}

/// Writes the source text of `node` without trivia.
fn write_tokens(
    fmt: &mut biome_console::fmt::Formatter<'_>,
    node: &impl AstNode<Language = JsLanguage>,
) -> std::io::Result<()> {
    for token in node.syntax().descendants_tokens(Direction::Next) {
        fmt.write_str(token.text_trimmed())?;
    }
    Ok(())
}
