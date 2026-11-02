use crate::css_module_info::CssClassReference;
use biome_js_semantic::Binding;
use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsBinding, AnyJsBindingPattern, AnyJsDeclarationClause,
    AnyJsExportClause, AnyJsExportDefaultDeclaration, AnyJsExpression, AnyJsImportLike,
    AnyJsModuleItem, AnyJsObjectBindingPatternMember, AnyJsRoot, AnyJsStatement,
    AnyJsxAttributeName, AnyJsxAttributeValue, AnyTsExternalModuleDeclarationBody,
    AnyTsIdentifierBinding, AnyTsModuleName, AnyTsModuleReference, JsExport,
    JsExportDefaultDeclarationClause, JsExportFromClause, JsExportNamedFromClause,
    JsExportNamedSpecifierList, JsIdentifierBinding, JsLanguage, JsSyntaxNode,
    JsVariableDeclaratorList, JsxAttribute, TsExportAssignmentClause, TsExternalModuleDeclaration,
    TsGlobalDeclaration, TsModuleBlock, binding_ext::AnyJsBindingDeclaration, unescape_js_string,
};
use biome_js_type_info::{ImportSymbol, RawTypeCollector, TypeData, TypeReference};
use biome_rowan::{AstNode, AstNodeList, SyntaxKindSet, TokenText, WalkEvent};
use camino::Utf8PathBuf;

use crate::{
    JsAmbientModule, JsImport, JsImportPhase, JsModuleInfo, JsReexport,
    js_module_info::collector::{JsCollectedExport, TypeInferenceMode},
};

use super::collector::JsModuleInfoCollector;

pub(crate) struct JsModuleVisitor {
    root: AnyJsRoot,
    file_path: Utf8PathBuf,
    semantic_model: std::sync::Arc<biome_js_semantic::SemanticModel>,
    inference_mode: TypeInferenceMode,
}

impl JsModuleVisitor {
    pub fn new(
        root: AnyJsRoot,
        file_path: Utf8PathBuf,
        semantic_model: std::sync::Arc<biome_js_semantic::SemanticModel>,
        inference_mode: TypeInferenceMode,
    ) -> Self {
        Self {
            root,
            file_path,
            semantic_model,
            inference_mode,
        }
    }

    pub fn collect_info(mut self) -> JsModuleInfo {
        let mut collector = JsModuleInfoCollector::new(self.semantic_model.clone());
        let kinds = AnyJsImportLike::KIND_SET
            .union(JsExport::KIND_SET)
            .union(JsxAttribute::KIND_SET)
            .union(JsModuleInfoCollector::LEAVE_NODE_KINDS);
        let root = self.root.syntax().clone();
        self.collect_from(&root, kinds, &mut collector);
        let ambient_modules = self.collect_ambient_modules(&collector);
        collector.ambient_modules = ambient_modules;

        JsModuleInfo::new(collector, self.semantic_model, self.inference_mode)
    }

    /// Visits the nodes of `kinds` in `node` and its descendants, and collects
    /// their imports, exports, class references, and types into `collector`.
    fn collect_from(
        &mut self,
        node: &JsSyntaxNode,
        kinds: SyntaxKindSet<JsLanguage>,
        collector: &mut JsModuleInfoCollector,
    ) {
        for event in node.preorder_of_kinds(kinds) {
            match event {
                WalkEvent::Enter(node) => {
                    if let Some(import) = AnyJsImportLike::cast_ref(&node) {
                        self.visit_import(import, collector);
                    } else if let Some(export) = JsExport::cast_ref(&node) {
                        self.visit_export(export, collector);
                    } else if let Some(attr) = JsxAttribute::cast_ref(&node) {
                        self.visit_jsx_attribute(attr, collector);
                    }
                }
                WalkEvent::Leave(node) => {
                    collector.leave_node(&node);
                }
            }
        }
    }

    /// Collects the modules that the file declares with
    /// `declare module "name"` at the top level.
    ///
    /// Returns nothing for a file with top-level imports or exports, because
    /// each such declaration there augments an existing module.
    ///
    /// `file` is the collector of the file, before it infers types.
    fn collect_ambient_modules(&mut self, file: &JsModuleInfoCollector) -> Box<[JsAmbientModule]> {
        let items = match &self.root {
            AnyJsRoot::JsModule(module) => module.items(),
            AnyJsRoot::TsDeclarationModule(module) => module.items(),
            _ => return Box::default(),
        };
        if items.iter().any(|item| is_module_syntax(&item)) {
            return Box::default();
        }

        items
            .iter()
            .filter_map(|item| {
                let AnyJsModuleItem::AnyJsStatement(AnyJsStatement::TsDeclareStatement(statement)) =
                    item
                else {
                    return None;
                };
                match statement.declaration().ok()? {
                    AnyJsDeclarationClause::TsExternalModuleDeclaration(declaration) => {
                        Some(declaration)
                    }
                    _ => None,
                }
            })
            .filter_map(|declaration| self.collect_ambient_module(file, &declaration))
            .collect()
    }

    fn collect_ambient_module(
        &mut self,
        file: &JsModuleInfoCollector,
        declaration: &TsExternalModuleDeclaration,
    ) -> Option<JsAmbientModule> {
        let name = declaration
            .source()
            .ok()?
            .as_js_module_source()?
            .inner_string_text()
            .ok()?;
        let info = match declaration.body() {
            Some(AnyTsExternalModuleDeclarationBody::TsModuleBlock(block)) => {
                Some(self.collect_ambient_module_info(file, declaration, &block))
            }
            Some(AnyTsExternalModuleDeclarationBody::TsEmptyExternalModuleDeclarationBody(_))
            | None => None,
        };

        Some(JsAmbientModule {
            name: name.into(),
            range: declaration.syntax().text_trimmed_range(),
            info,
        })
    }

    fn collect_ambient_module_info(
        &mut self,
        file: &JsModuleInfoCollector,
        declaration: &TsExternalModuleDeclaration,
        block: &TsModuleBlock,
    ) -> JsModuleInfo {
        let scope = self.semantic_model.scope(declaration.syntax());
        let mut collector = JsModuleInfoCollector::new_for_ambient_module(file, scope.id());
        // The module info of an ambient module has no types, so the walk only
        // needs the imports and exports.
        let kinds = AnyJsImportLike::KIND_SET
            .union(JsExport::KIND_SET)
            .union(biome_js_syntax::JsImport::KIND_SET);
        self.collect_from(block.syntax(), kinds, &mut collector);

        // TypeScript exports every declaration of an ambient module that has
        // no export declaration or export assignment, even a declaration
        // without the `export` modifier.
        if !has_export_declarations(block) {
            for binding in scope.bindings() {
                if !is_implicitly_exported(&binding) {
                    continue;
                }
                let Ok(name) = binding.tree().name_token() else {
                    continue;
                };
                let name = name.token_text_trimmed();
                collector.register_export_with_name(name.clone(), name);
            }
        }

        JsModuleInfo::new(
            collector,
            self.semantic_model.clone(),
            TypeInferenceMode::Disabled,
        )
    }

    /// Collects static CSS class references from JSX `class` and `className`
    /// attributes.
    ///
    /// Only static string literals are collected; dynamic expressions are
    /// skipped to avoid false positives in `noUnusedClasses`.
    fn visit_jsx_attribute(&self, attr: JsxAttribute, collector: &mut JsModuleInfoCollector) {
        if let Some(inner) = self.extract_jsx_class_attribute_inner(&attr) {
            collector
                .referenced_classes
                .push(CssClassReference::new(inner, self.file_path.clone()));
        }
    }

    /// Extracts the inner (quote-stripped) text from a JSX `class` or
    /// `className` attribute, if it is a static string literal.
    ///
    /// Returns `None` if the attribute is not a class attribute, is not a
    /// static string, or has malformed structure.
    fn extract_jsx_class_attribute_inner(&self, attr: &JsxAttribute) -> Option<TokenText> {
        let name_node = attr.name().ok()?;
        let name_text = match name_node {
            AnyJsxAttributeName::JsxName(n) => n.value_token().ok()?.token_text_trimmed(),
            AnyJsxAttributeName::JsxNamespaceName(_) => return None,
        };
        if name_text != "class" && name_text != "className" {
            return None;
        }

        let jsx_string = attr
            .initializer()
            .and_then(|init| init.value().ok())
            .and_then(|val| match val {
                AnyJsxAttributeValue::JsxString(s) => Some(s),
                _ => None,
            })?;

        jsx_string.inner_string_text().ok()
    }

    fn visit_import(&mut self, node: AnyJsImportLike, collector: &mut JsModuleInfoCollector) {
        let Some(specifier) = node.inner_string_text() else {
            return;
        };

        match node {
            AnyJsImportLike::JsModuleSource(source) => {
                // TODO: support defer or source imports
                let phase = if source.imports_only_types() {
                    JsImportPhase::Type
                } else {
                    JsImportPhase::Default
                };

                collector.register_static_import_path(specifier, phase);
            }
            AnyJsImportLike::JsCallExpression(_) | AnyJsImportLike::JsImportCallExpression(_) => {
                collector.register_dynamic_import_path(
                    specifier,
                    JsImportPhase::Default, // TODO: support defer or source imports
                );
            }
        }
    }

    fn visit_export(
        &mut self,
        node: JsExport,
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
        let scope_id = collector.export_scope;
        let type_data =
            TypeData::from_any_js_expression(collector, scope_id, &node.expression().ok()?);
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
        let name = match node {
            AnyJsExportDefaultDeclaration::JsClassExportDefaultDeclaration(node) => {
                node.id().and_then(get_name)
            }
            AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(node) => {
                node.id().and_then(get_name)
            }
            AnyJsExportDefaultDeclaration::TsDeclareFunctionExportDefaultDeclaration(node) => {
                node.id().and_then(get_name)
            }
            AnyJsExportDefaultDeclaration::TsInterfaceDeclaration(node) => {
                node.id().ok().and_then(get_ts_name)
            }
        };

        if let Some(name) = name {
            collector.register_export_with_name("default", name);
        } else if matches!(
            node,
            AnyJsExportDefaultDeclaration::JsClassExportDefaultDeclaration(_)
                | AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(_)
                | AnyJsExportDefaultDeclaration::TsDeclareFunctionExportDefaultDeclaration(_)
        ) {
            collector.register_default_export_declaration(node);
        } else {
            return None;
        }

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
                let scope_id = collector.export_scope;
                let type_data = TypeData::from_any_js_expression(collector, scope_id, node);
                let ty = TypeReference::from(collector.register_and_resolve(type_data));
                collector.register_export(JsCollectedExport::ExportDefault { ty });
            }
        }

        Some(())
    }

    fn visit_export_from_clause(
        &mut self,
        node: JsExportFromClause,
        collector: &mut JsModuleInfoCollector,
    ) -> Option<()> {
        let module_source = node.source().ok()?;
        let specifier = module_source
            .as_js_module_source()?
            .inner_string_text()
            .ok()?;
        let import = JsImport {
            specifier: specifier.into(),
            symbol: ImportSymbol::All,
        };

        let export_range = node.syntax().parent().map(|p| p.text_trimmed_range());
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
                    export_range,
                },
            });
        } else {
            collector.register_blanket_reexport(JsReexport {
                import,
                export_range,
            });
        }

        Some(())
    }

    fn visit_export_named_from_clause(
        &mut self,
        node: JsExportNamedFromClause,
        collector: &mut JsModuleInfoCollector,
    ) -> Option<()> {
        let module_source = node.source().ok()?;
        let import_specifier = module_source
            .as_js_module_source()?
            .inner_string_text()
            .ok()?;
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
                        symbol: ImportSymbol::Named(imported_name),
                    },
                    export_range: None,
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
}

/// Returns whether `item` makes its file a module: an import, an export, or
/// `import name = require("specifier")`.
fn is_module_syntax(item: &AnyJsModuleItem) -> bool {
    match item {
        AnyJsModuleItem::JsImport(_) | AnyJsModuleItem::JsExport(_) => true,
        AnyJsModuleItem::AnyJsStatement(AnyJsStatement::TsImportEqualsDeclaration(declaration)) => {
            matches!(
                declaration.module_reference(),
                Ok(AnyTsModuleReference::TsExternalModuleReference(_))
            )
        }
        AnyJsModuleItem::AnyJsStatement(_) => false,
    }
}

/// Returns whether TypeScript exports `binding` under its own name from an
/// ambient module without export declarations.
///
/// Imports aren't exported. A declaration in a `global { ... }` block declares
/// a global instead. `export default function name()` exports only `default`.
fn is_implicitly_exported(binding: &Binding) -> bool {
    let tree = binding.tree();
    let declaration = tree.declaration();
    let is_import = binding.declaration_kind().is_import_declaration()
        || matches!(
            declaration,
            Some(AnyJsBindingDeclaration::TsImportEqualsDeclaration(_))
        );
    let is_default_export = declaration.is_some_and(|declaration| {
        declaration
            .syntax()
            .parent()
            .is_some_and(|parent| JsExportDefaultDeclarationClause::can_cast(parent.kind()))
    });
    let is_global = tree
        .syntax()
        .ancestors()
        .any(|node| TsGlobalDeclaration::can_cast(node.kind()));

    !is_import && !is_default_export && !is_global
}

/// Returns whether `block` contains an export declaration, such as
/// `export { name }` or `export * from "specifier"`, or an export assignment,
/// such as `export = name` or `export default name`.
fn has_export_declarations(block: &TsModuleBlock) -> bool {
    block.items().iter().any(|item| {
        let AnyJsModuleItem::JsExport(export) = item else {
            return false;
        };
        matches!(
            export.export_clause(),
            Ok(AnyJsExportClause::JsExportNamedClause(_)
                | AnyJsExportClause::JsExportFromClause(_)
                | AnyJsExportClause::JsExportNamedFromClause(_)
                | AnyJsExportClause::JsExportDefaultExpressionClause(_)
                | AnyJsExportClause::TsExportAssignmentClause(_))
        )
    })
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
