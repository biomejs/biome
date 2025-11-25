//! Type ID constants for global types.
//!
//! These constants define the indices for predefined global types in the type store.
//! The specific index values matter because they determine the order of types in the
//! GlobalsResolver's TypeStore.
//!
//! TODO(tidefield): Auto-generate this file from TypeScript .d.ts files.

use crate::{ResolvedTypeId, TypeId, TypeResolverLevel};

use super::globals::GLOBAL_LEVEL;

// Type ID constants
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

/// Total number of predefined types.
/// Must be one more than the highest TypeId above.
pub const NUM_PREDEFINED_TYPES: usize = 41;

// Resolved type ID constants (TypeId wrapped with GlobalLevel)
pub const GLOBAL_UNKNOWN_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, UNKNOWN_ID);
pub const GLOBAL_UNDEFINED_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, UNDEFINED_ID);
pub const GLOBAL_VOID_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, VOID_ID);
pub const GLOBAL_CONDITIONAL_ID: ResolvedTypeId =
    ResolvedTypeId::new(GLOBAL_LEVEL, CONDITIONAL_ID);
pub const GLOBAL_NUMBER_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, NUMBER_ID);
pub const GLOBAL_STRING_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, STRING_ID);
pub const GLOBAL_ARRAY_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, ARRAY_ID);
pub const GLOBAL_GLOBAL_ID /* :smirk: */: ResolvedTypeId =
    ResolvedTypeId::new(GLOBAL_LEVEL, GLOBAL_ID);
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
