use biome_js_factory::make;
use biome_js_formatter::{context::JsFormatOptions, format_node};
use biome_js_syntax::{
    AnyJsDeclarationClause, AnyJsExportClause, AnyJsModuleItem, AnyJsObjectMemberName, AnyTsName,
    AnyTsType, AnyTsTypeMember, JsSyntaxToken, T, TriviaPieceKind, TsReferenceType,
};
use biome_languages::JsFileSource;
use biome_rowan::AstNode;
use biome_string_case::Case;
use quote::{format_ident, quote};
use xtask_glue::{Mode, Result, project_root};

use crate::js_kinds_src::{AstSrc, Field, TokenKind};
use crate::language_kind::LanguageKind;
use crate::update;

pub(crate) fn generate_js_plugin_ast(ast: &AstSrc, mode: &Mode) -> Result<()> {
    let rust_path = project_root().join("crates/biome_js_runtime/src/generated/js_ast.rs");
    let rust = generate_rust(ast)?;
    update(&rust_path, &rust, mode)?;

    let types_path = project_root().join("packages/@biomejs/plugin-api/js_ast.d.ts");
    let types = generate_typescript(ast);
    update(&types_path, &types, mode)?;

    Ok(())
}

fn generate_rust(ast: &AstSrc) -> Result<String> {
    let mut prototype_arms = Vec::new();

    for node in &ast.nodes {
        let node_type = format_ident!("{}", node.name);
        let node_kind = format_ident!("{}", Case::Constant.convert(&node.name));
        let mut prototype_fields = Vec::new();

        for field in &node.fields {
            let method_name = rust_method_name(field);
            let property_name = property_name(field);
            let accessor_value = match field {
                Field::Token { optional, .. } => {
                    let value = if *optional {
                        quote! { node.#method_name() }
                    } else {
                        quote! { node.#method_name().ok() }
                    };

                    quote! { Self::wrap_token(#value) }
                }
                Field::Node {
                    ty, optional: _, ..
                } if ast.is_list(ty) => {
                    let list = if ast
                        .lists
                        .get(ty)
                        .is_some_and(|list| list.separator.is_some())
                    {
                        quote! { node.#method_name().into_iter().flatten() }
                    } else {
                        quote! { node.#method_name() }
                    };

                    quote! { Self::wrap_node_list(#list, context) }
                }
                Field::Node { optional, .. } => {
                    let value = if *optional {
                        quote! { node.#method_name() }
                    } else {
                        quote! { node.#method_name().ok() }
                    };

                    quote! { Self::wrap_optional_node(#value, context) }
                }
            };

            prototype_fields.push(quote! {
                (#property_name, |node, context| #accessor_value)
            });
        }

        prototype_arms.push(quote! {
            JsSyntaxKind::#node_kind => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::#node_kind,
                    #node_type,
                    #(#prototype_fields,)*
                );
            }
        });
    }

    let tokens = quote! {
        use super::*;
        use biome_js_syntax::*;

        impl JsAstNode {
            pub(super) fn create_generated_prototype(
                kind: JsSyntaxKind,
                base_prototype: JsObject,
                context: &mut Context,
            ) -> JsObject {
                let mut prototype = ObjectInitializer::with_native_data_and_proto(
                    OrdinaryObject,
                    base_prototype,
                    context,
                );
                match kind {
                    #(#prototype_arms,)*
                    _ => {}
                }
                prototype.build()
            }
        }
    };

    Ok(xtask_glue::reformat(tokens)?.replacen("//!", "//", 1))
}

fn generate_typescript(ast: &AstSrc) -> String {
    let leading_comment = [
        (
            TriviaPieceKind::SingleLineComment,
            "// Generated file, do not edit by hand, see `xtask/codegen`.",
        ),
        (TriviaPieceKind::Newline, "\n"),
        (TriviaPieceKind::Newline, "\n"),
    ];

    let export_token = make::token(T![export]).with_leading_trivia(leading_comment);
    let mut items = vec![export_interface(
        export_token,
        "JsAstNode",
        None,
        [
            property("kind", string_type()),
            property("text", string_type()),
        ],
    )];

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
            &union.name,
            union_type(
                union
                    .variants
                    .iter()
                    .map(|variant| reference_type(variant).into()),
            ),
        ));
    }

    for (name, list) in ast.lists() {
        let array_type = make::ts_array_type(
            reference_type(&list.element_name).into(),
            make::token(T!['[']),
            make::token(T![']']),
        );
        let readonly_array_type = make::ts_type_operator_type(
            make::token(T![readonly]),
            AnyTsType::TsArrayType(array_type),
        );
        items.push(export_type_alias(name, readonly_array_type.into()));
    }

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

fn export_type_alias(name: &str, ty: AnyTsType) -> AnyJsModuleItem {
    AnyJsModuleItem::JsExport(make::js_export(
        make::js_decorator_list([]),
        make::token(T![export]),
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

    match (name.as_str(), field) {
        ("kind" | "text", Field::Token { .. }) => format!("{name}Token"),
        ("kind" | "text", Field::Node { .. }) => format!("{name}Node"),
        _ => name,
    }
}
