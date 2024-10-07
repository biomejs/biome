//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::assists;
use crate::lint;

pub type NoDuplicateObjectKeys = < lint :: suspicious :: no_duplicate_object_keys :: NoDuplicateObjectKeys as biome_analyze :: Rule > :: Options ;
pub type UseSortedKeys =
    <assists::source::use_sorted_keys::UseSortedKeys as biome_analyze::Rule>::Options;
