//! Provenance tracking system for configuration values
//!
//! This crate provides a language-agnostic framework for tracking where
//! configuration values come from (which file, line number, actual source value).
//!
//! ## Architecture
//!
//! - [`ProvenanceValue`]: Represents source values (strings, numbers, booleans, etc.)
//! - [`ProvenanceSourceNode`]: Language-agnostic AST node interface (implemented per language)
//! - [`ProvenanceTrackable`]: Trait for types that can be constructed with provenance tracking
//! - [`ProvenanceContext`]: Tracks current path and collects entries during deserialization
//! - [`ProvenanceIndex`]: Query interface for looking up field provenance
//!
//! ## Example
//!
//! ```ignore
//! use biome_provenance::{ProvenanceTrackable, ProvenanceContext, ProvenanceIndex};
//!
//! // Parse JSON
//! let json_value = /* ... */;
//!
//! // Create context
//! let mut ctx = ProvenanceContext::new("/path/to/config.json".into(), 0);
//!
//! // Deserialize with provenance
//! let config = Configuration::from_source_with_provenance(&json_value, &mut ctx)?;
//!
//! // Query provenance
//! let index = ProvenanceIndex::new(ctx.into_entries());
//! if let Some(entry) = index.query("formatter.indentWidth") {
//!     println!("Value: {}, Source: {}:{}", entry.source_value, entry.source_path, entry.range);
//! }
//! ```

mod context;
mod entry;
mod value;

// Re-export public API
pub use context::ProvenanceContext;
pub use entry::{ProvenanceEntry, ProvenanceIndex};
pub use value::ProvenanceValue;

use biome_rowan::TextRange;

/// Language-agnostic interface for a source configuration node
///
/// This trait is implemented by language-specific AST nodes (JSON, TOML, YAML, etc.)
/// and provides a uniform interface for extracting provenance information.
///
/// Similar to `DeserializableValue` from `biome_deserialize`, but designed specifically
/// for provenance tracking.
pub trait ProvenanceSourceNode: Sized {
    /// Extract the source value representation of this node
    ///
    /// Returns `None` for invalid/bogus nodes.
    fn source_value(&self) -> Option<ProvenanceValue>;

    /// Get the text range of this node in the source file
    fn range(&self) -> TextRange;

    /// Check if this node represents an object/map
    fn is_object(&self) -> bool;

    /// Check if this node represents an array
    fn is_array(&self) -> bool;

    /// Traverse object fields (key-value pairs)
    ///
    /// For each field in the object, calls the visitor with (field_name, value_node).
    /// Does nothing if this node is not an object.
    fn traverse_fields(&self, visitor: &mut dyn FnMut(&str, &Self));

    /// Traverse array elements
    ///
    /// For each element in the array, calls the visitor with (index, element_node).
    /// Does nothing if this node is not an array.
    fn traverse_array(&self, visitor: &mut dyn FnMut(usize, &Self));
}

/// Trait for types that can be constructed while tracking provenance
///
/// This is the main trait that configuration types implement (usually via derive macro).
/// It's similar to `Deserializable` but designed to track provenance alongside deserialization.
///
/// ## Implementation
///
/// Most types should use `#[derive(ProvenanceTrackable)]` rather than implementing manually:
///
/// ```ignore
/// use biome_provenance_macros::ProvenanceTrackable;
///
/// #[derive(ProvenanceTrackable)]
/// struct Configuration {
///     indent_width: u32,
///     indent_style: String,
/// }
/// ```
///
/// ## Manual Implementation
///
/// Primitive types and collections implement this trait manually in `impls.rs`.
pub trait ProvenanceTrackable: Sized {
    /// Construct this type from a source node, tracking provenance
    ///
    /// Returns `None` if the source node cannot be converted to this type
    /// (type mismatch, invalid value, missing required fields, etc.)
    fn from_source_with_provenance(
        source: &impl ProvenanceSourceNode,
        ctx: &mut ProvenanceContext,
    ) -> Option<Self>;
}

// Optional JSON support
#[cfg(feature = "json")]
mod json;

// Primitive and collection implementations
mod impls;
