//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_syntax_group;
pub mod no_duplicate_private_class_members;
pub mod no_initializer_with_definite;
pub mod no_super_without_extends;
pub mod no_type_only_import_attributes;
declare_syntax_group! { pub Correctness { name : "correctness" , rules : [self :: no_duplicate_private_class_members :: NoDuplicatePrivateClassMembers , self :: no_initializer_with_definite :: NoInitializerWithDefinite , self :: no_super_without_extends :: NoSuperWithoutExtends , self :: no_type_only_import_attributes :: NoTypeOnlyImportAttributes ,] } }
