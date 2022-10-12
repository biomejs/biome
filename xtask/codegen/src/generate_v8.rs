use std::collections::BTreeSet;

use crate::kinds_src::{AstSrc, Field, KindsSrc, TokenKind};
use crate::{to_upper_snake_case, LanguageKind};
use quote::{format_ident, quote};
use xtask::Result;

pub fn generate_v8(
    grammar: &KindsSrc,
    ast: &AstSrc,
    language_kind: LanguageKind,
) -> Result<String> {
    let mut stmts = Vec::new();
    let mut ops = Vec::new();

    let syntax_crate = language_kind.syntax_crate();
    let syntax_kind = language_kind.syntax_kind();
    let language = language_kind.language();

    let syntax_kind_name = syntax_kind.to_string();
    let syntax_kind = quote! {
        #syntax_crate::#syntax_kind
    };

    let syntax_kinds: Vec<_> = grammar
        .punct
        .iter()
        .map(|(_token, name)| format_ident!("{}", name))
        .chain(
            grammar
                .keywords
                .iter()
                .map(|name| format_ident!("{}_KW", to_upper_snake_case(name))),
        )
        .chain(
            grammar
                .literals
                .iter()
                .map(|name| format_ident!("{}", name)),
        )
        .chain(grammar.tokens.iter().map(|name| format_ident!("{}", name)))
        .chain(grammar.nodes.iter().map(|name| format_ident!("{}", name)))
        .map(|variant| {
            let name = variant.to_string();
            quote! { .variant(#name, #syntax_kind::#variant) }
        })
        .collect();

    let syntax_node = quote! {
        rome_rowan::SyntaxNode<#syntax_crate::#language>
    };

    for node in &ast.nodes {
        let node_name = format_ident!("{}", node.name);
        let node_type = quote! {
            #syntax_crate::#node_name
        };

        let mut methods = Vec::new();

        ops.push(quote! {
            impl<'s> ToV8<'s> for #node_type {
                fn to_v8(
                    self,
                    scope: &mut v8::HandleScope<'s>,
                ) -> anyhow::Result<v8::Local<'s, v8::Value>> {
                    let node = self.into_syntax();
                    crate::registry::instantiate_as::<#node_type, #syntax_node>(scope, node).map(Into::into)
                }
            }
        });

        for method in &node.fields {
            let write_result = quote! {
                let result = ToV8::to_v8(result, scope).unwrap();
                res.set(result);
            };

            let write_option = quote! {
                if let Some(result) = result {
                    #write_result
                } else {
                    res.set_undefined();
                }
            };

            let write_syntax_result = quote! {
                match result {
                    Ok(result) => {
                        #write_result
                    }
                    Err(err) => {
                        let message = err.to_string();
                        let message = v8::String::new(scope, &message).unwrap();
                        let exception = v8::Exception::error(scope, message);
                        scope.throw_exception(exception);
                    }
                }
            };

            let (method_name, write_result) = match method {
                Field::Token { name, kind, .. } => {
                    let method_name = if matches!(kind, TokenKind::Many(_)) {
                        format_ident!("{}", name)
                    } else {
                        method.method_name(language_kind)
                    };

                    let write_result = if method.is_optional() {
                        write_option
                    } else {
                        write_syntax_result
                    };

                    (method_name, write_result)
                }
                Field::Node { ty, optional, .. } => {
                    let method_name = method.method_name(language_kind);

                    let write_result = if ast.is_list(ty) {
                        write_result
                    } else if *optional {
                        write_option
                    } else {
                        write_syntax_result
                    };

                    (method_name, write_result)
                }
            };

            let op_id = format_ident!("{}_{}", node.name, method_name);

            ops.push(quote! {
                #[allow(non_snake_case)]
                fn #op_id<'s>(scope: &mut v8::HandleScope<'s>, args: v8::FunctionCallbackArguments<'s>, mut res: v8::ReturnValue) {
                    let this = args.this().into();
                    let this = std::cell::Ref::<#syntax_node>::from_v8(scope, this).unwrap();
                    let this = #node_type::cast_ref(&*this).unwrap();
                    let result = this.#method_name();
                    #write_result
                }
            });

            let method_name = method_name.to_string();
            methods.push(quote! {
                .method(scope, #method_name, #op_id)
            });
        }

        let node_name = node_name.to_string();
        stmts.push(quote! {
            registry.build_class::<#node_type>(scope, global, #node_name)
                .extends::<#syntax_node>(scope)
                #( #methods )*
                .finish(scope);
        });
    }

    for union in &ast.unions {
        let union_name = format_ident!("{}", union.name);
        let union_type = quote! {
            #syntax_crate::#union_name
        };

        let mut match_arms = Vec::new();

        for variant in &union.variants {
            let variant_name = format_ident!("{}", variant);
            match_arms.push(quote! {
                Self::#variant_name(node) => ToV8::to_v8(node, scope),
            });
        }

        ops.push(quote! {
            impl<'s> ToV8<'s> for #union_type {
                fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
                    match self {
                        #( #match_arms )*
                    }
                }
            }
        });
    }

    for unknown in &ast.unknowns {
        let unknown_name = format_ident!("{}", unknown);
        let unknown_type = quote! {
            #syntax_crate::#unknown_name
        };

        ops.push(quote! {
            crate::convert::impl_convert_native!(#unknown_type);
        });

        let unknown_name = unknown_name.to_string();
        stmts.push(quote! {
            registry.build_class::<#unknown_type>(scope, global, #unknown_name)
                .finish(scope);
        });
    }

    let mut iter_nodes = BTreeSet::new();

    for (name, list) in ast.lists() {
        let list_name = format_ident!("{}", name);
        let list_type = quote! {
            #syntax_crate::#list_name
        };

        iter_nodes.insert((list.element_name.as_str(), list.separator.is_some()));

        let iter_id = format_ident!("{}_iter", name);

        ops.push(quote! {
            crate::convert::impl_convert_native!(#list_type);

            #[allow(non_snake_case)]
            fn #iter_id<'s>(scope: &mut v8::HandleScope<'s>, args: v8::FunctionCallbackArguments<'s>, mut res: v8::ReturnValue) {
                let this = args.this().into();
                let this = std::cell::Ref::<#list_type>::from_v8(scope, this).unwrap();
                let iter = this.iter();
                let iter = ToV8::to_v8(iter, scope).unwrap();
                res.set(iter);
            }
        });

        let list_name = list_name.to_string();
        stmts.push(quote! {
            registry.build_class::<#list_type>(scope, global, #list_name)
                .method(scope, "iter", #iter_id)
                .finish(scope);
        });
    }

    for (node_name, is_separated) in iter_nodes {
        let node_name = format_ident!("{}", node_name);
        let node_type = quote! {
            #syntax_crate::#node_name
        };

        let iterator_type = if is_separated {
            quote! {
                rome_rowan::AstSeparatedListNodesIterator<#syntax_crate::#language, #node_type>
            }
        } else {
            quote! {
                rome_rowan::AstNodeListIterator<#syntax_crate::#language, #node_type>
            }
        };

        ops.push(quote! {
            crate::convert::impl_convert_native!(#iterator_type);
        });

        let iterable_callback = if is_separated {
            quote! {
                AstSeparatedListNodesIterator_next
            }
        } else {
            quote! {
                ToV8::to_v8
            }
        };

        stmts.push(quote! {
            registry.build_interface::<#iterator_type>(scope)
                .iterable(scope, #iterable_callback)
                .finish(scope);
        });
    }

    let ast = quote! {
        use crate::convert::{FromV8, ToV8};
        use super::TemplateRegistry;
        use rome_rowan::{AstNode as _, AstNodeList as _, AstSeparatedList as _};

        pub(super) fn register_interfaces(scope: &mut v8::HandleScope<'_, ()>, global: v8::Local<'_, v8::ObjectTemplate>, registry: &mut TemplateRegistry) {
            registry.build_enum::<#syntax_kind>(scope, global, #syntax_kind_name)
                .variant("EOF", #syntax_kind::EOF)
                #( #syntax_kinds )*
                .finish(scope);

            #( #stmts )*
        }

        #[allow(non_snake_case)]
        fn AstSeparatedListNodesIterator_next<'s, T: ToV8<'s>>(item: rome_rowan::SyntaxResult<T>, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
            ToV8::to_v8(item?, scope)
        }

        #( #ops )*
    };

    let ast = ast
        .to_string()
        .replace("T ! [ ", "T![")
        .replace(" ] )", "])");

    let pretty = xtask::reformat(ast)?;
    Ok(pretty)
}
