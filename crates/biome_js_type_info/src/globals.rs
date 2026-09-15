use std::sync::LazyLock;

use biome_rowan::Text;

use crate::{
    Class, Function, FunctionParameter, GenericTypeParameter, Literal, PatternFunctionParameter,
    RawTypeId, ReturnType, TypeData, TypeInstance, TypeReference, TypeReferenceQualifier,
    TypeStore, Union, interned_types::TypeData as InferredTypeData,
};

use super::globals_builder::GlobalsResolverBuilder;
use crate::generated::global_types::{generated_local_types, set_generated_global_type_data};

pub use super::globals_ids::*;

pub(super) struct RawGlobalTypes {
    pub(super) types: TypeStore,
}

impl Default for RawGlobalTypes {
    /// Generated globals take precedence; manual definitions only fill missing slots.
    fn default() -> Self {
        // Builds an empty-body global `Class` with `name` and `type_parameters`.
        let class = |name: &'static str, type_parameters: Box<[TypeReference]>| {
            TypeData::Class(Box::new(Class {
                name: Some(Text::new_static(name)),
                type_parameters,
                extends: None,
                implements: Box::default(),
                members: Box::default(),
            }))
        };

        // Builds a string-literal `TypeData` whose value is the static text
        // `value`.
        let string_literal = |value: &'static str| -> TypeData {
            TypeData::from(Literal::String(Text::new_static(value).into()))
        };

        let mut builder = GlobalsResolverBuilder::default();
        set_generated_global_type_data(&mut builder);

        builder.set_manual_type_data(UNKNOWN_ID_GLOBAL_TYPE_ID, || TypeData::Unknown);
        builder.set_manual_type_data(UNDEFINED_ID_GLOBAL_TYPE_ID, || TypeData::Undefined);
        builder.set_manual_type_data(VOID_ID_GLOBAL_TYPE_ID, || TypeData::VoidKeyword);
        builder.set_manual_type_data(CONDITIONAL_ID_GLOBAL_TYPE_ID, || TypeData::Conditional);
        builder.set_manual_type_data(NUMBER_ID_GLOBAL_TYPE_ID, || TypeData::Number);
        builder.set_manual_type_data(STRING_ID_GLOBAL_TYPE_ID, || TypeData::String);
        builder.set_manual_type_data(BOOLEAN_ID_GLOBAL_TYPE_ID, || TypeData::Boolean);

        builder.set_manual_type_data(INSTANCEOF_ARRAY_T_ID_GLOBAL_TYPE_ID, || {
            TypeData::instance_of(TypeReference::from(GLOBAL_ARRAY_ID))
        });
        builder.set_manual_type_data(INSTANCEOF_ARRAY_U_ID_GLOBAL_TYPE_ID, || {
            TypeData::instance_of(TypeInstance {
                ty: TypeReference::from(GLOBAL_ARRAY_ID),
                type_parameters: [GLOBAL_U_ID.into()].into(),
            })
        });
        builder.set_manual_type_data(GLOBAL_ID_GLOBAL_TYPE_ID, || TypeData::Global);
        builder.set_manual_type_data(INSTANCEOF_PROMISE_ID_GLOBAL_TYPE_ID, || {
            TypeData::instance_of(TypeReference::from(GLOBAL_PROMISE_ID))
        });
        builder.set_manual_type_data(BIGINT_STRING_LITERAL_ID_GLOBAL_TYPE_ID, || {
            string_literal("bigint")
        });
        builder.set_manual_type_data(BOOLEAN_STRING_LITERAL_ID_GLOBAL_TYPE_ID, || {
            string_literal("boolean")
        });
        builder.set_manual_type_data(FUNCTION_STRING_LITERAL_ID_GLOBAL_TYPE_ID, || {
            string_literal("function")
        });
        builder.set_manual_type_data(NUMBER_STRING_LITERAL_ID_GLOBAL_TYPE_ID, || {
            string_literal("number")
        });
        builder.set_manual_type_data(OBJECT_STRING_LITERAL_ID_GLOBAL_TYPE_ID, || {
            string_literal("object")
        });
        builder.set_manual_type_data(STRING_STRING_LITERAL_ID_GLOBAL_TYPE_ID, || {
            string_literal("string")
        });
        builder.set_manual_type_data(SYMBOL_STRING_LITERAL_ID_GLOBAL_TYPE_ID, || {
            string_literal("symbol")
        });
        builder.set_manual_type_data(UNDEFINED_STRING_LITERAL_ID_GLOBAL_TYPE_ID, || {
            string_literal("undefined")
        });
        builder.set_manual_type_data(TYPEOF_OPERATOR_RETURN_UNION_ID_GLOBAL_TYPE_ID, || {
            TypeData::Union(Box::new(Union(Box::new([
                GLOBAL_BIGINT_STRING_LITERAL_ID.into(),
                GLOBAL_BOOLEAN_STRING_LITERAL_ID.into(),
                GLOBAL_FUNCTION_STRING_LITERAL_ID.into(),
                GLOBAL_NUMBER_STRING_LITERAL_ID.into(),
                GLOBAL_OBJECT_STRING_LITERAL_ID.into(),
                GLOBAL_STRING_STRING_LITERAL_ID.into(),
                GLOBAL_SYMBOL_STRING_LITERAL_ID.into(),
                GLOBAL_UNDEFINED_STRING_LITERAL_ID.into(),
            ]))))
        });
        builder.set_manual_type_data(T_ID_GLOBAL_TYPE_ID, || {
            TypeData::from(GenericTypeParameter {
                name: Text::new_static("T"),
                constraint: TypeReference::unknown(),
                default: TypeReference::unknown(),
            })
        });
        builder.set_manual_type_data(U_ID_GLOBAL_TYPE_ID, || {
            TypeData::from(GenericTypeParameter {
                name: Text::new_static("U"),
                constraint: TypeReference::unknown(),
                default: TypeReference::unknown(),
            })
        });
        builder.set_manual_type_data(CONDITIONAL_CALLBACK_ID_GLOBAL_TYPE_ID, || {
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(CONDITIONAL_CALLBACK_ID_NAME)),
                parameters: Default::default(),
                return_type: ReturnType::Type(GLOBAL_CONDITIONAL_ID.into()),
            })
        });
        builder.set_manual_type_data(MAP_CALLBACK_ID_GLOBAL_TYPE_ID, || {
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
            })
        });
        builder.set_manual_type_data(VOID_CALLBACK_ID_GLOBAL_TYPE_ID, || {
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(VOID_CALLBACK_ID_NAME)),
                parameters: Default::default(),
                return_type: ReturnType::Type(GLOBAL_VOID_ID.into()),
            })
        });
        builder.set_manual_type_data(FETCH_ID_GLOBAL_TYPE_ID, || {
            TypeData::from(Function {
                is_async: false,
                type_parameters: Default::default(),
                name: Some(Text::new_static(FETCH_ID_NAME)),
                parameters: Default::default(),
                return_type: ReturnType::Type(GLOBAL_INSTANCEOF_PROMISE_ID.into()),
            })
        });
        builder.set_manual_type_data(INSTANCEOF_REGEXP_ID_GLOBAL_TYPE_ID, || {
            TypeData::instance_of(TypeReference::from(GLOBAL_REGEXP_ID))
        });
        builder.set_manual_type_data(INSTANCEOF_DATE_ID_GLOBAL_TYPE_ID, || {
            TypeData::instance_of(TypeReference::from(GLOBAL_DATE_ID))
        });
        builder.set_manual_type_data(INSTANCEOF_MAP_ID_GLOBAL_TYPE_ID, || {
            TypeData::instance_of(TypeReference::from(GLOBAL_MAP_ID))
        });
        builder.set_manual_type_data(INSTANCEOF_SET_ID_GLOBAL_TYPE_ID, || {
            TypeData::instance_of(TypeReference::from(GLOBAL_SET_ID))
        });
        builder.set_manual_type_data(INSTANCEOF_WEAK_MAP_ID_GLOBAL_TYPE_ID, || {
            TypeData::instance_of(TypeReference::from(GLOBAL_WEAK_MAP_ID))
        });
        builder.set_manual_type_data(INSTANCEOF_ERROR_ID_GLOBAL_TYPE_ID, || {
            TypeData::instance_of(TypeReference::from(GLOBAL_ERROR_ID))
        });
        builder.set_manual_type_data(ERROR_ID_GLOBAL_TYPE_ID, || {
            class(ERROR_ID_NAME, Box::default())
        });
        builder.set_manual_type_data(INSTANCEOF_SYMBOL_ID_GLOBAL_TYPE_ID, || {
            TypeData::instance_of(TypeReference::from(GLOBAL_SYMBOL_ID))
        });
        builder.build()
    }
}

static RAW_GLOBAL_TYPES: LazyLock<RawGlobalTypes> = LazyLock::new(RawGlobalTypes::default);

pub(crate) fn raw_global_type(type_id: GlobalTypeId) -> &'static TypeData {
    RAW_GLOBAL_TYPES.types.get_by_id(type_id.as_type_id())
}

pub fn global_type_id_for_qualifier(qualifier: &TypeReferenceQualifier) -> Option<GlobalTypeId> {
    if qualifier.type_only
        && let Some(name) = qualifier.path.identifier()
        && let Some((_, RawTypeId::Global(id))) =
            crate::generated::global_types::DECLARATION_GLOBALS
                .iter()
                .find(|(declared, _)| *declared == name.text())
    {
        return Some(*id);
    }
    let id = if qualifier.has_known_type_parameters() {
        return None;
    } else if qualifier.is_array() {
        ARRAY_ID_GLOBAL_TYPE_ID
    } else if qualifier.is_promise() {
        PROMISE_ID_GLOBAL_TYPE_ID
    } else if qualifier.is_regex() {
        REGEXP_ID_GLOBAL_TYPE_ID
    } else if qualifier.is_symbol() {
        SYMBOL_ID_GLOBAL_TYPE_ID
    } else if qualifier.is_date() {
        DATE_ID_GLOBAL_TYPE_ID
    } else if qualifier.is_map() {
        MAP_ID_GLOBAL_TYPE_ID
    } else if qualifier.is_set() {
        SET_ID_GLOBAL_TYPE_ID
    } else if qualifier.is_weak_map() {
        WEAK_MAP_ID_GLOBAL_TYPE_ID
    } else if qualifier.is_error() {
        ERROR_ID_GLOBAL_TYPE_ID
    } else if qualifier.is_disposable() {
        DISPOSABLE_ID_GLOBAL_TYPE_ID
    } else if qualifier.is_async_disposable() {
        ASYNC_DISPOSABLE_ID_GLOBAL_TYPE_ID
    } else {
        return qualifier
            .path
            .identifier()
            .and_then(|name| global_type_id_for_value(name.text()));
    };
    Some(id)
}

pub fn global_type_id_for_value(name: &str) -> Option<GlobalTypeId> {
    match name {
        "fetch" => Some(FETCH_ID_GLOBAL_TYPE_ID),
        "globalThis" | "window" => Some(GLOBAL_ID_GLOBAL_TYPE_ID),
        _ => None,
    }
}

#[derive(Clone, Copy)]
pub struct GlobalTypes<'db> {
    db: &'db dyn crate::TypeDb,
}

impl<'db> GlobalTypes<'db> {
    pub fn get(&self, id: GlobalTypeId) -> InferredTypeData<'db> {
        resolve_global_type(self.db, GlobalTypeInput::new(self.db, id, None))
    }

    pub fn typeof_literal(&self, value: &str) -> InferredTypeData<'db> {
        let id = match value {
            "bigint" => BIGINT_STRING_LITERAL_ID_GLOBAL_TYPE_ID,
            "boolean" => BOOLEAN_STRING_LITERAL_ID_GLOBAL_TYPE_ID,
            "function" => FUNCTION_STRING_LITERAL_ID_GLOBAL_TYPE_ID,
            "number" => NUMBER_STRING_LITERAL_ID_GLOBAL_TYPE_ID,
            "object" => OBJECT_STRING_LITERAL_ID_GLOBAL_TYPE_ID,
            "string" => STRING_STRING_LITERAL_ID_GLOBAL_TYPE_ID,
            "symbol" => SYMBOL_STRING_LITERAL_ID_GLOBAL_TYPE_ID,
            "undefined" => UNDEFINED_STRING_LITERAL_ID_GLOBAL_TYPE_ID,
            _ => return InferredTypeData::Unknown,
        };
        self.get(id)
    }

    pub fn typeof_return_union(&self) -> InferredTypeData<'db> {
        self.get(TYPEOF_OPERATOR_RETURN_UNION_ID_GLOBAL_TYPE_ID)
    }
}

/// Provides memoized access to individual globals without resolving unrelated types.
pub fn global_types(db: &dyn crate::TypeDb) -> GlobalTypes<'_> {
    GlobalTypes { db }
}

/// A predefined global or a local entry scoped to that global's supporting-type table.
#[salsa::interned(no_lifetime)]
struct GlobalTypeInput {
    owner: GlobalTypeId,
    local: Option<crate::TypeId>,
}

#[salsa::tracked]
fn resolve_global_type<'db>(
    db: &'db dyn crate::TypeDb,
    input: GlobalTypeInput,
) -> InferredTypeData<'db> {
    let owner = input.owner(db);
    let local = input.local(db);
    let raw = match local {
        None => raw_global_type(owner),
        Some(id) => {
            let raw = generated_local_types(owner).get(id.index());
            debug_assert!(
                raw.is_some(),
                "generated local references must index their owner's supporting-type table"
            );
            let Some(raw) = raw else {
                return InferredTypeData::Unknown;
            };
            raw
        }
    };
    InferredTypeData::from_raw_with_resolver(db, raw, true, &mut |reference| {
        match reference {
            TypeReference::Resolved(RawTypeId::Global(target)) => {
                // The typeof result contains literal values rather than deferred global handles.
                if local.is_none() && owner == TYPEOF_OPERATOR_RETURN_UNION_ID_GLOBAL_TYPE_ID {
                    global_types(db).get(*target)
                } else {
                    InferredTypeData::GlobalType(*target)
                }
            }
            TypeReference::Resolved(RawTypeId::Local(target)) => {
                if let Some(source) = local {
                    // Dependency order prevents recursive queries from cycling.
                    debug_assert!(
                        target.index() < source.index(),
                        "generated local types must be in dependency order"
                    );
                }
                resolve_global_type(db, GlobalTypeInput::new(db, owner, Some(*target)))
            }
            TypeReference::Qualifier(_) | TypeReference::Import(_) => InferredTypeData::Unknown,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[salsa::db]
    #[derive(Default)]
    struct TestDb {
        storage: salsa::Storage<Self>,
    }

    #[salsa::db]
    impl salsa::Database for TestDb {}

    #[salsa::db]
    impl biome_db::Db for TestDb {
        fn parsed_source_for_path(
            &self,
            _path: &camino::Utf8Path,
        ) -> Option<biome_db::ParsedSource> {
            None
        }
    }

    #[salsa::db]
    impl crate::TypeDb for TestDb {}

    #[test]
    fn generated_weak_map_keys_are_non_nullish() {
        let db = TestDb::default();
        let InferredTypeData::Class(weak_map) = global_types(&db).get(WEAK_MAP_ID_GLOBAL_TYPE_ID)
        else {
            panic!("expected WeakMap class");
        };
        let key = weak_map.type_parameters(&db)[0];
        assert!(crate::InferredType::new(&db, key).is_non_nullish());
    }

    #[test]
    fn local_query_keys_distinguish_owners() {
        let db = TestDb::default();
        let local = Some(crate::TypeId::new(0));
        let weak_map = GlobalTypeInput::new(&db, WEAK_MAP_ID_GLOBAL_TYPE_ID, local);
        let map = GlobalTypeInput::new(&db, MAP_ID_GLOBAL_TYPE_ID, local);
        assert!(weak_map != map);
        assert!(weak_map == GlobalTypeInput::new(&db, WEAK_MAP_ID_GLOBAL_TYPE_ID, local));
    }

    #[test]
    fn global_lookup_defers_unrelated_classes_and_reuses_results() {
        let events = biome_db::testing::Events::default();
        let db = TestDb {
            storage: salsa::Storage::new(Some(Box::new({
                let events = events.clone();
                move |event| events.0.lock().unwrap().push(event)
            }))),
        };
        let globals = global_types(&db);
        let promise = globals.get(PROMISE_ID_GLOBAL_TYPE_ID);
        assert!(matches!(promise, InferredTypeData::Class(_)));
        events.0.lock().unwrap().clear();

        let weak_map = globals.get(WEAK_MAP_ID_GLOBAL_TYPE_ID);
        assert!(matches!(weak_map, InferredTypeData::Class(_)));
        assert!(
            events.0.lock().unwrap().iter().any(|event| {
                let salsa::EventKind::DidInternValue { key, .. } = event.kind else {
                    return false;
                };
                salsa::Database::ingredient_debug_name(&db, key.ingredient_index())
                    == "InternedClass"
            }),
            "Promise lookup must not precompute WeakMap"
        );
        events.0.lock().unwrap().clear();

        assert_eq!(globals.get(PROMISE_ID_GLOBAL_TYPE_ID), promise);
        assert_eq!(globals.get(WEAK_MAP_ID_GLOBAL_TYPE_ID), weak_map);
        assert!(!events.0.lock().unwrap().iter().any(|event| matches!(
            event.kind,
            salsa::EventKind::WillExecute { .. } | salsa::EventKind::DidInternValue { .. }
        )));
    }

    #[test]
    fn typeof_literals_use_canonical_global_entries() {
        let db = TestDb::default();
        let globals = global_types(&db);
        assert_eq!(
            globals.typeof_literal("string"),
            globals.get(STRING_STRING_LITERAL_ID_GLOBAL_TYPE_ID)
        );
        assert_eq!(
            globals.typeof_return_union(),
            globals.get(TYPEOF_OPERATOR_RETURN_UNION_ID_GLOBAL_TYPE_ID)
        );
        let InferredTypeData::Union(union) = globals.typeof_return_union() else {
            panic!("typeof must return a union");
        };
        assert!(
            union
                .types(&db)
                .iter()
                .all(|ty| matches!(ty, InferredTypeData::Literal(_)))
        );
    }

    #[test]
    fn structured_globals_keep_their_members() {
        let db = TestDb::default();
        let InferredTypeData::Class(promise) = global_types(&db).get(PROMISE_ID_GLOBAL_TYPE_ID)
        else {
            panic!("Promise must be a class");
        };
        assert!(!promise.members(&db).is_empty());
    }

    #[test]
    fn generated_symbol_globals_keep_static_members() {
        let db = TestDb::default();
        let globals = global_types(&db);
        let InferredTypeData::Class(symbol) = globals.get(SYMBOL_ID_GLOBAL_TYPE_ID) else {
            panic!("Symbol must be a class");
        };
        let members = symbol.members(&db);

        for (name, global_type_id) in [
            ("dispose", SYMBOL_DISPOSE_ID_GLOBAL_TYPE_ID),
            ("asyncDispose", SYMBOL_ASYNC_DISPOSE_ID_GLOBAL_TYPE_ID),
        ] {
            let member = members
                .iter()
                .find(|member| member.kind.has_name(name))
                .unwrap_or_else(|| panic!("Symbol.{name} must exist"));
            assert!(member.kind.is_static());
            assert_eq!(member.ty, InferredTypeData::GlobalType(global_type_id));
            assert_eq!(globals.get(global_type_id), InferredTypeData::Symbol);
        }
    }
}
