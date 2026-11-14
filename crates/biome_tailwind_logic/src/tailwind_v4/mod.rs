//! The Tailwind CSS v4 design system that the Tailwind rules share: the
//! default configuration generated from Tailwind, the user's stylesheet, and
//! the CSS Tailwind generates for a candidate.

mod arbitrary_value_match;
mod candidate;
mod candidate_properties;
mod design_system;
mod tailwind_preset_v4;
mod tailwind_preset_v4_types;
mod variants;

pub(crate) use candidate::{CandidatePlacement, Signature};
pub use candidate_properties::CandidateProperties;
pub use design_system::TailwindDesignSystem;
pub use tailwind_preset_v4::THEME_COLOR_VALUES;
pub(crate) use variants::VariantKey;
