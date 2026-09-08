use anyhow::Context;
use biome_diagnostics::Severity;
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
use schemars::schema_for;
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

    let diagnostics_path = project_root().join("packages/@biomejs/plugin-api/diagnostics.d.ts");
    let diagnostics = generate_diagnostics_typescript()?;
    update(&diagnostics_path, &diagnostics, mode)?;

    Ok(())
}

fn generate_rust(ast: &AstSrc) -> Result<String> {
    let mut prototype_arms = Vec::new();
    let mut kind_name_arms = Vec::new();

    for name in ast.nodes.iter().map(|node| &node.name).chain(&ast.bogus) {
        let kind_name = Case::Constant.convert(name);
        let node_kind = format_ident!("{kind_name}");
        kind_name_arms.push(quote! { #kind_name => JsSyntaxKind::#node_kind });
    }

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
                Field::Node { ty, .. } if ast.is_list(ty) => {
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

            /// Resolves a syntax kind from the name used in the plugin API type definitions,
            /// e.g. `"JS_CALL_EXPRESSION"`.
            pub(crate) fn syntax_kind_from_ast_name(name: &str) -> Option<JsSyntaxKind> {
                Some(match name {
                    #(#kind_name_arms,)*
                    _ => return None,
                })
            }
        }
    };

    Ok(xtask_glue::reformat(tokens)?.replacen("//!", "//", 1))
}

fn generate_typescript(ast: &AstSrc) -> String {
    let mut items = vec![export_interface(
        generated_export_token(),
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
            .map(|name| property(&Case::Constant.convert(name), reference_type(name).into())),
    ));

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
