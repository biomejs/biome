use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsBinding, AnyJsBindingPattern, AnyJsDeclarationClause,
    AnyJsExportClause, AnyJsImportLike, AnyJsModuleSource, AnyJsObjectBindingPatternMember,
    AnyJsRoot, AnyTsIdentifierBinding, AnyTsModuleName, AnyTsType, JsExport, JsExportFromClause,
    JsExportNamedFromClause, JsExportNamedSpecifierList, JsIdentifierBinding,
    JsVariableDeclaratorList, unescape_js_string,
};
use biome_js_type_info::Type;
use biome_rowan::{AstNode, Text, TokenText, TriviaPieceKind, WalkEvent};
use camino::{Utf8Path, Utf8PathBuf};
use oxc_resolver::{ResolveError, ResolverGeneric};

use crate::{
    dependency_graph::{Export, Import, ModuleDependencyData, OwnExport, ReexportAll},
    resolver_cache::ResolverCache,
};

pub(crate) struct ModuleVisitor<'a> {
    root: AnyJsRoot,
    directory: &'a Utf8Path,
    resolver: &'a ResolverGeneric<ResolverCache<'a>>,
    data: ModuleDependencyData,
}

impl<'a> ModuleVisitor<'a> {
    pub fn new(
        root: AnyJsRoot,
        directory: &'a Utf8Path,
        resolver: &'a ResolverGeneric<ResolverCache<'a>>,
    ) -> Self {
        Self {
            root,
            directory,
            resolver,
            data: Default::default(),
        }
    }

    pub fn collect_data(mut self) -> ModuleDependencyData {
        let iter = self.root.syntax().preorder();
        for event in iter {
            match event {
                WalkEvent::Enter(node) => {
                    if let Some(import) = AnyJsImportLike::cast_ref(&node) {
                        self.visit_import(import);
                    } else if let Some(export) = JsExport::cast_ref(&node) {
                        self.visit_export(export);
                    }
                }
                WalkEvent::Leave(_) => {}
            }
        }

        self.data
    }

    fn visit_import(&mut self, node: AnyJsImportLike) {
        let Some(specifier) = node.inner_string_text() else {
            return;
        };

        let import = self.import_from_specifier(specifier.text());

        match node {
            AnyJsImportLike::JsModuleSource(_) => {
                self.data
                    .static_imports
                    .insert(specifier.to_string(), import);
            }
            AnyJsImportLike::JsCallExpression(_) | AnyJsImportLike::JsImportCallExpression(_) => {
                self.data
                    .dynamic_imports
                    .insert(specifier.to_string(), import);
            }
        }
    }

    fn visit_export(&mut self, node: JsExport) -> Option<()> {
        let jsdoc_comment = node.export_token().ok().and_then(|token| {
            token
                .leading_trivia()
                .pieces()
                .rev()
                .find_map(|trivia| match trivia.kind() {
                    TriviaPieceKind::MultiLineComment | TriviaPieceKind::SingleLineComment => {
                        // JSDoc comments must start with exactly `/**`.
                        // Either more or less asterisks are ignored.
                        let text = trivia.text();
                        (text.starts_with("/**")
                            && text.as_bytes().get(3).is_some_and(|c| *c != b'*')
                            && text.ends_with("*/"))
                        .then(|| normalize_jsdoc_comment(text))
                    }
                    _ => None,
                })
        });

        match node.export_clause().ok()? {
            AnyJsExportClause::AnyJsDeclarationClause(node) => {
                self.visit_export_declaration_clause(node, jsdoc_comment)
            }
            AnyJsExportClause::JsExportDefaultDeclarationClause(node) => {
                let ty = node
                    .declaration()
                    .map(|decl| Type::from_any_js_export_default_declaration(&decl))
                    .unwrap_or_default();
                self.register_export("default", Export::Own(OwnExport { jsdoc_comment, ty }))
            }
            AnyJsExportClause::JsExportDefaultExpressionClause(_) => {
                self.register_export_with_name_and_jsdoc_comment("default", jsdoc_comment)
            }
            AnyJsExportClause::JsExportFromClause(node) => self.visit_export_from_clause(node),
            AnyJsExportClause::JsExportNamedClause(node) => {
                self.visit_export_named_specifiers(node.specifiers(), jsdoc_comment)
            }
            AnyJsExportClause::JsExportNamedFromClause(node) => {
                self.visit_export_named_from_clause(node)
            }
            AnyJsExportClause::TsExportAsNamespaceClause(node) => {
                let token = node.name().ok()?.value_token().ok()?;
                let name = token.token_text_trimmed();
                self.register_export_with_name_and_jsdoc_comment(name, jsdoc_comment)
            }
            AnyJsExportClause::TsExportAssignmentClause(_) => {
                // This is somewhat misleading, since the `export =` syntax is
                // used for CommonJS exports rather than ES6 `default` exports.
                // Thankfully, bundlers are responsible for normalising this,
                // which isn't really Biome's concern.
                self.register_export_with_name_and_jsdoc_comment("default", jsdoc_comment)
            }
            AnyJsExportClause::TsExportDeclareClause(node) => {
                self.visit_export_declaration_clause(node.declaration().ok()?, jsdoc_comment)
            }
        }
    }

    fn visit_export_declaration_clause(
        &mut self,
        node: AnyJsDeclarationClause,
        jsdoc_comment: Option<String>,
    ) -> Option<()> {
        let name = match &node {
            AnyJsDeclarationClause::JsClassDeclaration(node) => {
                node.id().ok().and_then(get_name)?
            }
            AnyJsDeclarationClause::JsFunctionDeclaration(node) => {
                node.id().ok().and_then(get_name)?
            }
            AnyJsDeclarationClause::JsVariableDeclarationClause(node) => {
                return self.visit_export_variable_declarations(
                    node.declaration().ok()?.declarators(),
                    jsdoc_comment,
                );
            }
            AnyJsDeclarationClause::TsDeclareFunctionDeclaration(node) => {
                node.id().ok().and_then(get_name)?
            }
            AnyJsDeclarationClause::TsEnumDeclaration(node) => node.id().ok().and_then(get_name)?,
            AnyJsDeclarationClause::TsExternalModuleDeclaration(_)
            | AnyJsDeclarationClause::TsGlobalDeclaration(_)
            | AnyJsDeclarationClause::TsImportEqualsDeclaration(_) => return None,
            AnyJsDeclarationClause::TsInterfaceDeclaration(node) => {
                node.id().ok().and_then(get_ts_name)?
            }
            AnyJsDeclarationClause::TsModuleDeclaration(node) => match node.name().ok()? {
                AnyTsModuleName::AnyTsIdentifierBinding(node) => get_ts_name(node)?,
                AnyTsModuleName::TsQualifiedModuleName(_) => return None,
            },
            AnyJsDeclarationClause::TsTypeAliasDeclaration(node) => {
                node.binding_identifier().ok().and_then(get_ts_name)?
            }
        };
        let ty = Type::from_any_js_declaration_clause(&node);

        self.register_export(name, Export::Own(OwnExport { jsdoc_comment, ty }))
    }

    fn visit_export_from_clause(&mut self, node: JsExportFromClause) -> Option<()> {
        let module_source = node.source().ok()?;
        let import = self.import_from_module_source(module_source)?;

        if let Some(export_as) = node.export_as() {
            let name = export_as
                .exported_name()
                .and_then(|name| name.inner_string_text())
                .map(unescape_js_string)
                .ok()?;
            self.data
                .exports
                .insert(name, Export::ReexportAll(ReexportAll { import }));
        } else {
            self.data.blanket_reexports.push(ReexportAll { import });
        }

        Some(())
    }

    fn visit_export_named_from_clause(&mut self, node: JsExportNamedFromClause) -> Option<()> {
        let module_source = node.source().ok()?;
        let import = self.import_from_module_source(module_source)?;

        for specifier in node.specifiers() {
            let Ok(specifier) = specifier else {
                continue;
            };

            let name = specifier
                .export_as()
                .and_then(|export_as| export_as.exported_name().ok())
                .or_else(|| specifier.source_name().ok())
                .and_then(|name| name.inner_string_text().ok())
                .map(unescape_js_string)?;
            self.data
                .exports
                .insert(name, Export::Reexport(import.clone()));
        }

        Some(())
    }

    fn visit_export_named_specifiers(
        &mut self,
        specifiers: JsExportNamedSpecifierList,
        jsdoc_comment: Option<String>,
    ) -> Option<()> {
        // TODO: This registers the comment if is attached to the `export`
        //       statement, but ignores any JSDoc comments that might be
        //       attached to the symbols themselves.
        //       We need https://github.com/biomejs/biome/issues/5312 for this.
        for specifier in specifiers {
            if let Ok(name) = specifier.and_then(|specifier| specifier.exported_name()) {
                self.register_export_with_name_and_jsdoc_comment(name, jsdoc_comment.clone());
            }
        }

        Some(())
    }

    fn visit_export_variable_declarations(
        &mut self,
        declarators: JsVariableDeclaratorList,
        jsdoc_comment: Option<String>,
    ) -> Option<()> {
        for declarator in declarators.into_iter().flatten() {
            if let Ok(binding) = declarator.id() {
                self.visit_binding_pattern(
                    binding,
                    jsdoc_comment.clone(),
                    declarator
                        .variable_annotation()
                        .and_then(|annotation| annotation.type_annotation().ok().flatten())
                        .and_then(|annotation| annotation.ty().ok()),
                );
            }
        }

        Some(())
    }

    fn visit_binding_pattern(
        &mut self,
        binding: AnyJsBindingPattern,
        jsdoc_comment: Option<String>,
        ty: Option<AnyTsType>,
    ) -> Option<()> {
        match binding {
            AnyJsBindingPattern::AnyJsBinding(node) => {
                if let Some(binding) = node.as_js_identifier_binding() {
                    self.visit_identifier_binding(binding, jsdoc_comment, ty)?;
                }
            }
            AnyJsBindingPattern::JsArrayBindingPattern(node) => {
                let mut tuple_elements = ty
                    .as_ref()
                    .and_then(|ty| ty.as_ts_tuple_type())
                    .map(|ty| ty.elements().into_iter());
                let mut get_next_element_type = || {
                    if let Some(mut elements) = tuple_elements.take() {
                        let next = elements.next();
                        tuple_elements = Some(elements);
                        next.and_then(|el| el.ok())
                            .as_ref()
                            .and_then(|el| el.as_any_ts_type())
                            .cloned()
                    } else {
                        None
                    }
                };

                for element in node.elements().into_iter().flatten() {
                    match element {
                        AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(node) => {
                            if let Ok(binding) = node.pattern() {
                                self.visit_binding_pattern(
                                    binding,
                                    jsdoc_comment.clone(),
                                    get_next_element_type(),
                                );
                            }
                        }
                        AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(node) => {
                            if let Ok(binding) = node.pattern() {
                                self.visit_binding_pattern(
                                    binding,
                                    jsdoc_comment.clone(),
                                    get_next_element_type(),
                                );
                            }
                        }
                        AnyJsArrayBindingPatternElement::JsArrayHole(_) => {
                            get_next_element_type();
                        }
                    }
                }
            }
            AnyJsBindingPattern::JsObjectBindingPattern(node) => {
                let mut object_members = ty
                    .as_ref()
                    .and_then(|ty| ty.as_ts_object_type())
                    .map(|ty| ty.members().into_iter());
                let mut get_next_member_type = || {
                    if let Some(mut elements) = object_members.take() {
                        let next = elements.next();
                        object_members = Some(elements);
                        next.as_ref()
                            .and_then(|member| member.as_ts_property_signature_type_member())
                            .and_then(|property| property.type_annotation())
                            .and_then(|annotation| annotation.ty().ok())
                    } else {
                        None
                    }
                };

                for property in node.properties().into_iter().flatten() {
                    match property {
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(node) => {
                            if let Ok(binding) = node.pattern() {
                                self.visit_binding_pattern(binding, jsdoc_comment.clone(), get_next_member_type());
                            }
                        },
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(node) => {
                            if let Ok(binding) = node.binding() {
                                if let Some(binding) = binding.as_js_identifier_binding() {
                                   self.visit_identifier_binding(binding, jsdoc_comment.clone(), get_next_member_type());
                                }
                            }
                        },
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(node) => {
                            if let Ok(binding) = node.identifier() {
                                if let Some(binding) = binding.as_js_identifier_binding() {
                                   self.visit_identifier_binding(binding, jsdoc_comment.clone(), get_next_member_type());
                                }
                            }
                        },
                        AnyJsObjectBindingPatternMember::JsBogusBinding(_) | AnyJsObjectBindingPatternMember::JsMetavariable(_) => {}
                    }
                }
            }
        }

        Some(())
    }

    fn visit_identifier_binding(
        &mut self,
        binding: &JsIdentifierBinding,
        jsdoc_comment: Option<String>,
        ty: Option<AnyTsType>,
    ) -> Option<()> {
        let name = binding.name_token().ok()?.token_text_trimmed();
        self.register_export(
            name,
            Export::Own(OwnExport {
                jsdoc_comment,
                ty: ty.map(|ty| Type::from_any_ts_type(&ty)).unwrap_or_default(),
            }),
        )
    }

    fn register_export(&mut self, name: impl Into<Text>, export: Export) -> Option<()> {
        self.data.exports.insert(name.into(), export);

        Some(())
    }

    fn register_export_with_name_and_jsdoc_comment(
        &mut self,
        name: impl Into<Text>,
        jsdoc_comment: Option<String>,
    ) -> Option<()> {
        self.register_export(
            name,
            Export::Own(OwnExport {
                jsdoc_comment,
                ty: Type::Unknown,
            }),
        )
    }

    fn import_from_module_source(&self, module_source: AnyJsModuleSource) -> Option<Import> {
        let specifier = module_source
            .as_js_module_source()?
            .inner_string_text()
            .ok()?;
        Some(self.import_from_specifier(specifier.text()))
    }

    fn import_from_specifier(&self, specifier: &str) -> Import {
        let resolved_path = self
            .resolver
            .resolve(self.directory, specifier)
            .and_then(|resolution| {
                Utf8PathBuf::from_path_buf(resolution.into_path_buf())
                    .map_err(|path| ResolveError::NotFound(path.to_string_lossy().to_string()))
            })
            .map_err(|error| error.to_string());
        Import { resolved_path }
    }
}

fn get_name(binding_result: AnyJsBinding) -> Option<TokenText> {
    let name = binding_result
        .as_js_identifier_binding()?
        .name_token()
        .ok()?
        .token_text_trimmed();
    Some(name)
}

fn get_ts_name(binding_result: AnyTsIdentifierBinding) -> Option<TokenText> {
    let name = binding_result
        .as_ts_identifier_binding()?
        .name_token()
        .ok()?
        .token_text_trimmed();
    Some(name)
}

/// Normalizes the text in a JSDoc comment by stripping the opening and trailing
/// markers, and trimming the leading asterisk from every line.
fn normalize_jsdoc_comment(text: &str) -> String {
    debug_assert!(text.starts_with("/**") && text.ends_with("*/"));

    let mut result = text[3..text.len() - 2]
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            trimmed.strip_prefix('*').map_or(trimmed, str::trim_start)
        })
        .fold(String::new(), |mut result, line| {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(line);
            result
        });

    // Trim trailing newlines.
    while result.ends_with('\n') {
        result.truncate(result.len() - 1);
    }

    result
}
