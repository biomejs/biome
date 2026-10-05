//! Sorts Tailwind CSS classes in the order Tailwind CSS v4 emits their CSS.

mod arbitrary_value_match;
mod design_system;
mod sort_v4;
mod sort_v4_variants;
mod tailwind_preset_v4;
mod tailwind_preset_v4_types;

pub use design_system::TailwindDesignSystem;
pub use sort_v4::sort_class_list;
