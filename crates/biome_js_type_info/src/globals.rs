//! Hardcoded global definitions.

// FIXME: Implement inference from type definitions: https://github.com/biomejs/biome/issues/5977

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

use super::globals_builder::GlobalsResolverBuilder;

// Re-export all type ID constants from globals_ids
pub use super::globals_ids::*;

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
        26 => "instanceof RegExp",
        27 => "RegExp",
        28 => "RegExp.exec",
        29 => "\"bigint\"",
        30 => "\"boolean\"",
        31 => "\"function\"",
        32 => "\"number\"",
        33 => "\"object\"",
        34 => "\"string\"",
        35 => "\"symbol\"",
        36 => "\"undefined\"",
        37 => {
            "\"bigint\" | \"boolean\" | \"function\" | \"number\" | \"object\" \
                | \"string\" | \"symbol\" | \"undefined\""
        }
        38 => "T",
        39 => "U",
        40 => "() => conditional",
        41 => "<U>(item: T) => U",
        42 => "() => void",
        43 => "fetch",
        _ => "inferred type",
    }
}

/// Resolver that is limited to resolving symbols in the global scope.
///
/// This resolver does not check whether qualifiers that are being resolved have
/// been shadowed by local declarations, so it should generally only be used
/// after all other resolvers have failed.
pub struct GlobalsResolver {
    pub(crate) types: TypeStore,
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

        let regexp_method_definition = |id: TypeId| {
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(global_type_name(id))),
                parameters: Default::default(),
                return_type: ReturnType::Type(GLOBAL_INSTANCEOF_REGEXP_ID.into()),
            })
        };

        let string_literal = |value: &'static str| -> TypeData {
            TypeData::from(Literal::String(Text::new_static(value).into()))
        };

        let mut builder = GlobalsResolverBuilder::new();

        // Reserve all type IDs and ensure IDs match the constants defined above
        assert_eq!(builder.reserve_id(), UNKNOWN_ID);
        assert_eq!(builder.reserve_id(), UNDEFINED_ID);
        assert_eq!(builder.reserve_id(), VOID_ID);
        assert_eq!(builder.reserve_id(), CONDITIONAL_ID);
        assert_eq!(builder.reserve_id(), NUMBER_ID);
        assert_eq!(builder.reserve_id(), STRING_ID);
        assert_eq!(builder.reserve_id(), INSTANCEOF_ARRAY_T_ID);
        assert_eq!(builder.reserve_id(), INSTANCEOF_ARRAY_U_ID);
        assert_eq!(builder.reserve_id(), ARRAY_ID);
        assert_eq!(builder.reserve_id(), ARRAY_FILTER_ID);
        assert_eq!(builder.reserve_id(), ARRAY_FOREACH_ID);
        assert_eq!(builder.reserve_id(), ARRAY_MAP_ID);
        assert_eq!(builder.reserve_id(), GLOBAL_ID);
        assert_eq!(builder.reserve_id(), INSTANCEOF_PROMISE_ID);
        assert_eq!(builder.reserve_id(), PROMISE_ID);
        assert_eq!(builder.reserve_id(), PROMISE_CONSTRUCTOR_ID);
        assert_eq!(builder.reserve_id(), PROMISE_CATCH_ID);
        assert_eq!(builder.reserve_id(), PROMISE_FINALLY_ID);
        assert_eq!(builder.reserve_id(), PROMISE_THEN_ID);
        assert_eq!(builder.reserve_id(), PROMISE_ALL_ID);
        assert_eq!(builder.reserve_id(), PROMISE_ALL_SETTLED_ID);
        assert_eq!(builder.reserve_id(), PROMISE_ANY_ID);
        assert_eq!(builder.reserve_id(), PROMISE_RACE_ID);
        assert_eq!(builder.reserve_id(), PROMISE_REJECT_ID);
        assert_eq!(builder.reserve_id(), PROMISE_RESOLVE_ID);
        assert_eq!(builder.reserve_id(), PROMISE_TRY_ID);
        assert_eq!(builder.reserve_id(), BIGINT_STRING_LITERAL_ID);
        assert_eq!(builder.reserve_id(), BOOLEAN_STRING_LITERAL_ID);
        assert_eq!(builder.reserve_id(), FUNCTION_STRING_LITERAL_ID);
        assert_eq!(builder.reserve_id(), NUMBER_STRING_LITERAL_ID);
        assert_eq!(builder.reserve_id(), OBJECT_STRING_LITERAL_ID);
        assert_eq!(builder.reserve_id(), STRING_STRING_LITERAL_ID);
        assert_eq!(builder.reserve_id(), SYMBOL_STRING_LITERAL_ID);
        assert_eq!(builder.reserve_id(), UNDEFINED_STRING_LITERAL_ID);
        assert_eq!(builder.reserve_id(), TYPEOF_OPERATOR_RETURN_UNION_ID);
        assert_eq!(builder.reserve_id(), T_ID);
        assert_eq!(builder.reserve_id(), U_ID);
        assert_eq!(builder.reserve_id(), CONDITIONAL_CALLBACK_ID);
        assert_eq!(builder.reserve_id(), MAP_CALLBACK_ID);
        assert_eq!(builder.reserve_id(), VOID_CALLBACK_ID);
        assert_eq!(builder.reserve_id(), FETCH_ID);

        // Fill in all types
        builder.set_type(UNKNOWN_ID, TypeData::Unknown);
        builder.set_type(UNDEFINED_ID, TypeData::Undefined);
        builder.set_type(VOID_ID, TypeData::VoidKeyword);
        builder.set_type(CONDITIONAL_ID, TypeData::Conditional);
        builder.set_type(NUMBER_ID, TypeData::Number);
        builder.set_type(STRING_ID, TypeData::String);
        builder.set_type(
            INSTANCEOF_ARRAY_T_ID,
            TypeData::instance_of(TypeReference::from(GLOBAL_ARRAY_ID)),
        );
        builder.set_type(
            INSTANCEOF_ARRAY_U_ID,
            TypeData::instance_of(TypeInstance {
                ty: TypeReference::from(GLOBAL_ARRAY_ID),
                type_parameters: [GLOBAL_U_ID.into()].into(),
            }),
        );

        builder.set_type(
            ARRAY_ID,
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
        );

        builder.set_type(
            ARRAY_FILTER_ID,
            array_method_definition(
                ARRAY_FILTER_ID,
                CONDITIONAL_CALLBACK_ID,
                INSTANCEOF_ARRAY_T_ID,
                Default::default(),
            ),
        );
        builder.set_type(
            ARRAY_FOREACH_ID,
            array_method_definition(
                ARRAY_FOREACH_ID,
                VOID_CALLBACK_ID,
                VOID_ID,
                Default::default(),
            ),
        );
        builder.set_type(
            ARRAY_MAP_ID,
            array_method_definition(
                ARRAY_MAP_ID,
                MAP_CALLBACK_ID,
                INSTANCEOF_ARRAY_U_ID,
                [GLOBAL_U_ID.into()].into(),
            ),
        );

        builder.set_type(GLOBAL_ID, TypeData::Global);
        builder.set_type(
            INSTANCEOF_PROMISE_ID,
            TypeData::instance_of(TypeReference::from(GLOBAL_PROMISE_ID)),
        );

        // Promise class
        builder.set_type(
            PROMISE_ID,
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
        );

        builder.set_type(
            PROMISE_CONSTRUCTOR_ID,
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
        );
        builder.set_type(
            PROMISE_CATCH_ID,
            promise_method_definition(PROMISE_CATCH_ID),
        );
        builder.set_type(
            PROMISE_FINALLY_ID,
            promise_method_definition(PROMISE_FINALLY_ID),
        );
        builder.set_type(PROMISE_THEN_ID, promise_method_definition(PROMISE_THEN_ID));
        builder.set_type(PROMISE_ALL_ID, promise_method_definition(PROMISE_ALL_ID));
        builder.set_type(
            PROMISE_ALL_SETTLED_ID,
            promise_method_definition(PROMISE_ALL_SETTLED_ID),
        );
        builder.set_type(PROMISE_ANY_ID, promise_method_definition(PROMISE_ANY_ID));
        builder.set_type(PROMISE_RACE_ID, promise_method_definition(PROMISE_RACE_ID));
        builder.set_type(
            PROMISE_REJECT_ID,
            promise_method_definition(PROMISE_REJECT_ID),
        );
        builder.set_type(
            PROMISE_RESOLVE_ID,
            promise_method_definition(PROMISE_RESOLVE_ID),
        );
        builder.set_type(PROMISE_TRY_ID, promise_method_definition(PROMISE_TRY_ID));

        // String literals for typeof operator
        builder.set_type(BIGINT_STRING_LITERAL_ID, string_literal("bigint"));
        builder.set_type(BOOLEAN_STRING_LITERAL_ID, string_literal("boolean"));
        builder.set_type(FUNCTION_STRING_LITERAL_ID, string_literal("function"));
        builder.set_type(NUMBER_STRING_LITERAL_ID, string_literal("number"));
        builder.set_type(OBJECT_STRING_LITERAL_ID, string_literal("object"));
        builder.set_type(STRING_STRING_LITERAL_ID, string_literal("string"));
        builder.set_type(SYMBOL_STRING_LITERAL_ID, string_literal("symbol"));
        builder.set_type(UNDEFINED_STRING_LITERAL_ID, string_literal("undefined"));
        builder.set_type(
            TYPEOF_OPERATOR_RETURN_UNION_ID,
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
        );

        // Generic type parameters
        builder.set_type(
            T_ID,
            TypeData::from(GenericTypeParameter {
                name: Text::new_static("T"),
                constraint: TypeReference::unknown(),
                default: TypeReference::unknown(),
            }),
        );
        builder.set_type(
            U_ID,
            TypeData::from(GenericTypeParameter {
                name: Text::new_static("U"),
                constraint: TypeReference::unknown(),
                default: TypeReference::unknown(),
            }),
        );

        // Callback functions
        builder.set_type(
            CONDITIONAL_CALLBACK_ID,
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(global_type_name(CONDITIONAL_CALLBACK_ID))),
                parameters: Default::default(),
                return_type: ReturnType::Type(GLOBAL_CONDITIONAL_ID.into()),
            }),
        );
        builder.set_type(
            MAP_CALLBACK_ID,
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
        );
        builder.set_type(
            VOID_CALLBACK_ID,
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(global_type_name(VOID_CALLBACK_ID))),
                parameters: Default::default(),
                return_type: ReturnType::Type(GLOBAL_VOID_ID.into()),
            }),
        );

        // Fetch function
        builder.set_type(
            FETCH_ID,
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(global_type_name(FETCH_ID))),
                parameters: Default::default(),
                return_type: ReturnType::Type(GLOBAL_INSTANCEOF_PROMISE_ID.into()),
            }),
        );

        // Build and return
        builder.build()
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
        } else if qualifier.is_regex() && !qualifier.has_known_type_parameters() {
            Some(GLOBAL_REGEXP_ID)
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
