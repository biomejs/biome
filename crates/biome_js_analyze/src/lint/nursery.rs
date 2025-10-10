//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_deprecated_imports;
pub mod no_floating_promises;
pub mod no_import_cycles;
pub mod no_jsx_literals;
pub mod no_misused_promises;
pub mod no_next_async_client_component;
pub mod no_non_null_asserted_optional_chain;
pub mod no_qwik_use_visible_task;
pub mod no_react_forward_ref;
pub mod no_secrets;
pub mod no_shadow;
pub mod no_unnecessary_conditions;
pub mod no_unresolved_imports;
pub mod no_unused_expressions;
pub mod no_useless_catch_binding;
pub mod no_useless_undefined;
pub mod no_vue_data_object_declaration;
pub mod no_vue_duplicate_keys;
pub mod no_vue_reserved_keys;
pub mod no_vue_reserved_props;
pub mod no_vue_setup_props_reactivity_loss;
pub mod use_anchor_href;
pub mod use_consistent_arrow_return;
pub mod use_consistent_type_definitions;
pub mod use_exhaustive_switch_cases;
pub mod use_explicit_type;
pub mod use_image_size;
pub mod use_max_params;
pub mod use_qwik_classlist;
pub mod use_qwik_method_usage;
pub mod use_qwik_valid_lexical_scope;
pub mod use_react_function_components;
pub mod use_sorted_classes;
pub mod use_vue_multi_word_component_names;
declare_lint_group! { pub Nursery { name : "nursery" , rules : [self :: no_deprecated_imports :: NoDeprecatedImports , self :: no_floating_promises :: NoFloatingPromises , self :: no_import_cycles :: NoImportCycles , self :: no_jsx_literals :: NoJsxLiterals , self :: no_misused_promises :: NoMisusedPromises , self :: no_next_async_client_component :: NoNextAsyncClientComponent , self :: no_non_null_asserted_optional_chain :: NoNonNullAssertedOptionalChain , self :: no_qwik_use_visible_task :: NoQwikUseVisibleTask , self :: no_react_forward_ref :: NoReactForwardRef , self :: no_secrets :: NoSecrets , self :: no_shadow :: NoShadow , self :: no_unnecessary_conditions :: NoUnnecessaryConditions , self :: no_unresolved_imports :: NoUnresolvedImports , self :: no_unused_expressions :: NoUnusedExpressions , self :: no_useless_catch_binding :: NoUselessCatchBinding , self :: no_useless_undefined :: NoUselessUndefined , self :: no_vue_data_object_declaration :: NoVueDataObjectDeclaration , self :: no_vue_duplicate_keys :: NoVueDuplicateKeys , self :: no_vue_reserved_keys :: NoVueReservedKeys , self :: no_vue_reserved_props :: NoVueReservedProps , self :: no_vue_setup_props_reactivity_loss :: NoVueSetupPropsReactivityLoss , self :: use_anchor_href :: UseAnchorHref , self :: use_consistent_arrow_return :: UseConsistentArrowReturn , self :: use_consistent_type_definitions :: UseConsistentTypeDefinitions , self :: use_exhaustive_switch_cases :: UseExhaustiveSwitchCases , self :: use_explicit_type :: UseExplicitType , self :: use_image_size :: UseImageSize , self :: use_max_params :: UseMaxParams , self :: use_qwik_classlist :: UseQwikClasslist , self :: use_qwik_method_usage :: UseQwikMethodUsage , self :: use_qwik_valid_lexical_scope :: UseQwikValidLexicalScope , self :: use_react_function_components :: UseReactFunctionComponents , self :: use_sorted_classes :: UseSortedClasses , self :: use_vue_multi_word_component_names :: UseVueMultiWordComponentNames ,] } }
