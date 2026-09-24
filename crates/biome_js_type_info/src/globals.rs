use std::sync::OnceLock;

use biome_rowan::Text;

use crate::{
    Function, FunctionParameter, GenericTypeParameter, Literal, PatternFunctionParameter,
    RawTypeId, ReturnType, TypeData, TypeInstance, TypeReference, TypeReferenceQualifier, Union,
    interned_types::TypeData as InferredTypeData,
};

use crate::generated::global_types::{
    GENERATED_GLOBAL_BUILDERS, GENERATED_GLOBAL_NAMES, generated_local_types,
};

pub use super::globals_ids::*;
pub(crate) use crate::generated::global_types::ids::*;

const _: () = assert!(GENERATED_GLOBAL_BUILDERS.len() == GENERATED_GLOBAL_NAMES.len());

/// Type data for each global, built the first time it is looked up.
static GLOBAL_TYPE_DATA: [OnceLock<TypeData>; NUM_PREDEFINED_TYPES] =
    [const { OnceLock::new() }; NUM_PREDEFINED_TYPES];

pub(crate) fn raw_global_type(id: GlobalTypeId) -> &'static TypeData {
    GLOBAL_TYPE_DATA[id.index()].get_or_init(|| {
        match id.index().checked_sub(PREDEFINED_ID_ROWS.len()) {
            Some(index) => GENERATED_GLOBAL_BUILDERS[index](),
            None => predefined_type_data(id),
        }
    })
}

/// Builds the intrinsics that no TypeScript declaration file defines.
fn predefined_type_data(id: GlobalTypeId) -> TypeData {
    // Builds a string-literal `TypeData` whose value is the static text `value`.
    let string_literal =
        |value: &'static str| TypeData::from(Literal::String(Text::new_static(value).into()));

    match id {
        UNKNOWN_ID_GLOBAL_TYPE_ID => TypeData::Unknown,
        UNDEFINED_ID_GLOBAL_TYPE_ID => TypeData::Undefined,
        VOID_ID_GLOBAL_TYPE_ID => TypeData::VoidKeyword,
        CONDITIONAL_ID_GLOBAL_TYPE_ID => TypeData::Conditional,
        NUMBER_KEYWORD_ID_GLOBAL_TYPE_ID => TypeData::Number,
        STRING_KEYWORD_ID_GLOBAL_TYPE_ID => TypeData::String,
        BOOLEAN_KEYWORD_ID_GLOBAL_TYPE_ID => TypeData::Boolean,
        BIGINT_KEYWORD_ID_GLOBAL_TYPE_ID => TypeData::BigInt,
        SYMBOL_KEYWORD_ID_GLOBAL_TYPE_ID => TypeData::Symbol,
        NULL_KEYWORD_ID_GLOBAL_TYPE_ID => TypeData::Null,
        ANY_KEYWORD_ID_GLOBAL_TYPE_ID => TypeData::AnyKeyword,
        NEVER_KEYWORD_ID_GLOBAL_TYPE_ID => TypeData::NeverKeyword,
        OBJECT_KEYWORD_ID_GLOBAL_TYPE_ID => TypeData::ObjectKeyword,
        UNKNOWN_KEYWORD_ID_GLOBAL_TYPE_ID => TypeData::UnknownKeyword,
        INSTANCEOF_ARRAY_T_ID_GLOBAL_TYPE_ID => {
            TypeData::instance_of(TypeReference::from(GLOBAL_ARRAY_ID))
        }
        INSTANCEOF_ARRAY_U_ID_GLOBAL_TYPE_ID => TypeData::instance_of(TypeInstance {
            ty: TypeReference::from(GLOBAL_ARRAY_ID),
            type_parameters: [GLOBAL_U_ID.into()].into(),
        }),
        GLOBAL_ID_GLOBAL_TYPE_ID => TypeData::Global,
        INSTANCEOF_PROMISE_ID_GLOBAL_TYPE_ID => {
            TypeData::instance_of(TypeReference::from(GLOBAL_PROMISE_ID))
        }
        BIGINT_STRING_LITERAL_ID_GLOBAL_TYPE_ID => string_literal("bigint"),
        BOOLEAN_STRING_LITERAL_ID_GLOBAL_TYPE_ID => string_literal("boolean"),
        FUNCTION_STRING_LITERAL_ID_GLOBAL_TYPE_ID => string_literal("function"),
        NUMBER_STRING_LITERAL_ID_GLOBAL_TYPE_ID => string_literal("number"),
        OBJECT_STRING_LITERAL_ID_GLOBAL_TYPE_ID => string_literal("object"),
        STRING_STRING_LITERAL_ID_GLOBAL_TYPE_ID => string_literal("string"),
        SYMBOL_STRING_LITERAL_ID_GLOBAL_TYPE_ID => string_literal("symbol"),
        UNDEFINED_STRING_LITERAL_ID_GLOBAL_TYPE_ID => string_literal("undefined"),
        TYPEOF_OPERATOR_RETURN_UNION_ID_GLOBAL_TYPE_ID => {
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
        }
        T_ID_GLOBAL_TYPE_ID => TypeData::from(GenericTypeParameter {
            is_const: false,
            name: Text::new_static("T"),
            constraint: TypeReference::unknown(),
            default: TypeReference::unknown(),
        }),
        U_ID_GLOBAL_TYPE_ID => TypeData::from(GenericTypeParameter {
            is_const: false,
            name: Text::new_static("U"),
            constraint: TypeReference::unknown(),
            default: TypeReference::unknown(),
        }),
        CONDITIONAL_CALLBACK_ID_GLOBAL_TYPE_ID => TypeData::from(Function {
            is_async: false,
            type_parameters: Default::default(),
            name: Some(Text::new_static(CONDITIONAL_CALLBACK_ID_NAME)),
            parameters: Default::default(),
            return_type: ReturnType::Type(GLOBAL_CONDITIONAL_ID.into()),
        }),
        MAP_CALLBACK_ID_GLOBAL_TYPE_ID => TypeData::from(Function {
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
        VOID_CALLBACK_ID_GLOBAL_TYPE_ID => TypeData::from(Function {
            is_async: false,
            type_parameters: Default::default(),
            name: Some(Text::new_static(VOID_CALLBACK_ID_NAME)),
            parameters: Default::default(),
            return_type: ReturnType::Type(GLOBAL_VOID_ID.into()),
        }),
        FETCH_ID_GLOBAL_TYPE_ID => TypeData::from(Function {
            is_async: false,
            type_parameters: Default::default(),
            name: Some(Text::new_static(FETCH_ID_NAME)),
            parameters: Default::default(),
            return_type: ReturnType::Type(GLOBAL_INSTANCEOF_PROMISE_ID.into()),
        }),
        INSTANCEOF_REG_EXP_ID_GLOBAL_TYPE_ID => {
            TypeData::instance_of(TypeReference::from(GLOBAL_REG_EXP_ID))
        }
        INSTANCEOF_DATE_ID_GLOBAL_TYPE_ID => {
            TypeData::instance_of(TypeReference::from(GLOBAL_DATE_ID))
        }
        INSTANCEOF_MAP_ID_GLOBAL_TYPE_ID => {
            TypeData::instance_of(TypeReference::from(GLOBAL_MAP_ID))
        }
        INSTANCEOF_SET_ID_GLOBAL_TYPE_ID => {
            TypeData::instance_of(TypeReference::from(GLOBAL_SET_ID))
        }
        INSTANCEOF_WEAK_MAP_ID_GLOBAL_TYPE_ID => {
            TypeData::instance_of(TypeReference::from(GLOBAL_WEAK_MAP_ID))
        }
        INSTANCEOF_ERROR_ID_GLOBAL_TYPE_ID => {
            TypeData::instance_of(TypeReference::from(GLOBAL_ERROR_ID))
        }
        INSTANCEOF_SYMBOL_ID_GLOBAL_TYPE_ID => {
            TypeData::instance_of(TypeReference::from(GLOBAL_SYMBOL_ID))
        }
        _ => unreachable!("every manifest row has hand-written type data"),
    }
}

/// Resolves a qualifier to a global declared by the TypeScript standard library.
///
/// Type-only qualifiers resolve to globals that type annotations can name. Other
/// qualifiers resolve to globals that expressions can name. Qualified paths such as
/// `Intl.DateTimeFormatOptions` resolve through their full name.
pub fn global_type_id_for_qualifier(qualifier: &TypeReferenceQualifier) -> Option<GlobalTypeId> {
    let joined;
    let name = match qualifier.path.identifier() {
        Some(identifier) => identifier.text(),
        None => {
            joined = qualifier
                .path
                .iter()
                .map(Text::text)
                .collect::<Vec<_>>()
                .join(".");
            joined.as_str()
        }
    };
    if qualifier.type_only {
        lookup_global(crate::generated::global_types::TYPE_GLOBALS, name)
    } else {
        global_type_id_for_value(name)
    }
}

/// Resolves a name that an expression refers to.
pub fn global_type_id_for_value(name: &str) -> Option<GlobalTypeId> {
    match name {
        "fetch" => Some(FETCH_ID_GLOBAL_TYPE_ID),
        "globalThis" | "window" => Some(GLOBAL_ID_GLOBAL_TYPE_ID),
        _ => lookup_global(crate::generated::global_types::VALUE_GLOBALS, name),
    }
}

/// Finds `name` in a generated index sorted by name.
fn lookup_global(index: &[(&str, GlobalTypeId)], name: &str) -> Option<GlobalTypeId> {
    index
        .binary_search_by(|(declared, _)| (*declared).cmp(name))
        .ok()
        .map(|position| index[position].1)
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

/// Returns whether `id` is a predefined keyword type such as `any` or `string`.
fn is_keyword(id: GlobalTypeId) -> bool {
    [
        ANY_KEYWORD_ID_GLOBAL_TYPE_ID,
        BIGINT_KEYWORD_ID_GLOBAL_TYPE_ID,
        BOOLEAN_KEYWORD_ID_GLOBAL_TYPE_ID,
        NEVER_KEYWORD_ID_GLOBAL_TYPE_ID,
        NULL_KEYWORD_ID_GLOBAL_TYPE_ID,
        NUMBER_KEYWORD_ID_GLOBAL_TYPE_ID,
        OBJECT_KEYWORD_ID_GLOBAL_TYPE_ID,
        STRING_KEYWORD_ID_GLOBAL_TYPE_ID,
        SYMBOL_KEYWORD_ID_GLOBAL_TYPE_ID,
        UNDEFINED_ID_GLOBAL_TYPE_ID,
        UNKNOWN_KEYWORD_ID_GLOBAL_TYPE_ID,
        VOID_ID_GLOBAL_TYPE_ID,
    ]
    .contains(&id)
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
                // Keywords and the typeof result's literals resolve to their data rather than
                // deferred global handles, because matchers such as `is_any_keyword()` don't
                // expand handles.
                if is_keyword(*target)
                    || (local.is_none() && owner == TYPEOF_OPERATOR_RETURN_UNION_ID_GLOBAL_TYPE_ID)
                {
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
    #[cfg(debug_assertions)]
    fn every_global_has_type_data() {
        for index in 0..NUM_PREDEFINED_TYPES {
            raw_global_type(GlobalTypeId::new_for_test(index));
        }
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
