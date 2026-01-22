//! Configuration provenance tracking
//!
//! This module provides types and utilities for tracking where each configuration
//! value comes from (base config, extends, overrides, editorconfig, CLI args).

mod entry;
mod field_query;
mod override_metadata;
mod source;

pub use entry::ProvenanceEntry;
pub use field_query::{FieldQuery, FieldQuerySegment};
pub use override_metadata::{GlobMatcher, OverrideProvenanceMetadata};
pub use source::ProvenanceSource;
