//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_approximative_numeric_constant;
pub mod no_assign_in_expressions;
pub mod no_async_promise_executor;
pub mod no_comment_text;
pub mod no_compare_neg_zero;
pub mod no_confusing_labels;
pub mod no_confusing_void_type;
pub mod no_const_enum;
pub mod no_control_characters_in_regex;
pub mod no_debugger;
pub mod no_double_equals;
pub mod no_duplicate_case;
pub mod no_duplicate_class_members;
pub mod no_duplicate_jsx_props;
pub mod no_duplicate_object_keys;
pub mod no_empty_interface;
pub mod no_explicit_any;
pub mod no_extra_non_null_assertion;
pub mod no_fallthrough_switch_clause;
pub mod no_implicit_any_let;
pub mod no_misleading_instantiator;
pub mod no_misrefactored_shorthand_assign;
pub mod no_prototype_builtins;
pub mod no_redundant_use_strict;
pub mod no_self_compare;
pub mod no_shadow_restricted_names;
pub mod no_sparse_array;
pub mod no_unsafe_negation;
pub mod use_default_switch_clause_last;
pub mod use_getter_return;
pub mod use_namespace_keyword;
pub mod use_valid_typeof;

declare_group! {
    pub Suspicious {
        name : "suspicious" ,
        rules : [
            self :: no_approximative_numeric_constant :: NoApproximativeNumericConstant ,
            self :: no_assign_in_expressions :: NoAssignInExpressions ,
            self :: no_async_promise_executor :: NoAsyncPromiseExecutor ,
            self :: no_comment_text :: NoCommentText ,
            self :: no_compare_neg_zero :: NoCompareNegZero ,
            self :: no_confusing_labels :: NoConfusingLabels ,
            self :: no_confusing_void_type :: NoConfusingVoidType ,
            self :: no_const_enum :: NoConstEnum ,
            self :: no_control_characters_in_regex :: NoControlCharactersInRegex ,
            self :: no_debugger :: NoDebugger ,
            self :: no_double_equals :: NoDoubleEquals ,
            self :: no_duplicate_case :: NoDuplicateCase ,
            self :: no_duplicate_class_members :: NoDuplicateClassMembers ,
            self :: no_duplicate_jsx_props :: NoDuplicateJsxProps ,
            self :: no_duplicate_object_keys :: NoDuplicateObjectKeys ,
            self :: no_empty_interface :: NoEmptyInterface ,
            self :: no_explicit_any :: NoExplicitAny ,
            self :: no_extra_non_null_assertion :: NoExtraNonNullAssertion ,
            self :: no_fallthrough_switch_clause :: NoFallthroughSwitchClause ,
            self :: no_implicit_any_let :: NoImplicitAnyLet ,
            self :: no_misleading_instantiator :: NoMisleadingInstantiator ,
            self :: no_misrefactored_shorthand_assign :: NoMisrefactoredShorthandAssign ,
            self :: no_prototype_builtins :: NoPrototypeBuiltins ,
            self :: no_redundant_use_strict :: NoRedundantUseStrict ,
            self :: no_self_compare :: NoSelfCompare ,
            self :: no_shadow_restricted_names :: NoShadowRestrictedNames ,
            self :: no_sparse_array :: NoSparseArray ,
            self :: no_unsafe_negation :: NoUnsafeNegation ,
            self :: use_default_switch_clause_last :: UseDefaultSwitchClauseLast ,
            self :: use_getter_return :: UseGetterReturn ,
            self :: use_namespace_keyword :: UseNamespaceKeyword ,
            self :: use_valid_typeof :: UseValidTypeof ,
        ]
     }
}
