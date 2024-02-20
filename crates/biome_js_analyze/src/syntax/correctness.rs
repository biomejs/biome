//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_duplicate_private_class_members;
pub mod no_initializer_with_definite;
pub mod no_super_without_extends;

declare_group! {
    pub Correctness {
        name : "correctness" ,
        rules : [
            self :: no_duplicate_private_class_members :: NoDuplicatePrivateClassMembers ,
            self :: no_initializer_with_definite :: NoInitializerWithDefinite ,
            self :: no_super_without_extends :: NoSuperWithoutExtends ,
        ]
     }
}
