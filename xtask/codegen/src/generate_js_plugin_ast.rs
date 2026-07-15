use std::collections::BTreeSet;
use std::fmt::Write;

use biome_string_case::Case;
use quote::{format_ident, quote};
use xtask_glue::{Mode, Result, project_root};

use crate::js_kinds_src::{AstSrc, Field, TokenKind};
use crate::language_kind::LanguageKind;
use crate::update;

pub(crate) fn generate_js_plugin_ast(ast: &AstSrc, mode: &Mode) -> Result<()> {
    let rust_path = project_root().join("crates/biome_plugin_loader/src/generated/js_ast.rs");
    let rust = generate_rust(ast)?;
    update(&rust_path, &rust, mode)?;

    let types_path = project_root().join("packages/@biomejs/plugin-api/js_ast.d.ts");
    let types = generate_typescript(ast);
    update(&types_path, &types, mode)?;

    Ok(())
}

fn generate_rust(ast: &AstSrc) -> Result<String> {
    let mut accessors = BTreeSet::<String>::new();
    let mut variants = Vec::new();
    let mut cast_arms = Vec::new();
    let mut syntax_arms = Vec::new();
    let mut field_arms = Vec::new();

    for node in &ast.nodes {
        let node_type = format_ident!("{}", node.name);
        let node_kind = format_ident!("{}", Case::Constant.convert(&node.name));
        variants.push(quote! { #node_type(biome_js_syntax::#node_type) });
        cast_arms.push(quote! {
            biome_js_syntax::JsSyntaxKind::#node_kind =>
                <biome_js_syntax::#node_type as AstNode>::cast(node).map(Self::#node_type)
        });
        syntax_arms.push(quote! { Self::#node_type(node) => node.syntax() });

        let mut node_fields = Vec::new();

        for field in &node.fields {
            let method_name = rust_method_name(field);
            let property_name = property_name(field);
            let field_arm = match field {
                Field::Token { optional, .. } => {
                    let value = if *optional {
                        quote! { node.#method_name() }
                    } else {
                        quote! { node.#method_name().ok() }
                    };

                    quote! {
                        #property_name => Ok(JsAstNode::wrap_token(#value))
                    }
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

                    quote! {
                        #property_name => JsAstNode::wrap_node_list(#list, context)
                    }
                }
                Field::Node { optional, .. } => {
                    let value = if *optional {
                        quote! { node.#method_name() }
                    } else {
                        quote! { node.#method_name().ok() }
                    };

                    quote! {
                        #property_name => JsAstNode::wrap_optional_node(#value, context)
                    }
                }
            };

            accessors.insert(property_name);
            node_fields.push(field_arm);
        }

        if node_fields.is_empty() {
            field_arms.push(quote! { Self::#node_type(_) => Ok(JsValue::undefined()) });
        } else {
            field_arms.push(quote! {
                Self::#node_type(node) => match field {
                    #(#node_fields,)*
                    _ => Ok(JsValue::undefined()),
                }
            });
        }
    }

    for bogus in &ast.bogus {
        let node_type = format_ident!("{bogus}");
        let node_kind = format_ident!("{}", Case::Constant.convert(bogus));
        variants.push(quote! { #node_type(biome_js_syntax::#node_type) });
        cast_arms.push(quote! {
            biome_js_syntax::JsSyntaxKind::#node_kind =>
                <biome_js_syntax::#node_type as AstNode>::cast(node).map(Self::#node_type)
        });
        syntax_arms.push(quote! { Self::#node_type(node) => node.syntax() });
        field_arms.push(quote! { Self::#node_type(_) => Ok(JsValue::undefined()) });
    }

    let getter_methods = accessors.iter().map(|property_name| {
        let getter_name = format_ident!("get_field_{}", Case::Snake.convert(property_name));
        quote! {
            fn #getter_name(
                this: &JsValue,
                _args: &[JsValue],
                context: &mut Context,
            ) -> JsResult<JsValue> {
                Self::resolve_field(this, #property_name, context)
            }
        }
    });

    let registrations = accessors.iter().map(|property_name| {
        let getter_name = format_ident!("get_field_{}", Case::Snake.convert(property_name));
        quote! {
            let getter = NativeFunction::from_fn_ptr(Self::#getter_name)
                .to_js_function(class.context().realm());
            class.accessor(
                js_string!(#property_name),
                Some(getter),
                None,
                Attribute::ENUMERABLE,
            );
        }
    });

    let tokens = quote! {
        #[derive(Debug)]
        enum JsAstNodeData {
            #(#variants,)*
        }

        impl JsAstNodeData {
            fn cast(node: JsSyntaxNode) -> Option<Self> {
                match node.kind() {
                    #(#cast_arms,)*
                    _ => None,
                }
            }

            fn syntax(&self) -> &JsSyntaxNode {
                match self {
                    #(#syntax_arms,)*
                }
            }

            fn resolve_field(&self, field: &str, context: &mut Context) -> JsResult<JsValue> {
                match self {
                    #(#field_arms,)*
                }
            }
        }

        impl JsAstNode {
            #(#getter_methods)*

            fn init_generated(class: &mut ClassBuilder<'_>) {
                #(#registrations)*
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
