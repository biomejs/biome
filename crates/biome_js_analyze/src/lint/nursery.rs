//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_console;
pub mod no_constant_math_min_max_clamp;
pub mod no_done_callback;
pub mod no_duplicate_else_if;
pub mod no_evolving_any;
pub mod no_flat_map_identity;
pub mod no_misplaced_assertion;
pub mod no_nodejs_modules;
pub mod no_react_specific_props;
pub mod no_restricted_imports;
pub mod no_undeclared_dependencies;
pub mod no_useless_string_concat;
pub mod no_useless_undefined_initialization;
pub mod use_array_literals;
pub mod use_consistent_builtin_instantiation;
pub mod use_default_switch_clause;
pub mod use_explicit_length_check;
pub mod use_focusable_interactive;
pub mod use_import_restrictions;
pub mod use_sorted_classes;

declare_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: no_console :: NoConsole ,
            self :: no_constant_math_min_max_clamp :: NoConstantMathMinMaxClamp ,
            self :: no_done_callback :: NoDoneCallback ,
            self :: no_duplicate_else_if :: NoDuplicateElseIf ,
            self :: no_evolving_any :: NoEvolvingAny ,
            self :: no_flat_map_identity :: NoFlatMapIdentity ,
            self :: no_misplaced_assertion :: NoMisplacedAssertion ,
            self :: no_nodejs_modules :: NoNodejsModules ,
            self :: no_react_specific_props :: NoReactSpecificProps ,
            self :: no_restricted_imports :: NoRestrictedImports ,
            self :: no_undeclared_dependencies :: NoUndeclaredDependencies ,
            self :: no_useless_string_concat :: NoUselessStringConcat ,
            self :: no_useless_undefined_initialization :: NoUselessUndefinedInitialization ,
            self :: use_array_literals :: UseArrayLiterals ,
            self :: use_consistent_builtin_instantiation :: UseConsistentBuiltinInstantiation ,
            self :: use_default_switch_clause :: UseDefaultSwitchClause ,
            self :: use_explicit_length_check :: UseExplicitLengthCheck ,
            self :: use_focusable_interactive :: UseFocusableInteractive ,
            self :: use_import_restrictions :: UseImportRestrictions ,
            self :: use_sorted_classes :: UseSortedClasses ,
        ]
     }
}
