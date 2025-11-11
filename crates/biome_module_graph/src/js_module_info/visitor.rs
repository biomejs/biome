use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsBinding, AnyJsBindingPattern, AnyJsDeclarationClause,
    AnyJsExportClause, AnyJsExportDefaultDeclaration, AnyJsExpression, AnyJsImportClause,
    AnyJsImportLike, AnyJsObjectBindingPatternMember, AnyJsRoot, AnyTsIdentifierBinding,
    AnyTsModuleName, JsExportFromClause, JsExportNamedFromClause, JsExportNamedSpecifierList,
    JsIdentifierBinding, JsVariableDeclaratorList, TsExportAssignmentClause, unescape_js_string,
};
use biome_js_type_info::{ImportSymbol, ScopeId, TypeData, TypeReference, TypeResolver};
use biome_jsdoc_comment::JsdocComment;
use biome_resolver::{ResolveOptions, resolve};
use biome_rowan::{AstNode, TokenText, WalkEvent};
use camino::Utf8Path;

use crate::{
    JsImport, JsImportPhase, JsModuleInfo, JsReexport, SUPPORTED_EXTENSIONS,
    js_module_info::collector::JsCollectedExport, module_graph::ModuleGraphFsProxy,
};

use super::{ResolvedPath, collector::JsModuleInfoCollector};

/// Extensions to try to resolve based on the extension in the import specifier.
/// ref: https://www.typescriptlang.org/docs/handbook/modules/reference.html#the-moduleresolution-compiler-option
const EXTENSION_ALIASES: &[(&str, &[&str])] = &[
    ("js", &["ts", "tsx", "d.ts", "js", "jsx"]),
    ("mjs", &["mts", "d.mts", "mjs"]),
    ("cjs", &["cts", "d.cts", "cjs"]),
];

pub(crate) struct JsModuleVisitor<'a> {
    root: AnyJsRoot,
    directory: &'a Utf8Path,
    fs_proxy: &'a ModuleGraphFsProxy<'a>,
}

impl<'a> JsModuleVisitor<'a> {
    pub fn new(root: AnyJsRoot, directory: &'a Utf8Path, fs_proxy: &'a ModuleGraphFsProxy) -> Self {
        Self {
            root,
            directory,
            fs_proxy,
        }
    }

    pub fn collect_info(self) -> JsModuleInfo {
        let mut collector = JsModuleInfoCollector::default();

        let iter = self.root.syntax().preorder();
        for event in iter {
            match event {
                WalkEvent::Enter(node) => {
                    if let Some(import) = AnyJsImportLike::cast_ref(&node) {
                        self.visit_import(import, &mut collector);
                    } else if let Some(export) = biome_js_syntax::JsExport::cast_ref(&node) {
                        self.visit_export(export, &mut collector);
                    }

                    collector.push_node(&node);
                }
                WalkEvent::Leave(node) => {
                    collector.leave_node(&node);
                }
            }
        }

        JsModuleInfo::new(collector)
    }

    fn visit_import(&self, node: AnyJsImportLike, collector: &mut JsModuleInfoCollector) {
        let Some(specifier) = node.inner_string_text() else {
            return;
        };

        let resolved_path = self.resolved_path_from_specifier(specifier.text());

        match node {
            AnyJsImportLike::JsModuleSource(source) => {
                // TODO: support defer or source imports
                let phase = if let Some(clause) = source.parent::<AnyJsImportClause>() {
                    match clause {
                        AnyJsImportClause::JsImportDefaultClause(clause)
                            if clause.type_token().is_some() =>
                        {
                            JsImportPhase::Type
                        }
                        AnyJsImportClause::JsImportNamedClause(clause)
                            if clause.type_token().is_some() =>
                        {
                            JsImportPhase::Type
                        }
                        AnyJsImportClause::JsImportNamespaceClause(clause)
                            if clause.type_token().is_some() =>
                        {
                            JsImportPhase::Type
                        }
                        _ => JsImportPhase::Default,
                    }
                } else {
                    JsImportPhase::Default
                };

                collector.register_static_import_path(specifier, resolved_path, phase);
            }
            AnyJsImportLike::JsCallExpression(_) | AnyJsImportLike::JsImportCallExpression(_) => {
                collector.register_dynamic_import_path(
                    specifier,
                    resolved_path,
                    JsImportPhase::Default, // TODO: support defer or source imports
                );
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
                self.visit_export_default_expression(&node.expression().ok()?, collector)
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
            AnyJsExportClause::TsExportAsNamespaceClause(_node) => {
                // FIXME: We may need to implement this if we want to fully
                //        support global namespace merging.
                None
            }
            AnyJsExportClause::TsExportAssignmentClause(node) => {
                self.visit_export_assignment_clause(node, collector)
            }
            AnyJsExportClause::TsExportDeclareClause(node) => {
                self.visit_export_declaration_clause(node.declaration().ok()?, collector)
            }
        }
    }

    /// Handles `export =` clauses.
    ///
    /// Export assignments create both a `default` export as well as named
    /// exports for any members of the symbol being exported.
    fn visit_export_assignment_clause(
        &self,
        node: TsExportAssignmentClause,
        collector: &mut JsModuleInfoCollector,
    ) -> Option<()> {
        let type_data =
            TypeData::from_any_js_expression(collector, ScopeId::GLOBAL, &node.expression().ok()?);
        let ty = TypeReference::from(collector.register_and_resolve(type_data));
        collector.register_export(JsCollectedExport::ExportDefaultAssignment { ty });

        Some(())
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

        collector.register_export_with_name(name.clone(), name);
        Some(())
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

        collector.register_export_with_name("default", name);
        Some(())
    }

    fn visit_export_default_expression(
        &self,
        node: &AnyJsExpression,
        collector: &mut JsModuleInfoCollector,
    ) -> Option<()> {
        match node {
            AnyJsExpression::JsIdentifierExpression(ident) => {
                let local_name = ident.name().ok()?.name().ok()?;
                collector.register_export(JsCollectedExport::ExportNamedDefault { local_name });
            }
            _ => {
                let type_data = TypeData::from_any_js_expression(collector, ScopeId::GLOBAL, node);
                let ty = TypeReference::from(collector.register_and_resolve(type_data));
                collector.register_export(JsCollectedExport::ExportDefault { ty });
            }
        }

        Some(())
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
            let export_name = export_as
                .exported_name()
                .and_then(|name| name.inner_string_text())
                .map(unescape_js_string)
                .ok()?;
            collector.register_export(JsCollectedExport::Reexport {
                export_name,
                reexport: JsReexport {
                    import,
                    jsdoc_comment,
                },
            });
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
            let export_name = specifier
                .export_as()
                .and_then(|export_as| export_as.exported_name().ok())
                .and_then(|name| name.inner_string_text().ok())
                .map_or_else(|| imported_name.clone(), unescape_js_string);
            collector.register_export(JsCollectedExport::Reexport {
                export_name,
                reexport: JsReexport {
                    import: JsImport {
                        specifier: import_specifier.clone().into(),
                        resolved_path: resolved_path.clone(),
                        symbol: ImportSymbol::Named(imported_name),
                    },
                    jsdoc_comment: None,
                },
            });
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
                if let Some(local_name) = local_name {
                    collector.register_export_with_name(export_name, local_name);
                }
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
                            if let Ok(binding) = node.binding()
                                && let Some(binding) = binding.as_js_identifier_binding() {
                                    self.visit_identifier_binding(
                                        binding,
                                        collector,
                                    );
                                }
                        }
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                            node,
                        ) => {
                            if let Ok(binding) = node.identifier()
                                && let Some(binding) = binding.as_js_identifier_binding() {
                                    self.visit_identifier_binding(
                                        binding,
                                        collector,
                                    );
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
        collector.register_export_with_name(name.clone(), name);
        Some(())
    }

    fn resolved_path_from_specifier(&self, specifier: &str) -> ResolvedPath {
        let options = ResolveOptions {
            condition_names: &["types", "import", "default"],
            default_files: &["index"],
            extensions: SUPPORTED_EXTENSIONS,
            extension_aliases: EXTENSION_ALIASES,
            resolve_node_builtins: true,
            resolve_types: true,
            ..Default::default()
        };
        let resolved_path = resolve(specifier, self.directory, self.fs_proxy, &options);
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
