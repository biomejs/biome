//! Hardcoded global definitions.

// FIXME: Implement inference from type definitions.

use std::{
    borrow::Cow,
    sync::{Arc, LazyLock},
};

use biome_js_syntax::AnyJsExpression;
use biome_rowan::Text;

use crate::{
    Class, Function, FunctionParameter, GenericTypeParameter, Literal, PatternFunctionParameter,
    Resolvable, ResolvedTypeData, ResolvedTypeId, ResolverId, ReturnType, ScopeId, TypeData,
    TypeId, TypeInstance, TypeMember, TypeMemberKind, TypeReference, TypeReferenceQualifier,
    TypeResolver, TypeResolverLevel, TypeStore, Union, flattening::MAX_FLATTEN_DEPTH,
};

pub(super) const GLOBAL_LEVEL: TypeResolverLevel = TypeResolverLevel::Global;
pub(super) const GLOBAL_RESOLVER_ID: ResolverId = ResolverId::from_level(GLOBAL_LEVEL);

pub static GLOBAL_RESOLVER: LazyLock<Arc<GlobalsResolver>> =
    LazyLock::new(|| Arc::new(GlobalsResolver::default()));

pub static GLOBAL_TYPE_MEMBERS: LazyLock<Vec<TypeMember>> = LazyLock::new(|| {
    (0..NUM_PREDEFINED_TYPES)
        .map(TypeId::new)
        .map(|id| TypeMember {
            kind: TypeMemberKind::Named(Text::new_static(global_type_name(id))),
            ty: ResolvedTypeId::new(GLOBAL_LEVEL, id).into(),
        })
        .collect()
});

pub const UNKNOWN_ID: TypeId = TypeId::new(0);
pub const UNDEFINED_ID: TypeId = TypeId::new(1);
pub const VOID_ID: TypeId = TypeId::new(2);
pub const CONDITIONAL_ID: TypeId = TypeId::new(3);
pub const NUMBER_ID: TypeId = TypeId::new(4);
pub const STRING_ID: TypeId = TypeId::new(5);
pub const INSTANCEOF_ARRAY_T_ID: TypeId = TypeId::new(6);
pub const INSTANCEOF_ARRAY_U_ID: TypeId = TypeId::new(7);
pub const ARRAY_ID: TypeId = TypeId::new(8);
pub const ARRAY_FILTER_ID: TypeId = TypeId::new(9);
pub const ARRAY_FOREACH_ID: TypeId = TypeId::new(10);
pub const ARRAY_MAP_ID: TypeId = TypeId::new(11);
pub const GLOBAL_ID: TypeId = TypeId::new(12);
pub const INSTANCEOF_PROMISE_ID: TypeId = TypeId::new(13);
pub const PROMISE_ID: TypeId = TypeId::new(14);
pub const PROMISE_CONSTRUCTOR_ID: TypeId = TypeId::new(15);
pub const PROMISE_CATCH_ID: TypeId = TypeId::new(16);
pub const PROMISE_FINALLY_ID: TypeId = TypeId::new(17);
pub const PROMISE_THEN_ID: TypeId = TypeId::new(18);
pub const PROMISE_ALL_ID: TypeId = TypeId::new(19);
pub const PROMISE_ALL_SETTLED_ID: TypeId = TypeId::new(20);
pub const PROMISE_ANY_ID: TypeId = TypeId::new(21);
pub const PROMISE_RACE_ID: TypeId = TypeId::new(22);
pub const PROMISE_REJECT_ID: TypeId = TypeId::new(23);
pub const PROMISE_RESOLVE_ID: TypeId = TypeId::new(24);
pub const PROMISE_TRY_ID: TypeId = TypeId::new(25);
pub const BIGINT_STRING_LITERAL_ID: TypeId = TypeId::new(26);
pub const BOOLEAN_STRING_LITERAL_ID: TypeId = TypeId::new(27);
pub const FUNCTION_STRING_LITERAL_ID: TypeId = TypeId::new(28);
pub const NUMBER_STRING_LITERAL_ID: TypeId = TypeId::new(29);
pub const OBJECT_STRING_LITERAL_ID: TypeId = TypeId::new(30);
pub const STRING_STRING_LITERAL_ID: TypeId = TypeId::new(31);
pub const SYMBOL_STRING_LITERAL_ID: TypeId = TypeId::new(32);
pub const UNDEFINED_STRING_LITERAL_ID: TypeId = TypeId::new(33);
pub const TYPEOF_OPERATOR_RETURN_UNION_ID: TypeId = TypeId::new(34);
pub const T_ID: TypeId = TypeId::new(35);
pub const U_ID: TypeId = TypeId::new(36);
pub const CONDITIONAL_CALLBACK_ID: TypeId = TypeId::new(37);
pub const MAP_CALLBACK_ID: TypeId = TypeId::new(38);
pub const VOID_CALLBACK_ID: TypeId = TypeId::new(39);
pub const FETCH_ID: TypeId = TypeId::new(40);
pub const NUM_PREDEFINED_TYPES: usize = 41; // Must be one more than the highest `TypeId` above.

pub const GLOBAL_UNKNOWN_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, UNKNOWN_ID);
pub const GLOBAL_UNDEFINED_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, UNDEFINED_ID);
pub const GLOBAL_VOID_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, VOID_ID);
pub const GLOBAL_CONDITIONAL_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, CONDITIONAL_ID);
pub const GLOBAL_NUMBER_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, NUMBER_ID);
pub const GLOBAL_STRING_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, STRING_ID);
pub const GLOBAL_ARRAY_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, ARRAY_ID);
pub const GLOBAL_GLOBAL_ID /* :smirk: */: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, GLOBAL_ID);
pub const GLOBAL_INSTANCEOF_PROMISE_ID: ResolvedTypeId =
    ResolvedTypeId::new(GLOBAL_LEVEL, INSTANCEOF_PROMISE_ID);
pub const GLOBAL_PROMISE_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, PROMISE_ID);
pub const GLOBAL_PROMISE_CONSTRUCTOR_ID: ResolvedTypeId =
    ResolvedTypeId::new(GLOBAL_LEVEL, PROMISE_CONSTRUCTOR_ID);
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
pub const GLOBAL_T_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, T_ID);
pub const GLOBAL_U_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, U_ID);
pub const GLOBAL_FETCH_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, FETCH_ID);

/// Returns a string for formatting global IDs in test snapshots.
pub fn global_type_name(id: TypeId) -> &'static str {
    match id.index() {
        0 => "unknown",
        1 => "undefined",
        2 => "void",
        3 => "conditional",
        4 => "number",
        5 => "string",
        6 => "instanceof Array<T>",
        7 => "instanceof Array<U>",
        8 => "Array",
        9 => "Array.prototype.filter",
        10 => "Array.prototype.forEach",
        11 => "Array.prototype.map",
        12 => "globalThis",
        13 => "instanceof Promise",
        14 => "Promise",
        15 => "Promise.constructor",
        16 => "Promise.prototype.catch",
        17 => "Promise.prototype.finally",
        18 => "Promise.prototype.then",
        19 => "Promise.all",
        20 => "Promise.allSettled",
        21 => "Promise.any",
        22 => "Promise.race",
        23 => "Promise.reject",
        24 => "Promise.resolve",
        25 => "Promise.try",
        26 => "\"bigint\"",
        27 => "\"boolean\"",
        28 => "\"function\"",
        29 => "\"number\"",
        30 => "\"object\"",
        31 => "\"string\"",
        32 => "\"symbol\"",
        33 => "\"undefined\"",
        34 => {
            "\"bigint\" | \"boolean\" | \"function\" | \"number\" | \"object\" \
                | \"string\" | \"symbol\" | \"undefined\""
        }
        35 => "T",
        36 => "U",
        37 => "() => conditional",
        38 => "<U>(item: T) => U",
        39 => "() => void",
        40 => "fetch",
        _ => "inferred type",
    }
}

/// Resolver that is limited to resolving symbols in the global scope.
///
/// This resolver does not check whether qualifiers that are being resolved have
/// been shadowed by local declarations, so it should generally only be used
/// after all other resolvers have failed.
pub struct GlobalsResolver {
    types: TypeStore,
}

impl Default for GlobalsResolver {
    fn default() -> Self {
        let method = |name: &'static str, id: TypeId| TypeMember {
            kind: TypeMemberKind::Named(Text::new_static(name)),
            ty: ResolvedTypeId::new(TypeResolverLevel::Global, id).into(),
        };

        let static_method = |name: &'static str, id: TypeId| TypeMember {
            kind: TypeMemberKind::NamedStatic(Text::new_static(name)),
            ty: ResolvedTypeId::new(TypeResolverLevel::Global, id).into(),
        };

        let array_method_definition =
            |id: TypeId,
             param_type_id: TypeId,
             return_type_id: TypeId,
             type_parameters: Box<[TypeReference]>| {
                TypeData::from(Function {
                    is_async: false,
                    type_parameters,
                    name: Some(Text::new_static(global_type_name(id))),
                    parameters: [FunctionParameter::Pattern(PatternFunctionParameter {
                        bindings: Default::default(),
                        is_optional: false,
                        is_rest: false,
                        ty: ResolvedTypeId::new(TypeResolverLevel::Global, param_type_id).into(),
                    })]
                    .into(),
                    return_type: ReturnType::Type(
                        ResolvedTypeId::new(TypeResolverLevel::Global, return_type_id).into(),
                    ),
                })
            };

        let promise_method_definition = |id: TypeId| {
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(global_type_name(id))),
                parameters: Default::default(),
                return_type: ReturnType::Type(GLOBAL_INSTANCEOF_PROMISE_ID.into()),
            })
        };

        let string_literal = |value: &'static str| -> TypeData {
            TypeData::from(Literal::String(Text::new_static(value).into()))
        };

        let types = vec![
            TypeData::Unknown,
            TypeData::Undefined,
            TypeData::VoidKeyword,
            TypeData::Conditional,
            TypeData::Number,
            TypeData::String,
            TypeData::instance_of(TypeReference::from(GLOBAL_ARRAY_ID)),
            TypeData::instance_of(TypeInstance {
                ty: TypeReference::from(GLOBAL_ARRAY_ID),
                type_parameters: [GLOBAL_U_ID.into()].into(),
            }),
            TypeData::Class(Box::new(Class {
                name: Some(Text::new_static("Array")),
                type_parameters: Box::new([TypeReference::from(GLOBAL_T_ID)]),
                extends: None,
                implements: [].into(),
                members: Box::new([
                    method("filter", ARRAY_FILTER_ID),
                    method("forEach", ARRAY_FOREACH_ID),
                    method("map", ARRAY_MAP_ID),
                    TypeMember {
                        kind: TypeMemberKind::Named(Text::new_static("length")),
                        ty: GLOBAL_NUMBER_ID.into(),
                    },
                ]),
            })),
            array_method_definition(
                ARRAY_FILTER_ID,
                CONDITIONAL_CALLBACK_ID,
                INSTANCEOF_ARRAY_T_ID,
                Default::default(),
            ),
            array_method_definition(
                ARRAY_FOREACH_ID,
                VOID_CALLBACK_ID,
                VOID_ID,
                Default::default(),
            ),
            array_method_definition(
                ARRAY_MAP_ID,
                MAP_CALLBACK_ID,
                INSTANCEOF_ARRAY_U_ID,
                [GLOBAL_U_ID.into()].into(),
            ),
            TypeData::Global,
            TypeData::instance_of(TypeReference::from(GLOBAL_PROMISE_ID)),
            TypeData::Class(Box::new(Class {
                name: Some(Text::new_static("Promise")),
                type_parameters: Box::new([TypeReference::from(GLOBAL_T_ID)]),
                extends: None,
                implements: [].into(),
                members: Box::new([
                    TypeMember {
                        kind: TypeMemberKind::Constructor,
                        ty: GLOBAL_PROMISE_CONSTRUCTOR_ID.into(),
                    },
                    method("catch", PROMISE_CATCH_ID),
                    method("finally", PROMISE_FINALLY_ID),
                    method("then", PROMISE_THEN_ID),
                    static_method("all", PROMISE_ALL_ID),
                    static_method("allSettled", PROMISE_ALL_SETTLED_ID),
                    static_method("any", PROMISE_ANY_ID),
                    static_method("race", PROMISE_RACE_ID),
                    static_method("reject", PROMISE_REJECT_ID),
                    static_method("resolve", PROMISE_RESOLVE_ID),
                    static_method("try", PROMISE_TRY_ID),
                ]),
            })),
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(global_type_name(PROMISE_CONSTRUCTOR_ID))),
                parameters: [FunctionParameter::Pattern(PatternFunctionParameter {
                    bindings: Default::default(),
                    is_optional: false,
                    is_rest: false,
                    ty: ResolvedTypeId::new(GLOBAL_LEVEL, VOID_CALLBACK_ID).into(),
                })]
                .into(),
                return_type: ReturnType::Type(GLOBAL_VOID_ID.into()),
            }),
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
            TypeData::Union(Box::new(Union(Box::new([
                GLOBAL_BIGINT_STRING_LITERAL_ID.into(),
                GLOBAL_BOOLEAN_STRING_LITERAL_ID.into(),
                GLOBAL_FUNCTION_STRING_LITERAL_ID.into(),
                GLOBAL_NUMBER_STRING_LITERAL_ID.into(),
                GLOBAL_OBJECT_STRING_LITERAL_ID.into(),
                GLOBAL_STRING_STRING_LITERAL_ID.into(),
                GLOBAL_SYMBOL_STRING_LITERAL_ID.into(),
                GLOBAL_UNDEFINED_STRING_LITERAL_ID.into(),
            ])))),
            TypeData::from(GenericTypeParameter {
                name: Text::new_static("T"),
                constraint: TypeReference::unknown(),
                default: TypeReference::unknown(),
            }),
            TypeData::from(GenericTypeParameter {
                name: Text::new_static("U"),
                constraint: TypeReference::unknown(),
                default: TypeReference::unknown(),
            }),
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(global_type_name(CONDITIONAL_CALLBACK_ID))),
                parameters: Default::default(),
                return_type: ReturnType::Type(GLOBAL_CONDITIONAL_ID.into()),
            }),
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(global_type_name(MAP_CALLBACK_ID))),
                parameters: [FunctionParameter::Pattern(PatternFunctionParameter {
                    ty: GLOBAL_U_ID.into(),
                    bindings: Default::default(),
                    is_optional: false,
                    is_rest: false,
                })]
                .into(),
                return_type: ReturnType::Type(GLOBAL_U_ID.into()),
            }),
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(global_type_name(VOID_CALLBACK_ID))),
                parameters: Default::default(),
                return_type: ReturnType::Type(GLOBAL_VOID_ID.into()),
            }),
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(global_type_name(FETCH_ID))),
                parameters: Default::default(),
                return_type: ReturnType::Type(GLOBAL_INSTANCEOF_PROMISE_ID.into()),
            }),
        ];

        let types: Vec<_> = types.into_iter().map(Arc::new).collect();
        Self {
            types: TypeStore::from_types(types),
        }
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
            if let Some(ty) = self.types.get(i).resolved(self) {
                self.types.replace(i, ty)
            }
            i += 1;
        }
    }

    fn flatten_all(&mut self) {
        for _ in 0..MAX_FLATTEN_DEPTH {
            let mut did_flatten = false;

            let mut i = NUM_PREDEFINED_TYPES;
            while i < self.types.len() {
                if let Some(ty) = self.types.get(i).flattened(self) {
                    self.types.replace(i, ty);
                    did_flatten = true;
                }
                i += 1;
            }

            if !did_flatten {
                break;
            }
        }
    }
}

impl TypeResolver for GlobalsResolver {
    fn level(&self) -> TypeResolverLevel {
        GLOBAL_LEVEL
    }

    fn find_type(&self, type_data: &TypeData) -> Option<TypeId> {
        self.types.find(type_data)
    }

    fn get_by_id(&self, id: TypeId) -> &TypeData {
        self.types.get_by_id(id)
    }

    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<ResolvedTypeData<'_>> {
        (id.level() == GLOBAL_LEVEL).then(|| (id, self.get_by_id(id.id())).into())
    }

    fn register_type(&mut self, type_data: Cow<TypeData>) -> TypeId {
        self.types.insert_cow(type_data)
    }

    fn resolve_reference(&self, ty: &TypeReference) -> Option<ResolvedTypeId> {
        match ty {
            TypeReference::Qualifier(qualifier) => self.resolve_qualifier(qualifier),
            TypeReference::Resolved(resolved_id) => {
                (resolved_id.level() == GLOBAL_LEVEL).then_some(*resolved_id)
            }
            TypeReference::Import(_) => None,
        }
    }

    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<ResolvedTypeId> {
        if qualifier.is_array() && !qualifier.has_known_type_parameters() {
            Some(GLOBAL_ARRAY_ID)
        } else if qualifier.is_promise() && !qualifier.has_known_type_parameters() {
            Some(GLOBAL_PROMISE_ID)
        } else if !qualifier.type_only
            && let Some(ident) = qualifier.path.identifier()
        {
            self.resolve_type_of(ident, qualifier.scope_id)
        } else {
            None
        }
    }

    fn resolve_type_of(&self, identifier: &Text, _scope_id: ScopeId) -> Option<ResolvedTypeId> {
        match identifier.text() {
            "fetch" => Some(GLOBAL_FETCH_ID),
            "globalThis" | "window" => Some(GLOBAL_GLOBAL_ID),
            _ => None,
        }
    }

    fn resolve_expression(
        &mut self,
        scope_id: ScopeId,
        expr: &AnyJsExpression,
    ) -> Cow<'_, TypeData> {
        Cow::Owned(TypeData::from_any_js_expression(self, scope_id, expr))
    }

    fn registered_types(&self) -> Vec<&TypeData> {
        self.types.as_references()[NUM_PREDEFINED_TYPES..].to_vec()
    }
}
