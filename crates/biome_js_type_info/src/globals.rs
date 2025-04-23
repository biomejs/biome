//! Hardcoded global definitions.

// FIXME: Implement inference from type definitions.

use std::sync::{Arc, LazyLock};

use biome_rowan::Text;

use crate::{
    Class, GenericTypeParameter, MethodTypeMember, Object, PropertyTypeMember, Type, TypeId,
    TypeInner, TypeMember, TypeReference, TypeReferenceQualifier,
};

pub(crate) static ARRAY_TYPE: LazyLock<Type> =
    LazyLock::new(|| TypeInner::Class(Box::new(ARRAY.clone())).into());

pub(crate) static ARRAY: LazyLock<Class> = LazyLock::new(|| {
    // TODO: Use generics to propagate return value of function argument as
    //       return value of the then/catch handlers.
    Class {
        id: TypeId::new(),
        name: Some(Text::Static("Array")),
        type_parameters: Arc::new([GenericTypeParameter {
            name: Text::Static("T"),
            ty: Type::unknown(),
        }]),
        extends: None,
        members: Arc::new([TypeMember::Property(
            PropertyTypeMember::default()
                .with_name(Text::Static("length"))
                .with_type(Type::number()),
        )]),
    }
});

pub(crate) static PROMISE_TYPE: LazyLock<Type> =
    LazyLock::new(|| TypeInner::Class(Box::new(PROMISE.clone())).into());

pub(crate) static PROMISE: LazyLock<Class> = LazyLock::new(|| {
    // Don't use `Type::promise_of()` or we'll create a deadlock trying to
    // acquire `PROMISE` itself.
    fn promise_of(ty: Type) -> Type {
        TypeInner::Reference(Box::new(TypeReference {
            qualifier: TypeReferenceQualifier::from_name(Text::Static("Promise")),
            ty: Type::unknown(),
            type_parameters: Arc::new([ty]),
        }))
        .into()
    }

    // TODO: Use generics to propagate return value of function argument as
    //       return value of the then/catch handlers.
    Class {
        id: TypeId::new(),
        name: Some(Text::Static("Promise")),
        type_parameters: Arc::new([GenericTypeParameter {
            name: Text::Static("T"),
            ty: Type::unknown(),
        }]),
        extends: None,
        members: Arc::new([
            TypeMember::Method(
                MethodTypeMember::default()
                    .with_name(Text::Static("all"))
                    .with_static()
                    .with_return_type(promise_of(Type::unknown())),
            ),
            TypeMember::Method(
                MethodTypeMember::default()
                    .with_name(Text::Static("allSettled"))
                    .with_static()
                    .with_return_type(promise_of(Type::unknown())),
            ),
            TypeMember::Method(
                MethodTypeMember::default()
                    .with_name(Text::Static("any"))
                    .with_static()
                    .with_return_type(promise_of(Type::unknown())),
            ),
            TypeMember::Method(
                MethodTypeMember::default()
                    .with_name(Text::Static("race"))
                    .with_static()
                    .with_return_type(promise_of(Type::unknown())),
            ),
            TypeMember::Method(
                MethodTypeMember::default()
                    .with_name(Text::Static("reject"))
                    .with_static()
                    .with_return_type(promise_of(Type::unknown())),
            ),
            TypeMember::Method(
                MethodTypeMember::default()
                    .with_name(Text::Static("resolve"))
                    .with_static()
                    .with_return_type(promise_of(Type::unknown())),
            ),
            TypeMember::Method(
                MethodTypeMember::default()
                    .with_name(Text::Static("try"))
                    .with_static()
                    .with_return_type(promise_of(Type::unknown())),
            ),
            TypeMember::Method(
                MethodTypeMember::default()
                    .with_name(Text::Static("catch"))
                    .with_return_type(promise_of(Type::unknown())),
            ),
            TypeMember::Method(
                MethodTypeMember::default()
                    .with_name(Text::Static("finally"))
                    .with_return_type(promise_of(Type::unknown())),
            ),
            TypeMember::Method(
                MethodTypeMember::default()
                    .with_name(Text::Static("then"))
                    .with_return_type(promise_of(Type::unknown())),
            ),
        ]),
    }
});

pub(crate) static WINDOW_TYPE: LazyLock<Type> =
    LazyLock::new(|| TypeInner::Object(Box::new(WINDOW.clone())).into());

pub(crate) static WINDOW: LazyLock<Object> = LazyLock::new(|| Object {
    prototype: None,
    members: Arc::new([TypeMember::Property(PropertyTypeMember {
        name: Text::Static("Promise"),
        ty: TypeInner::TypeofType(Box::new(
            TypeInner::Reference(Box::new(TypeReference {
                qualifier: TypeReferenceQualifier::from_name(Text::Static("Promise")),
                ty: Type::unknown(),
                type_parameters: Arc::new([]),
            }))
            .into(),
        ))
        .into(),
        is_optional: false,
        is_static: false,
    })]),
});
