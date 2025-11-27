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
        .map(|id| {
            let name = global_type_name(id).unwrap_or("unknown");
            TypeMember {
                kind: TypeMemberKind::Named(Text::new_static(name)),
                ty: ResolvedTypeId::new(GLOBAL_LEVEL, id).into(),
            }
        })
        .collect()
});

// TODO(tidefield): Generate this function in codegen
// Returns a string for formatting global IDs in test snapshots.
pub fn global_type_name(id: TypeId) -> Option<&'static str> {
    use crate::globals_ids::*;
    // TODO(tidefield): Enforce exhaustiveness in this list
    match id {
        UNKNOWN_ID => Some(UNKNOWN_ID_NAME),
        UNDEFINED_ID => Some(UNDEFINED_ID_NAME),
        VOID_ID => Some(VOID_ID_NAME),
        CONDITIONAL_ID => Some(CONDITIONAL_ID_NAME),
        NUMBER_ID => Some(NUMBER_ID_NAME),
        STRING_ID => Some(STRING_ID_NAME),
        INSTANCEOF_ARRAY_T_ID => Some(INSTANCEOF_ARRAY_T_ID_NAME),
        INSTANCEOF_ARRAY_U_ID => Some(INSTANCEOF_ARRAY_U_ID_NAME),
        ARRAY_ID => Some(ARRAY_ID_NAME),
        ARRAY_FILTER_ID => Some(ARRAY_FILTER_ID_NAME),
        ARRAY_FOREACH_ID => Some(ARRAY_FOREACH_ID_NAME),
        ARRAY_MAP_ID => Some(ARRAY_MAP_ID_NAME),
        GLOBAL_ID => Some(GLOBAL_ID_NAME),
        INSTANCEOF_PROMISE_ID => Some(INSTANCEOF_PROMISE_ID_NAME),
        PROMISE_ID => Some(PROMISE_ID_NAME),
        PROMISE_CONSTRUCTOR_ID => Some(PROMISE_CONSTRUCTOR_ID_NAME),
        PROMISE_CATCH_ID => Some(PROMISE_CATCH_ID_NAME),
        PROMISE_FINALLY_ID => Some(PROMISE_FINALLY_ID_NAME),
        PROMISE_THEN_ID => Some(PROMISE_THEN_ID_NAME),
        PROMISE_ALL_ID => Some(PROMISE_ALL_ID_NAME),
        PROMISE_ALL_SETTLED_ID => Some(PROMISE_ALL_SETTLED_ID_NAME),
        PROMISE_ANY_ID => Some(PROMISE_ANY_ID_NAME),
        PROMISE_RACE_ID => Some(PROMISE_RACE_ID_NAME),
        PROMISE_REJECT_ID => Some(PROMISE_REJECT_ID_NAME),
        PROMISE_RESOLVE_ID => Some(PROMISE_RESOLVE_ID_NAME),
        PROMISE_TRY_ID => Some(PROMISE_TRY_ID_NAME),
        BIGINT_STRING_LITERAL_ID => Some(BIGINT_STRING_LITERAL_ID_NAME),
        BOOLEAN_STRING_LITERAL_ID => Some(BOOLEAN_STRING_LITERAL_ID_NAME),
        FUNCTION_STRING_LITERAL_ID => Some(FUNCTION_STRING_LITERAL_ID_NAME),
        NUMBER_STRING_LITERAL_ID => Some(NUMBER_STRING_LITERAL_ID_NAME),
        OBJECT_STRING_LITERAL_ID => Some(OBJECT_STRING_LITERAL_ID_NAME),
        STRING_STRING_LITERAL_ID => Some(STRING_STRING_LITERAL_ID_NAME),
        SYMBOL_STRING_LITERAL_ID => Some(SYMBOL_STRING_LITERAL_ID_NAME),
        UNDEFINED_STRING_LITERAL_ID => Some(UNDEFINED_STRING_LITERAL_ID_NAME),
        TYPEOF_OPERATOR_RETURN_UNION_ID => Some(TYPEOF_OPERATOR_RETURN_UNION_ID_NAME),
        T_ID => Some(T_ID_NAME),
        U_ID => Some(U_ID_NAME),
        CONDITIONAL_CALLBACK_ID => Some(CONDITIONAL_CALLBACK_ID_NAME),
        MAP_CALLBACK_ID => Some(MAP_CALLBACK_ID_NAME),
        VOID_CALLBACK_ID => Some(VOID_CALLBACK_ID_NAME),
        FETCH_ID => Some(FETCH_ID_NAME),
        INSTANCEOF_REGEXP_ID => Some(INSTANCEOF_REGEXP_ID_NAME),
        REGEXP_ID => Some(REGEXP_ID_NAME),
        REGEXP_EXEC_ID => Some(REGEXP_EXEC_ID_NAME),
        _ => None,
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
                    name: Some(Text::new_static(global_type_name(id).unwrap_or("unknown"))),
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
                name: Some(Text::new_static(global_type_name(id).unwrap_or("unknown"))),
                parameters: Default::default(),
                return_type: ReturnType::Type(GLOBAL_INSTANCEOF_PROMISE_ID.into()),
            })
        };

        let string_literal = |value: &'static str| -> TypeData {
            TypeData::from(Literal::String(Text::new_static(value).into()))
        };

        let mut builder = GlobalsResolverBuilder::with_capacity(NUM_PREDEFINED_TYPES);

        // Primitive types
        builder.set_type_data(UNKNOWN_ID, TypeData::Unknown);
        builder.set_type_data(UNDEFINED_ID, TypeData::Undefined);
        builder.set_type_data(VOID_ID, TypeData::VoidKeyword);
        builder.set_type_data(CONDITIONAL_ID, TypeData::Conditional);
        builder.set_type_data(NUMBER_ID, TypeData::Number);
        builder.set_type_data(STRING_ID, TypeData::String);

        // TODO(tidefield): Use biome parser to parse Typescript .d.ts files
        // and generate the following `TypeData`s as much as possible

        builder.set_type_data(
            INSTANCEOF_ARRAY_T_ID,
            TypeData::instance_of(TypeReference::from(GLOBAL_ARRAY_ID)),
        );
        builder.set_type_data(
            INSTANCEOF_ARRAY_U_ID,
            TypeData::instance_of(TypeInstance {
                ty: TypeReference::from(GLOBAL_ARRAY_ID),
                type_parameters: [GLOBAL_U_ID.into()].into(),
            }),
        );
        builder.set_type_data(
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
        builder.set_type_data(
            ARRAY_FILTER_ID,
            array_method_definition(
                ARRAY_FILTER_ID,
                CONDITIONAL_CALLBACK_ID,
                INSTANCEOF_ARRAY_T_ID,
                Default::default(),
            ),
        );
        builder.set_type_data(
            ARRAY_FOREACH_ID,
            array_method_definition(
                ARRAY_FOREACH_ID,
                VOID_CALLBACK_ID,
                VOID_ID,
                Default::default(),
            ),
        );
        builder.set_type_data(
            ARRAY_MAP_ID,
            array_method_definition(
                ARRAY_MAP_ID,
                MAP_CALLBACK_ID,
                INSTANCEOF_ARRAY_U_ID,
                [GLOBAL_U_ID.into()].into(),
            ),
        );
        builder.set_type_data(GLOBAL_ID, TypeData::Global);
        builder.set_type_data(
            INSTANCEOF_PROMISE_ID,
            TypeData::instance_of(TypeReference::from(GLOBAL_PROMISE_ID)),
        );
        // Promise class
        builder.set_type_data(
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

        builder.set_type_data(
            PROMISE_CONSTRUCTOR_ID,
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(PROMISE_CONSTRUCTOR_ID_NAME)),
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
        builder.set_type_data(
            PROMISE_CATCH_ID,
            promise_method_definition(PROMISE_CATCH_ID),
        );
        builder.set_type_data(
            PROMISE_FINALLY_ID,
            promise_method_definition(PROMISE_FINALLY_ID),
        );
        builder.set_type_data(PROMISE_THEN_ID, promise_method_definition(PROMISE_THEN_ID));
        builder.set_type_data(PROMISE_ALL_ID, promise_method_definition(PROMISE_ALL_ID));
        builder.set_type_data(
            PROMISE_ALL_SETTLED_ID,
            promise_method_definition(PROMISE_ALL_SETTLED_ID),
        );
        builder.set_type_data(PROMISE_ANY_ID, promise_method_definition(PROMISE_ANY_ID));
        builder.set_type_data(PROMISE_RACE_ID, promise_method_definition(PROMISE_RACE_ID));
        builder.set_type_data(
            PROMISE_REJECT_ID,
            promise_method_definition(PROMISE_REJECT_ID),
        );
        builder.set_type_data(
            PROMISE_RESOLVE_ID,
            promise_method_definition(PROMISE_RESOLVE_ID),
        );
        builder.set_type_data(PROMISE_TRY_ID, promise_method_definition(PROMISE_TRY_ID));
        // String literals for typeof operator
        builder.set_type_data(BIGINT_STRING_LITERAL_ID, string_literal("bigint"));
        builder.set_type_data(BOOLEAN_STRING_LITERAL_ID, string_literal("boolean"));
        builder.set_type_data(FUNCTION_STRING_LITERAL_ID, string_literal("function"));
        builder.set_type_data(NUMBER_STRING_LITERAL_ID, string_literal("number"));
        builder.set_type_data(OBJECT_STRING_LITERAL_ID, string_literal("object"));
        builder.set_type_data(STRING_STRING_LITERAL_ID, string_literal("string"));
        builder.set_type_data(SYMBOL_STRING_LITERAL_ID, string_literal("symbol"));
        builder.set_type_data(UNDEFINED_STRING_LITERAL_ID, string_literal("undefined"));
        builder.set_type_data(
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
        builder.set_type_data(
            T_ID,
            TypeData::from(GenericTypeParameter {
                name: Text::new_static("T"),
                constraint: TypeReference::unknown(),
                default: TypeReference::unknown(),
            }),
        );
        builder.set_type_data(
            U_ID,
            TypeData::from(GenericTypeParameter {
                name: Text::new_static("U"),
                constraint: TypeReference::unknown(),
                default: TypeReference::unknown(),
            }),
        );
        // Callback functions
        builder.set_type_data(
            CONDITIONAL_CALLBACK_ID,
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(CONDITIONAL_CALLBACK_ID_NAME)),
                parameters: Default::default(),
                return_type: ReturnType::Type(GLOBAL_CONDITIONAL_ID.into()),
            }),
        );
        builder.set_type_data(
            MAP_CALLBACK_ID,
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(MAP_CALLBACK_ID_NAME)),
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
        builder.set_type_data(
            VOID_CALLBACK_ID,
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(VOID_CALLBACK_ID_NAME)),
                parameters: Default::default(),
                return_type: ReturnType::Type(GLOBAL_VOID_ID.into()),
            }),
        );
        // Fetch function
        builder.set_type_data(
            FETCH_ID,
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(FETCH_ID_NAME)),
                parameters: Default::default(),
                return_type: ReturnType::Type(GLOBAL_INSTANCEOF_PROMISE_ID.into()),
            }),
        );
        builder.set_type_data(
            INSTANCEOF_REGEXP_ID,
            TypeData::instance_of(TypeReference::from(GLOBAL_REGEXP_ID)),
        );
        builder.set_type_data(
            REGEXP_ID,
            TypeData::Class(Box::new(Class {
                name: Some(Text::new_static(REGEXP_ID_NAME)),
                type_parameters: Box::default(),
                extends: None,
                implements: [].into(),
                members: Box::new([method("exec", REGEXP_EXEC_ID)]),
            })),
        );
        builder.set_type_data(
            REGEXP_EXEC_ID,
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(REGEXP_EXEC_ID_NAME)),
                parameters: Default::default(),
                return_type: ReturnType::Type(GLOBAL_INSTANCEOF_REGEXP_ID.into()),
            }),
        );

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
