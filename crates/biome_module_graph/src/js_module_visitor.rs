use std::collections::BTreeMap;

use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsBinding, AnyJsBindingPattern, AnyJsDeclarationClause,
    AnyJsExportClause, AnyJsImportLike, AnyJsModuleSource, AnyJsObjectBindingPatternMember,
    AnyJsRoot, AnyTsIdentifierBinding, AnyTsModuleName, AnyTsType, JsExport, JsExportFromClause,
    JsExportNamedFromClause, JsExportNamedSpecifierList, JsIdentifierBinding,
    JsVariableDeclaratorList, unescape_js_string,
};
use biome_js_type_info::Type;
use biome_rowan::{AstNode, Text, TokenText, WalkEvent};
use camino::{Utf8Path, Utf8PathBuf};
use oxc_resolver::{ResolveError, ResolverGeneric};

use crate::{
    js_semantic_model::{JsDeclarationKind, JsSemanticModel, JsSemanticModelBuilder},
    jsdoc_comment::JsdocComment,
    module_info::{Export, Import, ModuleInfo, OwnExport, ReexportAll},
    resolver_cache::ResolverCache,
};

pub(crate) struct JsModuleVisitor<'a> {
    root: AnyJsRoot,
    directory: &'a Utf8Path,
    resolver: &'a ResolverGeneric<ResolverCache<'a>>,
    info: ModuleInfo,
}

impl<'a> JsModuleVisitor<'a> {
    pub fn new(
        root: AnyJsRoot,
        directory: &'a Utf8Path,
        resolver: &'a ResolverGeneric<ResolverCache<'a>>,
    ) -> Self {
        Self {
            root,
            directory,
            resolver,
            info: Default::default(),
        }
    }

    pub fn collect_info(mut self) -> ModuleInfo {
        let mut semantic_builder = JsSemanticModelBuilder::default();

        let iter = self.root.syntax().preorder();
        for event in iter {
            match event {
                WalkEvent::Enter(node) => {
                    semantic_builder.push_node(&node);

                    if let Some(import) = AnyJsImportLike::cast_ref(&node) {
                        self.visit_import(import);
                    } else if let Some(export) = JsExport::cast_ref(&node) {
                        self.visit_export(export);
                    }
                }
                WalkEvent::Leave(_node) => {}
            }
        }

        // Make a second pass over all registered exports, and use the semantic
        // model to see if it allows us to resolve any exports symbols that we
        // couldn't assign a type to initially.
        self.collect_uninferred_exports_from_semantic_model(semantic_builder.build());

        self.info
    }

    fn collect_uninferred_exports_from_semantic_model(&mut self, semantic_model: JsSemanticModel) {
        // Temporarily swap the exports out, so we can still call other methods
        // while we hold a mutable reference to the exports.
        let mut exports = BTreeMap::new();
        std::mem::swap(&mut self.info.exports, &mut exports);

        for (name, export) in &mut exports {
            match export {
                Export::Own(own_export) if !own_export.ty.is_inferred() => {
                    let Some(decl) = semantic_model
                        .get_value(name)
                        .or_else(|| semantic_model.get_type(name))
                    else {
                        continue;
                    };

                    match &decl.kind {
                        JsDeclarationKind::Class(ty)
                        | JsDeclarationKind::HoistedValue(ty)
                        | JsDeclarationKind::Value(ty) => {
                            if own_export.jsdoc_comment.is_none() {
                                own_export.jsdoc_comment.clone_from(&decl.jsdoc_comment);
                            }
                            own_export.ty = ty.clone();
                        }
                        JsDeclarationKind::Type(ty) => {
                            *export = Export::OwnType(OwnExport {
                                jsdoc_comment: None,
                                ty: ty.clone(),
                            })
                        }
                        JsDeclarationKind::Import(specifier) => {
                            *export =
                                Export::Reexport(self.import_from_specifier(specifier.text()));
                        }
                        JsDeclarationKind::ImportType(specifier) => {
                            *export =
                                Export::ReexportType(self.import_from_specifier(specifier.text()));
                        }
                        JsDeclarationKind::Interface
                        | JsDeclarationKind::Module
                        | JsDeclarationKind::Namespace => {
                            // TODO
                        }
                    }
                }
                Export::OwnType(own_type_export) if !own_type_export.ty.is_inferred() => {
                    let Some(decl) = semantic_model
                        .get_type(name)
                        .or_else(|| semantic_model.get_value(name))
                    else {
                        continue;
                    };

                    match &decl.kind {
                        JsDeclarationKind::Class(ty)
                        | JsDeclarationKind::HoistedValue(ty)
                        | JsDeclarationKind::Type(ty)
                        | JsDeclarationKind::Value(ty) => {
                            if own_type_export.jsdoc_comment.is_none() {
                                own_type_export
                                    .jsdoc_comment
                                    .clone_from(&decl.jsdoc_comment);
                            }
                            own_type_export.ty = ty.clone();
                        }
                        JsDeclarationKind::Import(specifier)
                        | JsDeclarationKind::ImportType(specifier) => {
                            *export =
                                Export::ReexportType(self.import_from_specifier(specifier.text()));
                        }
                        JsDeclarationKind::Interface
                        | JsDeclarationKind::Module
                        | JsDeclarationKind::Namespace => {
                            // TODO
                        }
                    }
                }
                _ => {}
            }
        }

        std::mem::swap(&mut self.info.exports, &mut exports);
    }

    fn visit_import(&mut self, node: AnyJsImportLike) {
        let Some(specifier) = node.inner_string_text() else {
            return;
        };

        let import = self.import_from_specifier(specifier.text());

        match node {
            AnyJsImportLike::JsModuleSource(_) => {
                self.info
                    .static_imports
                    .insert(specifier.to_string(), import);
            }
            AnyJsImportLike::JsCallExpression(_) | AnyJsImportLike::JsImportCallExpression(_) => {
                self.info
                    .dynamic_imports
                    .insert(specifier.to_string(), import);
            }
        }
    }

    fn visit_export(&mut self, node: JsExport) -> Option<()> {
        let jsdoc_comment = node
            .export_token()
            .ok()
            .and_then(|token| JsdocComment::try_from(token).ok());

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
                self.visit_export_named_specifiers(node.specifiers())
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
        jsdoc_comment: Option<JsdocComment>,
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
        let jsdoc_comment = node
            .syntax()
            .parent()
            .and_then(|parent| JsdocComment::try_from(parent).ok());

        if let Some(export_as) = node.export_as() {
            let name = export_as
                .exported_name()
                .and_then(|name| name.inner_string_text())
                .map(unescape_js_string)
                .ok()?;
            self.info.exports.insert(
                name,
                Export::ReexportAll(ReexportAll {
                    import,
                    jsdoc_comment,
                }),
            );
        } else {
            self.info.blanket_reexports.push(ReexportAll {
                import,
                jsdoc_comment,
            });
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
            self.info
                .exports
                .insert(name, Export::Reexport(import.clone()));
        }

        Some(())
    }

    fn visit_export_named_specifiers(
        &mut self,
        specifiers: JsExportNamedSpecifierList,
    ) -> Option<()> {
        for specifier in specifiers {
            if let Ok(name) = specifier.and_then(|specifier| specifier.exported_name()) {
                self.register_export_with_name_and_jsdoc_comment(name, None);
            }
        }

        Some(())
    }

    fn visit_export_variable_declarations(
        &mut self,
        declarators: JsVariableDeclaratorList,
        jsdoc_comment: Option<JsdocComment>,
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
        jsdoc_comment: Option<JsdocComment>,
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
        jsdoc_comment: Option<JsdocComment>,
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
        self.info.exports.insert(name.into(), export);

        Some(())
    }

    fn register_export_with_name_and_jsdoc_comment(
        &mut self,
        name: impl Into<Text>,
        jsdoc_comment: Option<JsdocComment>,
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
