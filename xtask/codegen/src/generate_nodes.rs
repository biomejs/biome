use crate::js_kinds_src::{AstNodeSrc, AstSrc, Field, TokenKind};
use crate::language_kind::LanguageKind;
use biome_string_case::Case;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashMap;
use xtask::Result;

pub fn generate_nodes(ast: &AstSrc, language_kind: LanguageKind) -> Result<String> {
    let (node_defs, node_boilerplate_impls): (Vec<_>, Vec<_>) = ast
        .nodes
        .iter()
        .map(|node| {
            let name = format_ident!("{}", node.name);
            let node_kind = format_ident!("{}", Case::Constant.convert(node.name.as_str()));
            let needs_dynamic_slots = node.dynamic;

            let methods = node
                .fields
                .iter()
                .enumerate()
                .map(|(slot_index, field)| match field {
                    Field::Token { name, kind, .. } => {
                        let many = matches!(kind, TokenKind::Many(_));

                        let method_name = if many {
                            format_ident!("{}", name)
                        } else {
                            field.method_name(language_kind)
                        };

                        let is_optional = field.is_optional();

                        let slot_index_access = if field.is_unordered() {
                            quote! { self.slot_map[#slot_index] as usize }
                        } else {
                            quote! { #slot_index }
                        };

                        if is_optional {
                            quote! {
                                pub fn #method_name(&self) -> Option<SyntaxToken> {
                                    support::token(&self.syntax, #slot_index_access)
                                }
                            }
                        } else {
                            quote! {
                                pub fn #method_name(&self) -> SyntaxResult<SyntaxToken> {
                                    support::required_token(&self.syntax, #slot_index_access)
                                }
                            }
                        }
                    }
                    Field::Node { ty, optional, .. } => {
                        let is_list = ast.is_list(ty);
                        let ty = format_ident!("{}", &ty);

                        let slot_index_access = if field.is_unordered() {
                            quote! { self.slot_map[#slot_index] as usize }
                        } else {
                            quote! { #slot_index }
                        };

                        let method_name = field.method_name(language_kind);
                        if is_list {
                            if *optional {
                                panic!("Lists cannot be optional. Instead, the grammar should handle the situation where the list is empty.");
                            }

                            quote! {
                                pub fn #method_name(&self) -> #ty {
                                    support::list(&self.syntax, #slot_index_access)
                                }
                            }
                        } else if *optional {
                            quote! {
                                pub fn #method_name(&self) -> Option<#ty> {
                                    support::node(&self.syntax, #slot_index_access)
                                }
                            }
                        } else {
                            quote! {
                                pub fn #method_name(&self) -> SyntaxResult<#ty> {
                                    support::required_node(&self.syntax, #slot_index_access)
                                }
                            }
                        }
                    }
                });

            let fields = node.fields.iter().map(|field| {
                let name = match field {
                    Field::Token {
                        name,
                        kind: TokenKind::Many(_),
                        ..
                    } => format_ident!("{}", name),
                    _ => field.method_name(language_kind),
                };

                let is_list = match field {
                    Field::Node { ty, .. } => ast.is_list(ty),
                    _ => false,
                };

                let string_name = name.to_string();

                if is_list {
                    quote! {
                        .field(#string_name, &self.#name())
                    }
                } else if field.is_optional() {
                    quote! {
                        .field(#string_name, &support::DebugOptionalElement(self.#name()))
                    }
                } else {
                    quote! {
                        .field(#string_name, &support::DebugSyntaxResult(self.#name()))
                    }
                }
            });

            let string_name = name.to_string();

            let slots_name = format_ident!("{}Fields", node.name);

            let (slot_fields, slot_constructors): (Vec<_>, Vec<_>) = node
                .fields
                .iter()
                .map(|field| match field {
                    Field::Token { name, kind, .. } => {
                        let many = matches!(kind, TokenKind::Many(_));

                        let method_name = if many {
                            format_ident!("{}", name)
                        } else {
                            field.method_name(language_kind)
                        };

                        let is_optional = field.is_optional();

                        let field = if is_optional {
                            quote! { #method_name: Option<SyntaxToken> }
                        } else {
                            quote! { #method_name: SyntaxResult<SyntaxToken> }
                        };

                        (field, quote! { #method_name: self.#method_name() })
                    }
                    Field::Node { ty, optional, .. } => {
                        let is_list = ast.is_list(ty);
                        let ty = format_ident!("{}", &ty);

                        let method_name = field.method_name(language_kind);
                        let field = if is_list {
                            quote! { #method_name: #ty }
                        } else if *optional {
                            quote! { #method_name: Option<#ty> }
                        } else {
                            quote! { #method_name: SyntaxResult<#ty> }
                        };

                        (field, quote! { #method_name: self.#method_name() })
                    }
                })
                .unzip();

            let slot_count = node.fields.len();
            let slot_map_type = quote! { [u8; #slot_count] };
            let maybe_dynamic_slot_map_member = if needs_dynamic_slots {
                quote! { pub(crate) slot_map: #slot_map_type, }
            } else {
                Default::default()
            };

            let new_unchecked_constructor = if needs_dynamic_slots {
                let slot_map_builder_impl = get_slot_map_builder_impl(node, language_kind);
                quote! {
                    /// Create an AstNode from a SyntaxNode without checking its kind
                    ///
                    /// # Safety
                    /// This function must be guarded with a call to [AstNode::can_cast]
                    /// or a match on [SyntaxNode::kind]
                    #[inline]
                    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
                        let slot_map = #name::build_slot_map(&syntax);
                        Self { syntax, slot_map }
                    }

                    // The allow for clippy is needed because nodes that _only_ have
                    // unordered fields will have a single loop where the loop counter
                    // is the same as the current slot, but nodes that mix ordered and
                    // unordered fields will need the value outside of the loop. Generating
                    // code that does this appeasingly for both cases is not worth the
                    // effort and has no performance cost.

                    /// Construct the `slot_map` for this node by checking the `kind` of
                    /// each child of `syntax` against the defined grammar for the node.
                    #![allow(clippy::explicit_counter_loop)]
                    pub fn build_slot_map(syntax: &SyntaxNode) -> #slot_map_type {
                        #slot_map_builder_impl
                    }
                }
            } else {
                quote! {
                    /// Create an AstNode from a SyntaxNode without checking its kind
                    ///
                    /// # Safety
                    /// This function must be guarded with a call to [AstNode::can_cast]
                    /// or a match on [SyntaxNode::kind]
                    #[inline]
                    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
                        Self { syntax }
                    }
                }
            };

            let cast_impl = if needs_dynamic_slots {
                quote! {
                    if Self::can_cast(syntax.kind()) {
                        let slot_map = #name::build_slot_map(&syntax);
                        Some(Self { syntax, slot_map })
                    } else { None }
                }
            } else {
                quote! { if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None } }
            };

            let ast_node_slot_map_impl = if needs_dynamic_slots {
                quote! {
                    impl AstNodeSlotMap<#slot_count> for #name {
                        fn slot_map(&self) -> &#slot_map_type {
                            &self.slot_map
                        }
                    }
                }
            } else {
                Default::default()
            };

            let debug_fmt_impl = if fields.len() > 0 {
                quote! {
                    thread_local! { static DEPTH: std::cell::Cell<u8> = const { std::cell::Cell::new(0) } };
                    let current_depth = DEPTH.get();
                    let result = if current_depth < 16 {
                        DEPTH.set(current_depth + 1);
                        f.debug_struct(#string_name)
                            #(#fields)*
                            .finish()
                    } else {
                        f.debug_struct(#string_name).finish()
                    };
                    DEPTH.set(current_depth);
                    result
                }
            } else {
                quote! {
                    f.debug_struct(#string_name).finish()
                }
            };

            (
                quote! {
                    // TODO: review documentation
                    // #[doc = #documentation]
                    #[derive(Clone, PartialEq, Eq, Hash)]
                    pub struct #name {
                        pub(crate) syntax: SyntaxNode,
                        #maybe_dynamic_slot_map_member
                    }

                    impl #name {
                        #new_unchecked_constructor

                        pub fn as_fields(&self) -> #slots_name {
                            #slots_name {
                                #( #slot_constructors, )*
                            }
                        }

                        #(#methods)*
                    }

                    impl Serialize for #name {
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where
                        S: Serializer,
                        {
                            self.as_fields().serialize(serializer)
                        }
                    }

                    #[derive(Serialize)]
                    pub struct #slots_name {
                        #( pub #slot_fields, )*
                    }
                },
                quote! {
                    impl AstNode for #name {
                        type Language = Language;

                        const KIND_SET: SyntaxKindSet<Language> =
                            SyntaxKindSet::from_raw(RawSyntaxKind(#node_kind as u16));

                        fn can_cast(kind: SyntaxKind) -> bool {
                            kind == #node_kind
                        }
                        fn cast(syntax: SyntaxNode) -> Option<Self> {
                            #cast_impl
                        }
                        fn syntax(&self) -> &SyntaxNode { &self.syntax }
                        fn into_syntax(self) -> SyntaxNode { self.syntax }
                    }

                    #ast_node_slot_map_impl

                    impl std::fmt::Debug for #name {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            #debug_fmt_impl
                        }
                    }

                    impl From<#name> for SyntaxNode {
                        fn from(n: #name) -> SyntaxNode {
                            n.syntax
                        }
                    }

                    impl From<#name> for SyntaxElement {
                        fn from(n: #name) -> SyntaxElement {
                            n.syntax.into()
                        }
                    }
                },
            )
        })
        .unzip();

    // it maps enum name A and its corresponding variants
    let name_to_variants: HashMap<_, _> = ast
        .unions
        .iter()
        .map(|current_enum| (current_enum.name.clone(), current_enum.variants.clone()))
        .collect();

    let (union_defs, union_boilerplate_impls): (Vec<_>, Vec<_>) = ast
        .unions
        .iter()
        .map(|union| {
            let name = format_ident!("{}", union.name);

            // here we collect all the variants because this will generate the enums
            // so we don't care about filtered variants
            let variants_for_union: Vec<_> = union
                .variants
                .iter()
                .map(|variant| {
                    let variant_name = format_ident!("{}", variant);
                    quote! {
                        #variant_name(#variant_name)
                    }
                })
                .collect();

            let as_method_for_variants_for_union: Vec<_> = union
                .variants
                .iter()
                .map(|variant| {
                    let variant_name = format_ident!("{}", variant);
                    let fn_name = format_ident!("as_{}", Case::Snake.convert(variant));
                    quote! {
                        pub fn #fn_name(&self) -> Option<&#variant_name> {
                           match &self {
                            #name::#variant_name(item) => Some(item),
                               _ => None
                           }
                        }
                    }
                })
                .collect();

            // Here we make the partition
            //
            // Inside an enum, we can have variants that point to a "flat" type or to another enum;
            // we want to divide these variants as we will generate a different code based on these requirements
            let (variant_of_variants, simple_variants): (Vec<_>, Vec<_>) =
                union.variants.iter().partition(|current_enum| {
                    if let Some(variants) = name_to_variants.get(*current_enum) {
                        !variants.is_empty()
                    } else {
                        false
                    }
                });

            let variants: Vec<_> = simple_variants
                .iter()
                .map(|var| format_ident!("{}", var))
                .collect();

            let kinds: Vec<_> = variants
                .iter()
                .map(|name| format_ident!("{}", Case::Constant.convert(&name.to_string())))
                .collect();

            let variant_cast: Vec<_> = simple_variants
                .iter()
                .map(|current_enum| {
                    let variant_is_enum = ast.unions.iter().find(|e| &e.name == *current_enum);
                    let variant_name = format_ident!("{}", current_enum);

                    let variant_is_dynamic = ast
                        .nodes
                        .iter()
                        .find(|e| &e.name == *current_enum)
                        .is_some_and(|node| node.fields.iter().any(|field| field.is_unordered()));

                    if variant_is_enum.is_some() || variant_is_dynamic {
                        quote! {
                            #variant_name::cast(syntax)?
                        }
                    } else {
                        quote! {
                            #variant_name { syntax }
                        }
                    }
                })
                .collect();

            // variant of variants
            let vv: Vec<_> = variant_of_variants
                .iter()
                .enumerate()
                .map(|(i, en)| {
                    let variant_name = format_ident!("{}", en);
                    let variable_name = format_ident!("{}", Case::Snake.convert(en.as_str()));
                    (
                        // try_cast() code
                        if i != variant_of_variants.len() - 1 {
                            quote! {
                            let syntax = match #variant_name::try_cast(syntax) {
                                Ok(#variable_name) => {
                                    return Some(#name::#variant_name(#variable_name));
                                }
                                Err(syntax) => syntax,
                            };}
                        } else {
                            // if this is the last variant, do not clone syntax
                            quote! {
                                if let Some(#variable_name) = #variant_name::cast(syntax) {
                                    return Some(#name::#variant_name(#variable_name));
                            }}
                        },
                        // can_cast() code
                        quote! {
                            k if #variant_name::can_cast(k) => true,
                        },
                        // syntax() code
                        quote! {
                            #name::#variant_name(it) => it.syntax()
                        },
                        // into_syntax() code
                        quote! {
                            #name::#variant_name(it) => it.into_syntax()
                        },
                    )
                })
                .collect();

            let vv_cast = vv.iter().map(|v| v.0.clone());

            let vv_can_cast = vv.iter().map(|v| v.1.clone());
            let vv_syntax = vv.iter().map(|v| v.2.clone());
            let vv_into_syntax = vv.iter().map(|v| v.3.clone());

            let all_kinds = if !kinds.is_empty() {
                quote! {
                    #(#kinds)|* => true,
                }
            } else {
                quote! {}
            };

            let cast_fn = if !kinds.is_empty() {
                quote! {
                    let res = match syntax.kind() {
                        #(
                            #kinds => #name::#variants(#variant_cast),
                        )*
                        _ =>  {
                            #(
                                #vv_cast
                            )*
                            return None
                        }
                    };
                    Some(res)
                }
            } else {
                quote! {
                        #(
                        #vv_cast
                    )*
                    None
                }
            };

            let can_cast_fn = if union.variants.iter().any(|v| !simple_variants.contains(&v)) {
                quote! {
                    match kind {
                        #all_kinds
                        #(#vv_can_cast)*
                        _ => false
                    }
                }
            } else {
                quote! {
                    matches!(kind, #(#kinds)|*)
                }
            };

            let kind_set: Vec<_> = union
                .variants
                .iter()
                .enumerate()
                .map(|(index, v)| {
                    let ident = format_ident!("{}", v);
                    if index == 0 {
                        quote!( #ident::KIND_SET )
                    } else {
                        quote!( .union(#ident::KIND_SET) )
                    }
                })
                .collect();

            let (variant_syntax, variant_into_syntax): (Vec<_>, Vec<_>) = simple_variants
                .iter()
                .map(|_| {
                    (
                        quote! {
                            &it.syntax
                        },
                        quote! {
                            it.syntax
                        },
                    )
                })
                .unzip();

            let all_variant_names: Vec<_> = union
                .variants
                .iter()
                .map(|variant| format_ident!("{}", variant))
                .collect();

            (
                quote! {
                    // #[doc = #doc]
                    #[derive(Clone, PartialEq, Eq, Hash, Serialize)]
                    pub enum #name {
                        #(#variants_for_union),*
                    }

                    impl #name {
                        #(#as_method_for_variants_for_union)*
                    }
                },
                quote! {
                    #(
                    impl From<#variants> for #name {
                        fn from(node: #variants) -> #name {
                            #name::#variants(node)
                        }
                    }
                    )*

                    impl AstNode for #name {
                        type Language = Language;

                        const KIND_SET: SyntaxKindSet<Language> = #( #kind_set )*;

                        fn can_cast(kind: SyntaxKind) -> bool {
                            #can_cast_fn
                        }
                        fn cast(syntax: SyntaxNode) -> Option<Self> {
                                #cast_fn
                        }
                        fn syntax(&self) -> &SyntaxNode {
                            match self {
                                #(
                                #name::#variants(it) => #variant_syntax,
                                )*
                                #(
                                    #vv_syntax
                                ),*
                            }
                        }
                        fn into_syntax(self) -> SyntaxNode {
                            match self {
                                #(
                                #name::#variants(it) => #variant_into_syntax,
                                )*
                                #(
                                    #vv_into_syntax
                                ),*
                            }
                        }
                    }

                    impl std::fmt::Debug for #name {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            match self {
                            #(
                                #name::#all_variant_names(it) => std::fmt::Debug::fmt(it, f),
                            )*
                        }
                            }
                    }

                    impl From<#name> for SyntaxNode {
                        fn from(n: #name) -> SyntaxNode {
                            match n {
                                #(
                                #name::#all_variant_names(it) => it.into(),
                                )*
                            }
                        }
                    }

                    impl From<#name> for SyntaxElement {
                        fn from(n: #name) -> SyntaxElement {
                            let node: SyntaxNode = n.into();
                            node.into()
                        }
                    }
                },
            )
        })
        .unzip();

    let union_names = ast.unions.iter().map(|it| &it.name);
    let node_names = ast.nodes.iter().map(|it| &it.name);

    let display_impls = union_names
        .chain(node_names.clone())
        .map(|it| format_ident!("{}", it))
        .map(|name| {
            quote! {
                impl std::fmt::Display for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        std::fmt::Display::fmt(self.syntax(), f)
                    }
                }
            }
        });

    let bogus = ast.bogus.iter().map(|bogus_name| {
        let ident = format_ident!("{}", bogus_name);
        let string_name = bogus_name;
        let kind = format_ident!("{}", Case::Constant.convert(bogus_name));

        quote! {
            #[derive(Clone, PartialEq, Eq, Hash, Serialize)]
            pub struct #ident {
                syntax: SyntaxNode
            }

            impl #ident {
                /// Create an AstNode from a SyntaxNode without checking its kind
                ///
                /// # Safety
                /// This function must be guarded with a call to [AstNode::can_cast]
                /// or a match on [SyntaxNode::kind]
                #[inline]
                pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
                    Self { syntax }
                }

                pub fn items(&self) -> SyntaxElementChildren {
                    support::elements(&self.syntax)
                }
            }

            impl AstNode for #ident {
                type Language = Language;

                const KIND_SET: SyntaxKindSet<Language> =
                    SyntaxKindSet::from_raw(RawSyntaxKind(#kind as u16));

                fn can_cast(kind: SyntaxKind) -> bool {
                    kind == #kind
                }

                fn cast(syntax: SyntaxNode) -> Option<Self> {
                    if Self::can_cast(syntax.kind()) {
                        Some(Self { syntax })
                    } else {
                        None
                    }
                }
                fn syntax(&self) -> &SyntaxNode {
                    &self.syntax
                }
                fn into_syntax(self) -> SyntaxNode {
                    self.syntax
                }
            }

            impl std::fmt::Debug for #ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(#string_name)
                        .field("items", &DebugSyntaxElementChildren(self.items()))
                        .finish()
                }
            }

            impl From<#ident> for SyntaxNode {
                fn from(n: #ident) -> SyntaxNode {
                    n.syntax
                }
            }

            impl From<#ident> for SyntaxElement {
                fn from(n: #ident) -> SyntaxElement {
                    n.syntax.into()
                }
            }
        }
    });

    let any_bogus = {
        let kinds = ast.bogus.iter().enumerate().map(|(i, bogus_name)| {
            let ident = format_ident!("{bogus_name}");
            if i == 0 {
                quote! { #ident }
            } else {
                quote! { | #ident }
            }
        });
        let ident = format_ident!(
            "Any{}BogusNode",
            ast.bogus
                .iter()
                .find_map(|bogus_name| bogus_name.strip_suffix("Bogus"))
                .expect("expected a plain *Bogus node")
        );
        quote! {
            biome_rowan::declare_node_union! {
                pub #ident = #(#kinds)*
            }
        }
    };

    let lists = ast.lists().map(|(name, list)| {
        let list_name = format_ident!("{}", name);
        let list_kind = format_ident!("{}", Case::Constant.convert(name));
        let element_type = format_ident!("{}", list.element_name);

        let node_impl = quote! {
            impl #list_name {
                /// Create an AstNode from a SyntaxNode without checking its kind
                ///
                /// # Safety
                /// This function must be guarded with a call to [AstNode::can_cast]
                /// or a match on [SyntaxNode::kind]
                #[inline]
                pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
                    Self { syntax_list: syntax.into_list() }
                }
            }

            impl AstNode for #list_name {
                type Language = Language;

                const KIND_SET: SyntaxKindSet<Language> =
                    SyntaxKindSet::from_raw(RawSyntaxKind(#list_kind as u16));

                fn can_cast(kind: SyntaxKind) -> bool {
                    kind == #list_kind
                }

                fn cast(syntax: SyntaxNode) -> Option<#list_name> {
                    if Self::can_cast(syntax.kind()) {
                        Some(#list_name { syntax_list: syntax.into_list() })
                    } else {
                        None
                    }
                }

                fn syntax(&self) -> &SyntaxNode {
                    self.syntax_list.node()
                }
                fn into_syntax(self) -> SyntaxNode {
                    self.syntax_list.into_node()
                }
            }
        };

        let padded_name = format!("{name} ");

        let list_impl = if list.separator.is_some() {
            quote! {
                impl Serialize for #list_name {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where
                        S: Serializer,
                        {
                            let mut seq = serializer.serialize_seq(Some(self.len()))?;
                            for e in self.iter() {
                                seq.serialize_element(&e)?;
                            }
                            seq.end()
                        }
                }

                impl AstSeparatedList for #list_name {
                    type Language = Language;
                    type Node = #element_type;
                    fn syntax_list(&self) -> &SyntaxList {
                        &self.syntax_list
                    }
                    fn into_syntax_list(self) -> SyntaxList {
                        self.syntax_list
                    }
                }

                impl Debug for #list_name {
                    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                        f.write_str(#padded_name)?;
                        f.debug_list().entries(self.elements()).finish()
                    }
                }

                impl IntoIterator for #list_name {
                    type Item = SyntaxResult<#element_type>;
                    type IntoIter = AstSeparatedListNodesIterator<Language, #element_type>;

                    fn into_iter(self) -> Self::IntoIter {
                        self.iter()
                    }
                }

                impl IntoIterator for &#list_name {
                    type Item = SyntaxResult<#element_type>;
                    type IntoIter = AstSeparatedListNodesIterator<Language, #element_type>;

                    fn into_iter(self) -> Self::IntoIter {
                        self.iter()
                    }
                }
            }
        } else {
            quote! {
                impl Serialize for #list_name {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where
                        S: Serializer,
                        {
                            let mut seq = serializer.serialize_seq(Some(self.len()))?;
                            for e in self.iter() {
                                seq.serialize_element(&e)?;
                            }
                            seq.end()
                        }
                }

                impl AstNodeList for #list_name {
                    type Language = Language;
                    type Node = #element_type;
                    fn syntax_list(&self) -> &SyntaxList {
                        &self.syntax_list
                    }
                    fn into_syntax_list(self) -> SyntaxList {
                        self.syntax_list
                    }
                }

                impl Debug for #list_name {
                    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                        f.write_str(#padded_name)?;
                        f.debug_list().entries(self.iter()).finish()
                    }
                }

                impl IntoIterator for &#list_name {
                    type Item = #element_type;
                    type IntoIter = AstNodeListIterator<Language, #element_type>;

                    fn into_iter(self) -> Self::IntoIter {
                        self.iter()
                    }
                }

                impl IntoIterator for #list_name {
                    type Item = #element_type;
                    type IntoIter = AstNodeListIterator<Language, #element_type>;

                    fn into_iter(self) -> Self::IntoIter {
                        self.iter()
                    }
                }

            }
        };

        quote! {
            #[derive(Clone, Eq, PartialEq, Hash)]
            pub struct #list_name {
              syntax_list: SyntaxList,
            }

            #node_impl
            #list_impl
        }
    });

    let syntax_kind = language_kind.syntax_kind();
    let syntax_node = language_kind.syntax_node();
    let syntax_element = language_kind.syntax_element();
    let syntax_element_children = language_kind.syntax_element_children();
    let syntax_list = language_kind.syntax_list();
    let syntax_token = language_kind.syntax_token();
    let language = language_kind.language();

    let serde_import = quote! {
        use serde::{Serialize, Serializer};
        use serde::ser::SerializeSeq;
    };

    let ast = quote! {
        #![allow(dead_code)]
        #![allow(unused)]
        use crate::{
            macros::map_syntax_node,
            #language as Language, #syntax_element as SyntaxElement, #syntax_element_children as SyntaxElementChildren,
            #syntax_kind::{self as SyntaxKind, *},
            #syntax_list as SyntaxList, #syntax_node as SyntaxNode, #syntax_token as SyntaxToken,
        };
        use biome_rowan::{
            AstNodeList, AstNodeListIterator,  AstNodeSlotMap, AstSeparatedList, AstSeparatedListNodesIterator,
            support, AstNode,SyntaxKindSet, RawSyntaxKind, SyntaxResult
        };
        use std::fmt::{Debug, Formatter};
        #serde_import

        /// Sentinel value indicating a missing element in a dynamic node, where
        /// the slots are not statically known.
        pub(crate) const SLOT_MAP_EMPTY_VALUE: u8 = u8::MAX;

        #(#node_defs)*
        #(#union_defs)*
        #(#node_boilerplate_impls)*
        #(#union_boilerplate_impls)*
        #(#display_impls)*
        #(#bogus)*
        #any_bogus
        #(#lists)*

        #[derive(Clone)]
        pub struct DebugSyntaxElementChildren(pub SyntaxElementChildren);

        impl Debug for DebugSyntaxElementChildren {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.debug_list()
                    .entries(self.clone().0.map(DebugSyntaxElement))
                    .finish()
            }
        }

        struct DebugSyntaxElement(SyntaxElement);

        impl Debug for DebugSyntaxElement {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match &self.0 {
                    SyntaxElement::Node(node) => {
                        map_syntax_node!(node.clone(), node => std::fmt::Debug::fmt(&node, f))
                    }
                    SyntaxElement::Token(token) => Debug::fmt(token, f),
                }
            }
        }

    };

    let ast = ast
        .to_string()
        .replace("T ! [ ", "crate::T![")
        .replace(" ] )", "])");

    let pretty = xtask::reformat(ast)?;
    Ok(pretty)
}

pub(crate) fn token_kind_to_code(name: &str, language_kind: LanguageKind) -> TokenStream {
    let kind_variant_name = Case::Constant.convert(name);

    let kind_source = language_kind.kinds();
    if kind_source.literals.contains(&kind_variant_name.as_str())
        || kind_source.tokens.contains(&kind_variant_name.as_str())
    {
        let ident = format_ident!("{}", kind_variant_name);
        quote! {  #ident }
    } else if kind_source.keywords.contains(&name) {
        // we need to replace "-" with "_" for the keywords
        // e.g. we have `color-profile` in css but it's an invalid ident in rust code
        let token = name.replace('-', "_");
        // also mark uppercase differently from lowercase
        // e.g. "query" => "QUERY", "QUERY" => "QUERY_UPPERCASE"
        let token = if token.chars().all(|c| c.is_uppercase()) {
            "UPPER_".to_string() + token.as_str()
        } else {
            token
        };
        let token: TokenStream = token.parse().unwrap();
        quote! { T![#token] }
    } else {
        // `$`, `[`, and `]` is valid syntax in rust and it's part of macros,
        // so we need to decorate the tokens with quotes
        if should_token_be_quoted(name) {
            let token = Literal::string(name);
            quote! { T![#token] }
        } else {
            let token: TokenStream = name.parse().unwrap();
            quote! { T![#token] }
        }
    }
}

/// Return a function body that iterates over a SyntaxNode's children, comparing
/// each against the allowed kinds for that slot from the grammar, tracking each
/// filled slot in a `slot_map` and returning that value once all of the children
/// have been consumed.
///
/// To do this:
///   - Ordered fields simply check if the current slot is filled with the
///     expected syntax kind.
///   - Unordered fields are grouped with other consecutive unordered fields,
///     and each child is checked to find the first matching, unfilled slot
///     from the grammar, until either all of the fields are filled or the
///     current child does not match any of the fields.
///   - The `slot_map` entry for the defined grammar node is filled with the
///     index of the current child.
fn get_slot_map_builder_impl(node: &AstNodeSrc, language_kind: LanguageKind) -> TokenStream {
    let slot_count = node.fields.len();

    // Chunk the fields of the node into groups of unordered nodes that need
    // to be checked in parallel and ordered nodes that get checked one by one.
    let field_groups = group_fields_for_ordering(node);
    let mut field_index = 0;

    let last_field = field_groups.last().and_then(|group| group.last());

    let field_mappers = field_groups.iter()
        .map(|group| {
            match group.len() {
                0 => unreachable!("Somehow encountered a group of fields with no entries"),
                // Single-field groups are assumed to act like ordered fields, so
                // they can just check the kind and move on if there's no match.
                1 => {
                    let field = group[0];
                    let this_field_index: usize = field_index;
                    field_index += 1;
                    let field_predicate = get_field_predicate(field, language_kind);

                    let is_last = last_field.is_some_and(|last| field == *last);

                    // Don't increment current_element and current_slot if this is the
                    // last element, otherwise Rust warns about the value being unused.
                    if is_last {
                        quote! {
                            if let Some(element) = &current_element {
                                if #field_predicate {
                                    slot_map[#this_field_index] = current_slot;
                                }
                            }
                        }
                    } else {
                        quote! {
                            if let Some(element) = &current_element {
                                if #field_predicate {
                                    slot_map[#this_field_index] = current_slot;
                                }
                            }
                            current_slot += 1;
                            current_element = children.next();
                        }
                    }
                }
                _ => {
                    let variants = group.iter().enumerate().map(|(index, field)| {
                        let this_field_index = field_index;
                        field_index += 1;
                        let field_predicate = get_field_predicate(field, language_kind);

                        let maybe_else = if index > 0 {
                            quote! { else }
                        } else {
                            Default::default()
                        };

                        quote! {
                            #maybe_else if slot_map[#this_field_index] == SLOT_MAP_EMPTY_VALUE && #field_predicate {
                                slot_map[#this_field_index] = current_slot;
                            }
                        }
                    });

                    let group_length = group.len();

                    quote! {
                        for _ in 0usize..#group_length {
                            if let Some(element) = &current_element {
                                #(#variants)*
                            };
                            current_slot += 1;
                            current_element = children.next();
                        }
                    }
                }
            }
        })
        .collect::<Vec<TokenStream>>();

    quote! {
        let mut children = syntax.children();
        let mut slot_map = [SLOT_MAP_EMPTY_VALUE; #slot_count];
        let mut current_slot = 0;
        let mut current_element = children.next();

        #(#field_mappers)*

        slot_map
    }
}

pub(crate) fn get_field_predicate(field: &Field, language_kind: LanguageKind) -> TokenStream {
    match field {
        Field::Node { ty, .. } => {
            let ast_type_name = format_ident!("{}", ty);

            quote! {
                #ast_type_name::can_cast(element.kind())
            }
        }
        Field::Token { kind, .. } => match kind {
            TokenKind::Single(expected) => {
                let expected_kind = token_kind_to_code(expected, language_kind);
                quote! { element.kind() == #expected_kind}
            }
            TokenKind::Many(expected) => {
                let expected_kinds = expected
                    .iter()
                    .map(|kind| token_kind_to_code(kind, language_kind));
                quote! {
                    matches!(element.kind(), #(#expected_kinds)|*)
                }
            }
        },
    }
}

/// Group the fields of a node into groups of consecutive unordered fields,
/// keeping each ordered field as its own group. This allows the code generation
/// to create loops/switches/etc for the unordered fields while preserving the
/// sequential sorting of the ordered fields.
pub(crate) fn group_fields_for_ordering(node: &AstNodeSrc) -> Vec<Vec<&Field>> {
    let mut groups = vec![];
    let mut current_group = vec![];
    let mut last_was_ordered = true;

    for field in node.fields.iter() {
        if (!field.is_unordered() || last_was_ordered) && !current_group.is_empty() {
            groups.push(current_group);
            current_group = vec![];
        }
        current_group.push(field);
        last_was_ordered = !field.is_unordered();
    }

    groups.push(current_group);
    groups
}

/// Whether or not a token should be surrounded by quotes when being printed in the generated code.
///
/// Some tokens need to be quoted in the `T![]` macro because they conflict with Rust syntax.
pub fn should_token_be_quoted(token: &str) -> bool {
    matches!(
        token,
        "$=" | "$_" | "U+" | "<![CDATA[" | "]]>" | "   " | "_"
    )
}
