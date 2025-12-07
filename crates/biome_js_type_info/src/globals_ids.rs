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
    ($name:ident, $index:expr, $name_str:expr) => {
        pub const $name: TypeId = TypeId::new($index);
        paste::paste! {
            pub const [<$name _NAME>]: &str = $name_str;
        }
    };
}
// define_global_type!(ARRAY_ID, 8, "Array");         // Creates ARRAY_ID and ARRAY_ID_NAME

// Type ID constants with their names defined together
define_global_type!(UNKNOWN_ID, 0, "unknown");
define_global_type!(UNDEFINED_ID, 1, "undefined");
define_global_type!(VOID_ID, 2, "void");
define_global_type!(CONDITIONAL_ID, 3, "conditional");
define_global_type!(NUMBER_ID, 4, "number");
define_global_type!(STRING_ID, 5, "string");
define_global_type!(INSTANCEOF_ARRAY_T_ID, 6, "instanceof Array<T>");
define_global_type!(INSTANCEOF_ARRAY_U_ID, 7, "instanceof Array<U>");
define_global_type!(ARRAY_ID, 8, "Array");
define_global_type!(ARRAY_FILTER_ID, 9, "Array.prototype.filter");
define_global_type!(ARRAY_FOREACH_ID, 10, "Array.prototype.forEach");
define_global_type!(ARRAY_MAP_ID, 11, "Array.prototype.map");
define_global_type!(GLOBAL_ID, 12, "globalThis");
define_global_type!(INSTANCEOF_PROMISE_ID, 13, "instanceof Promise");
define_global_type!(PROMISE_ID, 14, "Promise");
define_global_type!(PROMISE_CONSTRUCTOR_ID, 15, "Promise.constructor");
define_global_type!(PROMISE_CATCH_ID, 16, "Promise.prototype.catch");
define_global_type!(PROMISE_FINALLY_ID, 17, "Promise.prototype.finally");
define_global_type!(PROMISE_THEN_ID, 18, "Promise.prototype.then");
define_global_type!(PROMISE_ALL_ID, 19, "Promise.all");
define_global_type!(PROMISE_ALL_SETTLED_ID, 20, "Promise.allSettled");
define_global_type!(PROMISE_ANY_ID, 21, "Promise.any");
define_global_type!(PROMISE_RACE_ID, 22, "Promise.race");
define_global_type!(PROMISE_REJECT_ID, 23, "Promise.reject");
define_global_type!(PROMISE_RESOLVE_ID, 24, "Promise.resolve");
define_global_type!(PROMISE_TRY_ID, 25, "Promise.try");
define_global_type!(BIGINT_STRING_LITERAL_ID, 26, "\"bigint\"");
define_global_type!(BOOLEAN_STRING_LITERAL_ID, 27, "\"boolean\"");
define_global_type!(FUNCTION_STRING_LITERAL_ID, 28, "\"function\"");
define_global_type!(NUMBER_STRING_LITERAL_ID, 29, "\"number\"");
define_global_type!(OBJECT_STRING_LITERAL_ID, 30, "\"object\"");
define_global_type!(STRING_STRING_LITERAL_ID, 31, "\"string\"");
define_global_type!(SYMBOL_STRING_LITERAL_ID, 32, "\"symbol\"");
define_global_type!(UNDEFINED_STRING_LITERAL_ID, 33, "\"undefined\"");
define_global_type!(
    TYPEOF_OPERATOR_RETURN_UNION_ID,
    34,
    "\"bigint\" | \"boolean\" | \"function\" | \"number\" | \"object\" | \"string\" | \"symbol\" | \"undefined\""
);
define_global_type!(T_ID, 35, "T");
define_global_type!(U_ID, 36, "U");
define_global_type!(CONDITIONAL_CALLBACK_ID, 37, "() => conditional");
define_global_type!(MAP_CALLBACK_ID, 38, "<U>(item: T) => U");
define_global_type!(VOID_CALLBACK_ID, 39, "() => void");
define_global_type!(FETCH_ID, 40, "fetch");
define_global_type!(INSTANCEOF_REGEXP_ID, 41, "instanceof RegExp");
define_global_type!(REGEXP_ID, 42, "RegExp");
define_global_type!(REGEXP_EXEC_ID, 43, "RegExp.exec");

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
