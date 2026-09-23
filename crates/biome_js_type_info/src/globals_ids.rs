//! Global IDs consist of a fixed manifest followed by generated declarations.
//!
//! The manifest holds intrinsics that no TypeScript declaration file defines, such
//! as primitive keywords and `typeof` results. Row position in
//! [`PREDEFINED_ID_ROWS`] is the fixed `TypeId` value. Every global declared by the
//! TypeScript standard library follows the manifest, in the order chosen by
//! `just gen-global-types`.

use std::cmp::Ordering;

use crate::{RawTypeId, TypeId};

/// Compile-time guard for manifest length; ordering is checked by `manifest_names_match_id_name_constants`.
const PREDEFINED_TYPE_COUNT: usize = 33;

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

        /// Ordered manifest of fixed predefined global type IDs.
        pub(crate) const PREDEFINED_ID_ROWS: &[&str] = &[
            $(
                $id_name,
            )+
        ];

        const _: () = assert!(PREDEFINED_ID_ROWS.len() == PREDEFINED_TYPE_COUNT);

        /// Number of fixed and generated global type IDs.
        pub const NUM_PREDEFINED_TYPES: usize = PREDEFINED_ID_ROWS.len()
            + crate::generated::global_types::GENERATED_GLOBAL_NAMES.len();

        /// Returns a string for formatting global IDs in test snapshots.
        pub(crate) fn global_type_name(id: TypeId) -> Option<&'static str> {
            PREDEFINED_ID_ROWS.get(id.index()).copied().or_else(|| {
                let index = id.index().checked_sub(PREDEFINED_ID_ROWS.len())?;
                crate::generated::global_types::GENERATED_GLOBAL_NAMES.get(index).copied()
            })
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
    (NUMBER_KEYWORD_ID, NUMBER_KEYWORD_ID_NAME, NUMBER_KEYWORD_ID_GLOBAL_TYPE_ID, GLOBAL_NUMBER_KEYWORD_ID, "number", Primitive),
    (STRING_KEYWORD_ID, STRING_KEYWORD_ID_NAME, STRING_KEYWORD_ID_GLOBAL_TYPE_ID, GLOBAL_STRING_KEYWORD_ID, "string", Primitive),
    (BOOLEAN_KEYWORD_ID, BOOLEAN_KEYWORD_ID_NAME, BOOLEAN_KEYWORD_ID_GLOBAL_TYPE_ID, _, "boolean", Primitive),
    (INSTANCEOF_ARRAY_T_ID, INSTANCEOF_ARRAY_T_ID_NAME, INSTANCEOF_ARRAY_T_ID_GLOBAL_TYPE_ID, GLOBAL_INSTANCEOF_ARRAY_T_ID, "instanceof Array<T>", Helper),
    (INSTANCEOF_ARRAY_U_ID, INSTANCEOF_ARRAY_U_ID_NAME, INSTANCEOF_ARRAY_U_ID_GLOBAL_TYPE_ID, GLOBAL_INSTANCEOF_ARRAY_U_ID, "instanceof Array<U>", Helper),
    (GLOBAL_ID, GLOBAL_ID_NAME, GLOBAL_ID_GLOBAL_TYPE_ID, GLOBAL_GLOBAL_ID, "globalThis", Helper),
    (INSTANCEOF_PROMISE_ID, INSTANCEOF_PROMISE_ID_NAME, INSTANCEOF_PROMISE_ID_GLOBAL_TYPE_ID, GLOBAL_INSTANCEOF_PROMISE_ID, "instanceof Promise", Helper),
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
    (CONDITIONAL_CALLBACK_ID, CONDITIONAL_CALLBACK_ID_NAME, CONDITIONAL_CALLBACK_ID_GLOBAL_TYPE_ID, GLOBAL_CONDITIONAL_CALLBACK_ID, "() => conditional", Helper),
    (MAP_CALLBACK_ID, MAP_CALLBACK_ID_NAME, MAP_CALLBACK_ID_GLOBAL_TYPE_ID, GLOBAL_MAP_CALLBACK_ID, "<U>(item: T) => U", Helper),
    (VOID_CALLBACK_ID, VOID_CALLBACK_ID_NAME, VOID_CALLBACK_ID_GLOBAL_TYPE_ID, GLOBAL_VOID_CALLBACK_ID, "() => void", Helper),
    (FETCH_ID, FETCH_ID_NAME, FETCH_ID_GLOBAL_TYPE_ID, _, "fetch", HostManual),
    (INSTANCEOF_REG_EXP_ID, INSTANCEOF_REG_EXP_ID_NAME, INSTANCEOF_REG_EXP_ID_GLOBAL_TYPE_ID, _, "instanceof RegExp", Helper),
    (INSTANCEOF_SYMBOL_ID, INSTANCEOF_SYMBOL_ID_NAME, INSTANCEOF_SYMBOL_ID_GLOBAL_TYPE_ID, _, "instanceof Symbol", Helper),
    (INSTANCEOF_DATE_ID, INSTANCEOF_DATE_ID_NAME, INSTANCEOF_DATE_ID_GLOBAL_TYPE_ID, _, "instanceof Date", Helper),
    (INSTANCEOF_MAP_ID, INSTANCEOF_MAP_ID_NAME, INSTANCEOF_MAP_ID_GLOBAL_TYPE_ID, _, "instanceof Map", Helper),
    (INSTANCEOF_SET_ID, INSTANCEOF_SET_ID_NAME, INSTANCEOF_SET_ID_GLOBAL_TYPE_ID, _, "instanceof Set", Helper),
    (INSTANCEOF_WEAK_MAP_ID, INSTANCEOF_WEAK_MAP_ID_NAME, INSTANCEOF_WEAK_MAP_ID_GLOBAL_TYPE_ID, _, "instanceof WeakMap", Helper),
    (INSTANCEOF_ERROR_ID, INSTANCEOF_ERROR_ID_NAME, INSTANCEOF_ERROR_ID_GLOBAL_TYPE_ID, GLOBAL_INSTANCEOF_ERROR_ID, "instanceof Error", Helper),
}
