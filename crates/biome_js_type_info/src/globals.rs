//! Hardcoded global definitions.

// FIXME: Implement inference from type definitions.

use std::{borrow::Cow, sync::LazyLock};

use biome_js_syntax::AnyJsExpression;
use biome_rowan::Text;

use crate::{
    Class, Function, GenericTypeParameter, Literal, Resolvable, ResolvedTypeData, ResolvedTypeId,
    ReturnType, ScopeId, TypeData, TypeId, TypeMember, TypeMemberKind, TypeReference,
    TypeReferenceQualifier, TypeResolver, TypeResolverLevel,
};

const GLOBAL_LEVEL: TypeResolverLevel = TypeResolverLevel::Global;

pub static GLOBAL_RESOLVER: LazyLock<GlobalsResolver> = LazyLock::new(GlobalsResolver::default);

pub static GLOBAL_TYPE_MEMBERS: LazyLock<Vec<TypeMember>> = LazyLock::new(|| {
    (0..NUM_PREDEFINED_TYPES)
        .map(TypeId::new)
        .map(|id| TypeMember {
            kind: TypeMemberKind::Named(Text::Static(global_type_name(id))),
            is_static: false,
            ty: ResolvedTypeId::new(GLOBAL_LEVEL, id).into(),
        })
        .collect()
});

pub const UNKNOWN_ID: TypeId = TypeId::new(0);
pub const UNDEFINED_ID: TypeId = TypeId::new(1);
pub const ARRAY_ID: TypeId = TypeId::new(2);
pub const GLOBAL_ID: TypeId = TypeId::new(3);
pub const INSTANCEOF_PROMISE_ID: TypeId = TypeId::new(4);
pub const NUMBER_ID: TypeId = TypeId::new(5);
pub const PROMISE_ID: TypeId = TypeId::new(6);
pub const PROMISE_CATCH_ID: TypeId = TypeId::new(7);
pub const PROMISE_FINALLY_ID: TypeId = TypeId::new(8);
pub const PROMISE_THEN_ID: TypeId = TypeId::new(9);
pub const PROMISE_ALL_ID: TypeId = TypeId::new(10);
pub const PROMISE_ALL_SETTLED_ID: TypeId = TypeId::new(11);
pub const PROMISE_ANY_ID: TypeId = TypeId::new(12);
pub const PROMISE_RACE_ID: TypeId = TypeId::new(13);
pub const PROMISE_REJECT_ID: TypeId = TypeId::new(14);
pub const PROMISE_RESOLVE_ID: TypeId = TypeId::new(15);
pub const PROMISE_TRY_ID: TypeId = TypeId::new(16);
pub const BIGINT_STRING_LITERAL_ID: TypeId = TypeId::new(17);
pub const BOOLEAN_STRING_LITERAL_ID: TypeId = TypeId::new(18);
pub const FUNCTION_STRING_LITERAL_ID: TypeId = TypeId::new(19);
pub const NUMBER_STRING_LITERAL_ID: TypeId = TypeId::new(20);
pub const OBJECT_STRING_LITERAL_ID: TypeId = TypeId::new(21);
pub const STRING_STRING_LITERAL_ID: TypeId = TypeId::new(22);
pub const SYMBOL_STRING_LITERAL_ID: TypeId = TypeId::new(23);
pub const UNDEFINED_STRING_LITERAL_ID: TypeId = TypeId::new(24);
pub const TYPEOF_OPERATOR_RETURN_UNION_ID: TypeId = TypeId::new(25);
pub const STRING_ID: TypeId = TypeId::new(26);
pub const T_ID: TypeId = TypeId::new(27);
pub const NUM_PREDEFINED_TYPES: usize = 28; // Most be one more than the highest `TypeId` above.

pub const GLOBAL_UNKNOWN_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, UNKNOWN_ID);
pub const GLOBAL_UNDEFINED_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, UNDEFINED_ID);
pub const GLOBAL_ARRAY_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, ARRAY_ID);
pub const GLOBAL_GLOBAL_ID /* :smirk: */: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, GLOBAL_ID);
pub const GLOBAL_INSTANCEOF_PROMISE_ID: ResolvedTypeId =
    ResolvedTypeId::new(GLOBAL_LEVEL, INSTANCEOF_PROMISE_ID);
pub const GLOBAL_NUMBER_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, NUMBER_ID);
pub const GLOBAL_PROMISE_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, PROMISE_ID);
pub const GLOBAL_BIGINT_STRING_LITERAL_ID: ResolvedTypeId =
    ResolvedTypeId::new(GLOBAL_LEVEL, BIGINT_STRING_LITERAL_ID);
pub const GLOBAL_BOOLEAN_STRING_LITERAL_ID: ResolvedTypeId =
    ResolvedTypeId::new(GLOBAL_LEVEL, BOOLEAN_STRING_LITERAL_ID);
pub const GLOBAL_FUNCTION_STRING_LITERAL_ID: ResolvedTypeId =
    ResolvedTypeId::new(GLOBAL_LEVEL, FUNCTION_STRING_LITERAL_ID);
pub const GLOBAL_NUMBER_STRING_LITERAL_ID: ResolvedTypeId =
    ResolvedTypeId::new(GLOBAL_LEVEL, NUMBER_STRING_LITERAL_ID);
pub const GLOBAL_OBJECT_STRING_LITERAL_ID: ResolvedTypeId =
    ResolvedTypeId::new(GLOBAL_LEVEL, OBJECT_STRING_LITERAL_ID);
pub const GLOBAL_STRING_STRING_LITERAL_ID: ResolvedTypeId =
    ResolvedTypeId::new(GLOBAL_LEVEL, STRING_STRING_LITERAL_ID);
pub const GLOBAL_SYMBOL_STRING_LITERAL_ID: ResolvedTypeId =
    ResolvedTypeId::new(GLOBAL_LEVEL, SYMBOL_STRING_LITERAL_ID);
pub const GLOBAL_UNDEFINED_STRING_LITERAL_ID: ResolvedTypeId =
    ResolvedTypeId::new(GLOBAL_LEVEL, UNDEFINED_STRING_LITERAL_ID);
pub const GLOBAL_TYPEOF_OPERATOR_RETURN_UNION_ID: ResolvedTypeId =
    ResolvedTypeId::new(GLOBAL_LEVEL, TYPEOF_OPERATOR_RETURN_UNION_ID);
pub const GLOBAL_STRING_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, STRING_ID);
pub const GLOBAL_T_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, T_ID);

/// Returns a string for formatting global IDs in test snapshots.
pub fn global_type_name(id: TypeId) -> &'static str {
    match id.index() {
        0 => "unknown",
        1 => "undefined",
        2 => "Array",
        3 => "globalThis",
        4 => "instanceof Promise",
        5 => "number",
        6 => "Promise",
        7 => "Promise.prototype.catch",
        8 => "Promise.prototype.finally",
        9 => "Promise.prototype.then",
        10 => "Promise.all",
        11 => "Promise.allSettled",
        12 => "Promise.any",
        13 => "Promise.race",
        14 => "Promise.reject",
        15 => "Promise.resolve",
        16 => "Promise.try",
        17 => "\"bigint\"",
        18 => "\"boolean\"",
        19 => "\"function\"",
        20 => "\"number\"",
        21 => "\"object\"",
        22 => "\"string\"",
        23 => "\"symbol\"",
        24 => "\"undefined\"",
        25 => {
            "\"bigint\" | \"boolean\" | \"function\" | \"number\" | \"object\" \
                | \"string\" | \"symbol\" | \"undefined\""
        }
        26 => "string",
        27 => "T",
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
        let promise_method = |name: &'static str, id: TypeId| TypeMember {
            kind: TypeMemberKind::Named(Text::Static(name)),
            is_static: false,
            ty: ResolvedTypeId::new(TypeResolverLevel::Global, id).into(),
        };

        let static_promise_method = |name: &'static str, id: TypeId| TypeMember {
            kind: TypeMemberKind::Named(Text::Static(name)),
            is_static: true,
            ty: ResolvedTypeId::new(TypeResolverLevel::Global, id).into(),
        };

        let promise_method_definition = |id: TypeId| {
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::Static(global_type_name(id))),
                parameters: Default::default(),
                return_type: ReturnType::Type(GLOBAL_INSTANCEOF_PROMISE_ID.into()),
            })
        };

        let string_literal = |value: &'static str| -> TypeData {
            TypeData::from(Literal::String(Text::Static(value).into()))
        };

        let types = vec![
            TypeData::Unknown,
            TypeData::Undefined,
            TypeData::Class(Box::new(Class {
                name: Some(Text::Static("Array")),
                type_parameters: Box::new([TypeReference::from(GLOBAL_T_ID)]),
                extends: None,
                implements: [].into(),
                members: Box::new([TypeMember {
                    kind: TypeMemberKind::Named(Text::Static("length")),
                    is_static: false,
                    ty: GLOBAL_NUMBER_ID.into(),
                }]),
            })),
            TypeData::Global,
            TypeData::instance_of(TypeReference::from(GLOBAL_PROMISE_ID)),
            TypeData::Number,
            TypeData::Class(Box::new(Class {
                name: Some(Text::Static("Promise")),
                type_parameters: Box::new([TypeReference::from(GLOBAL_T_ID)]),
                extends: None,
                implements: [].into(),
                members: Box::new([
                    promise_method("catch", PROMISE_CATCH_ID),
                    promise_method("finally", PROMISE_FINALLY_ID),
                    promise_method("then", PROMISE_THEN_ID),
                    static_promise_method("all", PROMISE_ALL_ID),
                    static_promise_method("allSettled", PROMISE_ALL_SETTLED_ID),
                    static_promise_method("any", PROMISE_ANY_ID),
                    static_promise_method("race", PROMISE_RACE_ID),
                    static_promise_method("reject", PROMISE_REJECT_ID),
                    static_promise_method("resolve", PROMISE_RESOLVE_ID),
                    static_promise_method("try", PROMISE_TRY_ID),
                ]),
            })),
            promise_method_definition(PROMISE_CATCH_ID),
            promise_method_definition(PROMISE_FINALLY_ID),
            promise_method_definition(PROMISE_THEN_ID),
            promise_method_definition(PROMISE_ALL_ID),
            promise_method_definition(PROMISE_ALL_SETTLED_ID),
            promise_method_definition(PROMISE_ANY_ID),
            promise_method_definition(PROMISE_RACE_ID),
            promise_method_definition(PROMISE_REJECT_ID),
            promise_method_definition(PROMISE_RESOLVE_ID),
            promise_method_definition(PROMISE_TRY_ID),
            string_literal("bigint"),
            string_literal("boolean"),
            string_literal("function"),
            string_literal("number"),
            string_literal("object"),
            string_literal("string"),
            string_literal("symbol"),
            string_literal("undefined"),
            TypeData::union_of(vec![
                GLOBAL_BIGINT_STRING_LITERAL_ID.into(),
                GLOBAL_BOOLEAN_STRING_LITERAL_ID.into(),
                GLOBAL_FUNCTION_STRING_LITERAL_ID.into(),
                GLOBAL_NUMBER_STRING_LITERAL_ID.into(),
                GLOBAL_OBJECT_STRING_LITERAL_ID.into(),
                GLOBAL_STRING_STRING_LITERAL_ID.into(),
                GLOBAL_SYMBOL_STRING_LITERAL_ID.into(),
                GLOBAL_UNDEFINED_STRING_LITERAL_ID.into(),
            ]),
            TypeData::String,
            TypeData::from(GenericTypeParameter {
                name: Text::Static("T"),
                constraint: TypeReference::Unknown,
                default: TypeReference::Unknown,
            }),
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

    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<ResolvedTypeData> {
        (id.level() == GLOBAL_LEVEL).then(|| (id, self.get_by_id(id.id())).into())
    }

    fn register_type(&mut self, type_data: Cow<TypeData>) -> TypeId {
        // Searching linearly may potentially become quite expensive, but it
        // should be outweighed by index lookups quite heavily.
        match self
            .types
            .iter()
            .position(|data| data == type_data.as_ref())
        {
            Some(index) => TypeId::new(index),
            None => {
                let id = TypeId::new(self.types.len());
                self.types.push(type_data.into_owned());
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
        } else if !qualifier.type_only && qualifier.path.len() == 1 {
            self.resolve_type_of(&qualifier.path[0], qualifier.scope_id)
        } else {
            None
        }
    }

    fn resolve_type_of(&self, identifier: &Text, _scope_id: ScopeId) -> Option<ResolvedTypeId> {
        match identifier.text() {
            "globalThis" | "window" => Some(GLOBAL_GLOBAL_ID),
            _ => None,
        }
    }

    fn resolve_expression(&mut self, scope_id: ScopeId, expr: &AnyJsExpression) -> Cow<TypeData> {
        Cow::Owned(TypeData::from_any_js_expression(self, scope_id, expr))
    }

    fn registered_types(&self) -> &[TypeData] {
        &self.types[NUM_PREDEFINED_TYPES..]
    }
}
