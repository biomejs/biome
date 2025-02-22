use biome_js_factory::make;
use biome_js_formatter::{context::JsFormatOptions, format_node};
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument, AnyJsDeclaration, AnyJsDeclarationClause,
    AnyJsExportClause, AnyJsExpression, AnyJsFormalParameter, AnyJsImportClause,
    AnyJsLiteralExpression, AnyJsModuleItem, AnyJsName, AnyJsNamedImportSpecifier,
    AnyJsObjectMember, AnyJsObjectMemberName, AnyJsParameter, AnyJsStatement, AnyTsName,
    AnyTsReturnType, AnyTsType, AnyTsTypeMember, JsFileSource, TriviaPieceKind, T,
};
use biome_rowan::AstNode;
use biome_service::workspace_types::{generate_type, methods, ModuleQueue};
use biome_string_case::Case;
use schemars::gen::{SchemaGenerator, SchemaSettings};
use xtask::{project_root, Mode, Result};
use xtask_codegen::update;

pub(crate) fn generate_workspace_bindings(mode: Mode) -> Result<()> {
    let bindings_path = project_root().join("packages/@biomejs/backend-jsonrpc/src/workspace.ts");
    let methods = methods();

    let mut declarations = Vec::new();
    let mut member_definitions = Vec::with_capacity(methods.len());
    let mut member_declarations = Vec::with_capacity(methods.len());
    let mut queue = ModuleQueue::default();

    for method in &methods {
        let params = generate_type(&mut declarations, &mut queue, &method.params);
        let result = generate_type(&mut declarations, &mut queue, &method.result);

        let camel_case = Case::Camel.convert(method.name);

        member_definitions.push(AnyTsTypeMember::TsMethodSignatureTypeMember(
            make::ts_method_signature_type_member(
                AnyJsObjectMemberName::JsLiteralMemberName(make::js_literal_member_name(
                    make::ident(&camel_case),
                )),
                make::js_parameters(
                    make::token(T!['(']),
                    make::js_parameter_list(
                        Some(AnyJsParameter::AnyJsFormalParameter(
                            AnyJsFormalParameter::JsFormalParameter(
                                make::js_formal_parameter(
                                    make::js_decorator_list([]),
                                    AnyJsBindingPattern::AnyJsBinding(
                                        AnyJsBinding::JsIdentifierBinding(
                                            make::js_identifier_binding(make::ident("params")),
                                        ),
                                    ),
                                )
                                .with_type_annotation(make::ts_type_annotation(
                                    make::token(T![:]),
                                    params,
                                ))
                                .build(),
                            ),
                        )),
                        None,
                    ),
                    make::token(T![')']),
                ),
            )
            .with_return_type_annotation(make::ts_return_type_annotation(
                make::token(T![:]),
                AnyTsReturnType::AnyTsType(AnyTsType::TsReferenceType(
                    make::ts_reference_type(AnyTsName::JsReferenceIdentifier(
                        make::js_reference_identifier(make::ident("Promise")),
                    ))
                    .with_type_arguments(make::ts_type_arguments(
                        make::token(T![<]),
                        make::ts_type_argument_list(Some(result), None),
                        make::token(T![>]),
                    ))
                    .build(),
                )),
            ))
            .build(),
        ));

        member_declarations.push(AnyJsObjectMember::JsMethodObjectMember(
            make::js_method_object_member(
                AnyJsObjectMemberName::JsLiteralMemberName(make::js_literal_member_name(
                    make::ident(&camel_case),
                )),
                make::js_parameters(
                    make::token(T!['(']),
                    make::js_parameter_list(
                        Some(AnyJsParameter::AnyJsFormalParameter(
                            AnyJsFormalParameter::JsFormalParameter(
                                make::js_formal_parameter(make::js_decorator_list([]),AnyJsBindingPattern::AnyJsBinding(
                                    AnyJsBinding::JsIdentifierBinding(make::js_identifier_binding(
                                        make::ident("params"),
                                    )),
                                ))
                                .build(),
                            ),
                        )),
                        None,
                    ),
                    make::token(T![')']),
                ),
                make::js_function_body(
                    make::token(T!['{']),
                    make::js_directive_list(None),
                    make::js_statement_list(Some(AnyJsStatement::JsReturnStatement(
                        make::js_return_statement(make::token(T![return]))
                            .with_argument(AnyJsExpression::JsCallExpression(
                                make::js_call_expression(
                                    AnyJsExpression::JsStaticMemberExpression(
                                        make::js_static_member_expression(
                                            AnyJsExpression::JsIdentifierExpression(
                                                make::js_identifier_expression(
                                                    make::js_reference_identifier(make::ident(
                                                        "transport",
                                                    )),
                                                ),
                                            ),
                                            make::token(T![.]),
                                            AnyJsName::JsName(make::js_name(make::ident(
                                                "request",
                                            ))),
                                        ),
                                    ),
                                    make::js_call_arguments(
                                        make::token(T!['(']),
                                        make::js_call_argument_list(
                                            [
                                                AnyJsCallArgument::AnyJsExpression(
                                                    AnyJsExpression::AnyJsLiteralExpression(
                                                        AnyJsLiteralExpression::JsStringLiteralExpression(make::js_string_literal_expression(make::js_string_literal(&format!("biome/{}", method.name)))),
                                                    ),
                                                ),
                                                AnyJsCallArgument::AnyJsExpression(
                                                    AnyJsExpression::JsIdentifierExpression(
                                                        make::js_identifier_expression(
                                                            make::js_reference_identifier(make::ident(
                                                                "params",
                                                            )),
                                                        ),
                                                    ),
                                                ),
                                            ],
                                            Some(make::token(T![,])),
                                        ),
                                        make::token(T![')']),
                                    ),
                                )
                                .build(),
                            ))
                            .build(),
                    ))),
                    make::token(T!['}']),
                ),
            )
            .build(),
        ));
    }
    // HACK: these types doesn't get picked up in the loop above, so we add it manually
    let support_kind_schema = SchemaGenerator::from(SchemaSettings::openapi3())
        .root_schema_for::<biome_service::workspace::SupportKind>();
    generate_type(&mut declarations, &mut queue, &support_kind_schema);
    let rule_domain_schema = SchemaGenerator::from(SchemaSettings::openapi3())
        .root_schema_for::<biome_analyze::RuleDomain>();
    generate_type(&mut declarations, &mut queue, &rule_domain_schema);
    let rule_domain_value_schema = SchemaGenerator::from(SchemaSettings::openapi3())
        .root_schema_for::<biome_configuration::analyzer::RuleDomainValue>(
    );
    generate_type(&mut declarations, &mut queue, &rule_domain_value_schema);

    let leading_comment = [
        (
            TriviaPieceKind::SingleLineComment,
            "// Generated file, do not edit by hand, see `xtask/codegen`",
        ),
        (TriviaPieceKind::Newline, "\n"),
    ];

    let mut items = vec![AnyJsModuleItem::JsImport(
        make::js_import(
            make::token(T![import]).with_leading_trivia(leading_comment.into_iter()),
            AnyJsImportClause::JsImportNamedClause(
                make::js_import_named_clause(
                    make::js_named_import_specifiers(
                        make::token(T!['{']),
                        make::js_named_import_specifier_list(
                            Some(AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(
                                make::js_shorthand_named_import_specifier(
                                    AnyJsBinding::JsIdentifierBinding(make::js_identifier_binding(
                                        make::ident("Transport"),
                                    )),
                                )
                                .build(),
                            )),
                            None,
                        ),
                        make::token(T!['}']),
                    ),
                    make::token(T![from]),
                    make::js_module_source(make::js_string_literal("./transport")).into(),
                )
                .with_type_token(make::token(T![type]))
                .build(),
            ),
        )
        .build(),
    )];

    items.extend(declarations.into_iter().map(|(decl, description)| {
        let mut export = make::token(T![export]);
        if let Some(description) = description {
            let comment = format!("/**\n\t* {} \n\t */\n", description);
            let trivia = vec![
                (TriviaPieceKind::Newline, "\n"),
                (TriviaPieceKind::MultiLineComment, comment.as_str()),
                (TriviaPieceKind::Newline, "\n"),
            ];
            export = export.with_leading_trivia(trivia);
        }
        AnyJsModuleItem::JsExport(make::js_export(
            make::js_decorator_list([]),
            export,
            AnyJsExportClause::AnyJsDeclarationClause(match decl {
                AnyJsDeclaration::JsClassDeclaration(decl) => {
                    AnyJsDeclarationClause::JsClassDeclaration(decl)
                }
                AnyJsDeclaration::JsFunctionDeclaration(decl) => {
                    AnyJsDeclarationClause::JsFunctionDeclaration(decl)
                }
                AnyJsDeclaration::JsVariableDeclaration(decl) => {
                    AnyJsDeclarationClause::JsVariableDeclarationClause(
                        make::js_variable_declaration_clause(decl).build(),
                    )
                }
                AnyJsDeclaration::TsDeclareFunctionDeclaration(decl) => {
                    AnyJsDeclarationClause::TsDeclareFunctionDeclaration(decl)
                }
                AnyJsDeclaration::TsEnumDeclaration(decl) => {
                    AnyJsDeclarationClause::TsEnumDeclaration(decl)
                }
                AnyJsDeclaration::TsExternalModuleDeclaration(decl) => {
                    AnyJsDeclarationClause::TsExternalModuleDeclaration(decl)
                }
                AnyJsDeclaration::TsGlobalDeclaration(decl) => {
                    AnyJsDeclarationClause::TsGlobalDeclaration(decl)
                }
                AnyJsDeclaration::TsImportEqualsDeclaration(decl) => {
                    AnyJsDeclarationClause::TsImportEqualsDeclaration(decl)
                }
                AnyJsDeclaration::TsInterfaceDeclaration(decl) => {
                    AnyJsDeclarationClause::TsInterfaceDeclaration(decl)
                }
                AnyJsDeclaration::TsModuleDeclaration(decl) => {
                    AnyJsDeclarationClause::TsModuleDeclaration(decl)
                }
                AnyJsDeclaration::TsTypeAliasDeclaration(decl) => {
                    AnyJsDeclarationClause::TsTypeAliasDeclaration(decl)
                }
            }),
        ))
    }));

    member_definitions.push(AnyTsTypeMember::TsMethodSignatureTypeMember(
        make::ts_method_signature_type_member(
            AnyJsObjectMemberName::JsLiteralMemberName(make::js_literal_member_name(make::ident(
                "destroy",
            ))),
            make::js_parameters(
                make::token(T!['(']),
                make::js_parameter_list(None, None),
                make::token(T![')']),
            ),
        )
        .with_return_type_annotation(make::ts_return_type_annotation(
            make::token(T![:]),
            AnyTsReturnType::AnyTsType(AnyTsType::TsVoidType(make::ts_void_type(make::token(T![
                void
            ])))),
        ))
        .build(),
    ));

    member_declarations.push(AnyJsObjectMember::JsMethodObjectMember(
        make::js_method_object_member(
            AnyJsObjectMemberName::JsLiteralMemberName(make::js_literal_member_name(make::ident(
                "destroy",
            ))),
            make::js_parameters(
                make::token(T!['(']),
                make::js_parameter_list(None, None),
                make::token(T![')']),
            ),
            make::js_function_body(
                make::token(T!['{']),
                make::js_directive_list(None),
                make::js_statement_list(Some(AnyJsStatement::JsExpressionStatement(
                    make::js_expression_statement(AnyJsExpression::JsCallExpression(
                        make::js_call_expression(
                            AnyJsExpression::JsStaticMemberExpression(
                                make::js_static_member_expression(
                                    AnyJsExpression::JsIdentifierExpression(
                                        make::js_identifier_expression(
                                            make::js_reference_identifier(make::ident("transport")),
                                        ),
                                    ),
                                    make::token(T![.]),
                                    AnyJsName::JsName(make::js_name(make::ident("destroy"))),
                                ),
                            ),
                            make::js_call_arguments(
                                make::token(T!['(']),
                                make::js_call_argument_list(None, None),
                                make::token(T![')']),
                            ),
                        )
                        .build(),
                    ))
                    .build(),
                ))),
                make::token(T!['}']),
            ),
        )
        .build(),
    ));

    items.push(AnyJsModuleItem::JsExport(make::js_export(
        make::js_decorator_list([]),
        make::token(T![export]),
        AnyJsExportClause::AnyJsDeclarationClause(AnyJsDeclarationClause::TsInterfaceDeclaration(
            make::ts_interface_declaration(
                make::token(T![interface]),
                make::ts_identifier_binding(make::ident("Workspace")).into(),
                make::token(T!['{']),
                make::ts_type_member_list(member_definitions),
                make::token(T!['}']),
            )
            .build(),
        )),
    )));

    let member_separators = (0..member_declarations.len()).map(|_| make::token(T![,]));

    items.push(AnyJsModuleItem::JsExport(make::js_export(
        make::js_decorator_list([]),
        make::token(T![export]),
        AnyJsExportClause::AnyJsDeclarationClause(AnyJsDeclarationClause::JsFunctionDeclaration(
            make::js_function_declaration(
                make::token(T![function]),
                AnyJsBinding::JsIdentifierBinding(make::js_identifier_binding(make::ident(
                    "createWorkspace",
                ))),
                make::js_parameters(
                    make::token(T!['(']),
                    make::js_parameter_list(
                        Some(AnyJsParameter::AnyJsFormalParameter(
                            AnyJsFormalParameter::JsFormalParameter(
                                make::js_formal_parameter(
                                    make::js_decorator_list([]),
                                    AnyJsBindingPattern::AnyJsBinding(
                                        AnyJsBinding::JsIdentifierBinding(
                                            make::js_identifier_binding(make::ident("transport")),
                                        ),
                                    ),
                                )
                                .with_type_annotation(make::ts_type_annotation(
                                    make::token(T![:]),
                                    AnyTsType::TsReferenceType(
                                        make::ts_reference_type(AnyTsName::JsReferenceIdentifier(
                                            make::js_reference_identifier(make::ident("Transport")),
                                        ))
                                        .build(),
                                    ),
                                ))
                                .build(),
                            ),
                        )),
                        None,
                    ),
                    make::token(T![')']),
                ),
                make::js_function_body(
                    make::token(T!['{']),
                    make::js_directive_list(None),
                    make::js_statement_list(Some(AnyJsStatement::JsReturnStatement(
                        make::js_return_statement(make::token(T![return]))
                            .with_argument(AnyJsExpression::JsObjectExpression(
                                make::js_object_expression(
                                    make::token(T!['{']),
                                    make::js_object_member_list(
                                        member_declarations,
                                        member_separators,
                                    ),
                                    make::token(T!['}']),
                                ),
                            ))
                            .build(),
                    ))),
                    make::token(T!['}']),
                ),
            )
            .with_return_type_annotation(make::ts_return_type_annotation(
                make::token(T![:]),
                AnyTsReturnType::AnyTsType(AnyTsType::TsReferenceType(
                    make::ts_reference_type(AnyTsName::JsReferenceIdentifier(
                        make::js_reference_identifier(make::ident("Workspace")),
                    ))
                    .build(),
                )),
            ))
            .build(),
        )),
    )));

    let module = make::js_module(
        make::js_directive_list(None),
        make::js_module_item_list(items),
        make::eof(),
    )
    .build();

    let formatted = format_node(JsFormatOptions::new(JsFileSource::ts()), module.syntax()).unwrap();
    let printed = formatted.print().unwrap();
    let code = printed.into_code();

    update(&bindings_path, &code, &mode)?;

    Ok(())
}
