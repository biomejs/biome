use crate::generate_nodes::token_kind_to_code;
use crate::js_kinds_src::{AstSrc, Field, JS_KINDS_SRC, TokenKind};
use crate::language_kind::LanguageKind;
use crate::update;
use anyhow::{Context, ensure};
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_formatter::{context::JsFormatOptions, format_node};
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsDeclarationClause, AnyJsExportClause,
    AnyJsFormalParameter, AnyJsModuleItem, AnyJsObjectMemberName, AnyJsParameter, AnyTsName,
    AnyTsReturnType, AnyTsType, AnyTsTypeMember, JsSyntaxToken, T, TriviaPieceKind,
    TsReferenceType,
};
use biome_languages::JsFileSource;
use biome_rowan::AstNode;
use biome_string_case::Case;
use quote::{format_ident, quote};
use schemars::schema_for;
use std::collections::HashSet;
use xtask_glue::{Mode, Result, project_root};

pub(crate) fn generate_js_plugin_ast(ast: &AstSrc, mode: &Mode) -> Result<()> {
    let rust_path = project_root().join("crates/biome_js_runtime/src/generated/js_ast.rs");
    let rust = generate_rust(ast)?;
    update(&rust_path, &rust, mode)?;

    let types_path = project_root().join("packages/@biomejs/runtime/js_ast.d.ts");
    let types = generate_typescript(ast);
    update(&types_path, &types, mode)?;

    let diagnostics_path = project_root().join("packages/@biomejs/runtime/diagnostics.d.ts");
    let diagnostics = generate_diagnostics_typescript()?;
    update(&diagnostics_path, &diagnostics, mode)?;

    Ok(())
}

fn generate_rust(ast: &AstSrc) -> Result<String> {
    validate_bindings(ast)?;
    let mut kind_name_arms = Vec::new();
    let mut field_arms = Vec::new();
    let mut field_tables = Vec::new();

    for name in ast
        .nodes
        .iter()
        .map(|node| &node.name)
        .chain(&ast.bogus)
        .chain(ast.lists.keys())
    {
        let kind_name = Case::Constant.convert(name);
        let node_kind = format_ident!("{kind_name}");
        kind_name_arms.push(quote! { #kind_name => JsSyntaxKind::#node_kind });
    }

    // Every node kind shares the same native getter and update functions, defined in
    // `crate::ast`. The generated code only describes the fields of each node kind.
    for node in &ast.nodes {
        if node.fields.is_empty() {
            continue;
        }
        let node_type = node.name.as_str();
        let kind_name = Case::Constant.convert(&node.name);
        let node_kind = format_ident!("{kind_name}");
        let table_name = format_ident!("{kind_name}_FIELDS");

        let fields = node.fields.iter().map(|field| {
            let property_name = property_name(field);
            let updater_name = updater_name_for_field(field);
            let optional = field.is_optional();
            let value = match field {
                Field::Token { kind, .. } => {
                    let (kinds, expected) = match kind {
                        TokenKind::Single(kind) => (
                            vec![token_kind_to_code(kind, LanguageKind::Js)],
                            format!("{kind:?}"),
                        ),
                        TokenKind::Many(kinds) => (
                            kinds
                                .iter()
                                .map(|kind| token_kind_to_code(kind, LanguageKind::Js))
                                .collect(),
                            kinds
                                .iter()
                                .map(|kind| format!("{kind:?}"))
                                .collect::<Vec<_>>()
                                .join(", "),
                        ),
                    };
                    quote! {
                        JsAstFieldValue::Token {
                            kinds: &[#(#kinds),*],
                            expected: #expected,
                        }
                    }
                }
                Field::Node { ty, .. } => {
                    let variant = if ast.is_list(ty) {
                        quote! { List }
                    } else {
                        quote! { Node }
                    };
                    let ty_ident = format_ident!("{ty}");
                    quote! {
                        JsAstFieldValue::#variant {
                            ty: #ty,
                            can_cast: #ty_ident::can_cast,
                        }
                    }
                }
            };
            quote! {
                JsAstField {
                    property: #property_name,
                    updater: #updater_name,
                    optional: #optional,
                    value: #value,
                }
            }
        });

        field_tables.push(quote! {
            static #table_name: JsAstNodeFields = JsAstNodeFields {
                kind: JsSyntaxKind::#node_kind,
                name: #node_type,
                fields: &[#(#fields),*],
            };
        });
        field_arms.push(quote! {
            JsSyntaxKind::#node_kind => &#table_name
        });
    }

    let token_kind_arms = token_kind_names().into_iter().map(|name| {
        let kind = format_ident!("{name}");
        quote! { #name => JsSyntaxKind::#kind }
    });
    let tokens = quote! {
        use crate::ast::{JsAstField, JsAstFieldValue, JsAstNode, JsAstNodeFields};
        use biome_js_syntax::{*, JsSyntaxKind::*};
        use biome_rowan::AstNode;

        impl JsAstNode {
            /// Resolves a constructible token kind by its exact native enum name.
            pub(crate) fn token_kind_from_name(name: &str) -> Option<JsSyntaxKind> {
                Some(match name {
                    #(#token_kind_arms,)*
                    _ => return None,
                })
            }

            /// Resolves a syntax kind from the name used in the plugin API type definitions,
            /// e.g. `"JS_CALL_EXPRESSION"`.
            pub(crate) fn syntax_kind_from_ast_name(name: &str) -> Option<JsSyntaxKind> {
                Some(match name {
                    #(#kind_name_arms,)*
                    _ => return None,
                })
            }

            /// Returns the plugin API fields of `kind`, one per slot in slot order.
            /// Node kinds without fields, lists, and bogus nodes have no descriptor.
            pub(crate) fn node_fields(kind: JsSyntaxKind) -> Option<&'static JsAstNodeFields> {
                Some(match kind {
                    #(#field_arms,)*
                    _ => return None,
                })
            }
        }

        #(#field_tables)*
    };

    // Establish line breaks before rustfmt, which can leave oversized expressions unchanged.
    let file = syn::parse2::<syn::File>(tokens)?;
    Ok(xtask_glue::reformat(prettyplease::unparse(&file))?.replacen("//!", "//", 1))
}

fn generate_typescript(ast: &AstSrc) -> String {
    let parent = make::ts_property_signature_type_member(
        make::js_literal_member_name(make::ident("parent")).into(),
    )
    .with_readonly_token(make::token(T![readonly]).with_leading_trivia([
        (TriviaPieceKind::Newline, "\n"),
        (
            TriviaPieceKind::MultiLineComment,
            "/** The immediate parent node, including list containers. Undefined at the root.\n * Repeated access does not guarantee the same JavaScript object identity. */",
        ),
        (TriviaPieceKind::Newline, "\n"),
    ]))
    .with_type_annotation(make::ts_type_annotation(
        make::token(T![:]),
        union_type([reference_type("AnyJsAstNode").into(), undefined_type()]),
    ))
    .with_separator_token_token(make::token(T![;]))
    .build();
    let [ancestors, children] = [
        (
            "ancestors",
            "/** Returns a fresh array of enclosing nodes, nearest first, excluding this node\n * and including list containers and the root. Roots return an empty array.\n * Returned nodes do not have stable JavaScript object identity. */",
        ),
        (
            "children",
            "/** Returns a fresh array of immediate child nodes in source order, including list\n * containers and omitting tokens. Call children() on a list node to iterate its elements.\n * Nodes without child nodes return an empty array. Named list fields remain arrays.\n * Returned nodes do not have stable JavaScript object identity. */",
        ),
    ].map(|(name, documentation)| make::ts_method_signature_type_member(
        make::js_literal_member_name(make::ident(name).with_leading_trivia([
            (TriviaPieceKind::Newline, "\n"),
            (
                TriviaPieceKind::MultiLineComment,
                documentation,
            ),
            (TriviaPieceKind::Newline, "\n"),
        ]))
        .into(),
        make::js_parameters(
            make::token(T!['(']),
            make::js_parameter_list([], []),
            make::token(T![')']),
        ),
    )
    .with_return_type_annotation(make::ts_return_type_annotation(
        make::token(T![:]),
        AnyTsReturnType::AnyTsType(make::ts_type_operator_type(
            make::token(T![readonly]),
            make::ts_array_type(
                reference_type("AnyJsAstNode").into(),
                make::token(T!['[']),
                make::token(T![']']),
            )
            .into(),
        ).into()),
    ))
    .with_separator_token_token(make::token(T![;]))
    .build());

    let mut items = vec![export_interface(
        generated_export_token(),
        "JsAstNode",
        None,
        [
            property("kind", string_type()),
            property("text", string_type()),
            parent.into(),
            ancestors.into(),
            children.into(),
            method(
                "token",
                [("field", string_type())],
                union_type([reference_type("JsAstToken").into(), undefined_type()]),
                Some(
                    "/**\n * Returns the native token handle for a named token field.\n * Use the public field name, such as `kindToken` or `operatorToken`. Unlike the\n * string-valued field getter, this handle can be passed to token mutation methods.\n * Returns `undefined` when the field is recognized but its token is absent.\n * Repeated access does not guarantee the same JavaScript object identity.\n *\n * @throws {TypeError} If the field name is unknown, names a non-token field,\n * or the receiver or arguments are invalid.\n */",
                ),
            ),
            method(
                "childrenWithTokens",
                [],
                make::ts_type_operator_type(
                    make::token(T![readonly]),
                    make::ts_array_type(
                        reference_type("JsAstElement").into(),
                        make::token(T!['[']),
                        make::token(T![']']),
                    )
                    .into(),
                )
                .into(),
                Some(
                    "/**\n * Returns a fresh array of immediate child nodes and tokens in source order.\n * Includes list containers without flattening them or descending into child nodes.\n * Call this method on a list node to obtain its elements and separator tokens.\n * Whitespace and comments remain token trivia, not separate array entries. Missing\n * slots are omitted; nodes without child nodes or tokens return an empty array.\n * The returned handles can be passed to element mutation methods. Changing the\n * array does not change the syntax tree, and handles have no stable JavaScript identity.\n */",
                ),
            ),
        ],
    )];

    // Source tokens include kinds, such as EOF, that the token factory cannot construct.
    let constructible_kinds = token_kind_names();
    let source_token_kind = union_type(
        std::iter::once(reference_type("JsTokenKind").into()).chain(
            JS_KINDS_SRC
                .tokens
                .iter()
                .copied()
                .chain(["EOF", "UNICODE_BOM"])
                .filter(|name| {
                    *name != "GRIT_METAVARIABLE"
                        && !constructible_kinds.iter().any(|kind| kind == *name)
                })
                .map(string_literal_type),
        ),
    );
    items.push(export_interface(
        make::token(T![export]),
        "JsAstToken",
        None,
        [
            property("kind", source_token_kind),
            property("text", string_type()),
            property(
                "parent",
                union_type([reference_type("AnyJsAstNode").into(), undefined_type()]),
            ),
        ],
    ));
    items.push(export_type_alias(
        make::token(T![export]),
        "JsAstElement",
        union_type([
            reference_type("AnyJsAstNode").into(),
            reference_type("JsAstToken").into(),
        ]),
    ));
    items.push(export_type_alias(
        make::token(T![export]),
        "JsTokenKind",
        union_type(
            constructible_kinds
                .iter()
                .map(|name| string_literal_type(name)),
        ),
    ));

    items.push(export_type_alias(
        make::token(T![export]),
        "AnyJsAstNode",
        make::ts_indexed_access_type(
            reference_type("JsNodeByKind").into(),
            make::token(T!['[']),
            make::ts_type_operator_type(
                make::token(T![keyof]),
                reference_type("JsNodeByKind").into(),
            )
            .into(),
            make::token(T![']']),
        )
        .into(),
    ));

    for node in &ast.nodes {
        let node_kind = Case::Constant.convert(&node.name);
        let mut members = vec![property("kind", string_literal_type(&node_kind))];

        for field in &node.fields {
            let property_name = property_name(field);
            let field_type = match field {
                Field::Token { .. } => union_type([string_type(), undefined_type()]),
                Field::Node { ty, .. } if ast.is_list(ty) => reference_type(ty).into(),
                Field::Node { ty, .. } => union_type([reference_type(ty).into(), undefined_type()]),
            };
            members.push(property(&property_name, field_type));
            let replacement_type = match field {
                Field::Token { .. } => reference_type("JsAstToken").into(),
                Field::Node { ty, .. } if ast.is_list(ty) => {
                    reference_type(&format!("{ty}Node")).into()
                }
                Field::Node { ty, .. } => reference_type(ty).into(),
            };
            let replacement_type = if field.is_optional() {
                union_type([replacement_type, undefined_type()])
            } else {
                replacement_type
            };
            members.push(method(
                &updater_name_for_field(field),
                [("value", replacement_type)],
                reference_type(&node.name).into(),
                None,
            ));
        }

        let token_fields = node
            .fields
            .iter()
            .filter(|field| matches!(field, Field::Token { .. }))
            .map(|field| string_literal_type(&property_name(field)))
            .collect::<Vec<_>>();
        if !token_fields.is_empty() {
            for field_type in [union_type(token_fields), string_type()] {
                members.push(method(
                    "token",
                    [("field", field_type)],
                    union_type([reference_type("JsAstToken").into(), undefined_type()]),
                    None,
                ));
            }
        }

        items.push(export_interface(
            make::token(T![export]),
            &node.name,
            Some("JsAstNode"),
            members,
        ));
    }

    for bogus in &ast.bogus {
        let node_kind = Case::Constant.convert(bogus);
        items.push(export_interface(
            make::token(T![export]),
            bogus,
            Some("JsAstNode"),
            [property("kind", string_literal_type(&node_kind))],
        ));
    }

    for union in &ast.unions {
        items.push(export_type_alias(
            make::token(T![export]),
            &union.name,
            union_type(
                union
                    .variants
                    .iter()
                    .map(|variant| reference_type(variant).into()),
            ),
        ));
    }

    items.push(export_interface(
        make::token(T![export]),
        "JsNodeByKind",
        None,
        ast.nodes
            .iter()
            .map(|node| &node.name)
            .chain(&ast.bogus)
            .map(|name| property(&Case::Constant.convert(name), reference_type(name).into()))
            .chain(ast.lists.keys().map(|name| {
                property(
                    &Case::Constant.convert(name),
                    reference_type(&format!("{name}Node")).into(),
                )
            })),
    ));

    for (name, list) in ast.lists() {
        items.push(export_interface(
            make::token(T![export]),
            &format!("{name}Node"),
            Some("JsAstNode"),
            [property(
                "kind",
                string_literal_type(&Case::Constant.convert(name)),
            )],
        ));
        let array_type = make::ts_array_type(
            reference_type(&list.element_name).into(),
            make::token(T!['[']),
            make::token(T![']']),
        );
        let readonly_array_type = make::ts_type_operator_type(
            make::token(T![readonly]),
            AnyTsType::TsArrayType(array_type),
        );
        items.push(export_type_alias(
            make::token(T![export]),
            name,
            readonly_array_type.into(),
        ));
    }

    print_module(items)
}

/// Generates the plugin API types mirroring [biome_diagnostics], so that
/// plugin authors get the same set of severities the runtime accepts.
fn generate_diagnostics_typescript() -> Result<String> {
    let schema = schema_for!(Severity);
    // Every variant is documented, so `schemars` describes them one by one
    // instead of emitting a single `enum` array.
    let variants = schema
        .get("oneOf")
        .and_then(|variants| variants.as_array())
        .context("expected the schema of `Severity` to be a `oneOf`")?
        .iter()
        .map(|variant| {
            variant
                .get("const")
                .and_then(|value| value.as_str())
                .map(string_literal_type)
                .context("expected every variant of `Severity` to be a constant")
        })
        .collect::<Result<Vec<_>>>()?;

    let items = vec![export_type_alias(
        generated_export_token(),
        "Severity",
        union_type(variants),
    )];

    Ok(print_module(items))
}

/// Returns the `export` token opening a generated file, carrying the comment
/// warning readers the file is generated.
fn generated_export_token() -> JsSyntaxToken {
    make::token(T![export]).with_leading_trivia([
        (
            TriviaPieceKind::SingleLineComment,
            "// Generated file, do not edit by hand, see `xtask/codegen`.",
        ),
        (TriviaPieceKind::Newline, "\n"),
        (TriviaPieceKind::Newline, "\n"),
    ])
}

fn print_module(items: Vec<AnyJsModuleItem>) -> String {
    let module = make::js_module(
        make::js_directive_list(None),
        make::js_module_item_list(items),
        make::eof(),
    )
    .build();

    let formatted = format_node(
        JsFormatOptions::new(JsFileSource::ts()),
        module.syntax(),
        vec![],
    )
    .unwrap();

    formatted.print().unwrap().into_code()
}

fn export_interface(
    export_token: JsSyntaxToken,
    name: &str,
    extends: Option<&str>,
    members: impl IntoIterator<Item = AnyTsTypeMember>,
) -> AnyJsModuleItem {
    let members = members.into_iter().collect::<Vec<_>>();
    let mut interface = make::ts_interface_declaration(
        make::token(T![interface]),
        make::ts_identifier_binding(make::ident(name)).into(),
        make::token(T!['{']),
        make::ts_type_member_list(members),
        make::token(T!['}']),
    );
    if let Some(extends) = extends {
        interface = interface.with_extends_clause(make::ts_extends_clause(
            make::token(T![extends]),
            make::ts_type_list([reference_type(extends)], []),
        ));
    }

    AnyJsModuleItem::JsExport(make::js_export(
        make::js_decorator_list([]),
        export_token,
        AnyJsExportClause::AnyJsDeclarationClause(AnyJsDeclarationClause::TsInterfaceDeclaration(
            interface.build(),
        )),
    ))
}

fn export_type_alias(export_token: JsSyntaxToken, name: &str, ty: AnyTsType) -> AnyJsModuleItem {
    AnyJsModuleItem::JsExport(make::js_export(
        make::js_decorator_list([]),
        export_token,
        AnyJsExportClause::AnyJsDeclarationClause(AnyJsDeclarationClause::TsTypeAliasDeclaration(
            make::ts_type_alias_declaration(
                make::token(T![type]),
                make::ts_identifier_binding(make::ident(name)).into(),
                make::token(T![=]),
                ty,
            )
            .with_semicolon_token(make::token(T![;]))
            .build(),
        )),
    ))
}

fn property(name: &str, ty: AnyTsType) -> AnyTsTypeMember {
    make::ts_property_signature_type_member(AnyJsObjectMemberName::JsLiteralMemberName(
        make::js_literal_member_name(make::ident(name)),
    ))
    .with_readonly_token(make::token(T![readonly]))
    .with_type_annotation(make::ts_type_annotation(make::token(T![:]), ty))
    .with_separator_token_token(make::token(T![;]))
    .build()
    .into()
}

fn method(
    name: &str,
    parameters: impl IntoIterator<Item = (&'static str, AnyTsType)>,
    return_type: AnyTsType,
    documentation: Option<&str>,
) -> AnyTsTypeMember {
    let mut name = make::ident(name);
    if let Some(documentation) = documentation {
        name = name.with_leading_trivia([
            (TriviaPieceKind::Newline, "\n"),
            (TriviaPieceKind::MultiLineComment, documentation),
            (TriviaPieceKind::Newline, "\n"),
        ]);
    }
    let parameters = parameters
        .into_iter()
        .map(|(name, ty)| {
            AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
                make::js_formal_parameter(
                    make::js_decorator_list([]),
                    AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
                        make::js_identifier_binding(make::ident(name)),
                    )),
                )
                .with_type_annotation(make::ts_type_annotation(make::token(T![:]), ty))
                .build(),
            ))
        })
        .collect::<Vec<_>>();
    let separators = (1..parameters.len()).map(|_| make::token(T![,]));
    make::ts_method_signature_type_member(
        make::js_literal_member_name(name).into(),
        make::js_parameters(
            make::token(T!['(']),
            make::js_parameter_list(parameters, separators),
            make::token(T![')']),
        ),
    )
    .with_return_type_annotation(make::ts_return_type_annotation(
        make::token(T![:]),
        AnyTsReturnType::AnyTsType(return_type),
    ))
    .with_separator_token_token(make::token(T![;]))
    .build()
    .into()
}

fn string_type() -> AnyTsType {
    make::ts_string_type(make::token(T![string])).into()
}

fn string_literal_type(value: &str) -> AnyTsType {
    make::ts_string_literal_type(make::js_string_literal(value)).into()
}

fn undefined_type() -> AnyTsType {
    make::ts_undefined_type(make::token(T![undefined])).into()
}

fn reference_type(name: &str) -> TsReferenceType {
    make::ts_reference_type(AnyTsName::JsReferenceIdentifier(
        make::js_reference_identifier(make::ident(name)),
    ))
    .build()
}

fn union_type(types: impl IntoIterator<Item = AnyTsType>) -> AnyTsType {
    let types = types.into_iter().collect::<Vec<_>>();
    let separators = (1..types.len()).map(|_| make::token(T![|]));

    make::ts_union_type(make::ts_union_type_variant_list(types, separators))
        .build()
        .into()
}

fn rust_method_name(field: &Field) -> proc_macro2::Ident {
    match field {
        Field::Token {
            name,
            kind: TokenKind::Many(_),
            ..
        } => format_ident!("{name}"),
        _ => field.method_name(LanguageKind::Js),
    }
}

fn property_name(field: &Field) -> String {
    let method_name = rust_method_name(field);
    let name = Case::Camel.convert(&method_name.to_string());

    if BASE_MEMBERS.contains(&name.as_str()) {
        match field {
            Field::Token { .. } => format!("{name}Token"),
            Field::Node { .. } => format!("{name}Node"),
        }
    } else {
        name
    }
}

const BASE_MEMBERS: &[&str] = &[
    "kind",
    "text",
    "parent",
    "ancestors",
    "children",
    "token",
    "childrenWithTokens",
];

fn updater_name_for_field(field: &Field) -> String {
    format!("with{}", Case::Pascal.convert(&property_name(field)))
}

fn validate_bindings(ast: &AstSrc) -> Result<()> {
    for node in &ast.nodes {
        ensure!(
            !node.dynamic,
            "JS plugin AST bindings do not support dynamic slots: {}",
            node.name
        );
        let mut names = BASE_MEMBERS
            .iter()
            .map(|name| (*name).to_owned())
            .collect::<HashSet<_>>();
        for field in &node.fields {
            for name in [property_name(field), updater_name_for_field(field)] {
                ensure!(
                    names.insert(name.clone()),
                    "Duplicate JS plugin AST member {}.{name}",
                    node.name
                );
            }
        }
    }
    Ok(())
}

fn token_kind_names() -> Vec<String> {
    JS_KINDS_SRC
        .punct
        .iter()
        .map(|(_, name)| (*name).to_owned())
        .chain(
            JS_KINDS_SRC
                .keywords
                .iter()
                // import.meta uses the META token, not META_KW.
                .filter(|keyword| **keyword != "meta")
                .map(|keyword| format!("{}_KW", Case::Constant.convert(keyword))),
        )
        .chain(JS_KINDS_SRC.literals.iter().map(|name| (*name).to_owned()))
        .chain(
            JS_KINDS_SRC
                .tokens
                .iter()
                .copied()
                // Whitespace and comments are attached to tokens as trivia. ERROR_TOKEN
                // represents a lexer error, and GRIT_METAVARIABLE belongs to Grit patterns.
                // None of these are replacement tokens for JavaScript plugin fixes.
                .filter(|name| {
                    !matches!(
                        *name,
                        "ERROR_TOKEN"
                            | "NEWLINE"
                            | "WHITESPACE"
                            | "COMMENT"
                            | "MULTILINE_COMMENT"
                            | "GRIT_METAVARIABLE"
                    )
                })
                .map(str::to_owned),
        )
        .collect()
}
