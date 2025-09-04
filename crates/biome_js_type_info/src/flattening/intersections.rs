use biome_rowan::Text;

use crate::{
    Class, Function, Interface, Intersection, Literal, Namespace, Object, Path, ResolvedTypeData,
    ReturnType, TypeData, TypeMember, TypeMemberKind, TypeResolver,
};

pub(super) fn flattened_intersection(
    intersection: &Intersection,
    resolver: &mut dyn TypeResolver,
) -> TypeData {
    match intersection.types() {
        [] => TypeData::NeverKeyword,
        [ty] => resolver
            .resolve_and_get(ty)
            .map_or_else(TypeData::unknown, ResolvedTypeData::to_data),
        types => {
            let merged_ty = resolver
                .resolve_and_get(&types[0])
                .map_or_else(TypeData::unknown, ResolvedTypeData::to_data);
            let mut merged_ty = MergedType::from_type(merged_ty, resolver);
            let mut primitive = merged_ty.as_primitive().cloned();
            for other in &types[1..] {
                if !merged_ty.is_mergeable() {
                    break;
                }

                let other_ty = resolver
                    .resolve_and_get(other)
                    .map_or_else(TypeData::unknown, ResolvedTypeData::to_data);
                let other_ty = MergedType::from_type(other_ty, resolver);
                if let Some(other_primitive) = other_ty.as_primitive() {
                    if primitive.is_some() {
                        return TypeData::NeverKeyword;
                    }

                    primitive = Some(other_primitive.clone());
                } else {
                    merged_ty = merged_ty.intersection_with(other_ty, resolver);
                }
            }

            if let Some(primitive) = primitive {
                primitive
            } else {
                let is_instance = merged_ty.is_instance();
                let ty = merged_ty.into_type();
                if is_instance {
                    TypeData::instance_of(resolver.reference_to_owned_data(ty))
                } else {
                    ty
                }
            }
        }
    }
}

// TODO: We may want explicit support for arrays and tuples too.
enum MergedType {
    Any,
    ClassInstance(Vec<TypeMember>),
    Function(Box<Function>),
    Interface(Vec<TypeMember>),
    Namespace(Vec<TypeMember>),
    Never,
    Object(Vec<TypeMember>),
    Primitive(TypeData),
    Unknown,
}

enum MergedTypeKind {
    Any,
    ClassInstance,
    Function,
    Interface,
    Namespace,
    Never,
    Object,
    Primitive,
    Unknown,
}

impl MergedTypeKind {
    /// Determines the kind of type that should be used when an intersection of
    /// two other types is created.
    fn intersection_with(self, other: Self) -> Self {
        match (self, other) {
            (Self::Any, _) | (_, Self::Any) => Self::Any,
            (Self::ClassInstance, Self::ClassInstance) => Self::ClassInstance,
            (
                Self::ClassInstance,
                Self::Function | Self::Interface | Self::Object | Self::Namespace,
            )
            | (
                Self::Function | Self::Interface | Self::Object | Self::Namespace,
                Self::ClassInstance,
            ) => Self::ClassInstance,
            (Self::Function, Self::Function) => Self::Function,
            (Self::Interface, Self::Interface) => Self::Interface,
            (Self::Interface, Self::Function) | (Self::Function, Self::Interface) => {
                Self::Interface
            }
            (Self::Namespace, Self::Namespace) => Self::Namespace,
            (Self::Namespace, Self::Function | Self::Object | Self::Interface)
            | (Self::Function | Self::Object | Self::Interface, Self::Namespace) => Self::Namespace,
            (Self::Never, some) | (some, Self::Never) => some,
            (Self::Object, Self::Object) => Self::Object,
            (Self::Object, Self::Function | Self::Interface)
            | (Self::Function | Self::Interface, Self::Object) => Self::Object,
            (Self::Primitive, Self::Primitive) => Self::Never,
            (Self::Primitive, _) | (_, Self::Primitive) => Self::Primitive,
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
        }
    }
}

impl MergedType {
    fn from_type(ty: TypeData, resolver: &dyn TypeResolver) -> Self {
        fn from_type_with_static(
            ty: TypeData,
            is_static: bool,
            resolver: &dyn TypeResolver,
        ) -> MergedType {
            let non_static = |member: &TypeMember| !member.is_static();

            // FIXME: We're throwing away info such as prototypes for now...
            match ty {
                TypeData::AnyKeyword | TypeData::Conditional => MergedType::Any,
                TypeData::BigInt
                | TypeData::Boolean
                | TypeData::Null
                | TypeData::Number
                | TypeData::String
                | TypeData::Symbol
                | TypeData::Undefined => MergedType::Primitive(ty),
                TypeData::Class(class) => {
                    if is_static {
                        MergedType::Object(
                            class
                                .members
                                .into_iter()
                                .filter_map(|member| match member.kind {
                                    TypeMemberKind::NamedStatic(name) => Some(TypeMember {
                                        kind: TypeMemberKind::Named(name),
                                        ty: member.ty,
                                    }),
                                    _ => None,
                                })
                                .collect(),
                        )
                    } else {
                        MergedType::ClassInstance(
                            class.members.into_iter().filter(non_static).collect(),
                        )
                    }
                }
                TypeData::Function(function) => MergedType::Function(function),
                TypeData::InstanceOf(instance) => match resolver.resolve_and_get(&instance.ty) {
                    Some(resolved_data) => {
                        if is_static {
                            from_type_with_static(resolved_data.to_data(), false, resolver)
                        } else {
                            MergedType::Unknown
                        }
                    }
                    None => MergedType::Unknown,
                },
                TypeData::Literal(literal) => match *literal {
                    Literal::BigInt(_)
                    | Literal::Boolean(_)
                    | Literal::Number(_)
                    | Literal::String(_)
                    | Literal::Template(_) => MergedType::Primitive(TypeData::Literal(literal)),
                    Literal::Object(object) => MergedType::Object(
                        object
                            .into_members()
                            .into_iter()
                            .filter(non_static)
                            .collect(),
                    ),
                    Literal::RegExp(_) => MergedType::Unknown, // TODO
                },
                TypeData::Interface(interface) => MergedType::Interface(
                    interface.members.into_iter().filter(non_static).collect(),
                ),
                TypeData::Namespace(namespace) => MergedType::Namespace(
                    namespace.members.into_iter().filter(non_static).collect(),
                ),
                TypeData::NeverKeyword => MergedType::Never,
                _ => MergedType::Unknown,
            }
        }

        from_type_with_static(ty, true, resolver)
    }

    fn from_kind_and_members(kind: MergedTypeKind, members: Vec<TypeMember>) -> Self {
        match kind {
            MergedTypeKind::Any => Self::Any,
            MergedTypeKind::ClassInstance => Self::ClassInstance(members),
            MergedTypeKind::Interface => Self::Interface(members),
            MergedTypeKind::Namespace => Self::Namespace(members),
            MergedTypeKind::Never => Self::Never,
            MergedTypeKind::Object => Self::Object(members),
            MergedTypeKind::Unknown | MergedTypeKind::Function | MergedTypeKind::Primitive => {
                Self::Unknown
            }
        }
    }

    fn as_primitive(&self) -> Option<&TypeData> {
        match self {
            Self::Primitive(primitive) => Some(primitive),
            _ => None,
        }
    }

    fn intersection_with(self, other: Self, resolver: &mut dyn TypeResolver) -> Self {
        match (self, other) {
            (Self::Any, _) | (_, Self::Any) => Self::Any,
            // TODO: We need some special handling still if we want to merge
            //       functions as a call signature into interfaces/objects/etc..
            (Self::Function(f1), Self::Function(f2)) => {
                Self::Function(Box::new(function_intersection(*f1, *f2, resolver)))
            }
            (Self::Never, some) | (some, Self::Never) => some,
            (Self::Primitive(_), Self::Primitive(_)) => Self::Never,
            (Self::Primitive(primitive), _) | (_, Self::Primitive(primitive)) => {
                Self::Primitive(primitive)
            }
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
            (ty1, ty2) => {
                let (kind1, members1) = ty1.into_kind_with_members();
                let (kind2, members2) = ty2.into_kind_with_members();

                Self::from_kind_and_members(
                    kind1.intersection_with(kind2),
                    member_intersection(members1, members2, resolver),
                )
            }
        }
    }

    fn into_kind_with_members(self) -> (MergedTypeKind, Vec<TypeMember>) {
        match self {
            Self::Any => (MergedTypeKind::Any, Vec::new()),
            Self::ClassInstance(members) => (MergedTypeKind::ClassInstance, members),
            Self::Function(_) => (MergedTypeKind::Function, Vec::new()),
            Self::Interface(members) => (MergedTypeKind::Interface, members),
            Self::Namespace(members) => (MergedTypeKind::Namespace, members),
            Self::Never => (MergedTypeKind::Never, Vec::new()),
            Self::Object(members) => (MergedTypeKind::Object, members),
            Self::Primitive(_) => (MergedTypeKind::Primitive, Vec::new()),
            Self::Unknown => (MergedTypeKind::Unknown, Vec::new()),
        }
    }

    fn into_type(self) -> TypeData {
        match self {
            Self::Any => TypeData::AnyKeyword,
            Self::ClassInstance(members) => TypeData::from(Class {
                extends: None,
                implements: [].into(),
                members: members.into_iter().collect(),
                name: None,
                type_parameters: [].into(),
            }),
            Self::Function(function) => TypeData::Function(function),
            Self::Interface(members) => TypeData::from(Interface {
                extends: [].into(),
                members: members.into_iter().collect(),
                name: Text::new_static("(merged)"),
                type_parameters: [].into(),
            }),
            Self::Namespace(members) => TypeData::from(Namespace {
                members: members.into_iter().collect(),
                path: Path::from(Text::new_static("")),
            }),
            Self::Never => TypeData::NeverKeyword,
            Self::Object(members) => TypeData::from(Object {
                members: members.into_iter().collect(),
                prototype: None,
            }),
            Self::Primitive(primitive) => primitive,
            Self::Unknown => TypeData::unknown(),
        }
    }

    fn is_instance(&self) -> bool {
        matches!(self, Self::ClassInstance(_))
    }

    fn is_mergeable(&self) -> bool {
        !matches!(self, Self::Any | Self::Never | Self::Unknown)
    }
}

fn function_intersection(f1: Function, f2: Function, resolver: &mut dyn TypeResolver) -> Function {
    Function {
        is_async: false,
        type_parameters: [].into(),
        name: None,
        parameters: [].into(),
        return_type: match (f1.return_type, f2.return_type) {
            (ReturnType::Type(r1), ReturnType::Type(r2)) => ReturnType::Type(
                resolver
                    .register_and_resolve(TypeData::union_of(resolver, [r1, r2].into()))
                    .into(),
            ),
            // TODO: We could be smarter about merging other return types.
            _ => ReturnType::Type(resolver.register_and_resolve(TypeData::Boolean).into()),
        },
    }
}

fn member_intersection(
    members1: Vec<TypeMember>,
    members2: Vec<TypeMember>,
    resolver: &mut dyn TypeResolver,
) -> Vec<TypeMember> {
    let mut merged = members1;
    for member in members2 {
        match merged.iter_mut().find(|merged_member| {
            member
                .name()
                .is_some_and(|name| merged_member.has_name(&name))
        }) {
            Some(merged_member) => {
                if member.ty != merged_member.ty {
                    let ty = std::mem::take(&mut merged_member.ty);
                    merged_member.ty = resolver
                        .register_and_resolve(TypeData::union_of(resolver, [member.ty, ty].into()))
                        .into()
                }
            }
            None => {
                merged.push(member);
            }
        }
    }

    merged
}
