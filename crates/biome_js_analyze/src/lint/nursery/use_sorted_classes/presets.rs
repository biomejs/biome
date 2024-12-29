//! Presets contain pre-defined sort configurations, notably from Tailwind CSS. They are a
//! starting point that can be extended (e.g. by adding custom utilities or variants).

use super::{
    sort_config::{UtilitiesConfig, VariantsConfig},
    tailwind_preset::{TAILWIND_LAYERS, VARIANT_CLASSES},
};

#[derive(Default)]
pub enum UseSortedClassesPreset {
    #[expect(unused)]
    None,
    #[default]
    TailwindCSS,
}

pub struct ConfigPreset {
    pub utilities: UtilitiesConfig,
    pub variants: VariantsConfig,
}

pub fn get_config_preset(preset: &UseSortedClassesPreset) -> ConfigPreset {
    match preset {
        UseSortedClassesPreset::None => get_empty_preset(),
        UseSortedClassesPreset::TailwindCSS => get_tailwind_css_preset(),
    }
}

pub fn get_empty_preset() -> ConfigPreset {
    ConfigPreset {
        utilities: [].as_slice(),
        variants: [].as_slice(),
    }
}

pub fn get_tailwind_css_preset() -> ConfigPreset {
    ConfigPreset {
        utilities: TAILWIND_LAYERS.as_slice(),
        variants: VARIANT_CLASSES.as_slice(),
    }
}
