//! Hardcoded global definitions.

// FIXME: Implement inference from type definitions.

use std::sync::LazyLock;

use biome_rowan::Text;

use crate::{
    Class, GenericTypeParameter, MethodTypeMember, PropertyTypeMember, Resolvable, ResolvedTypeId,
    TypeData, TypeId, TypeMember, TypeReference, TypeReferenceQualifier, TypeResolver,
    TypeResolverLevel,
};

const GLOBAL_LEVEL: TypeResolverLevel = TypeResolverLevel::Global;

pub static GLOBAL_RESOLVER: LazyLock<GlobalsResolver> = LazyLock::new(GlobalsResolver::default);

pub static GLOBAL_TYPE_MEMBERS: LazyLock<Vec<TypeMember>> = LazyLock::new(|| {
    (0..NUM_PREDEFINED_TYPES)
        .map(TypeId::new)
        .map(|id| {
            TypeMember::Property(PropertyTypeMember {
                name: Text::Static(global_type_name(id)),
                ty: ResolvedTypeId::new(GLOBAL_LEVEL, id).into(),
                is_optional: false,
                is_static: false,
            })
        })
        .collect()
});

pub const UNKNOWN_ID: TypeId = TypeId::new(0);
pub const ARRAY_ID: TypeId = TypeId::new(1);
pub const GLOBAL_ID: TypeId = TypeId::new(2);
pub const INSTANCEOF_PROMISE_ID: TypeId = TypeId::new(3);
pub const NUMBER_ID: TypeId = TypeId::new(4);
pub const PROMISE_ID: TypeId = TypeId::new(5);
pub const UNDEFINED_ID: TypeId = TypeId::new(6);
pub const NUM_PREDEFINED_TYPES: usize = 7; // Most be one more than the highest `TypeId` above.

pub const GLOBAL_UNKNOWN_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, UNKNOWN_ID);
pub const GLOBAL_ARRAY_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, ARRAY_ID);
pub const GLOBAL_GLOBAL_ID /* :smirk: */: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, GLOBAL_ID);
pub const GLOBAL_INSTANCEOF_PROMISE_ID: ResolvedTypeId =
    ResolvedTypeId::new(GLOBAL_LEVEL, INSTANCEOF_PROMISE_ID);
pub const GLOBAL_NUMBER_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, NUMBER_ID);
pub const GLOBAL_PROMISE_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, PROMISE_ID);
pub const GLOBAL_UNDEFINED_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, UNDEFINED_ID);

/// Returns a string for formatting global IDs in test snapshots.
pub fn global_type_name(id: TypeId) -> &'static str {
    match id.index() {
        0 => "unknown",
        1 => "Array",
        2 => "globalThis",
        3 => "instanceof Promise",
        4 => "number",
        5 => "Promise",
        6 => "undefined",
        _ => "inferred type",
    }
}

/// Resolver that is limited to resolving symbols in the global scope.
///
/// This resolver does not check whether qualifiers that are being resolved have
/// been shadowed by local declarations, so it should generally only be used
/// after all other resolvers have failed.
#[derive(Clone)]
pub struct GlobalsResolver {
    types: Vec<TypeData>,
}

impl Default for GlobalsResolver {
    fn default() -> Self {
        let promise_method = |name: &'static str| {
            TypeMember::Method(
                MethodTypeMember::default()
                    .with_name(Text::Static(name))
                    .with_return_type(GLOBAL_INSTANCEOF_PROMISE_ID.into()),
            )
        };

        let static_promise_method = |name: &'static str| {
            TypeMember::Method(
                MethodTypeMember::default()
                    .with_name(Text::Static(name))
                    .with_static()
                    .with_return_type(GLOBAL_INSTANCEOF_PROMISE_ID.into()),
            )
        };

        let types = vec![
            TypeData::Unknown,
            TypeData::Class(Box::new(Class {
                name: Some(Text::Static("Array")),
                type_parameters: Box::new([GenericTypeParameter {
                    name: Text::Static("T"),
                    ty: TypeReference::Unknown,
                }]),
                extends: None,
                members: Box::new([TypeMember::Property(
                    PropertyTypeMember::default()
                        .with_name(Text::Static("length"))
                        .with_type(GLOBAL_NUMBER_ID.into()),
                )]),
            })),
            TypeData::Global,
            TypeData::instance_of(TypeReference::from(GLOBAL_PROMISE_ID)),
            TypeData::Number,
            TypeData::Class(Box::new(Class {
                name: Some(Text::Static("Promise")),
                type_parameters: Box::new([GenericTypeParameter {
                    name: Text::Static("T"),
                    ty: TypeReference::Unknown,
                }]),
                extends: None,
                members: Box::new([
                    promise_method("catch"),
                    promise_method("finally"),
                    promise_method("then"),
                    static_promise_method("all"),
                    static_promise_method("allSettled"),
                    static_promise_method("any"),
                    static_promise_method("race"),
                    static_promise_method("reject"),
                    static_promise_method("resolve"),
                    static_promise_method("try"),
                ]),
            })),
            TypeData::Undefined,
        ];

        Self { types }
    }
}

impl GlobalsResolver {
    pub fn run_inference(&mut self) {
        self.resolve_all();
        self.flatten_all();
    }

    pub fn resolve_all(&mut self) {
        let mut i = NUM_PREDEFINED_TYPES;
        while i < self.types.len() {
            // First take the type to satisfy the borrow checker:
            let ty = std::mem::take(&mut self.types[i]);
            self.types[i] = ty.resolved(self);
            i += 1;
        }
    }

    fn flatten_all(&mut self) {
        let mut i = NUM_PREDEFINED_TYPES;
        while i < self.types.len() {
            // First take the type to satisfy the borrow checker:
            let ty = std::mem::take(&mut self.types[i]);
            self.types[i] = ty.flattened(self);
            i += 1;
        }
    }
}

impl TypeResolver for GlobalsResolver {
    fn level(&self) -> TypeResolverLevel {
        GLOBAL_LEVEL
    }

    fn find_type(&self, type_data: &TypeData) -> Option<TypeId> {
        self.types
            .iter()
            .position(|data| data == type_data)
            .map(TypeId::new)
    }

    fn get_by_id(&self, id: TypeId) -> &TypeData {
        &self.types[id.index()]
    }

    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<&TypeData> {
        (id.level() == GLOBAL_LEVEL).then(|| self.get_by_id(id.id()))
    }

    fn register_type(&mut self, type_data: TypeData) -> TypeId {
        // Searching linearly may potentially become quite expensive, but it
        // should be outweighed by index lookups quite heavily.
        match self.types.iter().position(|data| data == &type_data) {
            Some(index) => TypeId::new(index),
            None => {
                let id = TypeId::new(self.types.len());
                self.types.push(type_data);
                id
            }
        }
    }

    fn resolve_reference(&self, ty: &TypeReference) -> Option<ResolvedTypeId> {
        match ty {
            TypeReference::Qualifier(qualifier) => self.resolve_qualifier(qualifier),
            TypeReference::Resolved(resolved_id) => {
                (resolved_id.level() == GLOBAL_LEVEL).then_some(*resolved_id)
            }
            TypeReference::Import(_) => None,
            TypeReference::Unknown => Some(GLOBAL_UNKNOWN_ID),
        }
    }

    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<ResolvedTypeId> {
        if qualifier.is_array() && !qualifier.has_known_type_parameters() {
            Some(GLOBAL_ARRAY_ID)
        } else if qualifier.is_promise() && !qualifier.has_known_type_parameters() {
            Some(GLOBAL_PROMISE_ID)
        } else {
            None
        }
    }

    fn resolve_type_of(&self, identifier: &Text) -> Option<ResolvedTypeId> {
        match identifier.text() {
            "globalThis" | "window" => Some(GLOBAL_GLOBAL_ID),
            _ => None,
        }
    }

    fn registered_types(&self) -> &[TypeData] {
        &self.types[NUM_PREDEFINED_TYPES..]
    }
}
