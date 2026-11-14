//! Sorts Tailwind CSS classes in the order Tailwind CSS v4 emits their CSS.

mod sort_v4;
mod sort_v4_variants;

pub use sort_v4::sort_class_list;
