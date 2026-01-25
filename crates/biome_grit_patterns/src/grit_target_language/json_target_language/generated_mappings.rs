//! Generated file, do not edit by hand, see `xtask/codegen`
//!
//! Maps GritQL pattern names to Biome's internal syntax kinds for JSON.

use biome_json_syntax as lang;
use biome_rowan::AstNode;
use lang::JsonSyntaxKind;

/// A legacy TreeSitter pattern for backward compatibility.
pub struct LegacyTreeSitterPattern {
    pub name: &'static str,
    pub kind: JsonSyntaxKind,
    pub slots: &'static [(&'static str, u32)],
}

/// A list of legacy TreeSitter patterns for compatibility with existing Grit patterns.
/// These names align with TreeSitter's JSON grammar for compatibility with the
/// Grit playground and existing patterns.
pub const LEGACY_TREESITTER_COMPATIBILITY_PATTERNS: &[LegacyTreeSitterPattern] = &[
    // TreeSitter "object" → Biome JsonObjectValue
    // Slots: { members }
    LegacyTreeSitterPattern {
        name: "object",
        kind: JsonSyntaxKind::JSON_OBJECT_VALUE,
        slots: &[("members", 1)],
    },
    // TreeSitter "array" → Biome JsonArrayValue
    // Slots: [ elements ]
    LegacyTreeSitterPattern {
        name: "array",
        kind: JsonSyntaxKind::JSON_ARRAY_VALUE,
        slots: &[("elements", 1)],
    },
    // TreeSitter "pair" → Biome JsonMember
    // Slots: key : value
    LegacyTreeSitterPattern {
        name: "pair",
        kind: JsonSyntaxKind::JSON_MEMBER,
        slots: &[("key", 0), ("value", 2)],
    },
    // TreeSitter "string" → Biome JsonStringValue
    LegacyTreeSitterPattern {
        name: "string",
        kind: JsonSyntaxKind::JSON_STRING_VALUE,
        slots: &[],
    },
    // TreeSitter "number" → Biome JsonNumberValue
    LegacyTreeSitterPattern {
        name: "number",
        kind: JsonSyntaxKind::JSON_NUMBER_VALUE,
        slots: &[],
    },
    // TreeSitter "true" → Biome JsonBooleanValue
    LegacyTreeSitterPattern {
        name: "true",
        kind: JsonSyntaxKind::JSON_BOOLEAN_VALUE,
        slots: &[],
    },
    // TreeSitter "false" → Biome JsonBooleanValue
    LegacyTreeSitterPattern {
        name: "false",
        kind: JsonSyntaxKind::JSON_BOOLEAN_VALUE,
        slots: &[],
    },
    // TreeSitter "null" → Biome JsonNullValue
    LegacyTreeSitterPattern {
        name: "null",
        kind: JsonSyntaxKind::JSON_NULL_VALUE,
        slots: &[],
    },
    // TreeSitter "document" → Biome JsonRoot
    LegacyTreeSitterPattern {
        name: "document",
        kind: JsonSyntaxKind::JSON_ROOT,
        slots: &[("value", 1)],
    },
];

/// Returns the TreeSitter-compatible name for a syntax kind if it's part of the legacy set.
pub fn legacy_treesitter_name_for_kind(kind: JsonSyntaxKind) -> Option<&'static str> {
    LEGACY_TREESITTER_COMPATIBILITY_PATTERNS
        .iter()
        .find(|p| p.kind == kind)
        .map(|p| p.name)
}

/// Returns the slot mappings for a syntax kind if it's part of the legacy set.
pub fn legacy_treesitter_slots_for_kind(kind: JsonSyntaxKind) -> &'static [(&'static str, u32)] {
    LEGACY_TREESITTER_COMPATIBILITY_PATTERNS
        .iter()
        .find(|p| p.kind == kind)
        .map_or(&[], |p| p.slots)
}

/// Returns the syntax kind for a legacy or native node name.
pub fn kind_by_name(node_name: &str) -> Option<JsonSyntaxKind> {
    match node_name {
        // Native Biome AST patterns (PascalCase)
        "JsonRoot" => lang::JsonRoot::KIND_SET.iter().next(),
        "JsonObjectValue" => lang::JsonObjectValue::KIND_SET.iter().next(),
        "JsonArrayValue" => lang::JsonArrayValue::KIND_SET.iter().next(),
        "JsonMember" => lang::JsonMember::KIND_SET.iter().next(),
        "JsonMemberName" => lang::JsonMemberName::KIND_SET.iter().next(),
        "JsonMemberList" => lang::JsonMemberList::KIND_SET.iter().next(),
        "JsonArrayElementList" => lang::JsonArrayElementList::KIND_SET.iter().next(),
        "JsonStringValue" => lang::JsonStringValue::KIND_SET.iter().next(),
        "JsonNumberValue" => lang::JsonNumberValue::KIND_SET.iter().next(),
        "JsonBooleanValue" => lang::JsonBooleanValue::KIND_SET.iter().next(),
        "JsonNullValue" => lang::JsonNullValue::KIND_SET.iter().next(),
        "JsonMetavariable" => Some(JsonSyntaxKind::JSON_METAVARIABLE),

        // Legacy TreeSitter patterns (snake_case / lowercase)
        "json_root" | "document" => Some(JsonSyntaxKind::JSON_ROOT),
        "json_object" | "object" => Some(JsonSyntaxKind::JSON_OBJECT_VALUE),
        "json_array" | "array" => Some(JsonSyntaxKind::JSON_ARRAY_VALUE),
        "json_member" | "pair" => Some(JsonSyntaxKind::JSON_MEMBER),
        "json_member_name" | "property_name" | "key" => Some(JsonSyntaxKind::JSON_MEMBER_NAME),
        "json_string" | "string" => Some(JsonSyntaxKind::JSON_STRING_VALUE),
        "json_number" | "number" => Some(JsonSyntaxKind::JSON_NUMBER_VALUE),
        "json_boolean" | "true" | "false" => Some(JsonSyntaxKind::JSON_BOOLEAN_VALUE),
        "json_null" | "null" => Some(JsonSyntaxKind::JSON_NULL_VALUE),

        _ => None,
    }
}
