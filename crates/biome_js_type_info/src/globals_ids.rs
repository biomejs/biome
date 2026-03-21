//! Type ID constants for global types.
//!
//! TODO(tidefield): Implement a codegen for this file from TypeScript .d.ts files.

// use crate::define_global_type;
use crate::{ResolvedTypeId, TypeId};

use super::globals::GLOBAL_LEVEL;

// FIXME(tidefield): Get rid of the macro when implementing the codegen to improve compile time
// Right now, I'm preserving the names so that the snapshot tests don't break the snapshot tests
// to make sure I'm not breaking anything.
#[macro_export]
macro_rules! define_global_type {
    ($name:ident, $name_name:ident, $index:expr, $name_str:expr) => {
        pub const $name: TypeId = TypeId::new($index);
        pub const $name_name: &str = $name_str;
    };
}

// Type ID constants with their names defined together
define_global_type!(UNKNOWN_ID, UNKNOWN_ID_NAME, 0, "unknown");
define_global_type!(UNDEFINED_ID, UNDEFINED_ID_NAME, 1, "undefined");
define_global_type!(VOID_ID, VOID_ID_NAME, 2, "void");
define_global_type!(CONDITIONAL_ID, CONDITIONAL_ID_NAME, 3, "conditional");
define_global_type!(NUMBER_ID, NUMBER_ID_NAME, 4, "number");
define_global_type!(STRING_ID, STRING_ID_NAME, 5, "string");
define_global_type!(
    INSTANCEOF_ARRAY_T_ID,
    INSTANCEOF_ARRAY_T_ID_NAME,
    6,
    "instanceof Array<T>"
);
define_global_type!(
    INSTANCEOF_ARRAY_U_ID,
    INSTANCEOF_ARRAY_U_ID_NAME,
    7,
    "instanceof Array<U>"
);
define_global_type!(ARRAY_ID, ARRAY_ID_NAME, 8, "Array");
define_global_type!(
    ARRAY_FILTER_ID,
    ARRAY_FILTER_ID_NAME,
    9,
    "Array.prototype.filter"
);
define_global_type!(
    ARRAY_FOREACH_ID,
    ARRAY_FOREACH_ID_NAME,
    10,
    "Array.prototype.forEach"
);
define_global_type!(ARRAY_MAP_ID, ARRAY_MAP_ID_NAME, 11, "Array.prototype.map");
define_global_type!(GLOBAL_ID, GLOBAL_ID_NAME, 12, "globalThis");
define_global_type!(
    INSTANCEOF_PROMISE_ID,
    INSTANCEOF_PROMISE_ID_NAME,
    13,
    "instanceof Promise"
);
define_global_type!(PROMISE_ID, PROMISE_ID_NAME, 14, "Promise");
define_global_type!(
    PROMISE_CONSTRUCTOR_ID,
    PROMISE_CONSTRUCTOR_ID_NAME,
    15,
    "Promise.constructor"
);
define_global_type!(
    PROMISE_CATCH_ID,
    PROMISE_CATCH_ID_NAME,
    16,
    "Promise.prototype.catch"
);
define_global_type!(
    PROMISE_FINALLY_ID,
    PROMISE_FINALLY_ID_NAME,
    17,
    "Promise.prototype.finally"
);
define_global_type!(
    PROMISE_THEN_ID,
    PROMISE_THEN_ID_NAME,
    18,
    "Promise.prototype.then"
);
define_global_type!(PROMISE_ALL_ID, PROMISE_ALL_ID_NAME, 19, "Promise.all");
define_global_type!(
    PROMISE_ALL_SETTLED_ID,
    PROMISE_ALL_SETTLED_ID_NAME,
    20,
    "Promise.allSettled"
);
define_global_type!(PROMISE_ANY_ID, PROMISE_ANY_ID_NAME, 21, "Promise.any");
define_global_type!(PROMISE_RACE_ID, PROMISE_RACE_ID_NAME, 22, "Promise.race");
define_global_type!(
    PROMISE_REJECT_ID,
    PROMISE_REJECT_ID_NAME,
    23,
    "Promise.reject"
);
define_global_type!(
    PROMISE_RESOLVE_ID,
    PROMISE_RESOLVE_ID_NAME,
    24,
    "Promise.resolve"
);
define_global_type!(PROMISE_TRY_ID, PROMISE_TRY_ID_NAME, 25, "Promise.try");
define_global_type!(
    BIGINT_STRING_LITERAL_ID,
    BIGINT_STRING_LITERAL_ID_NAME,
    26,
    "\"bigint\""
);
define_global_type!(
    BOOLEAN_STRING_LITERAL_ID,
    BOOLEAN_STRING_LITERAL_ID_NAME,
    27,
    "\"boolean\""
);
define_global_type!(
    FUNCTION_STRING_LITERAL_ID,
    FUNCTION_STRING_LITERAL_ID_NAME,
    28,
    "\"function\""
);
define_global_type!(
    NUMBER_STRING_LITERAL_ID,
    NUMBER_STRING_LITERAL_ID_NAME,
    29,
    "\"number\""
);
define_global_type!(
    OBJECT_STRING_LITERAL_ID,
    OBJECT_STRING_LITERAL_ID_NAME,
    30,
    "\"object\""
);
define_global_type!(
    STRING_STRING_LITERAL_ID,
    STRING_STRING_LITERAL_ID_NAME,
    31,
    "\"string\""
);
define_global_type!(
    SYMBOL_STRING_LITERAL_ID,
    SYMBOL_STRING_LITERAL_ID_NAME,
    32,
    "\"symbol\""
);
define_global_type!(
    UNDEFINED_STRING_LITERAL_ID,
    UNDEFINED_STRING_LITERAL_ID_NAME,
    33,
    "\"undefined\""
);
define_global_type!(
    TYPEOF_OPERATOR_RETURN_UNION_ID,
    TYPEOF_OPERATOR_RETURN_UNION_ID_NAME,
    34,
    "\"bigint\" | \"boolean\" | \"function\" | \"number\" | \"object\" | \"string\" | \"symbol\" | \"undefined\""
);
define_global_type!(T_ID, T_ID_NAME, 35, "T");
define_global_type!(U_ID, U_ID_NAME, 36, "U");
define_global_type!(
    CONDITIONAL_CALLBACK_ID,
    CONDITIONAL_CALLBACK_ID_NAME,
    37,
    "() => conditional"
);
define_global_type!(
    MAP_CALLBACK_ID,
    MAP_CALLBACK_ID_NAME,
    38,
    "<U>(item: T) => U"
);
define_global_type!(VOID_CALLBACK_ID, VOID_CALLBACK_ID_NAME, 39, "() => void");
define_global_type!(FETCH_ID, FETCH_ID_NAME, 40, "fetch");
define_global_type!(
    INSTANCEOF_REGEXP_ID,
    INSTANCEOF_REGEXP_ID_NAME,
    41,
    "instanceof RegExp"
);
define_global_type!(REGEXP_ID, REGEXP_ID_NAME, 42, "RegExp");
define_global_type!(REGEXP_EXEC_ID, REGEXP_EXEC_ID_NAME, 43, "RegExp.exec");

/// Total number of predefined types.
/// Must be one more than the highest TypeId above.
pub const NUM_PREDEFINED_TYPES: usize = 44;

// Resolved type ID constants (TypeId wrapped with GlobalLevel)
pub const GLOBAL_UNKNOWN_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, UNKNOWN_ID);
pub const GLOBAL_UNDEFINED_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, UNDEFINED_ID);
pub const GLOBAL_VOID_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, VOID_ID);
pub const GLOBAL_CONDITIONAL_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, CONDITIONAL_ID);
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
pub const GLOBAL_INSTANCEOF_REGEXP_ID: ResolvedTypeId =
    ResolvedTypeId::new(GLOBAL_LEVEL, INSTANCEOF_REGEXP_ID);
pub const GLOBAL_REGEXP_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, REGEXP_ID);
