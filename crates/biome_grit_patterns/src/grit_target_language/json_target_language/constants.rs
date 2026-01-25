//! Constants for JSON GritQL target language.
use super::super::DisregardedSlotCondition;
use biome_json_syntax::JsonSyntaxKind;

/// Slots that should be disregarded when matching JSON snippets.
/// JSON has a simple structure, so there are no special slots to disregard.
pub const DISREGARDED_SNIPPET_SLOTS: &[(JsonSyntaxKind, u32, DisregardedSlotCondition)] = &[];
