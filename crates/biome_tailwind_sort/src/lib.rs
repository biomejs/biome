//! Language-agnostic Tailwind CSS utility class sorting.
//!
//! This crate provides the core sorting logic for Tailwind CSS utility classes.
//! It is designed to be consumed by language-specific analyzers (JS, CSS, HTML, etc.)
//! that handle extracting class strings from their respective AST nodes.
//!
//! # Usage
//!
//! ```rust
//! use biome_tailwind_sort::sort::{sort_class_name, SortContext};
//! use biome_tailwind_sort::sort_config::SortConfig;
//! use biome_tailwind_sort::presets::{UseSortedClassesPreset, get_config_preset};
//!
//! let config = SortConfig::new(&get_config_preset(&UseSortedClassesPreset::TailwindCSS));
//! let sorted = sort_class_name("px-2 foo p-4 bar", &config, &None);
//! assert_eq!(sorted, "foo bar p-4 px-2");
//! ```

pub mod class_info;
pub mod class_lexer;
pub mod presets;
pub mod sort;
pub mod sort_config;
mod tailwind_preset;
