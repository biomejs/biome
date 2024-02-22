//! Presets contain pre-defined sort configurations, notably from Tailwind CSS. They are a
//! starting point that can be extended (e.g. by adding custom utilities or variants).

use super::{sort_config::UtilitiesConfig, tailwind_preset::TAILWIND_LAYERS};

#[derive(Default)]
pub enum UseSortedClassesPreset {
    #[allow(unused)]
    None,
    #[default]
    TailwindCSS,
}

pub fn get_utilities_preset(preset: &UseSortedClassesPreset) -> UtilitiesConfig {
    match preset {
        UseSortedClassesPreset::None => [].as_slice(),
        UseSortedClassesPreset::TailwindCSS => TAILWIND_LAYERS.as_slice(),
    }
}
