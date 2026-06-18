//! Typed AST collector for TypeScript declaration files consumed by global types codegen.

use std::sync::Arc;

use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsDeclarationClause, AnyJsExportClause, AnyJsModuleItem,
    AnyJsModuleSource, AnyJsRoot, AnyJsStatement, AnyTsExternalModuleDeclarationBody,
    AnyTsIdentifierBinding, AnyTsModuleName, AnyTsVariableAnnotation, JsFileSource, JsLanguage,
    JsModuleItemList, JsSyntaxKind, JsVariableDeclaration,
};
use biome_rowan::{AstNode, AstNodeListIterator, Text, TextRange, TokenText};

/// Declaration scope path recorded by the collector.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScopePath {
    /// The declaration belongs to the top-level global declaration file scope.
    Global,
    /// The declaration is inside a `declare global` wrapper.
    DeclareGlobal,
    /// The declaration is inside a flattened namespace path.
    Namespace(Vec<Text>),
    /// The declaration is inside an external module declaration.
    ExternalModule(Text),
    /// The declaration is inside a namespace nested under an external module.
    ExternalModuleNamespace { module: Text, namespace: Vec<Text> },
}

/// Kind of declaration collected for later lowering.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeclarationKind {
    /// `interface X { ... }` declaration.
    Interface,
    /// `type X = ...` declaration.
    TypeAlias,
    /// `declare function ...` ambient function signature.
    DeclareFunction,
    /// One declarator from a variable declaration.
    VariableDeclarator {
        /// Source-order ordinal within its variable declaration.
        ordinal: u32,
    },
    /// `import X = Y` namespace alias.
    ImportEquals,
}

/// One supported declaration discovered by the collector.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarationRecord {
    pub file_repo_relative: Arc<str>,
    pub scope: ScopePath,
    pub declared_name: TokenText,
    pub kind: DeclarationKind,
    pub text_range: biome_rowan::TextRange,
    pub syntax_kind: biome_js_syntax::JsSyntaxKind,
}

/// Accounting entry emitted for each collector decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CoverageOutcome {
    /// Index into [`CollectorOutput::records`] for the collected declaration.
    Collected(usize),
    /// The collector entered a nested declaration scope.
    EnteredScope {
        scope: ScopePath,
        file: Arc<str>,
        range: biome_rowan::TextRange,
    },
    /// The collector exited a nested declaration scope.
    ExitedScope {
        scope: ScopePath,
        file: Arc<str>,
        range: biome_rowan::TextRange,
    },
    /// The collector reached a syntactically valid but unsupported construct.
    OutOfScope {
        /// Stable reason used by coverage fixture output.
        reason: &'static str,
        file: Arc<str>,
        scope: ScopePath,
        range: biome_rowan::TextRange,
        syntax_kind: biome_js_syntax::JsSyntaxKind,
    },
    /// The collector found malformed or unsupported syntax that should be diagnosed.
    Diagnostic(CoverageDiagnostic),
}

/// Diagnostic emitted while collecting declaration coverage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoverageDiagnostic {
    /// Stable diagnostic category used by coverage fixture output.
    pub category: &'static str,
    pub file: Arc<str>,
    pub scope: ScopePath,
    pub range: biome_rowan::TextRange,
    pub syntax_kind: biome_js_syntax::JsSyntaxKind,
    /// Optional extra detail for debugging coverage failures.
    pub detail: Option<String>,
}

/// Collector result containing supported declarations and full coverage accounting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectorOutput {
    /// Supported declarations discovered in source order.
    pub records: Vec<DeclarationRecord>,
    /// Coverage outcomes discovered in source order.
    pub coverage: Vec<CoverageOutcome>,
}

/// Mutable accumulator threaded through the collector's tree walk.
struct CollectorState {
    /// Repo-relative path of the source file being collected, shared with
    /// every record and coverage entry via cheap `Arc::clone`.
    file: Arc<str>,
    /// Supported declarations appended in source order; subset of `coverage`
    /// filtered to `Collected`.
    records: Vec<DeclarationRecord>,
    /// Coverage outcomes (collected, scope-enter/exit, out-of-scope, diagnostics)
    /// appended in source order across the whole file.
    coverage: Vec<CoverageOutcome>,
}

/// One pending scope on the collector's iterative DFS stack.
struct ScopeFrame {
    /// Effective scope used when recording declarations inside this frame.
    scope: ScopePath,
    /// Unconsumed module items for this scope, advanced one element per loop turn.
    items: AstNodeListIterator<JsLanguage, AnyJsModuleItem>,
    /// Coverage scope emitted on `ExitedScope` when this frame pops; differs
    /// from `scope` for `declare global` (see `enter_scope_with_effective_scope`).
    exit_scope: Option<ScopePath>,
    /// Wrapper-declaration range used as the `ExitedScope` range; `None` for
    /// the root frame which has no wrapper.
    exit_range: Option<TextRange>,
}

/// Parses one discovered `.d.ts` file and returns the supported declarations
/// alongside coverage diagnostics for every unsupported syntax surface.
pub fn collect(file: &super::source::DiscoveredFile) -> CollectorOutput {
    let mut state = CollectorState {
        file: Arc::<str>::from(file.repo_relative.as_str()),
        records: Vec::new(),
        coverage: Vec::new(),
    };

    let Ok(text) = std::str::from_utf8(&file.bytes) else {
        state.diagnostic(
            "invalid_utf8",
            ScopePath::Global,
            TextRange::default(),
            JsSyntaxKind::JS_BOGUS,
            None,
        );
        return state.into_output();
    };

    let parse = parse(text, JsFileSource::d_ts(), JsParserOptions::default());
    let root_syntax = parse.syntax();
    for diagnostic in parse.diagnostics() {
        state.diagnostic(
            "parser_error",
            ScopePath::Global,
            root_syntax.text_trimmed_range(),
            root_syntax.kind(),
            Some(diagnostic.message.to_string()),
        );
    }

    let AnyJsRoot::TsDeclarationModule(module) = parse.tree() else {
        state.diagnostic(
            "unsupported_root",
            ScopePath::Global,
            root_syntax.text_trimmed_range(),
            root_syntax.kind(),
            None,
        );
        return state.into_output();
    };

    let mut stack = vec![ScopeFrame {
        scope: ScopePath::Global,
        items: module.items().into_iter(),
        exit_scope: None,
        exit_range: None,
    }];

    while !stack.is_empty() {
        let (scope, next_item, exit_scope, exit_range) = {
            let frame = stack.last_mut().expect("stack is not empty");
            (
                frame.scope.clone(),
                frame.items.next(),
                frame.exit_scope.clone(),
                frame.exit_range,
            )
        };

        let Some(item) = next_item else {
            let Some(_frame) = stack.pop() else {
                break;
            };
            if let Some(range) = exit_range {
                state.exit_scope(
                    exit_scope.expect("scope frame with exit range has exit scope"),
                    range,
                );
            }
            continue;
        };

        handle_module_item(item, scope, &mut state, &mut stack);
    }

    state.into_output()
}

/// Dispatches one `AnyJsModuleItem` to the matching statement, export, or import handler.
fn handle_module_item(
    item: AnyJsModuleItem,
    scope: ScopePath,
    state: &mut CollectorState,
    stack: &mut Vec<ScopeFrame>,
) {
    match item {
        AnyJsModuleItem::AnyJsStatement(statement) => {
            handle_statement(statement, scope, state, stack);
        }
        AnyJsModuleItem::JsExport(export) => {
            let Ok(clause) = export.export_clause() else {
                state.diagnostic_for_node("missing_export_clause", &export, scope, None);
                return;
            };
            handle_export_clause(clause, scope, state, stack);
        }
        AnyJsModuleItem::JsImport(import) => {
            state.out_of_scope_for_node("unsupported_import", &import, scope);
        }
    }
}

/// Routes one statement either through the declaration-clause normalizer or
/// into the namespace / global / external-module scope handlers.
fn handle_statement(
    statement: AnyJsStatement,
    scope: ScopePath,
    state: &mut CollectorState,
    stack: &mut Vec<ScopeFrame>,
) {
    if let Some(declaration) = AnyJsDeclarationClause::cast(statement.syntax().clone()) {
        normalize_declaration_clause(&declaration, scope, state, stack);
        return;
    }

    match statement {
        AnyJsStatement::TsDeclareStatement(statement) => {
            let Ok(declaration) = statement.declaration() else {
                state.diagnostic_for_node("missing_declare_declaration", &statement, scope, None);
                return;
            };
            normalize_declaration_clause(&declaration, scope, state, stack);
        }
        AnyJsStatement::JsVariableStatement(statement) => {
            let Ok(declaration) = statement.declaration() else {
                state.diagnostic_for_node("missing_variable_declaration", &statement, scope, None);
                return;
            };
            collect_variable_declaration(declaration, scope, state);
        }
        statement => {
            state.diagnostic_for_node("unsupported_statement", &statement, scope, None);
        }
    }
}

/// Forwards the body of an `export` clause through the declaration handlers,
/// recording unsupported export forms as coverage diagnostics.
fn handle_export_clause(
    clause: AnyJsExportClause,
    scope: ScopePath,
    state: &mut CollectorState,
    stack: &mut Vec<ScopeFrame>,
) {
    match clause {
        AnyJsExportClause::AnyJsDeclarationClause(declaration) => {
            normalize_declaration_clause(&declaration, scope, state, stack);
        }
        AnyJsExportClause::TsExportDeclareClause(declaration) => {
            let Ok(declaration_clause) = declaration.declaration() else {
                state.diagnostic_for_node(
                    "missing_export_declare_declaration",
                    &declaration,
                    scope,
                    None,
                );
                return;
            };
            normalize_declaration_clause(&declaration_clause, scope, state, stack);
        }
        // Every other export shape — default, named, re-export, namespace,
        // assignment — is unsupported in declaration files and lands here.
        clause => {
            state.out_of_scope_for_node("unsupported_export_clause", &clause, scope);
        }
    }
}

/// Dispatches the inner declaration (interface, type alias, function,
/// import-equals, variable, namespace) to its collector.
fn normalize_declaration_clause(
    declaration: &AnyJsDeclarationClause,
    scope: ScopePath,
    state: &mut CollectorState,
    stack: &mut Vec<ScopeFrame>,
) {
    match declaration {
        AnyJsDeclarationClause::TsInterfaceDeclaration(declaration) => {
            collect_interface(declaration.clone(), scope, state);
        }
        AnyJsDeclarationClause::TsTypeAliasDeclaration(declaration) => {
            collect_type_alias(declaration.clone(), scope, state);
        }
        AnyJsDeclarationClause::TsDeclareFunctionDeclaration(declaration) => {
            collect_declare_function(declaration.clone(), scope, state);
        }
        AnyJsDeclarationClause::TsModuleDeclaration(declaration) => {
            enter_module_declaration(declaration.clone(), scope, state, stack);
        }
        AnyJsDeclarationClause::TsExternalModuleDeclaration(declaration) => {
            enter_external_module_declaration(declaration.clone(), scope, state, stack);
        }
        AnyJsDeclarationClause::TsGlobalDeclaration(declaration) => {
            enter_global_declaration(declaration.clone(), state, stack);
        }
        AnyJsDeclarationClause::JsVariableDeclarationClause(declaration) => {
            let Ok(variable_declaration) = declaration.declaration() else {
                state.diagnostic_for_node("missing_variable_declaration", declaration, scope, None);
                return;
            };
            collect_variable_declaration(variable_declaration, scope, state);
        }
        AnyJsDeclarationClause::TsImportEqualsDeclaration(declaration) => {
            collect_import_equals(declaration.clone(), scope, state);
        }
        AnyJsDeclarationClause::JsClassDeclaration(declaration) => {
            state.diagnostic_for_node("unsupported_declaration", declaration, scope, None);
        }
        AnyJsDeclarationClause::JsFunctionDeclaration(declaration) => {
            state.diagnostic_for_node("unsupported_declaration", declaration, scope, None);
        }
        AnyJsDeclarationClause::TsEnumDeclaration(declaration) => {
            state.diagnostic_for_node("unsupported_declaration", declaration, scope, None);
        }
    }
}

/// Records the interface name as a declaration in the current scope.
fn collect_interface(
    declaration: biome_js_syntax::TsInterfaceDeclaration,
    scope: ScopePath,
    state: &mut CollectorState,
) {
    let Ok(name) = declaration.id().and_then(name_from_ts_identifier_binding) else {
        state.diagnostic_for_node("missing_name", &declaration, scope, None);
        return;
    };
    state.collected(
        scope,
        name,
        DeclarationKind::Interface,
        declaration.range(),
        declaration.syntax().kind(),
    );
}

/// Records the type alias name as a declaration in the current scope.
fn collect_type_alias(
    declaration: biome_js_syntax::TsTypeAliasDeclaration,
    scope: ScopePath,
    state: &mut CollectorState,
) {
    let Ok(name) = declaration
        .binding_identifier()
        .and_then(name_from_ts_identifier_binding)
    else {
        state.diagnostic_for_node("missing_name", &declaration, scope, None);
        return;
    };
    state.collected(
        scope,
        name,
        DeclarationKind::TypeAlias,
        declaration.range(),
        declaration.syntax().kind(),
    );
}

/// Records the ambient function signature as a declaration in the current scope.
fn collect_declare_function(
    declaration: biome_js_syntax::TsDeclareFunctionDeclaration,
    scope: ScopePath,
    state: &mut CollectorState,
) {
    let Ok(name) = declaration.id().and_then(name_from_js_binding) else {
        state.diagnostic_for_node("missing_name", &declaration, scope, None);
        return;
    };
    state.collected(
        scope,
        name,
        DeclarationKind::DeclareFunction,
        declaration.range(),
        declaration.syntax().kind(),
    );
}

/// Records an `import =` namespace alias as a declaration in the current scope.
fn collect_import_equals(
    declaration: biome_js_syntax::TsImportEqualsDeclaration,
    scope: ScopePath,
    state: &mut CollectorState,
) {
    let Ok(name) = declaration.id().and_then(name_from_js_binding) else {
        state.diagnostic_for_node("missing_name", &declaration, scope, None);
        return;
    };
    state.collected(
        scope,
        name,
        DeclarationKind::ImportEquals,
        declaration.range(),
        declaration.syntax().kind(),
    );
}

/// Records each variable declarator's binding name as a declaration.
fn collect_variable_declaration(
    declaration: JsVariableDeclaration,
    scope: ScopePath,
    state: &mut CollectorState,
) {
    for (index, declarator_result) in declaration.declarators().into_iter().enumerate() {
        let Ok(ordinal_offset) = u32::try_from(index) else {
            state.diagnostic_for_node(
                "variable_ordinal_overflow",
                &declaration,
                scope.clone(),
                None,
            );
            continue;
        };
        let Ok(declarator) = declarator_result else {
            state.diagnostic_for_node(
                "missing_variable_declarator",
                &declaration,
                scope.clone(),
                None,
            );
            continue;
        };

        let Ok(name) = declarator.id().and_then(name_from_binding_pattern) else {
            state.diagnostic_for_node(
                "unsupported_variable_binding",
                &declarator,
                scope.clone(),
                None,
            );
            continue;
        };

        let Some(annotation) = declarator.variable_annotation() else {
            state.diagnostic_for_node(
                "missing_variable_annotation",
                &declarator,
                scope.clone(),
                None,
            );
            continue;
        };

        match annotation {
            AnyTsVariableAnnotation::TsTypeAnnotation(annotation) => {
                let Ok(_ty) = annotation.ty() else {
                    state.diagnostic_for_node(
                        "missing_variable_type",
                        &annotation,
                        scope.clone(),
                        None,
                    );
                    continue;
                };
                state.collected(
                    scope.clone(),
                    name,
                    DeclarationKind::VariableDeclarator {
                        ordinal: ordinal_offset,
                    },
                    declarator.range(),
                    declarator.syntax().kind(),
                );
            }
            AnyTsVariableAnnotation::TsDefiniteVariableAnnotation(annotation) => {
                state.diagnostic_for_node(
                    "unsupported_variable_annotation",
                    &annotation,
                    scope.clone(),
                    None,
                );
            }
        }
    }
}

/// Pushes a child scope frame for a `namespace` / `module X` body.
fn enter_module_declaration(
    declaration: biome_js_syntax::TsModuleDeclaration,
    parent_scope: ScopePath,
    state: &mut CollectorState,
    stack: &mut Vec<ScopeFrame>,
) {
    let Ok(name) = declaration.name() else {
        state.diagnostic_for_node("missing_module_name", &declaration, parent_scope, None);
        return;
    };

    let Ok(body) = declaration.body() else {
        state.diagnostic_for_node("missing_module_body", &declaration, parent_scope, None);
        return;
    };

    let Ok(parts) = namespace_parts(name) else {
        state.diagnostic_for_node("missing_module_name", &declaration, parent_scope, None);
        return;
    };

    let scope = namespace_scope(parent_scope, parts);
    enter_scope(scope, body.items(), declaration.range(), state, stack);
}

/// Pushes the implicit global scope for a `declare global { ... }` block.
fn enter_global_declaration(
    declaration: biome_js_syntax::TsGlobalDeclaration,
    state: &mut CollectorState,
    stack: &mut Vec<ScopeFrame>,
) {
    let Ok(body) = declaration.body() else {
        state.diagnostic_for_node("missing_global_body", &declaration, ScopePath::Global, None);
        return;
    };

    enter_scope_with_effective_scope(
        ScopePath::DeclareGlobal,
        ScopePath::Global,
        body.items(),
        declaration.range(),
        state,
        stack,
    );
}

/// Pushes a scope for an ambient external module body (`declare module "x" { ... }`).
fn enter_external_module_declaration(
    declaration: biome_js_syntax::TsExternalModuleDeclaration,
    parent_scope: ScopePath,
    state: &mut CollectorState,
    stack: &mut Vec<ScopeFrame>,
) {
    let Ok(source) = declaration.source() else {
        state.diagnostic_for_node(
            "missing_external_module_source",
            &declaration,
            parent_scope,
            None,
        );
        return;
    };

    let AnyJsModuleSource::JsModuleSource(source) = source else {
        state.diagnostic_for_node(
            "unsupported_external_module_source",
            &declaration,
            parent_scope,
            None,
        );
        return;
    };

    let Ok(module_name) = source.inner_string_text() else {
        state.diagnostic_for_node(
            "missing_external_module_source",
            &declaration,
            parent_scope,
            None,
        );
        return;
    };

    let Some(body) = declaration.body() else {
        state.diagnostic_for_node(
            "missing_external_module_body",
            &declaration,
            parent_scope,
            None,
        );
        return;
    };

    let scope = ScopePath::ExternalModule(Text::from(module_name));

    match body {
        AnyTsExternalModuleDeclarationBody::TsModuleBlock(block) => {
            enter_scope(scope, block.items(), declaration.range(), state, stack);
        }
        AnyTsExternalModuleDeclarationBody::TsEmptyExternalModuleDeclarationBody(_body) => {
            state.out_of_scope(
                "empty_external_module",
                scope,
                declaration.range(),
                declaration.syntax().kind(),
            );
        }
    }
}

/// Pushes a child scope frame whose coverage and effective scope coincide.
fn enter_scope(
    scope: ScopePath,
    items: JsModuleItemList,
    range: TextRange,
    state: &mut CollectorState,
    stack: &mut Vec<ScopeFrame>,
) {
    state.enter_scope(scope.clone(), range);
    stack.push(ScopeFrame {
        scope: scope.clone(),
        items: items.into_iter(),
        exit_scope: Some(scope),
        exit_range: Some(range),
    });
}

/// Pushes a child scope frame where coverage tracking and declaration namespace
/// differ (e.g., `declare global` inside a module records into the global scope
/// but enters/exits coverage at the syntactic position).
fn enter_scope_with_effective_scope(
    coverage_scope: ScopePath,
    effective_scope: ScopePath,
    items: JsModuleItemList,
    range: TextRange,
    state: &mut CollectorState,
    stack: &mut Vec<ScopeFrame>,
) {
    state.enter_scope(coverage_scope.clone(), range);
    stack.push(ScopeFrame {
        scope: effective_scope,
        items: items.into_iter(),
        exit_scope: Some(coverage_scope),
        exit_range: Some(range),
    });
}

/// Appends a sequence of namespace identifiers to the parent scope path.
fn namespace_scope(parent_scope: ScopePath, parts: Vec<Text>) -> ScopePath {
    match parent_scope {
        ScopePath::Namespace(mut parent_parts) => {
            parent_parts.extend(parts);
            ScopePath::Namespace(parent_parts)
        }
        ScopePath::ExternalModule(module) => ScopePath::ExternalModuleNamespace {
            module,
            namespace: parts,
        },
        ScopePath::ExternalModuleNamespace {
            module,
            mut namespace,
        } => {
            namespace.extend(parts);
            ScopePath::ExternalModuleNamespace { module, namespace }
        }
        ScopePath::Global | ScopePath::DeclareGlobal => ScopePath::Namespace(parts),
    }
}

/// Walks a qualified TypeScript module name into its leftmost-first identifier parts.
fn namespace_parts(name: AnyTsModuleName) -> biome_rowan::SyntaxResult<Vec<Text>> {
    let mut reversed = Vec::new();
    let mut current = name;

    loop {
        match current {
            AnyTsModuleName::AnyTsIdentifierBinding(binding) => {
                reversed.push(Text::from(name_from_ts_identifier_binding(binding)?));
                break;
            }
            AnyTsModuleName::TsQualifiedModuleName(qualified) => {
                let right = qualified.right()?.value_token()?.token_text_trimmed();
                reversed.push(Text::from(right));
                current = qualified.left()?;
            }
        }
    }

    reversed.reverse();
    Ok(reversed)
}

/// Extracts the trimmed text of a TypeScript identifier binding.
fn name_from_ts_identifier_binding(
    binding: AnyTsIdentifierBinding,
) -> biome_rowan::SyntaxResult<TokenText> {
    let Some(binding) = binding.as_ts_identifier_binding() else {
        return Err(biome_rowan::SyntaxError::UnexpectedMetavariable);
    };
    Ok(binding.name_token()?.token_text_trimmed())
}

/// Extracts the trimmed text of a JS identifier binding.
fn name_from_js_binding(binding: AnyJsBinding) -> biome_rowan::SyntaxResult<TokenText> {
    let Some(binding) = binding.as_js_identifier_binding() else {
        return Err(biome_rowan::SyntaxError::UnexpectedBogusNode);
    };
    Ok(binding.name_token()?.token_text_trimmed())
}

/// Extracts the binding name when the pattern is a single identifier; bails
/// for destructuring patterns.
fn name_from_binding_pattern(pattern: AnyJsBindingPattern) -> biome_rowan::SyntaxResult<TokenText> {
    let Some(binding) = pattern.as_any_js_binding() else {
        return Err(biome_rowan::SyntaxError::UnexpectedBogusNode);
    };
    name_from_js_binding(binding.clone())
}

impl CollectorState {
    /// Records a declaration in both the declaration list and the coverage log.
    ///
    /// The coverage entry stores the record's index in `self.records` rather
    /// than a copy of the [`DeclarationRecord`], so heavy fields (notably
    /// [`ScopePath::Namespace`]'s `Vec<Text>`) are not cloned per collected
    /// declaration.
    fn collected(
        &mut self,
        scope: ScopePath,
        declared_name: TokenText,
        kind: DeclarationKind,
        text_range: TextRange,
        syntax_kind: JsSyntaxKind,
    ) {
        let record = DeclarationRecord {
            file_repo_relative: self.file.clone(),
            scope,
            declared_name,
            kind,
            text_range,
            syntax_kind,
        };
        let index = self.records.len();
        self.records.push(record);
        self.coverage.push(CoverageOutcome::Collected(index));
    }

    /// Marks the syntactic start of a scope boundary in the coverage log.
    fn enter_scope(&mut self, scope: ScopePath, range: TextRange) {
        self.coverage.push(CoverageOutcome::EnteredScope {
            scope,
            file: self.file.clone(),
            range,
        });
    }

    /// Marks the syntactic end of a scope boundary in the coverage log.
    fn exit_scope(&mut self, scope: ScopePath, range: TextRange) {
        self.coverage.push(CoverageOutcome::ExitedScope {
            scope,
            file: self.file.clone(),
            range,
        });
    }

    /// Records a coverage outcome for syntax the collector intentionally skips.
    fn out_of_scope(
        &mut self,
        reason: &'static str,
        scope: ScopePath,
        range: TextRange,
        syntax_kind: JsSyntaxKind,
    ) {
        self.coverage.push(CoverageOutcome::OutOfScope {
            reason,
            file: self.file.clone(),
            scope,
            range,
            syntax_kind,
        });
    }

    /// [`out_of_scope`] overload that derives the range and syntax kind from a node.
    fn out_of_scope_for_node<N>(&mut self, reason: &'static str, node: &N, scope: ScopePath)
    where
        N: AstNode<Language = JsLanguage>,
    {
        self.out_of_scope(reason, scope, node.range(), node.syntax().kind());
    }

    /// Records a category-tagged diagnostic at the given range.
    fn diagnostic(
        &mut self,
        category: &'static str,
        scope: ScopePath,
        range: TextRange,
        syntax_kind: JsSyntaxKind,
        detail: Option<String>,
    ) {
        self.coverage
            .push(CoverageOutcome::Diagnostic(CoverageDiagnostic {
                category,
                file: self.file.clone(),
                scope,
                range,
                syntax_kind,
                detail,
            }));
    }

    /// [`diagnostic`] overload that derives the range and syntax kind from a node.
    fn diagnostic_for_node<N>(
        &mut self,
        category: &'static str,
        node: &N,
        scope: ScopePath,
        detail: Option<String>,
    ) where
        N: AstNode<Language = JsLanguage>,
    {
        self.diagnostic(category, scope, node.range(), node.syntax().kind(), detail);
    }

    /// Consumes the state, returning the collected records and coverage outcomes.
    fn into_output(self) -> CollectorOutput {
        CollectorOutput {
            records: self.records,
            coverage: self.coverage,
        }
    }
}
