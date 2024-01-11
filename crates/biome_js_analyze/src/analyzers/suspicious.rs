//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_approximative_numeric_constant;
pub(crate) mod no_assign_in_expressions;
pub(crate) mod no_async_promise_executor;
pub(crate) mod no_comment_text;
pub(crate) mod no_compare_neg_zero;
pub(crate) mod no_confusing_labels;
pub(crate) mod no_confusing_void_type;
pub(crate) mod no_const_enum;
pub(crate) mod no_control_characters_in_regex;
pub(crate) mod no_debugger;
pub(crate) mod no_double_equals;
pub(crate) mod no_duplicate_case;
pub(crate) mod no_duplicate_class_members;
pub(crate) mod no_duplicate_jsx_props;
pub(crate) mod no_duplicate_object_keys;
pub(crate) mod no_empty_interface;
pub(crate) mod no_explicit_any;
pub(crate) mod no_extra_non_null_assertion;
pub(crate) mod no_fallthrough_switch_clause;
pub(crate) mod no_implicit_any_let;
pub(crate) mod no_misleading_instantiator;
pub(crate) mod no_misrefactored_shorthand_assign;
pub(crate) mod no_prototype_builtins;
pub(crate) mod no_redundant_use_strict;
pub(crate) mod no_self_compare;
pub(crate) mod no_shadow_restricted_names;
pub(crate) mod no_sparse_array;
pub(crate) mod no_unsafe_negation;
pub(crate) mod use_default_switch_clause_last;
pub(crate) mod use_getter_return;
pub(crate) mod use_namespace_keyword;
pub(crate) mod use_valid_typeof;

declare_group! {
    pub (crate) Suspicious {
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
