use super::*;

pub(super) fn convert_import(ctx: &ConvertCtx<'_>, import: &JsImport) -> Result<ImportDeclaration> {
    let clause = import
        .import_clause()
        .map_err(|_| missing("JsImport", "import_clause"))?;
    let (specifiers, source, import_kind) = match clause {
        AnyJsImportClause::JsImportBareClause(clause) => (
            Vec::new(),
            clause
                .source()
                .map_err(|_| missing("JsImportBareClause", "source"))?,
            None,
        ),
        AnyJsImportClause::JsImportDefaultClause(clause) => {
            let local = convert_any_binding_identifier(
                ctx,
                &clause
                    .default_specifier()
                    .map_err(|_| missing("JsImportDefaultClause", "default_specifier"))?
                    .local_name()
                    .map_err(|_| missing("JsDefaultImportSpecifier", "local_name"))?,
            )?;
            (
                vec![ImportSpecifier::ImportDefaultSpecifier(
                    ImportDefaultSpecifierData {
                        base: ctx.base(clause.syntax().text_trimmed_range()),
                        local,
                    },
                )],
                clause
                    .source()
                    .map_err(|_| missing("JsImportDefaultClause", "source"))?,
                clause.type_token().map(|_| ImportKind::Type),
            )
        }
        AnyJsImportClause::JsImportNamedClause(clause) => (
            convert_named_import_specifiers(
                ctx,
                &clause
                    .named_specifiers()
                    .map_err(|_| missing("JsImportNamedClause", "named_specifiers"))?,
            )?,
            clause
                .source()
                .map_err(|_| missing("JsImportNamedClause", "source"))?,
            clause.type_token().map(|_| ImportKind::Type),
        ),
        AnyJsImportClause::JsImportNamespaceClause(clause) => {
            let namespace = clause
                .namespace_specifier()
                .map_err(|_| missing("JsImportNamespaceClause", "namespace_specifier"))?;
            (
                vec![ImportSpecifier::ImportNamespaceSpecifier(
                    ImportNamespaceSpecifierData {
                        base: ctx.base(namespace.syntax().text_trimmed_range()),
                        local: convert_any_binding_identifier(
                            ctx,
                            &namespace
                                .local_name()
                                .map_err(|_| missing("JsNamespaceImportSpecifier", "local_name"))?,
                        )?,
                    },
                )],
                clause
                    .source()
                    .map_err(|_| missing("JsImportNamespaceClause", "source"))?,
                clause.type_token().map(|_| ImportKind::Type),
            )
        }
        AnyJsImportClause::JsImportCombinedClause(clause) => {
            let mut specifiers = vec![ImportSpecifier::ImportDefaultSpecifier(
                ImportDefaultSpecifierData {
                    base: ctx.base(clause.syntax().text_trimmed_range()),
                    local: convert_any_binding_identifier(
                        ctx,
                        &clause
                            .default_specifier()
                            .map_err(|_| missing("JsImportCombinedClause", "default_specifier"))?
                            .local_name()
                            .map_err(|_| missing("JsDefaultImportSpecifier", "local_name"))?,
                    )?,
                },
            )];
            match clause
                .specifier()
                .map_err(|_| missing("JsImportCombinedClause", "specifier"))?
            {
                AnyJsCombinedSpecifier::JsNamedImportSpecifiers(named) => {
                    specifiers.extend(convert_named_import_specifiers(ctx, &named)?);
                }
                AnyJsCombinedSpecifier::JsNamespaceImportSpecifier(namespace) => {
                    specifiers.push(ImportSpecifier::ImportNamespaceSpecifier(
                        ImportNamespaceSpecifierData {
                            base: ctx.base(namespace.syntax().text_trimmed_range()),
                            local: convert_any_binding_identifier(
                                ctx,
                                &namespace.local_name().map_err(|_| {
                                    missing("JsNamespaceImportSpecifier", "local_name")
                                })?,
                            )?,
                        },
                    ));
                }
            }
            (
                specifiers,
                clause
                    .source()
                    .map_err(|_| missing("JsImportCombinedClause", "source"))?,
                None,
            )
        }
    };

    Ok(ImportDeclaration {
        base: ctx.base(import.syntax().text_trimmed_range()),
        specifiers,
        source: convert_module_source(ctx, source)?,
        import_kind,
        assertions: None,
        attributes: None,
    })
}

pub(super) fn convert_export(ctx: &ConvertCtx<'_>, export: &JsExport) -> Result<Statement> {
    match export
        .export_clause()
        .map_err(|_| missing("JsExport", "export_clause"))?
    {
        AnyJsExportClause::AnyJsDeclarationClause(declaration) => match declaration {
            AnyJsDeclarationClause::JsFunctionDeclaration(function) => {
                Ok(Statement::ExportNamedDeclaration(ExportNamedDeclaration {
                    base: ctx.base(export.syntax().text_trimmed_range()),
                    declaration: Some(Box::new(Declaration::FunctionDeclaration(
                        convert_function_declaration(ctx, &function)?,
                    ))),
                    specifiers: Vec::new(),
                    source: None,
                    export_kind: Some(ExportKind::Value),
                    assertions: None,
                    attributes: None,
                }))
            }
            AnyJsDeclarationClause::JsClassDeclaration(class) => {
                Ok(Statement::ExportNamedDeclaration(ExportNamedDeclaration {
                    base: ctx.base(export.syntax().text_trimmed_range()),
                    declaration: Some(Box::new(Declaration::ClassDeclaration(
                        convert_class_declaration(ctx, &class)?,
                    ))),
                    specifiers: Vec::new(),
                    source: None,
                    export_kind: Some(ExportKind::Value),
                    assertions: None,
                    attributes: None,
                }))
            }
            AnyJsDeclarationClause::JsVariableDeclarationClause(clause) => {
                let declaration = clause
                    .declaration()
                    .map_err(|_| missing("JsVariableDeclarationClause", "declaration"))?;
                let kind = match declaration
                    .kind()
                    .map_err(|_| missing("JsVariableDeclaration", "kind"))?
                    .text_trimmed()
                {
                    "const" => VariableDeclarationKind::Const,
                    "let" => VariableDeclarationKind::Let,
                    "var" => VariableDeclarationKind::Var,
                    _ => return Err(unsupported(declaration.syntax())),
                };
                Ok(Statement::ExportNamedDeclaration(ExportNamedDeclaration {
                    base: ctx.base(export.syntax().text_trimmed_range()),
                    declaration: Some(Box::new(Declaration::VariableDeclaration(
                        convert_variable_declaration(ctx, &declaration, kind)?,
                    ))),
                    specifiers: Vec::new(),
                    source: None,
                    export_kind: Some(ExportKind::Value),
                    assertions: None,
                    attributes: None,
                }))
            }
            declaration => Err(unsupported(declaration.syntax())),
        },
        AnyJsExportClause::JsExportDefaultExpressionClause(clause) => Ok(
            Statement::ExportDefaultDeclaration(ExportDefaultDeclaration {
                base: ctx.base(export.syntax().text_trimmed_range()),
                declaration: Box::new(ExportDefaultDecl::Expression(Box::new(convert_expression(
                    ctx,
                    clause
                        .expression()
                        .map_err(|_| missing("JsExportDefaultExpressionClause", "expression"))?,
                )?))),
                export_kind: Some(ExportKind::Value),
            }),
        ),
        AnyJsExportClause::JsExportDefaultDeclarationClause(clause) => {
            let declaration = clause
                .declaration()
                .map_err(|_| missing("JsExportDefaultDeclarationClause", "declaration"))?;
            match declaration {
                AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(function) => Ok(
                    Statement::ExportDefaultDeclaration(ExportDefaultDeclaration {
                        base: ctx.base(export.syntax().text_trimmed_range()),
                        declaration: Box::new(ExportDefaultDecl::FunctionDeclaration(
                            convert_default_function_declaration(ctx, &function)?,
                        )),
                        export_kind: Some(ExportKind::Value),
                    }),
                ),
                AnyJsExportDefaultDeclaration::JsClassExportDefaultDeclaration(class) => Ok(
                    Statement::ExportDefaultDeclaration(ExportDefaultDeclaration {
                        base: ctx.base(export.syntax().text_trimmed_range()),
                        declaration: Box::new(ExportDefaultDecl::ClassDeclaration(
                            convert_default_class_declaration(ctx, &class)?,
                        )),
                        export_kind: Some(ExportKind::Value),
                    }),
                ),
                declaration => Err(unsupported(declaration.syntax())),
            }
        }
        AnyJsExportClause::JsExportNamedClause(clause) => {
            Ok(Statement::ExportNamedDeclaration(ExportNamedDeclaration {
                base: ctx.base(export.syntax().text_trimmed_range()),
                declaration: None,
                specifiers: convert_export_named_specifiers(ctx, clause.specifiers())?,
                source: None,
                export_kind: Some(if clause.type_token().is_some() {
                    ExportKind::Type
                } else {
                    ExportKind::Value
                }),
                assertions: None,
                attributes: None,
            }))
        }
        AnyJsExportClause::JsExportFromClause(clause) => {
            if let Some(export_as) = clause.export_as() {
                return Ok(Statement::ExportNamedDeclaration(ExportNamedDeclaration {
                    base: ctx.base(export.syntax().text_trimmed_range()),
                    declaration: None,
                    specifiers: vec![ExportSpecifier::ExportNamespaceSpecifier(
                        ExportNamespaceSpecifierData {
                            base: ctx.base(export_as.syntax().text_trimmed_range()),
                            exported: convert_literal_export_name(
                                ctx,
                                export_as
                                    .exported_name()
                                    .map_err(|_| missing("JsExportAsClause", "exported_name"))?,
                            )?,
                        },
                    )],
                    source: Some(convert_module_source(
                        ctx,
                        clause
                            .source()
                            .map_err(|_| missing("JsExportFromClause", "source"))?,
                    )?),
                    export_kind: Some(if clause.type_token().is_some() {
                        ExportKind::Type
                    } else {
                        ExportKind::Value
                    }),
                    assertions: None,
                    attributes: None,
                }));
            }
            Ok(Statement::ExportAllDeclaration(ExportAllDeclaration {
                base: ctx.base(export.syntax().text_trimmed_range()),
                source: convert_module_source(
                    ctx,
                    clause
                        .source()
                        .map_err(|_| missing("JsExportFromClause", "source"))?,
                )?,
                export_kind: Some(if clause.type_token().is_some() {
                    ExportKind::Type
                } else {
                    ExportKind::Value
                }),
                assertions: None,
                attributes: None,
            }))
        }
        AnyJsExportClause::JsExportNamedFromClause(clause) => {
            Ok(Statement::ExportNamedDeclaration(ExportNamedDeclaration {
                base: ctx.base(export.syntax().text_trimmed_range()),
                declaration: None,
                specifiers: clause
                    .specifiers()
                    .into_iter()
                    .map(|specifier| {
                        let specifier = specifier
                            .map_err(|_| missing("JsExportNamedFromClause", "specifier"))?;
                        let local = convert_literal_export_name(
                            ctx,
                            specifier.source_name().map_err(|_| {
                                missing("JsExportNamedFromSpecifier", "source_name")
                            })?,
                        )?;
                        let exported = specifier
                            .export_as()
                            .map(|export_as| {
                                export_as
                                    .exported_name()
                                    .map_err(|_| missing("JsExportAsClause", "exported_name"))
                                    .and_then(|name| convert_literal_export_name(ctx, name))
                            })
                            .transpose()?
                            .unwrap_or_else(|| local.clone());
                        Ok(ExportSpecifier::ExportSpecifier(ExportSpecifierData {
                            base: ctx.base(specifier.syntax().text_trimmed_range()),
                            local,
                            exported,
                            export_kind: specifier.type_token().map(|_| ExportKind::Type),
                        }))
                    })
                    .collect::<Result<Vec<_>>>()?,
                source: Some(convert_module_source(
                    ctx,
                    clause
                        .source()
                        .map_err(|_| missing("JsExportNamedFromClause", "source"))?,
                )?),
                export_kind: Some(if clause.type_token().is_some() {
                    ExportKind::Type
                } else {
                    ExportKind::Value
                }),
                assertions: None,
                attributes: None,
            }))
        }
        clause => Err(unsupported(clause.syntax())),
    }
}

pub(super) fn convert_default_class_declaration(
    ctx: &ConvertCtx<'_>,
    class: &JsClassExportDefaultDeclaration,
) -> Result<ClassDeclaration> {
    Ok(ClassDeclaration {
        base: ctx.base(class.syntax().text_trimmed_range()),
        id: class
            .id()
            .map(|id| convert_any_binding_identifier(ctx, &id))
            .transpose()?,
        super_class: class
            .extends_clause()
            .map(|extends| {
                extends
                    .super_class()
                    .map_err(|_| missing("JsExtendsClause", "super_class"))
                    .and_then(|super_class| convert_expression(ctx, super_class))
                    .map(Box::new)
            })
            .transpose()?,
        body: convert_class_body(ctx, class.syntax(), class.members()),
        decorators: None,
        is_abstract: class.abstract_token().map(|_| true),
        declare: None,
        implements: None,
        super_type_parameters: None,
        type_parameters: None,
        mixins: None,
    })
}

pub(super) fn convert_export_named_specifiers(
    ctx: &ConvertCtx<'_>,
    specifiers: biome_js_syntax::JsExportNamedSpecifierList,
) -> Result<Vec<ExportSpecifier>> {
    specifiers
        .into_iter()
        .map(|specifier| {
            match specifier.map_err(|_| missing("JsExportNamedClause", "specifier"))? {
                biome_js_syntax::AnyJsExportNamedSpecifier::JsExportNamedShorthandSpecifier(
                    specifier,
                ) => {
                    let name = specifier
                        .name()
                        .map_err(|_| missing("JsExportNamedShorthandSpecifier", "name"))?;
                    let token = name
                        .value_token()
                        .map_err(|_| missing("JsReferenceIdentifier", "value_token"))?;
                    let identifier = Identifier {
                        base: ctx.base(name.syntax().text_trimmed_range()),
                        name: token.text_trimmed().to_string(),
                        type_annotation: None,
                        optional: None,
                        decorators: None,
                    };
                    Ok(ExportSpecifier::ExportSpecifier(ExportSpecifierData {
                        base: ctx.base(specifier.syntax().text_trimmed_range()),
                        local: ModuleExportName::Identifier(identifier.clone()),
                        exported: ModuleExportName::Identifier(identifier),
                        export_kind: specifier.type_token().map(|_| ExportKind::Type),
                    }))
                }
                biome_js_syntax::AnyJsExportNamedSpecifier::JsExportNamedSpecifier(specifier) => {
                    let local = specifier
                        .local_name()
                        .map_err(|_| missing("JsExportNamedSpecifier", "local_name"))?;
                    let local_token = local
                        .value_token()
                        .map_err(|_| missing("JsReferenceIdentifier", "value_token"))?;
                    Ok(ExportSpecifier::ExportSpecifier(ExportSpecifierData {
                        base: ctx.base(specifier.syntax().text_trimmed_range()),
                        local: ModuleExportName::Identifier(Identifier {
                            base: ctx.base(local.syntax().text_trimmed_range()),
                            name: local_token.text_trimmed().to_string(),
                            type_annotation: None,
                            optional: None,
                            decorators: None,
                        }),
                        exported: convert_literal_export_name(
                            ctx,
                            specifier
                                .exported_name()
                                .map_err(|_| missing("JsExportNamedSpecifier", "exported_name"))?,
                        )?,
                        export_kind: specifier.type_token().map(|_| ExportKind::Type),
                    }))
                }
            }
        })
        .collect()
}

pub(super) fn convert_default_function_declaration(
    ctx: &ConvertCtx<'_>,
    function: &biome_js_syntax::JsFunctionExportDefaultDeclaration,
) -> Result<FunctionDeclaration> {
    Ok(FunctionDeclaration {
        base: ctx.base(function.syntax().text_trimmed_range()),
        id: function
            .id()
            .map(|id| convert_any_binding_identifier(ctx, &id))
            .transpose()?,
        params: convert_function_parameters(
            ctx,
            &function
                .parameters()
                .map_err(|_| missing("JsFunctionExportDefaultDeclaration", "parameters"))?,
        )?,
        body: convert_function_body(
            ctx,
            &function
                .body()
                .map_err(|_| missing("JsFunctionExportDefaultDeclaration", "body"))?,
        )?,
        generator: function.star_token().is_some(),
        is_async: function.async_token().is_some(),
        declare: None,
        return_type: None,
        type_parameters: None,
        predicate: None,
        component_declaration: false,
        hook_declaration: false,
    })
}

pub(super) fn convert_named_import_specifiers(
    ctx: &ConvertCtx<'_>,
    specifiers: &JsNamedImportSpecifiers,
) -> Result<Vec<ImportSpecifier>> {
    specifiers
        .specifiers()
        .into_iter()
        .map(|specifier| {
            match specifier.map_err(|_| missing("JsNamedImportSpecifiers", "specifier"))? {
                biome_js_syntax::AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(
                    specifier,
                ) => {
                    let local = convert_any_binding_identifier(
                        ctx,
                        &specifier.local_name().map_err(|_| {
                            missing("JsShorthandNamedImportSpecifier", "local_name")
                        })?,
                    )?;
                    Ok(ImportSpecifier::ImportSpecifier(ImportSpecifierData {
                        base: ctx.base(specifier.syntax().text_trimmed_range()),
                        imported: ModuleExportName::Identifier(local.clone()),
                        local,
                        import_kind: specifier.type_token().map(|_| ImportKind::Type),
                    }))
                }
                biome_js_syntax::AnyJsNamedImportSpecifier::JsNamedImportSpecifier(specifier) => {
                    let local = convert_any_binding_identifier(
                        ctx,
                        &specifier
                            .local_name()
                            .map_err(|_| missing("JsNamedImportSpecifier", "local_name"))?,
                    )?;
                    Ok(ImportSpecifier::ImportSpecifier(ImportSpecifierData {
                        base: ctx.base(specifier.syntax().text_trimmed_range()),
                        imported: convert_literal_export_name(
                            ctx,
                            specifier
                                .name()
                                .map_err(|_| missing("JsNamedImportSpecifier", "name"))?,
                        )?,
                        local,
                        import_kind: specifier.type_token().map(|_| ImportKind::Type),
                    }))
                }
                specifier => Err(unsupported(specifier.syntax())),
            }
        })
        .collect()
}

pub(super) fn convert_literal_export_name(
    ctx: &ConvertCtx<'_>,
    name: AnyJsLiteralExportName,
) -> Result<ModuleExportName> {
    match name {
        AnyJsLiteralExportName::JsLiteralExportName(name) => {
            let token = name
                .value()
                .map_err(|_| missing("JsLiteralExportName", "value"))?;
            Ok(ModuleExportName::Identifier(Identifier {
                base: ctx.base(name.syntax().text_trimmed_range()),
                name: token.text_trimmed().to_string(),
                type_annotation: None,
                optional: None,
                decorators: None,
            }))
        }
        AnyJsLiteralExportName::JsMetavariable(name) => Err(unsupported(name.syntax())),
    }
}

pub(super) fn convert_module_source(
    ctx: &ConvertCtx<'_>,
    source: AnyJsModuleSource,
) -> Result<StringLiteral> {
    match source {
        AnyJsModuleSource::JsModuleSource(source) => {
            let token = source
                .value_token()
                .map_err(|_| missing("JsModuleSource", "value_token"))?;
            Ok(StringLiteral {
                base: ctx.base(source.syntax().text_trimmed_range()),
                value: inner_string_text(&token).to_string(),
            })
        }
        AnyJsModuleSource::JsMetavariable(source) => Err(unsupported(source.syntax())),
    }
}
