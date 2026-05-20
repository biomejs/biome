//! Predefined global type IDs derived from one ordered manifest, [`PREDEFINED_ID_ROWS`].
//! Row position is the `TypeId` value, so the manifest is append-only: reordering
//! or removing rows shifts every consumer's `*_ID` constant.

use std::cmp::Ordering;
use std::sync::LazyLock;

use biome_rowan::Text;

use crate::{ResolvedTypeId, TypeId, TypeMember, TypeMemberKind};

use super::globals::GLOBAL_LEVEL;

/// Compile-time guard for manifest length; ordering is checked by `manifest_names_match_id_name_constants`.
const PREDEFINED_TYPE_COUNT: usize = 63;

/// Type ID that is known to index the predefined global resolver.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct GlobalTypeId(TypeId);

impl GlobalTypeId {
    /// Wraps a `TypeId` that has been verified by the caller to address a
    /// predefined manifest slot. Internal constructor only.
    const fn from_type_id(id: TypeId) -> Self {
        Self(id)
    }

    /// Test-only constructor for synthesizing `GlobalTypeId` values at
    /// arbitrary indices, including out-of-range indices used by negative-path
    /// tests.
    #[cfg(test)]
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

/// Role of a predefined global manifest row.
///
/// `Sentinel`, `Primitive`, `Helper`, and `HostManual` rows stay manual
/// forever; `ManualGlobal` and `ManualSynthetic` rows are hand-written today
/// but will be replaced by generated entries as `MIGRATED_PREDEFINED_IDS` grows.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PredefinedGlobalRole {
    /// Reserved type ID (e.g. `unknown`, `undefined`, `void`); never generated.
    Sentinel,
    /// JS primitive types (`number`, `string`, `boolean`); never generated.
    Primitive,
    /// Synthetic helper instance/quoted-typeof rows that back resolver
    /// invariants; never generated.
    Helper,
    /// Host-side ambient binding (e.g. `fetch`) that must stay manual
    /// because TypeScript's `.d.ts` does not describe it.
    HostManual,
    /// Built-in global class/interface that codegen will replace once the
    /// matching `lib.*.d.ts` is migrated into `MIGRATED_PREDEFINED_IDS`.
    ManualGlobal,
    /// Synthetic projection (e.g. `Array.prototype.filter` member type) that
    /// will be derived from the generated class once lowering lands.
    ManualSynthetic,
}

impl PredefinedGlobalRole {
    /// True when the role belongs to the hand-maintained sentinel projection
    /// (`Sentinel` / `Primitive` / `Helper`) that must never appear in
    /// `MIGRATED_PREDEFINED_IDS`.
    pub(crate) const fn is_sentinel_projection(self) -> bool {
        matches!(self, Self::Sentinel | Self::Primitive | Self::Helper)
    }
}

/// One row in the predefined global manifest.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PredefinedGlobalRow {
    pub(crate) id: GlobalTypeId,
    pub(crate) name: &'static str,
    pub(crate) role: PredefinedGlobalRole,
}

/// Expands the manifest into IDs, names, roles, resolved global aliases, and the name lookup.
macro_rules! predefined_globals {
    ($(($id:ident, $name:literal, $role:ident $(,)?)),+ $(,)?) => {
        predefined_global_ids!(0usize; $(($id, $name, $role)),+);

        pastey::paste! {
            /// Single ordered manifest of every predefined global type ID.
            pub(crate) const PREDEFINED_ID_ROWS: &[&str] = &[
                $(
                    [<$id _NAME>],
                )+
            ];

            const _: () = assert!(PREDEFINED_ID_ROWS.len() == PREDEFINED_TYPE_COUNT);

            /// Single role-aware manifest for predefined global type IDs.
            pub(crate) const PREDEFINED_GLOBAL_ROWS: &[PredefinedGlobalRow] = &[
                $(
                    PredefinedGlobalRow {
                        id: [<$id _GLOBAL_TYPE_ID>],
                        name: [<$id _NAME>],
                        role: PredefinedGlobalRole::$role,
                    },
                )+
            ];

            /// Manifest-emitted member list used by every type-info resolver.
            pub(crate) static GLOBAL_TYPE_MEMBERS: LazyLock<Vec<TypeMember>> =
                LazyLock::new(|| {
                    vec![
                        $(
                            TypeMember {
                                kind: TypeMemberKind::Named(
                                    Text::new_static([<$id _NAME>]),
                                ),
                                ty: [<GLOBAL_ $id>].into(),
                            },
                        )+
                    ]
                });

            /// Hand-maintained predefined IDs that must not be generated.
            #[cfg_attr(
                not(test),
                expect(dead_code, reason = "consumed by migration guard tests and future generated rows")
            )]
            pub(crate) static SENTINEL_PREDEFINED_IDS: LazyLock<Box<[GlobalTypeId]>> =
                LazyLock::new(|| {
                    PREDEFINED_GLOBAL_ROWS
                        .iter()
                        .filter(|row| row.role.is_sentinel_projection())
                        .map(|row| row.id)
                        .collect()
                });
        }

        /// Number of predefined global type IDs derived from the manifest.
        pub const NUM_PREDEFINED_TYPES: usize = PREDEFINED_ID_ROWS.len();

        /// Returns a string for formatting global IDs in test snapshots.
        pub(crate) fn global_type_name(id: TypeId) -> Option<&'static str> {
            PREDEFINED_ID_ROWS.get(id.index()).copied()
        }
    };
}

/// Recursive helper of [`predefined_globals!`] that emits each row's ID, name, and resolved alias.
macro_rules! predefined_global_ids {
    ($index:expr;) => {};
    ($index:expr; ($id:ident, $name:literal, $role:ident $(,)?) $(, ($tail_id:ident, $tail_name:literal, $tail_role:ident $(,)?))* $(,)?) => {
        pastey::paste! {
            pub const $id: TypeId = TypeId::new($index);
            pub(crate) const [<$id _GLOBAL_TYPE_ID>]: GlobalTypeId = GlobalTypeId::from_type_id($id);
            pub const [<$id _NAME>]: &str = $name;
            pub const [<GLOBAL_ $id>]: ResolvedTypeId =
                ResolvedTypeId::new(GLOBAL_LEVEL, $id);
        }

        predefined_global_ids!($index + 1usize; $(($tail_id, $tail_name, $tail_role)),*);
    };
}

predefined_globals! {
    (UNKNOWN_ID, "unknown", Sentinel),
    (UNDEFINED_ID, "undefined", Sentinel),
    (VOID_ID, "void", Sentinel),
    (CONDITIONAL_ID, "conditional", Sentinel),
    (NUMBER_ID, "number", Primitive),
    (STRING_ID, "string", Primitive),
    (INSTANCEOF_ARRAY_T_ID, "instanceof Array<T>", Helper),
    (INSTANCEOF_ARRAY_U_ID, "instanceof Array<U>", Helper),
    (ARRAY_ID, "Array", ManualGlobal),
    (ARRAY_FILTER_ID, "Array.prototype.filter", ManualSynthetic),
    (ARRAY_FOREACH_ID, "Array.prototype.forEach", ManualSynthetic),
    (ARRAY_MAP_ID, "Array.prototype.map", ManualSynthetic),
    (GLOBAL_ID, "globalThis", Helper),
    (INSTANCEOF_PROMISE_ID, "instanceof Promise", Helper),
    (PROMISE_ID, "Promise", ManualGlobal),
    (PROMISE_CONSTRUCTOR_ID, "Promise.constructor", ManualSynthetic),
    (PROMISE_CATCH_ID, "Promise.prototype.catch", ManualSynthetic),
    (PROMISE_FINALLY_ID, "Promise.prototype.finally", ManualSynthetic),
    (PROMISE_THEN_ID, "Promise.prototype.then", ManualSynthetic),
    (PROMISE_ALL_ID, "Promise.all", ManualSynthetic),
    (PROMISE_ALL_SETTLED_ID, "Promise.allSettled", ManualSynthetic),
    (PROMISE_ANY_ID, "Promise.any", ManualSynthetic),
    (PROMISE_RACE_ID, "Promise.race", ManualSynthetic),
    (PROMISE_REJECT_ID, "Promise.reject", ManualSynthetic),
    (PROMISE_RESOLVE_ID, "Promise.resolve", ManualSynthetic),
    (PROMISE_TRY_ID, "Promise.try", ManualSynthetic),
    (BIGINT_STRING_LITERAL_ID, "\"bigint\"", Helper),
    (BOOLEAN_STRING_LITERAL_ID, "\"boolean\"", Helper),
    (FUNCTION_STRING_LITERAL_ID, "\"function\"", Helper),
    (NUMBER_STRING_LITERAL_ID, "\"number\"", Helper),
    (OBJECT_STRING_LITERAL_ID, "\"object\"", Helper),
    (STRING_STRING_LITERAL_ID, "\"string\"", Helper),
    (SYMBOL_STRING_LITERAL_ID, "\"symbol\"", Helper),
    (UNDEFINED_STRING_LITERAL_ID, "\"undefined\"", Helper),
    (
        TYPEOF_OPERATOR_RETURN_UNION_ID,
        "\"bigint\" | \"boolean\" | \"function\" | \"number\" | \"object\" | \"string\" | \"symbol\" | \"undefined\"",
        Helper,
    ),
    (T_ID, "T", Helper),
    (U_ID, "U", Helper),
    (CONDITIONAL_CALLBACK_ID, "() => conditional", Helper),
    (MAP_CALLBACK_ID, "<U>(item: T) => U", Helper),
    (VOID_CALLBACK_ID, "() => void", Helper),
    (FETCH_ID, "fetch", HostManual),
    (INSTANCEOF_REGEXP_ID, "instanceof RegExp", Helper),
    (REGEXP_ID, "RegExp", ManualGlobal),
    (REGEXP_EXEC_ID, "RegExp.exec", ManualSynthetic),
    (INSTANCEOF_SYMBOL_ID, "instanceof Symbol", Helper),
    (SYMBOL_ID, "Symbol", ManualGlobal),
    (SYMBOL_DISPOSE_ID, "Symbol.dispose", ManualSynthetic),
    (SYMBOL_ASYNC_DISPOSE_ID, "Symbol.asyncDispose", ManualSynthetic),
    (DISPOSABLE_ID, "Disposable", ManualGlobal),
    (DISPOSABLE_DISPOSE_ID, "Disposable[Symbol.dispose]", ManualSynthetic),
    (ASYNC_DISPOSABLE_ID, "AsyncDisposable", ManualGlobal),
    (
        ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID,
        "AsyncDisposable[Symbol.asyncDispose]",
        ManualSynthetic,
    ),
    (INSTANCEOF_DATE_ID, "instanceof Date", Helper),
    (DATE_ID, "Date", ManualGlobal),
    (INSTANCEOF_MAP_ID, "instanceof Map", Helper),
    (MAP_ID, "Map", ManualGlobal),
    (INSTANCEOF_SET_ID, "instanceof Set", Helper),
    (SET_ID, "Set", ManualGlobal),
    (INSTANCEOF_WEAK_MAP_ID, "instanceof WeakMap", Helper),
    (WEAK_MAP_ID, "WeakMap", ManualGlobal),
    (INSTANCEOF_ERROR_ID, "instanceof Error", Helper),
    (ERROR_ID, "Error", ManualGlobal),
    (BOOLEAN_ID, "boolean", Primitive),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_predefined_types_matches_manifest_len() {
        assert_eq!(NUM_PREDEFINED_TYPES, PREDEFINED_TYPE_COUNT);
    }

    #[test]
    fn manifest_names_match_id_name_constants() {
        let pairs: &[(TypeId, &str)] = &[
            (UNKNOWN_ID, UNKNOWN_ID_NAME),
            (UNDEFINED_ID, UNDEFINED_ID_NAME),
            (VOID_ID, VOID_ID_NAME),
            (CONDITIONAL_ID, CONDITIONAL_ID_NAME),
            (NUMBER_ID, NUMBER_ID_NAME),
            (STRING_ID, STRING_ID_NAME),
            (INSTANCEOF_ARRAY_T_ID, INSTANCEOF_ARRAY_T_ID_NAME),
            (INSTANCEOF_ARRAY_U_ID, INSTANCEOF_ARRAY_U_ID_NAME),
            (ARRAY_ID, ARRAY_ID_NAME),
            (ARRAY_FILTER_ID, ARRAY_FILTER_ID_NAME),
            (ARRAY_FOREACH_ID, ARRAY_FOREACH_ID_NAME),
            (ARRAY_MAP_ID, ARRAY_MAP_ID_NAME),
            (GLOBAL_ID, GLOBAL_ID_NAME),
            (INSTANCEOF_PROMISE_ID, INSTANCEOF_PROMISE_ID_NAME),
            (PROMISE_ID, PROMISE_ID_NAME),
            (PROMISE_CONSTRUCTOR_ID, PROMISE_CONSTRUCTOR_ID_NAME),
            (PROMISE_CATCH_ID, PROMISE_CATCH_ID_NAME),
            (PROMISE_FINALLY_ID, PROMISE_FINALLY_ID_NAME),
            (PROMISE_THEN_ID, PROMISE_THEN_ID_NAME),
            (PROMISE_ALL_ID, PROMISE_ALL_ID_NAME),
            (PROMISE_ALL_SETTLED_ID, PROMISE_ALL_SETTLED_ID_NAME),
            (PROMISE_ANY_ID, PROMISE_ANY_ID_NAME),
            (PROMISE_RACE_ID, PROMISE_RACE_ID_NAME),
            (PROMISE_REJECT_ID, PROMISE_REJECT_ID_NAME),
            (PROMISE_RESOLVE_ID, PROMISE_RESOLVE_ID_NAME),
            (PROMISE_TRY_ID, PROMISE_TRY_ID_NAME),
            (BIGINT_STRING_LITERAL_ID, BIGINT_STRING_LITERAL_ID_NAME),
            (BOOLEAN_STRING_LITERAL_ID, BOOLEAN_STRING_LITERAL_ID_NAME),
            (FUNCTION_STRING_LITERAL_ID, FUNCTION_STRING_LITERAL_ID_NAME),
            (NUMBER_STRING_LITERAL_ID, NUMBER_STRING_LITERAL_ID_NAME),
            (OBJECT_STRING_LITERAL_ID, OBJECT_STRING_LITERAL_ID_NAME),
            (STRING_STRING_LITERAL_ID, STRING_STRING_LITERAL_ID_NAME),
            (SYMBOL_STRING_LITERAL_ID, SYMBOL_STRING_LITERAL_ID_NAME),
            (
                UNDEFINED_STRING_LITERAL_ID,
                UNDEFINED_STRING_LITERAL_ID_NAME,
            ),
            (
                TYPEOF_OPERATOR_RETURN_UNION_ID,
                TYPEOF_OPERATOR_RETURN_UNION_ID_NAME,
            ),
            (T_ID, T_ID_NAME),
            (U_ID, U_ID_NAME),
            (CONDITIONAL_CALLBACK_ID, CONDITIONAL_CALLBACK_ID_NAME),
            (MAP_CALLBACK_ID, MAP_CALLBACK_ID_NAME),
            (VOID_CALLBACK_ID, VOID_CALLBACK_ID_NAME),
            (FETCH_ID, FETCH_ID_NAME),
            (INSTANCEOF_REGEXP_ID, INSTANCEOF_REGEXP_ID_NAME),
            (REGEXP_ID, REGEXP_ID_NAME),
            (REGEXP_EXEC_ID, REGEXP_EXEC_ID_NAME),
            (INSTANCEOF_SYMBOL_ID, INSTANCEOF_SYMBOL_ID_NAME),
            (SYMBOL_ID, SYMBOL_ID_NAME),
            (SYMBOL_DISPOSE_ID, SYMBOL_DISPOSE_ID_NAME),
            (SYMBOL_ASYNC_DISPOSE_ID, SYMBOL_ASYNC_DISPOSE_ID_NAME),
            (DISPOSABLE_ID, DISPOSABLE_ID_NAME),
            (DISPOSABLE_DISPOSE_ID, DISPOSABLE_DISPOSE_ID_NAME),
            (ASYNC_DISPOSABLE_ID, ASYNC_DISPOSABLE_ID_NAME),
            (
                ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID,
                ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID_NAME,
            ),
            (INSTANCEOF_DATE_ID, INSTANCEOF_DATE_ID_NAME),
            (DATE_ID, DATE_ID_NAME),
            (INSTANCEOF_MAP_ID, INSTANCEOF_MAP_ID_NAME),
            (MAP_ID, MAP_ID_NAME),
            (INSTANCEOF_SET_ID, INSTANCEOF_SET_ID_NAME),
            (SET_ID, SET_ID_NAME),
            (INSTANCEOF_WEAK_MAP_ID, INSTANCEOF_WEAK_MAP_ID_NAME),
            (WEAK_MAP_ID, WEAK_MAP_ID_NAME),
            (INSTANCEOF_ERROR_ID, INSTANCEOF_ERROR_ID_NAME),
            (ERROR_ID, ERROR_ID_NAME),
            (BOOLEAN_ID, BOOLEAN_ID_NAME),
        ];
        assert_eq!(pairs.len(), NUM_PREDEFINED_TYPES);
        assert_eq!(pairs.len(), PREDEFINED_ID_ROWS.len());
        for (expected_index, (id, name)) in pairs.iter().enumerate() {
            assert_eq!(id.index(), expected_index, "legacy TypeId drift for {name}");
            let row = PREDEFINED_ID_ROWS[expected_index];
            assert_eq!(row, *name, "manifest name drift at id {}", id.index());
        }
    }

    #[test]
    fn every_manifest_row_has_resolved_global_projection() {
        let pairs: &[(ResolvedTypeId, TypeId)] = &[
            (GLOBAL_UNKNOWN_ID, UNKNOWN_ID),
            (GLOBAL_UNDEFINED_ID, UNDEFINED_ID),
            (GLOBAL_VOID_ID, VOID_ID),
            (GLOBAL_CONDITIONAL_ID, CONDITIONAL_ID),
            (GLOBAL_NUMBER_ID, NUMBER_ID),
            (GLOBAL_STRING_ID, STRING_ID),
            (GLOBAL_INSTANCEOF_ARRAY_T_ID, INSTANCEOF_ARRAY_T_ID),
            (GLOBAL_INSTANCEOF_ARRAY_U_ID, INSTANCEOF_ARRAY_U_ID),
            (GLOBAL_ARRAY_ID, ARRAY_ID),
            (GLOBAL_ARRAY_FILTER_ID, ARRAY_FILTER_ID),
            (GLOBAL_ARRAY_FOREACH_ID, ARRAY_FOREACH_ID),
            (GLOBAL_ARRAY_MAP_ID, ARRAY_MAP_ID),
            (GLOBAL_GLOBAL_ID, GLOBAL_ID),
            (GLOBAL_INSTANCEOF_PROMISE_ID, INSTANCEOF_PROMISE_ID),
            (GLOBAL_PROMISE_ID, PROMISE_ID),
            (GLOBAL_PROMISE_CONSTRUCTOR_ID, PROMISE_CONSTRUCTOR_ID),
            (GLOBAL_PROMISE_CATCH_ID, PROMISE_CATCH_ID),
            (GLOBAL_PROMISE_FINALLY_ID, PROMISE_FINALLY_ID),
            (GLOBAL_PROMISE_THEN_ID, PROMISE_THEN_ID),
            (GLOBAL_PROMISE_ALL_ID, PROMISE_ALL_ID),
            (GLOBAL_PROMISE_ALL_SETTLED_ID, PROMISE_ALL_SETTLED_ID),
            (GLOBAL_PROMISE_ANY_ID, PROMISE_ANY_ID),
            (GLOBAL_PROMISE_RACE_ID, PROMISE_RACE_ID),
            (GLOBAL_PROMISE_REJECT_ID, PROMISE_REJECT_ID),
            (GLOBAL_PROMISE_RESOLVE_ID, PROMISE_RESOLVE_ID),
            (GLOBAL_PROMISE_TRY_ID, PROMISE_TRY_ID),
            (GLOBAL_BIGINT_STRING_LITERAL_ID, BIGINT_STRING_LITERAL_ID),
            (GLOBAL_BOOLEAN_STRING_LITERAL_ID, BOOLEAN_STRING_LITERAL_ID),
            (
                GLOBAL_FUNCTION_STRING_LITERAL_ID,
                FUNCTION_STRING_LITERAL_ID,
            ),
            (GLOBAL_NUMBER_STRING_LITERAL_ID, NUMBER_STRING_LITERAL_ID),
            (GLOBAL_OBJECT_STRING_LITERAL_ID, OBJECT_STRING_LITERAL_ID),
            (GLOBAL_STRING_STRING_LITERAL_ID, STRING_STRING_LITERAL_ID),
            (GLOBAL_SYMBOL_STRING_LITERAL_ID, SYMBOL_STRING_LITERAL_ID),
            (
                GLOBAL_UNDEFINED_STRING_LITERAL_ID,
                UNDEFINED_STRING_LITERAL_ID,
            ),
            (
                GLOBAL_TYPEOF_OPERATOR_RETURN_UNION_ID,
                TYPEOF_OPERATOR_RETURN_UNION_ID,
            ),
            (GLOBAL_T_ID, T_ID),
            (GLOBAL_U_ID, U_ID),
            (GLOBAL_CONDITIONAL_CALLBACK_ID, CONDITIONAL_CALLBACK_ID),
            (GLOBAL_MAP_CALLBACK_ID, MAP_CALLBACK_ID),
            (GLOBAL_VOID_CALLBACK_ID, VOID_CALLBACK_ID),
            (GLOBAL_FETCH_ID, FETCH_ID),
            (GLOBAL_INSTANCEOF_REGEXP_ID, INSTANCEOF_REGEXP_ID),
            (GLOBAL_REGEXP_ID, REGEXP_ID),
            (GLOBAL_REGEXP_EXEC_ID, REGEXP_EXEC_ID),
            (GLOBAL_INSTANCEOF_SYMBOL_ID, INSTANCEOF_SYMBOL_ID),
            (GLOBAL_SYMBOL_ID, SYMBOL_ID),
            (GLOBAL_SYMBOL_DISPOSE_ID, SYMBOL_DISPOSE_ID),
            (GLOBAL_SYMBOL_ASYNC_DISPOSE_ID, SYMBOL_ASYNC_DISPOSE_ID),
            (GLOBAL_DISPOSABLE_ID, DISPOSABLE_ID),
            (GLOBAL_DISPOSABLE_DISPOSE_ID, DISPOSABLE_DISPOSE_ID),
            (GLOBAL_ASYNC_DISPOSABLE_ID, ASYNC_DISPOSABLE_ID),
            (
                GLOBAL_ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID,
                ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID,
            ),
            (GLOBAL_INSTANCEOF_DATE_ID, INSTANCEOF_DATE_ID),
            (GLOBAL_DATE_ID, DATE_ID),
            (GLOBAL_INSTANCEOF_MAP_ID, INSTANCEOF_MAP_ID),
            (GLOBAL_MAP_ID, MAP_ID),
            (GLOBAL_INSTANCEOF_SET_ID, INSTANCEOF_SET_ID),
            (GLOBAL_SET_ID, SET_ID),
            (GLOBAL_INSTANCEOF_WEAK_MAP_ID, INSTANCEOF_WEAK_MAP_ID),
            (GLOBAL_WEAK_MAP_ID, WEAK_MAP_ID),
            (GLOBAL_INSTANCEOF_ERROR_ID, INSTANCEOF_ERROR_ID),
            (GLOBAL_ERROR_ID, ERROR_ID),
            (GLOBAL_BOOLEAN_ID, BOOLEAN_ID),
        ];
        assert_eq!(pairs.len(), NUM_PREDEFINED_TYPES);
        for (resolved_id, id) in pairs {
            assert_eq!(
                *resolved_id,
                ResolvedTypeId::new(GLOBAL_LEVEL, *id),
                "resolved global ID drift at id {}",
                id.index()
            );
        }
    }

    #[test]
    fn migrated_predefined_ids_require_structural_diff_harness() {
        let migrated = crate::codegen::global_types::MIGRATED_PREDEFINED_IDS;
        assert!(
            migrated.is_empty(),
            "Non-empty MIGRATED_PREDEFINED_IDS ({} entries) requires the structural diff comparator: \
             crates/biome_js_type_info/tests/generated_equivalence.rs + tests/fixtures/manual_globals.rs. \
             See plan v14 §'Structural equivalence'.",
            migrated.len()
        );
    }

    #[test]
    fn global_type_members_matches_manifest() {
        let members = &*GLOBAL_TYPE_MEMBERS;
        assert_eq!(members.len(), NUM_PREDEFINED_TYPES);
        for (i, member) in members.iter().enumerate() {
            let name_text = member.kind.name().expect("member has no name");
            let name = name_text.text();
            assert_eq!(
                name, PREDEFINED_ID_ROWS[i],
                "GLOBAL_TYPE_MEMBERS name drift at index {i}"
            );
            let expected_ty: crate::TypeReference =
                ResolvedTypeId::new(GLOBAL_LEVEL, TypeId::new(i)).into();
            assert_eq!(
                member.ty, expected_ty,
                "GLOBAL_TYPE_MEMBERS ty drift at index {i} (name={name})"
            );
        }
    }

    #[test]
    fn global_type_name_out_of_range_returns_none() {
        assert!(global_type_name(TypeId::new(NUM_PREDEFINED_TYPES)).is_none());
    }

    #[test]
    fn sentinel_and_migrated_ids_are_disjoint() {
        let sentinel: std::collections::HashSet<GlobalTypeId> =
            SENTINEL_PREDEFINED_IDS.iter().copied().collect();
        for migrated in crate::codegen::global_types::MIGRATED_PREDEFINED_IDS {
            assert!(
                !sentinel.contains(migrated),
                "ID {} ({}) appears in both SENTINEL and MIGRATED predefined lists",
                migrated.index(),
                global_type_name(migrated.as_type_id()).unwrap_or("<unknown>"),
            );
        }
    }

    #[test]
    fn sentinel_predefined_ids_are_in_range_and_unique() {
        let mut seen: std::collections::HashSet<GlobalTypeId> = std::collections::HashSet::new();
        for id in SENTINEL_PREDEFINED_IDS.iter().copied() {
            assert!(
                id.index() < NUM_PREDEFINED_TYPES,
                "sentinel ID index {} out of range",
                id.index(),
            );
            assert!(seen.insert(id), "sentinel ID {} duplicated", id.index());
        }
    }

    #[test]
    fn sentinel_predefined_ids_match_explicit_manifest_contract() {
        let expected = [
            UNKNOWN_ID_GLOBAL_TYPE_ID,
            UNDEFINED_ID_GLOBAL_TYPE_ID,
            VOID_ID_GLOBAL_TYPE_ID,
            CONDITIONAL_ID_GLOBAL_TYPE_ID,
            NUMBER_ID_GLOBAL_TYPE_ID,
            STRING_ID_GLOBAL_TYPE_ID,
            INSTANCEOF_ARRAY_T_ID_GLOBAL_TYPE_ID,
            INSTANCEOF_ARRAY_U_ID_GLOBAL_TYPE_ID,
            GLOBAL_ID_GLOBAL_TYPE_ID,
            INSTANCEOF_PROMISE_ID_GLOBAL_TYPE_ID,
            BIGINT_STRING_LITERAL_ID_GLOBAL_TYPE_ID,
            BOOLEAN_STRING_LITERAL_ID_GLOBAL_TYPE_ID,
            FUNCTION_STRING_LITERAL_ID_GLOBAL_TYPE_ID,
            NUMBER_STRING_LITERAL_ID_GLOBAL_TYPE_ID,
            OBJECT_STRING_LITERAL_ID_GLOBAL_TYPE_ID,
            STRING_STRING_LITERAL_ID_GLOBAL_TYPE_ID,
            SYMBOL_STRING_LITERAL_ID_GLOBAL_TYPE_ID,
            UNDEFINED_STRING_LITERAL_ID_GLOBAL_TYPE_ID,
            TYPEOF_OPERATOR_RETURN_UNION_ID_GLOBAL_TYPE_ID,
            T_ID_GLOBAL_TYPE_ID,
            U_ID_GLOBAL_TYPE_ID,
            CONDITIONAL_CALLBACK_ID_GLOBAL_TYPE_ID,
            MAP_CALLBACK_ID_GLOBAL_TYPE_ID,
            VOID_CALLBACK_ID_GLOBAL_TYPE_ID,
            INSTANCEOF_REGEXP_ID_GLOBAL_TYPE_ID,
            INSTANCEOF_SYMBOL_ID_GLOBAL_TYPE_ID,
            INSTANCEOF_DATE_ID_GLOBAL_TYPE_ID,
            INSTANCEOF_MAP_ID_GLOBAL_TYPE_ID,
            INSTANCEOF_SET_ID_GLOBAL_TYPE_ID,
            INSTANCEOF_WEAK_MAP_ID_GLOBAL_TYPE_ID,
            INSTANCEOF_ERROR_ID_GLOBAL_TYPE_ID,
            BOOLEAN_ID_GLOBAL_TYPE_ID,
        ];

        assert_eq!(SENTINEL_PREDEFINED_IDS.as_ref(), expected.as_slice());
    }

    #[test]
    fn predefined_roles_classify_sentinel_projection_boundaries() {
        fn row_for(id: GlobalTypeId) -> &'static PredefinedGlobalRow {
            PREDEFINED_GLOBAL_ROWS
                .iter()
                .find(|row| row.id == id)
                .expect("predefined row should exist")
        }

        let included = [
            (UNKNOWN_ID_GLOBAL_TYPE_ID, PredefinedGlobalRole::Sentinel),
            (NUMBER_ID_GLOBAL_TYPE_ID, PredefinedGlobalRole::Primitive),
            (GLOBAL_ID_GLOBAL_TYPE_ID, PredefinedGlobalRole::Helper),
        ];
        for (id, role) in included {
            assert_eq!(row_for(id).role, role);
            assert!(
                SENTINEL_PREDEFINED_IDS.contains(&id),
                "{} should be projected into sentinel predefined IDs",
                global_type_name(id.as_type_id()).expect("predefined ID should have a name")
            );
        }

        let excluded = [
            (FETCH_ID_GLOBAL_TYPE_ID, PredefinedGlobalRole::HostManual),
            (ARRAY_ID_GLOBAL_TYPE_ID, PredefinedGlobalRole::ManualGlobal),
            (
                ARRAY_MAP_ID_GLOBAL_TYPE_ID,
                PredefinedGlobalRole::ManualSynthetic,
            ),
        ];
        for (id, role) in excluded {
            assert_eq!(row_for(id).role, role);
            assert!(
                !SENTINEL_PREDEFINED_IDS.contains(&id),
                "{} should not be projected into sentinel predefined IDs",
                global_type_name(id.as_type_id()).expect("predefined ID should have a name")
            );
        }
    }

    #[test]
    fn migrated_predefined_ids_use_global_type_ids() {
        fn assert_global_type_ids(_: &[GlobalTypeId]) {}

        assert_global_type_ids(crate::codegen::global_types::MIGRATED_PREDEFINED_IDS);
    }

    #[test]
    fn migrated_predefined_ids_sorted_unique_in_bounds() {
        let ids = crate::codegen::global_types::MIGRATED_PREDEFINED_IDS;
        for window in ids.windows(2) {
            assert!(
                window[0].index() < window[1].index(),
                "MIGRATED_PREDEFINED_IDS must be strictly increasing: {} >= {}",
                window[0].index(),
                window[1].index(),
            );
        }
        for id in ids {
            assert!(
                id.index() < NUM_PREDEFINED_TYPES,
                "MIGRATED_PREDEFINED_IDS index {} is out of range",
                id.index(),
            );
        }
    }
}
