use std::sync::LazyLock;

use super::{
    presets::{UseSortedClassesPreset, get_config_preset},
    sort_config::SortConfig,
};

static SORT_CONFIG: LazyLock<SortConfig> =
    LazyLock::new(|| SortConfig::new(&get_config_preset(&UseSortedClassesPreset::default())));

/// Sort a space-separated list of Tailwind CSS class names.
pub fn sort_class_list(input: &str) -> String {
    let _ = &*SORT_CONFIG;
    todo!("sort_via_parser: not yet implemented")
}
