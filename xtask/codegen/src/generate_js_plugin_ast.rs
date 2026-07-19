use std::fmt::Write;

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
    let mut output = String::from(
        "// Generated file, do not edit by hand, see `xtask/codegen`.\n\n\
	export interface JsAstNode {\n\
	\treadonly kind: string;\n\
	\treadonly text: string;\n\
	}\n\n",
    );

    for node in &ast.nodes {
        let node_kind = Case::Constant.convert(&node.name);
        writeln!(
            output,
            "export interface {} extends JsAstNode {{",
            node.name
        )
        .unwrap();
        writeln!(output, "\treadonly kind: \"{node_kind}\";").unwrap();

        for field in &node.fields {
            let property_name = property_name(field);
            let field_type = match field {
                Field::Token { .. } => "string | undefined".to_string(),
                Field::Node { ty, .. } if ast.is_list(ty) => ty.clone(),
                Field::Node { ty, .. } => format!("{ty} | undefined"),
            };
            writeln!(output, "\treadonly {property_name}: {field_type};").unwrap();
        }

        output.push_str("}\n\n");
    }

    for bogus in &ast.bogus {
        let node_kind = Case::Constant.convert(bogus);
        writeln!(output, "export interface {bogus} extends JsAstNode {{").unwrap();
        writeln!(output, "\treadonly kind: \"{node_kind}\";").unwrap();
        output.push_str("}\n\n");
    }

    for union in &ast.unions {
        writeln!(
            output,
            "export type {} = {};",
            union.name,
            union.variants.join(" | ")
        )
        .unwrap();
    }

    output.push('\n');

    for (name, list) in ast.lists() {
        writeln!(
            output,
            "export type {name} = readonly {}[];",
            list.element_name
        )
        .unwrap();
    }

    output
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
