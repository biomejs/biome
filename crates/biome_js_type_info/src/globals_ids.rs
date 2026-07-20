//! Predefined global type IDs derived from one ordered manifest, [`PREDEFINED_ID_ROWS`].
//! Row position is the `TypeId` value, so the manifest is append-only: reordering
//! or removing rows shifts every consumer's `*_ID` constant.

use std::cmp::Ordering;

use crate::{RawTypeId, TypeId};

/// Compile-time guard for manifest length; ordering is checked by `manifest_names_match_id_name_constants`.
const PREDEFINED_TYPE_COUNT: usize = 65;

/// Type ID that is known to index the predefined global resolver.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, salsa::Update)]
pub struct GlobalTypeId(TypeId);

impl GlobalTypeId {
    /// Wraps a manifest type ID whose index is known at compile time.
    pub(crate) const fn new(id: TypeId) -> Self {
        Self(id)
    }

    /// Converts a type ID into a predefined global type ID when it indexes the
    /// global manifest.
    pub const fn try_from_type_id(id: TypeId) -> Option<Self> {
        if id.index() < NUM_PREDEFINED_TYPES {
            Some(Self(id))
        } else {
            None
        }
    }

    /// Test-only constructor for synthesizing `GlobalTypeId` values at
    /// arbitrary indices, including out-of-range indices used by negative-path
    /// tests.
    #[cfg(all(test, debug_assertions))]
    pub(crate) const fn new_for_test(index: usize) -> Self {
        Self(TypeId::new(index))
    }

    /// Unwraps to the underlying [`TypeId`] for APIs that do not require the
    /// predefined-resolver invariant carried by `GlobalTypeId`.
    pub const fn as_type_id(self) -> TypeId {
        self.0
    }

    /// Returns the manifest row position (0-based) backing this ID.
    pub const fn index(self) -> usize {
        self.0.index()
    }
}

impl From<GlobalTypeId> for TypeId {
    fn from(id: GlobalTypeId) -> Self {
        id.as_type_id()
    }
}

impl Ord for GlobalTypeId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index().cmp(&other.index())
    }
}

impl PartialOrd for GlobalTypeId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Expands the manifest into IDs, names, roles, required raw aliases, and the name lookup.
macro_rules! predefined_globals {
    ($(($id:ident, $id_name:ident, $global_type_id:ident, $resolved_id:tt, $name:literal, $role:ident $(,)?)),+ $(,)?) => {
        predefined_global_ids!(0usize; $(($id, $id_name, $global_type_id, $resolved_id, $name, $role)),+);

        /// Single ordered manifest of every predefined global type ID.
        pub(crate) const PREDEFINED_ID_ROWS: &[&str] = &[
            $(
                $id_name,
            )+
        ];

        const _: () = assert!(PREDEFINED_ID_ROWS.len() == PREDEFINED_TYPE_COUNT);

        /// Number of predefined global type IDs derived from the manifest.
        pub const NUM_PREDEFINED_TYPES: usize = PREDEFINED_ID_ROWS.len();

        /// Returns a string for formatting global IDs in test snapshots.
        pub(crate) fn global_type_name(id: TypeId) -> Option<&'static str> {
            PREDEFINED_ID_ROWS.get(id.index()).copied()
        }
    };
}

macro_rules! predefined_global_alias {
    ($global_type_id:ident, _) => {};
    ($global_type_id:ident, $resolved_id:ident) => {
        pub const $resolved_id: RawTypeId = RawTypeId::Global($global_type_id);
    };
}

/// Recursive helper that emits each row's ID, global ID, name, and optional raw alias.
macro_rules! predefined_global_ids {
    ($index:expr;) => {};
    ($index:expr; ($id:ident, $id_name:ident, $global_type_id:ident, $resolved_id:tt, $name:literal, $role:ident $(,)?) $(, ($tail_id:ident, $tail_id_name:ident, $tail_global_type_id:ident, $tail_resolved_id:tt, $tail_name:literal, $tail_role:ident $(,)?))* $(,)?) => {
        pub const $id: TypeId = TypeId::new($index);
        pub(crate) const $global_type_id: GlobalTypeId = GlobalTypeId::new($id);
        pub const $id_name: &str = $name;
        predefined_global_alias!($global_type_id, $resolved_id);

        predefined_global_ids!($index + 1usize; $(($tail_id, $tail_id_name, $tail_global_type_id, $tail_resolved_id, $tail_name, $tail_role)),*);
    };
}

predefined_globals! {
    (UNKNOWN_ID, UNKNOWN_ID_NAME, UNKNOWN_ID_GLOBAL_TYPE_ID, GLOBAL_UNKNOWN_ID, "unknown", Sentinel),
    (UNDEFINED_ID, UNDEFINED_ID_NAME, UNDEFINED_ID_GLOBAL_TYPE_ID, GLOBAL_UNDEFINED_ID, "undefined", Sentinel),
    (VOID_ID, VOID_ID_NAME, VOID_ID_GLOBAL_TYPE_ID, GLOBAL_VOID_ID, "void", Sentinel),
    (CONDITIONAL_ID, CONDITIONAL_ID_NAME, CONDITIONAL_ID_GLOBAL_TYPE_ID, GLOBAL_CONDITIONAL_ID, "conditional", Sentinel),
    (NUMBER_ID, NUMBER_ID_NAME, NUMBER_ID_GLOBAL_TYPE_ID, GLOBAL_NUMBER_ID, "number", Primitive),
    (STRING_ID, STRING_ID_NAME, STRING_ID_GLOBAL_TYPE_ID, GLOBAL_STRING_ID, "string", Primitive),
    (INSTANCEOF_ARRAY_T_ID, INSTANCEOF_ARRAY_T_ID_NAME, INSTANCEOF_ARRAY_T_ID_GLOBAL_TYPE_ID, _, "instanceof Array<T>", Helper),
    (INSTANCEOF_ARRAY_U_ID, INSTANCEOF_ARRAY_U_ID_NAME, INSTANCEOF_ARRAY_U_ID_GLOBAL_TYPE_ID, _, "instanceof Array<U>", Helper),
    (ARRAY_ID, ARRAY_ID_NAME, ARRAY_ID_GLOBAL_TYPE_ID, GLOBAL_ARRAY_ID, "Array", ManualGlobal),
    (ARRAY_FILTER_ID, ARRAY_FILTER_ID_NAME, ARRAY_FILTER_ID_GLOBAL_TYPE_ID, _, "Array.prototype.filter", ManualSynthetic),
    (ARRAY_FOREACH_ID, ARRAY_FOREACH_ID_NAME, ARRAY_FOREACH_ID_GLOBAL_TYPE_ID, _, "Array.prototype.forEach", ManualSynthetic),
    (ARRAY_MAP_ID, ARRAY_MAP_ID_NAME, ARRAY_MAP_ID_GLOBAL_TYPE_ID, _, "Array.prototype.map", ManualSynthetic),
    (GLOBAL_ID, GLOBAL_ID_NAME, GLOBAL_ID_GLOBAL_TYPE_ID, GLOBAL_GLOBAL_ID, "globalThis", Helper),
    (INSTANCEOF_PROMISE_ID, INSTANCEOF_PROMISE_ID_NAME, INSTANCEOF_PROMISE_ID_GLOBAL_TYPE_ID, GLOBAL_INSTANCEOF_PROMISE_ID, "instanceof Promise", Helper),
    (PROMISE_ID, PROMISE_ID_NAME, PROMISE_ID_GLOBAL_TYPE_ID, GLOBAL_PROMISE_ID, "Promise", ManualGlobal),
    (PROMISE_CONSTRUCTOR_ID, PROMISE_CONSTRUCTOR_ID_NAME, PROMISE_CONSTRUCTOR_ID_GLOBAL_TYPE_ID, GLOBAL_PROMISE_CONSTRUCTOR_ID, "Promise.constructor", ManualSynthetic),
    (PROMISE_CATCH_ID, PROMISE_CATCH_ID_NAME, PROMISE_CATCH_ID_GLOBAL_TYPE_ID, _, "Promise.prototype.catch", ManualSynthetic),
    (PROMISE_FINALLY_ID, PROMISE_FINALLY_ID_NAME, PROMISE_FINALLY_ID_GLOBAL_TYPE_ID, _, "Promise.prototype.finally", ManualSynthetic),
    (PROMISE_THEN_ID, PROMISE_THEN_ID_NAME, PROMISE_THEN_ID_GLOBAL_TYPE_ID, _, "Promise.prototype.then", ManualSynthetic),
    (PROMISE_ALL_ID, PROMISE_ALL_ID_NAME, PROMISE_ALL_ID_GLOBAL_TYPE_ID, _, "Promise.all", ManualSynthetic),
    (PROMISE_ALL_SETTLED_ID, PROMISE_ALL_SETTLED_ID_NAME, PROMISE_ALL_SETTLED_ID_GLOBAL_TYPE_ID, _, "Promise.allSettled", ManualSynthetic),
    (PROMISE_ANY_ID, PROMISE_ANY_ID_NAME, PROMISE_ANY_ID_GLOBAL_TYPE_ID, _, "Promise.any", ManualSynthetic),
    (PROMISE_RACE_ID, PROMISE_RACE_ID_NAME, PROMISE_RACE_ID_GLOBAL_TYPE_ID, _, "Promise.race", ManualSynthetic),
    (PROMISE_REJECT_ID, PROMISE_REJECT_ID_NAME, PROMISE_REJECT_ID_GLOBAL_TYPE_ID, _, "Promise.reject", ManualSynthetic),
    (PROMISE_RESOLVE_ID, PROMISE_RESOLVE_ID_NAME, PROMISE_RESOLVE_ID_GLOBAL_TYPE_ID, _, "Promise.resolve", ManualSynthetic),
    (PROMISE_TRY_ID, PROMISE_TRY_ID_NAME, PROMISE_TRY_ID_GLOBAL_TYPE_ID, _, "Promise.try", ManualSynthetic),
    (BIGINT_STRING_LITERAL_ID, BIGINT_STRING_LITERAL_ID_NAME, BIGINT_STRING_LITERAL_ID_GLOBAL_TYPE_ID, GLOBAL_BIGINT_STRING_LITERAL_ID, "\"bigint\"", Helper),
    (BOOLEAN_STRING_LITERAL_ID, BOOLEAN_STRING_LITERAL_ID_NAME, BOOLEAN_STRING_LITERAL_ID_GLOBAL_TYPE_ID, GLOBAL_BOOLEAN_STRING_LITERAL_ID, "\"boolean\"", Helper),
    (FUNCTION_STRING_LITERAL_ID, FUNCTION_STRING_LITERAL_ID_NAME, FUNCTION_STRING_LITERAL_ID_GLOBAL_TYPE_ID, GLOBAL_FUNCTION_STRING_LITERAL_ID, "\"function\"", Helper),
    (NUMBER_STRING_LITERAL_ID, NUMBER_STRING_LITERAL_ID_NAME, NUMBER_STRING_LITERAL_ID_GLOBAL_TYPE_ID, GLOBAL_NUMBER_STRING_LITERAL_ID, "\"number\"", Helper),
    (OBJECT_STRING_LITERAL_ID, OBJECT_STRING_LITERAL_ID_NAME, OBJECT_STRING_LITERAL_ID_GLOBAL_TYPE_ID, GLOBAL_OBJECT_STRING_LITERAL_ID, "\"object\"", Helper),
    (STRING_STRING_LITERAL_ID, STRING_STRING_LITERAL_ID_NAME, STRING_STRING_LITERAL_ID_GLOBAL_TYPE_ID, GLOBAL_STRING_STRING_LITERAL_ID, "\"string\"", Helper),
    (SYMBOL_STRING_LITERAL_ID, SYMBOL_STRING_LITERAL_ID_NAME, SYMBOL_STRING_LITERAL_ID_GLOBAL_TYPE_ID, GLOBAL_SYMBOL_STRING_LITERAL_ID, "\"symbol\"", Helper),
    (UNDEFINED_STRING_LITERAL_ID, UNDEFINED_STRING_LITERAL_ID_NAME, UNDEFINED_STRING_LITERAL_ID_GLOBAL_TYPE_ID, GLOBAL_UNDEFINED_STRING_LITERAL_ID, "\"undefined\"", Helper),
    (TYPEOF_OPERATOR_RETURN_UNION_ID, TYPEOF_OPERATOR_RETURN_UNION_ID_NAME, TYPEOF_OPERATOR_RETURN_UNION_ID_GLOBAL_TYPE_ID, _,
        "\"bigint\" | \"boolean\" | \"function\" | \"number\" | \"object\" | \"string\" | \"symbol\" | \"undefined\"",
        Helper,
    ),
    (T_ID, T_ID_NAME, T_ID_GLOBAL_TYPE_ID, GLOBAL_T_ID, "T", Helper),
    (U_ID, U_ID_NAME, U_ID_GLOBAL_TYPE_ID, GLOBAL_U_ID, "U", Helper),
    (CONDITIONAL_CALLBACK_ID, CONDITIONAL_CALLBACK_ID_NAME, CONDITIONAL_CALLBACK_ID_GLOBAL_TYPE_ID, _, "() => conditional", Helper),
    (MAP_CALLBACK_ID, MAP_CALLBACK_ID_NAME, MAP_CALLBACK_ID_GLOBAL_TYPE_ID, _, "<U>(item: T) => U", Helper),
    (VOID_CALLBACK_ID, VOID_CALLBACK_ID_NAME, VOID_CALLBACK_ID_GLOBAL_TYPE_ID, _, "() => void", Helper),
    (FETCH_ID, FETCH_ID_NAME, FETCH_ID_GLOBAL_TYPE_ID, _, "fetch", HostManual),
    (INSTANCEOF_REGEXP_ID, INSTANCEOF_REGEXP_ID_NAME, INSTANCEOF_REGEXP_ID_GLOBAL_TYPE_ID, GLOBAL_INSTANCEOF_REGEXP_ID, "instanceof RegExp", Helper),
    (REGEXP_ID, REGEXP_ID_NAME, REGEXP_ID_GLOBAL_TYPE_ID, GLOBAL_REGEXP_ID, "RegExp", ManualGlobal),
    (REGEXP_EXEC_ID, REGEXP_EXEC_ID_NAME, REGEXP_EXEC_ID_GLOBAL_TYPE_ID, _, "RegExp.exec", ManualSynthetic),
    (INSTANCEOF_SYMBOL_ID, INSTANCEOF_SYMBOL_ID_NAME, INSTANCEOF_SYMBOL_ID_GLOBAL_TYPE_ID, _, "instanceof Symbol", Helper),
    (SYMBOL_ID, SYMBOL_ID_NAME, SYMBOL_ID_GLOBAL_TYPE_ID, GLOBAL_SYMBOL_ID, "Symbol", ManualGlobal),
    (SYMBOL_DISPOSE_ID, SYMBOL_DISPOSE_ID_NAME, SYMBOL_DISPOSE_ID_GLOBAL_TYPE_ID, GLOBAL_SYMBOL_DISPOSE_ID, "Symbol.dispose", ManualSynthetic),
    (SYMBOL_ASYNC_DISPOSE_ID, SYMBOL_ASYNC_DISPOSE_ID_NAME, SYMBOL_ASYNC_DISPOSE_ID_GLOBAL_TYPE_ID, GLOBAL_SYMBOL_ASYNC_DISPOSE_ID, "Symbol.asyncDispose", ManualSynthetic),
    (DISPOSABLE_ID, DISPOSABLE_ID_NAME, DISPOSABLE_ID_GLOBAL_TYPE_ID, _, "Disposable", ManualGlobal),
    (DISPOSABLE_DISPOSE_ID, DISPOSABLE_DISPOSE_ID_NAME, DISPOSABLE_DISPOSE_ID_GLOBAL_TYPE_ID, GLOBAL_DISPOSABLE_DISPOSE_ID, "Disposable[Symbol.dispose]", ManualSynthetic),
    (ASYNC_DISPOSABLE_ID, ASYNC_DISPOSABLE_ID_NAME, ASYNC_DISPOSABLE_ID_GLOBAL_TYPE_ID, _, "AsyncDisposable", ManualGlobal),
    (ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID, ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID_NAME, ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID_GLOBAL_TYPE_ID, GLOBAL_ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID,
        "AsyncDisposable[Symbol.asyncDispose]",
        ManualSynthetic,
    ),
    (INSTANCEOF_DATE_ID, INSTANCEOF_DATE_ID_NAME, INSTANCEOF_DATE_ID_GLOBAL_TYPE_ID, _, "instanceof Date", Helper),
    (DATE_ID, DATE_ID_NAME, DATE_ID_GLOBAL_TYPE_ID, GLOBAL_DATE_ID, "Date", ManualGlobal),
    (INSTANCEOF_MAP_ID, INSTANCEOF_MAP_ID_NAME, INSTANCEOF_MAP_ID_GLOBAL_TYPE_ID, _, "instanceof Map", Helper),
    (MAP_ID, MAP_ID_NAME, MAP_ID_GLOBAL_TYPE_ID, GLOBAL_MAP_ID, "Map", ManualGlobal),
    (INSTANCEOF_SET_ID, INSTANCEOF_SET_ID_NAME, INSTANCEOF_SET_ID_GLOBAL_TYPE_ID, _, "instanceof Set", Helper),
    (SET_ID, SET_ID_NAME, SET_ID_GLOBAL_TYPE_ID, GLOBAL_SET_ID, "Set", ManualGlobal),
    (INSTANCEOF_WEAK_MAP_ID, INSTANCEOF_WEAK_MAP_ID_NAME, INSTANCEOF_WEAK_MAP_ID_GLOBAL_TYPE_ID, _, "instanceof WeakMap", Helper),
    (WEAK_MAP_ID, WEAK_MAP_ID_NAME, WEAK_MAP_ID_GLOBAL_TYPE_ID, GLOBAL_WEAK_MAP_ID, "WeakMap", ManualGlobal),
    (INSTANCEOF_ERROR_ID, INSTANCEOF_ERROR_ID_NAME, INSTANCEOF_ERROR_ID_GLOBAL_TYPE_ID, GLOBAL_INSTANCEOF_ERROR_ID, "instanceof Error", Helper),
    (ERROR_ID, ERROR_ID_NAME, ERROR_ID_GLOBAL_TYPE_ID, GLOBAL_ERROR_ID, "Error", ManualGlobal),
    (BOOLEAN_ID, BOOLEAN_ID_NAME, BOOLEAN_ID_GLOBAL_TYPE_ID, _, "boolean", Primitive),
    (ERROR_CONSTRUCTOR_ID, ERROR_CONSTRUCTOR_ID_NAME, ERROR_CONSTRUCTOR_ID_GLOBAL_TYPE_ID, GLOBAL_ERROR_CONSTRUCTOR_ID, "Error.constructor", ManualSynthetic),
    (ERROR_CALL_ID, ERROR_CALL_ID_NAME, ERROR_CALL_ID_GLOBAL_TYPE_ID, GLOBAL_ERROR_CALL_ID, "Error.call", ManualSynthetic),
}
