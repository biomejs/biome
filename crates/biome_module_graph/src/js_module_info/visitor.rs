use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsBinding, AnyJsBindingPattern, AnyJsDeclarationClause,
    AnyJsExportClause, AnyJsExportDefaultDeclaration, AnyJsImportLike,
    AnyJsObjectBindingPatternMember, AnyJsRoot, AnyTsIdentifierBinding, AnyTsModuleName,
    JsExportDefaultExpressionClause, JsExportFromClause, JsExportNamedFromClause,
    JsExportNamedSpecifierList, JsIdentifierBinding, JsVariableDeclaratorList, unescape_js_string,
};
use biome_js_type_info::{ImportSymbol, TypeData, TypeResolver};
use biome_rowan::{AstNode, TokenText, WalkEvent};
use camino::{Utf8Path, Utf8PathBuf};
use oxc_resolver::{ResolveError, ResolverGeneric};

use crate::{
    JsExport, JsImport, JsModuleInfo, JsOwnExport, JsReexport, jsdoc_comment::JsdocComment,
    resolver_cache::ResolverCache,
};

use super::{ResolvedPath, collector::JsModuleInfoCollector};

pub(crate) struct JsModuleVisitor<'a> {
    root: AnyJsRoot,
    directory: &'a Utf8Path,
    resolver: &'a ResolverGeneric<ResolverCache<'a>>,
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
        }
    }

    pub fn collect_info(self) -> JsModuleInfo {
        let mut collector = JsModuleInfoCollector::default();

        let iter = self.root.syntax().preorder();
        for event in iter {
            match event {
                WalkEvent::Enter(node) => {
                    collector.push_node(&node);

                    if let Some(import) = AnyJsImportLike::cast_ref(&node) {
                        self.visit_import(import, &mut collector);
                    } else if let Some(export) = biome_js_syntax::JsExport::cast_ref(&node) {
                        self.visit_export(export, &mut collector);
                    }
                }
                WalkEvent::Leave(node) => {
                    collector.leave_node(&node);
                }
            }
        }

        collector.finalise();

        JsModuleInfo::new(collector)
    }

    fn visit_import(&self, node: AnyJsImportLike, collector: &mut JsModuleInfoCollector) {
        let Some(specifier) = node.inner_string_text() else {
            return;
        };

        let resolved_path = self.resolved_path_from_specifier(specifier.text());

        match node {
            AnyJsImportLike::JsModuleSource(_) => {
                collector.register_static_import_path(specifier, resolved_path);
            }
            AnyJsImportLike::JsCallExpression(_) | AnyJsImportLike::JsImportCallExpression(_) => {
                collector.register_dynamic_import_path(specifier, resolved_path);
            }
        }
    }

    fn visit_export(
        &self,
        node: biome_js_syntax::JsExport,
        collector: &mut JsModuleInfoCollector,
    ) -> Option<()> {
        match node.export_clause().ok()? {
            AnyJsExportClause::AnyJsDeclarationClause(node) => {
                self.visit_export_declaration_clause(node, collector)
            }
            AnyJsExportClause::JsExportDefaultDeclarationClause(node) => {
                self.visit_export_default_declaration_clause(&node.declaration().ok()?, collector)
            }
            AnyJsExportClause::JsExportDefaultExpressionClause(node) => {
                self.visit_export_default_expression_clause(&node, collector)
            }
            AnyJsExportClause::JsExportFromClause(node) => {
                self.visit_export_from_clause(node, collector)
            }
            AnyJsExportClause::JsExportNamedClause(node) => {
                self.visit_export_named_specifiers(node.specifiers(), collector)
            }
            AnyJsExportClause::JsExportNamedFromClause(node) => {
                self.visit_export_named_from_clause(node, collector)
            }
            AnyJsExportClause::TsExportAsNamespaceClause(node) => {
                let token = node.name().ok()?.value_token().ok()?;
                let name = token.token_text_trimmed();
                collector.register_export_with_name(name, None)
            }
            AnyJsExportClause::TsExportAssignmentClause(_) => {
                // This is somewhat misleading, since the `export =` syntax is
                // used for CommonJS exports rather than ES6 `default` exports.
                // Thankfully, bundlers are responsible for normalising this,
                // which isn't really Biome's concern.
                collector.register_export_with_name("default", None)
            }
            AnyJsExportClause::TsExportDeclareClause(node) => {
                self.visit_export_declaration_clause(node.declaration().ok()?, collector)
            }
        }
    }

    fn visit_export_declaration_clause(
        &self,
        node: AnyJsDeclarationClause,
        collector: &mut JsModuleInfoCollector,
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
                    collector,
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

        collector.register_export_with_name(name.clone(), Some(name))
    }

    fn visit_export_default_declaration_clause(
        &self,
        node: &AnyJsExportDefaultDeclaration,
        collector: &mut JsModuleInfoCollector,
    ) -> Option<()> {
        let name = match &node {
            AnyJsExportDefaultDeclaration::JsClassExportDefaultDeclaration(node) => {
                node.id().and_then(get_name)?
            }
            AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(node) => {
                node.id().and_then(get_name)?
            }
            AnyJsExportDefaultDeclaration::TsDeclareFunctionExportDefaultDeclaration(node) => {
                node.id().and_then(get_name)?
            }
            AnyJsExportDefaultDeclaration::TsInterfaceDeclaration(node) => {
                node.id().ok().and_then(get_ts_name)?
            }
        };

        collector.register_export_with_name("default", Some(name))
    }

    fn visit_export_default_expression_clause(
        &self,
        node: &JsExportDefaultExpressionClause,
        collector: &mut JsModuleInfoCollector,
    ) -> Option<()> {
        let type_data = TypeData::from_any_js_expression(collector, &node.expression().ok()?);
        let ty = collector.register_and_resolve(type_data).into();
        collector.register_export(
            "default",
            JsExport::Own(JsOwnExport {
                jsdoc_comment: None,
                local_name: None,
                ty,
            }),
        )
    }

    fn visit_export_from_clause(
        &self,
        node: JsExportFromClause,
        collector: &mut JsModuleInfoCollector,
    ) -> Option<()> {
        let module_source = node.source().ok()?;
        let specifier = module_source
            .as_js_module_source()?
            .inner_string_text()
            .ok()?;
        let import = JsImport {
            resolved_path: self.resolved_path_from_specifier(&specifier),
            specifier: specifier.into(),
            symbol: ImportSymbol::All,
        };
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
            collector.register_export(
                name,
                JsExport::Reexport(JsReexport {
                    import,
                    jsdoc_comment,
                }),
            );
        } else {
            collector.register_blanket_reexport(JsReexport {
                import,
                jsdoc_comment,
            });
        }

        Some(())
    }

    fn visit_export_named_from_clause(
        &self,
        node: JsExportNamedFromClause,
        collector: &mut JsModuleInfoCollector,
    ) -> Option<()> {
        let module_source = node.source().ok()?;
        let import_specifier = module_source
            .as_js_module_source()?
            .inner_string_text()
            .ok()?;
        let resolved_path = self.resolved_path_from_specifier(&import_specifier);

        for specifier in node.specifiers() {
            let Ok(specifier) = specifier else {
                continue;
            };

            let imported_name = specifier
                .source_name()
                .ok()
                .and_then(|name| name.inner_string_text().ok())
                .map(unescape_js_string)?;
            let exported_name = specifier
                .export_as()
                .and_then(|export_as| export_as.exported_name().ok())
                .and_then(|name| name.inner_string_text().ok())
                .map_or_else(|| imported_name.clone(), unescape_js_string);
            collector.register_export(
                exported_name,
                JsExport::Reexport(JsReexport {
                    import: JsImport {
                        specifier: import_specifier.clone().into(),
                        resolved_path: resolved_path.clone(),
                        symbol: ImportSymbol::Named(imported_name),
                    },
                    jsdoc_comment: None,
                }),
            );
        }

        Some(())
    }

    fn visit_export_named_specifiers(
        &self,
        specifiers: JsExportNamedSpecifierList,
        collector: &mut JsModuleInfoCollector,
    ) -> Option<()> {
        for specifier in specifiers {
            let Ok(specifier) = specifier else {
                continue;
            };
            if let Ok(export_name) = specifier.exported_name() {
                let local_name = specifier
                    .local_name()
                    .ok()
                    .and_then(|name| name.value_token().ok())
                    .map(|name_token| name_token.token_text_trimmed());
                collector.register_export_with_name(export_name, local_name);
            }
        }

        Some(())
    }

    fn visit_export_variable_declarations(
        &self,
        declarators: JsVariableDeclaratorList,
        collector: &mut JsModuleInfoCollector,
    ) -> Option<()> {
        for declarator in declarators.into_iter().flatten() {
            if let Ok(binding) = declarator.id() {
                self.visit_binding_pattern(binding, collector);
            }
        }

        Some(())
    }

    fn visit_binding_pattern(
        &self,
        binding: AnyJsBindingPattern,
        collector: &mut JsModuleInfoCollector,
    ) -> Option<()> {
        match binding {
            AnyJsBindingPattern::AnyJsBinding(node) => {
                if let Some(binding) = node.as_js_identifier_binding() {
                    self.visit_identifier_binding(binding, collector)?;
                }
            }
            AnyJsBindingPattern::JsArrayBindingPattern(node) => {
                for element in node.elements().into_iter().flatten() {
                    match element {
                        AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(node) => {
                            if let Ok(binding) = node.pattern() {
                                self.visit_binding_pattern(binding, collector);
                            }
                        }
                        AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(node) => {
                            if let Ok(binding) = node.pattern() {
                                self.visit_binding_pattern(binding, collector);
                            }
                        }
                        AnyJsArrayBindingPatternElement::JsArrayHole(_) => {}
                    }
                }
            }
            AnyJsBindingPattern::JsObjectBindingPattern(node) => {
                for property in node.properties().into_iter().flatten() {
                    match property {
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(node) => {
                            if let Ok(binding) = node.pattern() {
                                self.visit_binding_pattern(
                                    binding,
                                   collector,
                                );
                            }
                        }
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(node) => {
                            if let Ok(binding) = node.binding() {
                                if let Some(binding) = binding.as_js_identifier_binding() {
                                    self.visit_identifier_binding(
                                        binding,
                                        collector,
                                    );
                                }
                            }
                        }
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                            node,
                        ) => {
                            if let Ok(binding) = node.identifier() {
                                if let Some(binding) = binding.as_js_identifier_binding() {
                                    self.visit_identifier_binding(
                                        binding,
                                        collector,
                                    );
                                }
                            }
                        }
                        AnyJsObjectBindingPatternMember::JsBogusBinding(_)
                        | AnyJsObjectBindingPatternMember::JsMetavariable(_) => {}
                    }
                }
            }
        }

        Some(())
    }

    fn visit_identifier_binding(
        &self,
        binding: &JsIdentifierBinding,
        collector: &mut JsModuleInfoCollector,
    ) -> Option<()> {
        let name = binding.name_token().ok()?.token_text_trimmed();
        collector.register_export_with_name(name.clone(), Some(name))
    }

    fn resolved_path_from_specifier(&self, specifier: &str) -> ResolvedPath {
        let resolved_path = self
            .resolver
            .resolve(self.directory, specifier)
            .and_then(|resolution| {
                Utf8PathBuf::from_path_buf(resolution.into_path_buf())
                    .map_err(|path| ResolveError::NotFound(path.to_string_lossy().to_string()))
            })
            .map_err(|error| error.to_string());
        ResolvedPath::new(resolved_path)
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
