//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_duplicate_private_class_members;
pub(crate) mod no_initializer_with_definite;
pub(crate) mod no_super_without_extends;

declare_group! {
    pub (crate) Correctness {
        name : "correctness" ,
        rules : [
            self :: no_duplicate_private_class_members :: NoDuplicatePrivateClassMembers ,
            self :: no_initializer_with_definite :: NoInitializerWithDefinite ,
            self :: no_super_without_extends :: NoSuperWithoutExtends ,
        ]
     }
}
