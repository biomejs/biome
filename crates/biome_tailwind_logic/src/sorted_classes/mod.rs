//! Sorts utility class strings the way Tailwind CSS v4 and
//! `prettier-plugin-tailwindcss` do, shared by the JavaScript and HTML
//! `useSortedClasses` rules.
//!
//! [`sort_v4`] holds the sorter; the preset it sorts against is the
//! generated [`tailwind_preset_v4`] snapshot, and a project's own
//! utilities, variants, and theme reach it through a
//! [`TailwindRegistry`].

mod arbitrary_value_match;
pub mod sort_v4;
mod sort_v4_variants;
mod tailwind_preset_v4;
mod tailwind_preset_v4_types;
mod tailwind_registry;

pub use sort_v4::sort_class_list;
pub use tailwind_registry::{EMPTY_REGISTRY, TailwindRegistry};

use biome_tailwind_parser::parse_tailwind;

/// Sort a whitespace-separated class string against `registry`.
///
/// Classes the sorter doesn't recognize keep their original order at the
/// front, and whitespace is collapsed to single spaces.
pub fn sort_class_string(class_string: &str, registry: &TailwindRegistry) -> String {
    sort_class_list(&parse_tailwind(class_string).tree(), registry)
}
