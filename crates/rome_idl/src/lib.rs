use std::collections::{btree_map, BTreeMap};

use prettyplease::unparse;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_quote, parse_str};
use weedle::{
    argument::Argument,
    attribute::{ExtendedAttribute, ExtendedAttributeList, IdentifierOrString},
    common::{Bracketed, Identifier, Punctuated},
    interface::{
        AttributeInterfaceMember, ConstructorInterfaceMember, InterfaceMember,
        IterableInterfaceMember, OperationInterfaceMember, StringifierOrStatic,
    },
    literal::DefaultValue,
    namespace::{AttributeNamespaceMember, NamespaceMember, OperationNamespaceMember},
    parse,
    term::Comma,
    types::{FloatingPointType, IntegerType, MayBeNull, NonAnyType, ReturnType, SingleType, Type},
    Definition, InterfaceDefinition, NamespaceDefinition,
};

pub fn generate(source: &str) -> String {
    let definitions = parse(source).expect("failed to parse IDL file");

    let definitions = DefinitionsMap::new(definitions);

    let mut stmts = Vec::new();
    let mut ops = Vec::new();

    for (_, definition) in definitions.iter() {
        handle_definition(&definitions, &mut ops, &mut stmts, definition);
    }

    unparse(&parse_quote! {
        use crate::convert::{FromV8, ToV8};
        use super::TemplateRegistry;

        pub(super) fn register_interfaces(scope: &mut v8::HandleScope<'_, ()>, global: v8::Local<'_, v8::ObjectTemplate>, registry: &mut TemplateRegistry) {
            #( #stmts )*
        }

        #( #ops )*
    })
}

fn handle_definition(
    definitions: &DefinitionsMap,
    ops: &mut Vec<TokenStream>,
    stmts: &mut Vec<TokenStream>,
    definition: &Definition,
) {
    match definition {
        Definition::Typedef(_) | Definition::Enum(_) => {}
        Definition::Interface(interface) => handle_class(definitions, ops, stmts, interface),
        Definition::Namespace(namespace) => handle_class(definitions, ops, stmts, namespace),
        _ => unimplemented!("definitions of this type are not implemented yet: {definition:?}"),
    }
}

trait ClassDefinition<'a> {
    type Member: ClassMember<'a> + 'a;

    const IS_NAMESPACE: bool;

    fn attributes(&self) -> Option<&ExtendedAttributeList>;
    fn identifier(&self) -> &str;
    fn members(&self) -> &[Self::Member];
}

impl<'a> ClassDefinition<'a> for InterfaceDefinition<'a> {
    type Member = InterfaceMember<'a>;

    const IS_NAMESPACE: bool = false;

    fn attributes(&self) -> Option<&ExtendedAttributeList> {
        self.attributes.as_ref()
    }

    fn identifier(&self) -> &str {
        self.identifier.0
    }

    fn members(&self) -> &[Self::Member] {
        &self.members.body
    }
}

impl<'a> ClassDefinition<'a> for NamespaceDefinition<'a> {
    type Member = NamespaceMember<'a>;

    const IS_NAMESPACE: bool = true;

    fn attributes(&self) -> Option<&ExtendedAttributeList> {
        self.attributes.as_ref()
    }

    fn identifier(&self) -> &str {
        self.identifier.0
    }

    fn members(&self) -> &[Self::Member] {
        &self.members.body
    }
}

fn handle_class<'a, D: ClassDefinition<'a>>(
    definitions: &DefinitionsMap,
    ops: &mut Vec<TokenStream>,
    stmts: &mut Vec<TokenStream>,
    definition: &'a D,
) {
    let mut is_exposed = false;

    if let Some(attrs) = definition.attributes() {
        for attr in &attrs.body.list {
            if let ExtendedAttribute::Ident(attr) = attr {
                if attr.lhs_identifier.0 == "Exposed" {
                    is_exposed = true;
                }
            }
        }
    }

    let interface_name = definition.identifier();
    let interface_type = find_rust_type(definition);

    let mut constructor = None;
    let mut members = Vec::new();

    for member in definition.members() {
        handle_member(
            definitions,
            ops,
            &mut constructor,
            &mut members,
            interface_name,
            &interface_type,
            member,
        );
    }

    let builder = if D::IS_NAMESPACE {
        quote! { build_namespace(scope, global, #interface_name) }
    } else if is_exposed {
        if let Some(constructor) = constructor {
            quote! { build_class_with_constructor::<#interface_type>(scope, global, #interface_name, #constructor) }
        } else {
            quote! { build_class::<#interface_type>(scope, global, #interface_name) }
        }
    } else {
        quote! { build_interface::<#interface_type>(scope) }
    };

    let maybe_finish = if !D::IS_NAMESPACE {
        quote! { .finish(scope) }
    } else {
        quote!()
    };

    stmts.push(quote! {
        registry.#builder
            #( #members )*
            #maybe_finish;
    });
}

enum MemberKind<'a, M: ClassMember<'a> + ?Sized> {
    Operation(&'a M::Operation),
    Iterable(&'a IterableInterfaceMember<'a>),
    Attribute(&'a M::Attribute),
    Constructor(&'a ConstructorInterfaceMember<'a>),
    Other,
}

trait ClassMember<'a> {
    type Operation: ClassOperation + 'a;
    type Attribute;

    fn kind(&'a self) -> MemberKind<'a, Self>;
}

impl<'a> ClassMember<'a> for InterfaceMember<'a> {
    type Operation = OperationInterfaceMember<'a>;
    type Attribute = AttributeInterfaceMember<'a>;

    fn kind(&'a self) -> MemberKind<'a, Self> {
        match self {
            Self::Operation(op) => MemberKind::Operation(op),
            Self::Iterable(iter) => MemberKind::Iterable(iter),
            Self::Attribute(op) => MemberKind::Attribute(op),
            Self::Constructor(op) => MemberKind::Constructor(op),
            _ => MemberKind::Other,
        }
    }
}

impl<'a> ClassMember<'a> for NamespaceMember<'a> {
    type Operation = OperationNamespaceMember<'a>;
    type Attribute = AttributeNamespaceMember<'a>;

    fn kind(&'a self) -> MemberKind<'a, Self> {
        match self {
            Self::Operation(op) => MemberKind::Operation(op),
            Self::Attribute(op) => MemberKind::Attribute(op),
        }
    }
}

trait ClassOperation {
    const IS_NAMESPACE: bool;

    fn attributes(&self) -> Option<&ExtendedAttributeList>;
    fn identifier(&self) -> Option<&str>;
    fn modifier(&self) -> Option<&StringifierOrStatic>;
    fn args(&self) -> &[Argument];
    fn return_type(&self) -> &ReturnType;
}

impl ClassOperation for OperationInterfaceMember<'_> {
    const IS_NAMESPACE: bool = false;

    fn attributes(&self) -> Option<&ExtendedAttributeList> {
        self.attributes.as_ref()
    }

    fn identifier(&self) -> Option<&str> {
        self.identifier.as_ref().map(|id| id.0)
    }

    fn modifier(&self) -> Option<&StringifierOrStatic> {
        self.modifier.as_ref()
    }

    fn args(&self) -> &[Argument] {
        &self.args.body.list
    }

    fn return_type(&self) -> &ReturnType {
        &self.return_type
    }
}

impl ClassOperation for OperationNamespaceMember<'_> {
    const IS_NAMESPACE: bool = true;

    fn attributes(&self) -> Option<&ExtendedAttributeList> {
        self.attributes.as_ref()
    }

    fn identifier(&self) -> Option<&str> {
        self.identifier.as_ref().map(|id| id.0)
    }

    fn modifier(&self) -> Option<&StringifierOrStatic> {
        None
    }

    fn args(&self) -> &[Argument] {
        &self.args.body.list
    }

    fn return_type(&self) -> &ReturnType {
        &self.return_type
    }
}

fn handle_member<'a, M: ClassMember<'a>>(
    definitions: &DefinitionsMap,
    ops: &mut Vec<TokenStream>,
    constructor_id: &mut Option<Ident>,
    members: &mut Vec<TokenStream>,
    interface_name: &str,
    interface_type: &syn::Type,
    member: &'a M,
) {
    match member.kind() {
        MemberKind::Operation(operation) => handle_operation(
            definitions,
            ops,
            members,
            interface_name,
            interface_type,
            operation,
        ),

        MemberKind::Constructor(member) => handle_constructor(
            definitions,
            ops,
            constructor_id,
            interface_name,
            interface_type,
            member,
        ),

        MemberKind::Iterable(IterableInterfaceMember::Single(_)) => {
            members.push(quote! {
                .iterable(scope, ToV8::to_v8)
            });
        }
        MemberKind::Iterable(IterableInterfaceMember::Double(_)) => {
            unimplemented!("key-value iterables are not implemented yet")
        }

        // Ignore for now
        MemberKind::Attribute(_) => {}

        MemberKind::Other => unimplemented!("non-operation members are not implemented yet"),
    }
}

fn handle_operation<O: ClassOperation>(
    definitions: &DefinitionsMap,
    ops: &mut Vec<TokenStream>,
    members: &mut Vec<TokenStream>,
    interface_name: &str,
    interface_type: &syn::Type,
    operation: &O,
) {
    let method_name = match operation.identifier() {
        Some(id) => id,
        None => unimplemented!("special operations are not implemented yet"),
    };

    let mut is_static = false;
    let mut has_custom = false;
    let mut has_custom_binding = false;
    let mut is_cloned_this = false;
    let mut is_mutable_this = false;
    let mut is_cloned_result = false;

    if let Some(modifier) = operation.modifier() {
        match modifier {
            StringifierOrStatic::Stringifier(_) => {}
            StringifierOrStatic::Static(_) => {
                is_static = true;
            }
        }
    }

    if let Some(attrs) = operation.attributes() {
        for attr in &attrs.body.list {
            if let ExtendedAttribute::NoArgs(attr) = attr {
                match (attr.0).0 {
                    "Custom" => {
                        has_custom = true;
                    }
                    "CustomBinding" => {
                        has_custom_binding = true;
                    }
                    "ClonedThis" => {
                        is_cloned_this = true;
                    }
                    "MutableThis" => {
                        is_mutable_this = true;
                    }
                    "ClonedResult" => {
                        is_cloned_result = true;
                    }
                    _ => {}
                }
            }
        }
    }

    let op_id = format_ident!("{interface_name}_{method_name}");
    let method_id = format_ident!("{method_name}");

    if !has_custom_binding {
        let load_this = if !O::IS_NAMESPACE && !is_static {
            let mut load_this = quote! {
                let this = args.this().into();
            };

            load_this = if is_mutable_this {
                quote! {
                    #load_this
                    let mut this = <std::cell::RefMut<#interface_type> as FromV8>::from_v8(scope, this).unwrap();
                }
            } else {
                quote! {
                    #load_this
                    let this = <std::cell::Ref<#interface_type> as FromV8>::from_v8(scope, this).unwrap();
                }
            };

            if is_cloned_this {
                load_this = quote! {
                    #load_this
                    let this = this.clone();
                };
            }

            load_this
        } else {
            quote!()
        };

        let mut arg_idents = Vec::new();
        let mut arg_bindings = Vec::new();

        for (arg_index, arg) in operation.args().iter().enumerate() {
            handle_argument(
                definitions,
                &mut arg_idents,
                &mut arg_bindings,
                arg_index,
                arg,
            );
        }

        let call_method = if has_custom {
            let custom_id = format_ident!("{method_name}_impl");
            quote! {
                #custom_id( #( #arg_idents ),* )
            }
        } else if O::IS_NAMESPACE || is_static {
            quote! {
                #interface_type::#method_id( #( #arg_idents ),* )
            }
        } else {
            quote! {
                this.#method_id( #( #arg_idents ),* )
            }
        };

        let (result_id, write_result) = match operation.return_type() {
            ReturnType::Undefined(_) => (quote! { _result }, quote! { #call_method; }),
            ReturnType::Type(ty) => {
                let write_result = match definitions.resolve_type(ty) {
                    TypeKind::Interface { optional, .. } => {
                        let mut set_result = quote! {
                            let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
                            result.set(res);
                        };

                        if is_cloned_result {
                            set_result = quote! {
                                let res = res.clone();
                                #set_result
                            };
                        };

                        if optional {
                            quote! {
                                if let Some(res) = #call_method {
                                    #set_result
                                }
                            }
                        } else {
                            quote! {
                                let res = #call_method;
                                #set_result
                            }
                        }
                    }
                    TypeKind::Value { .. } => quote! {
                        let res = #call_method;
                        let res = ToV8::to_v8(res, scope).expect("failed to serialize result to JS value");
                        result.set(res);
                    },
                };

                (quote! { mut result }, write_result)
            }
        };

        ops.push(quote! {
            #[allow(non_snake_case)]
            fn #op_id<'s>(scope: &mut v8::HandleScope<'s>, args: v8::FunctionCallbackArguments<'s>, #result_id: v8::ReturnValue) {
                #load_this
                #( #arg_bindings )*
                #write_result
            }
        });
    }

    members.push(if is_static {
        quote! {
            .method_static(scope, #method_name, #op_id)
        }
    } else {
        quote! {
            .method(scope, #method_name, #op_id)
        }
    });
}

fn handle_constructor(
    definitions: &DefinitionsMap,
    ops: &mut Vec<TokenStream>,
    constructor_id: &mut Option<Ident>,
    interface_name: &str,
    interface_type: &syn::Type,
    constructor: &ConstructorInterfaceMember,
) {
    let mut has_custom = false;
    let mut has_custom_binding = false;

    if let Some(attrs) = &constructor.attributes {
        for attr in &attrs.body.list {
            if let ExtendedAttribute::NoArgs(attr) = attr {
                match (attr.0).0 {
                    "Custom" => {
                        has_custom = true;
                    }
                    "CustomBinding" => {
                        has_custom_binding = true;
                    }
                    _ => {}
                }
            }
        }
    }

    let op_id = format_ident!("{interface_name}_constructor");

    if !has_custom_binding {
        let mut arg_idents = Vec::new();
        let mut arg_bindings = Vec::new();

        for (arg_index, arg) in constructor.args.body.list.iter().enumerate() {
            handle_argument(
                definitions,
                &mut arg_idents,
                &mut arg_bindings,
                arg_index,
                arg,
            );
        }

        let call_method = if has_custom {
            let custom_id = format_ident!("{interface_name}_constructor_impl");
            quote! {
                #custom_id( #( #arg_idents ),* )
            }
        } else {
            quote! {
                #interface_type::new( #( #arg_idents ),* )
            }
        };

        ops.push(quote! {
            #[allow(non_snake_case)]
            fn #op_id<'s>(scope: &mut v8::HandleScope<'s>, args: v8::FunctionCallbackArguments<'s>, _res: v8::ReturnValue) {
                #( #arg_bindings )*
                let res = #call_method;
                crate::registry::store_native(scope, args.this(), res);
            }
        });
    }

    *constructor_id = Some(op_id);
}

fn handle_argument(
    definitions: &DefinitionsMap,
    arg_idents: &mut Vec<TokenStream>,
    arg_bindings: &mut Vec<TokenStream>,
    arg_index: usize,
    arg: &Argument,
) {
    let arg = match arg {
        Argument::Single(arg) => arg,
        Argument::Variadic(_) => unimplemented!("variadic arguments"),
    };

    let mut is_cloned = false;
    let mut is_by_ref = false;

    for attr in arg.attributes.iter().flat_map(|item| &item.body.list) {
        if let ExtendedAttribute::NoArgs(attr) = attr {
            match (attr.0).0 {
                "Cloned" => {
                    is_cloned = true;
                }
                "ByRef" => {
                    is_by_ref = true;
                }
                _ => {}
            }
        }
    }

    let arg_ident = format_ident!("{}", arg.identifier.0);
    let arg_index = arg_index as i32;

    let load_arg = quote! {
        args.get(#arg_index)
    };

    let default = arg.default.map(|default| match default.value {
        DefaultValue::String(value) => syn::parse_str::<syn::Expr>(value.0).unwrap(),
        _ => unimplemented!("non-string default values are not supported yet"),
    });

    let is_optional = arg.optional.is_some();
    let parse_arg = match definitions.resolve_type(&arg.type_.type_) {
        TypeKind::Interface {
            rust_type,
            optional,
        } => {
            let mut rust_type = quote! {
                std::cell::Ref<#rust_type>
            };

            if is_optional || optional {
                rust_type = quote! {
                    Option<#rust_type>
                };
            }

            quote! {
                <#rust_type as FromV8>::from_v8(scope, #load_arg)
                    .expect("could not load native object from JS value")
            }
        }

        TypeKind::Value { rust_type } => {
            let mut rust_type = quote! {
                #rust_type
            };

            if is_optional {
                rust_type = quote! {
                    Option<#rust_type>
                };
            }

            quote! {
                <#rust_type as FromV8>::from_v8(scope, #load_arg)
                    .expect("failed to deserialize argument from V8 value")
            }
        }
    };

    let parse_arg = if is_cloned {
        quote! {
            #parse_arg.clone()
        }
    } else {
        parse_arg
    };

    let arg_binding = if let Some(default) = default {
        quote! {
            let #arg_ident = if let Some(#arg_ident) = #parse_arg { #arg_ident } else { #default };
        }
    } else {
        quote! {
            let #arg_ident = #parse_arg;
        }
    };

    arg_bindings.push(arg_binding);

    arg_idents.push(if is_by_ref {
        quote! { &#arg_ident }
    } else {
        quote! { #arg_ident }
    });
}

enum TypeKind {
    Interface {
        rust_type: syn::Type,
        optional: bool,
    },
    Value {
        rust_type: syn::Type,
    },
}

impl TypeKind {
    fn with_optional(self, with_optional: bool) -> Self {
        match self {
            TypeKind::Interface {
                rust_type,
                optional,
            } => TypeKind::Interface {
                rust_type,
                optional: optional || with_optional,
            },
            other => other,
        }
    }
}

struct DefinitionsMap<'a> {
    inner: BTreeMap<String, Definition<'a>>,
}

impl<'a> DefinitionsMap<'a> {
    fn new(definitions: Vec<Definition<'a>>) -> Self {
        let mut inner = BTreeMap::default();

        for definition in definitions {
            let key = match &definition {
                Definition::Interface(interface) => interface.identifier.0,
                Definition::Namespace(namespace) => namespace.identifier.0,
                Definition::Typedef(typedef) => typedef.identifier.0,
                Definition::Enum(enum_) => enum_.identifier.0,
                _ => unimplemented!(
                    "this type of definitions is not implemented yet: {definition:?}"
                ),
            };

            inner.insert(key.into(), definition);
        }

        Self { inner }
    }

    fn resolve_type(&self, ty: &Type) -> TypeKind {
        match ty {
            Type::Single(SingleType::NonAny(NonAnyType::Identifier(id))) => self
                .resolve_name(&id.type_)
                .with_optional(id.q_mark.is_some()),

            Type::Single(SingleType::NonAny(NonAnyType::Integer(ty))) => TypeKind::Value {
                rust_type: with_optional(
                    ty,
                    match &ty.type_ {
                        IntegerType::Short(ty) => {
                            if ty.unsigned.is_some() {
                                parse_quote! { u16 }
                            } else {
                                parse_quote! { i16 }
                            }
                        }
                        IntegerType::Long(ty) => {
                            if ty.unsigned.is_some() {
                                parse_quote! { u32 }
                            } else {
                                parse_quote! { i32 }
                            }
                        }
                        IntegerType::LongLong(ty) => {
                            if ty.unsigned.is_some() {
                                parse_quote! { u64 }
                            } else {
                                parse_quote! { i64 }
                            }
                        }
                    },
                ),
            },

            Type::Single(SingleType::NonAny(NonAnyType::FloatingPoint(ty))) => TypeKind::Value {
                rust_type: with_optional(
                    ty,
                    match &ty.type_ {
                        FloatingPointType::Float(_) => parse_quote! { f32 },
                        FloatingPointType::Double(_) => parse_quote! { f64 },
                    },
                ),
            },

            Type::Single(SingleType::NonAny(NonAnyType::Boolean(ty))) => TypeKind::Value {
                rust_type: with_optional(ty, parse_quote! { bool }),
            },

            Type::Single(SingleType::NonAny(NonAnyType::DOMString(ty))) => TypeKind::Value {
                rust_type: with_optional(ty, parse_quote! { String }),
            },

            Type::Single(SingleType::NonAny(NonAnyType::Sequence(ty))) => {
                match self.resolve_type(&ty.type_.generics.body) {
                    TypeKind::Value { rust_type } => TypeKind::Value {
                        rust_type: with_optional(ty, parse_quote! { Vec<#rust_type> }),
                    },
                    TypeKind::Interface { .. } => {
                        unimplemented!("sequences of interfaces are not supported, please manage the serialization manually using [CustomBinding]")
                    }
                }
            }

            Type::Single(SingleType::Any(_)) => TypeKind::Value {
                rust_type: parse_quote! { serde_v8::Value },
            },

            Type::Union(_) => unimplemented!("union types are not supported"),
            Type::Single(SingleType::NonAny(ty)) => {
                unimplemented!("type {ty:?} is not implemented")
            }
        }
    }

    fn resolve_name(&self, name: &Identifier) -> TypeKind {
        match self.inner.get(name.0) {
            Some(Definition::Typedef(typedef)) => typedef
                .attributes
                .as_ref()
                .and_then(|attrs| {
                    Some(TypeKind::Value {
                        rust_type: find_rust_type_attribute(attrs)?,
                    })
                })
                .unwrap_or_else(|| self.resolve_type(&typedef.type_.type_)),
            Some(Definition::Enum(enum_)) => enum_
                .attributes
                .as_ref()
                .and_then(|attrs| {
                    Some(TypeKind::Value {
                        rust_type: find_rust_type_attribute(attrs)?,
                    })
                })
                .unwrap_or_else(|| panic!("enum {} has no RustType attribute", enum_.identifier.0)),

            Some(Definition::Interface(interface)) => TypeKind::Interface {
                rust_type: find_rust_type(interface),
                optional: false,
            },

            None => panic!("type not found: {:?}", name.0),
            Some(def) => unimplemented!("unsupported definition {def:?}"),
        }
    }

    fn iter<'b>(&'b self) -> btree_map::Iter<'b, String, Definition<'a>> {
        self.inner.iter()
    }
}

fn with_optional<T>(outer: &MayBeNull<T>, inner: syn::Type) -> syn::Type {
    if outer.q_mark.is_some() {
        parse_quote! { Option<#inner> }
    } else {
        inner
    }
}

fn find_rust_type<'a, D: ClassDefinition<'a>>(interface: &D) -> syn::Type {
    interface
        .attributes()
        .and_then(find_rust_type_attribute)
        .unwrap_or_else(|| {
            let id = format_ident!("{}", interface.identifier());
            parse_quote! { #id }
        })
}

fn find_rust_type_attribute(
    attributes: &Bracketed<Punctuated<ExtendedAttribute, Comma>>,
) -> Option<syn::Type> {
    attributes.body.list.iter().find_map(|attr| match attr {
        ExtendedAttribute::Ident(attr) if attr.lhs_identifier.0 == "RustType" => match attr.rhs {
            IdentifierOrString::String(value) => Some(
                parse_str::<syn::Type>(value.0)
                    .expect("failed to parse value of RustType attribute"),
            ),
            IdentifierOrString::Identifier(_) => {
                panic!("unexpected identifier value for attribute RustType, expected string")
            }
        },
        _ => None,
    })
}
