use crate::{
    js_kinds_src::{AstSrc, Field},
    language_kind::LanguageKind,
};
use xtask_glue::Result;

pub fn generate_grit_mappings(ast: &AstSrc, language_kind: LanguageKind) -> Result<String> {
    let lang = LanguageConfig::new(language_kind);
    let native_nodes = ast
        .nodes
        .iter()
        // Filter out nodes that are lists or start with "Any"
        .filter(|node| !node.name.contains("List") && !node.name.starts_with("Any"))
        .collect::<Vec<_>>();

    let legacy_patterns = lang.legacy_patterns;
    let native_patterns = native_nodes
        .iter()
        .map(|node| {
            format!(
                r#"        "{}" => lang::{}::KIND_SET.iter().next(),"#,
                node.name, node.name,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let native_slot_mappings = native_nodes
        .iter()
        .filter_map(|node| {
            let slots = node
                .fields
                .iter()
                .enumerate()
                .filter_map(|(index, field)| match field {
                    Field::Node { .. } => Some(format!(
                        r#"("{}", {})"#,
                        field.method_name(language_kind),
                        index,
                    )),
                    Field::Token { .. } => None,
                })
                .collect::<Vec<_>>();

            if slots.is_empty() {
                None
            } else {
                Some(format!(
                    r#"        "{}" => &[{}],"#,
                    node.name,
                    slots.join(", "),
                ))
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let has_native_slot_mappings = !native_slot_mappings.is_empty();

    let header = "//! Maps GritQL pattern names to Biome's internal syntax kinds.";
    let has_legacy_patterns = !legacy_patterns.is_empty();
    let legacy_section = if has_legacy_patterns {
        format!(
            r#"
/// A legacy TreeSitter pattern for backward compatibility.
pub struct LegacyTreeSitterPattern {{
    pub name: &'static str,
    pub kind: {syntax_kind_type},
    pub slots: &'static [(&'static str, u32)],
}}

/// A list of legacy TreeSitter patterns for compatibility.
pub const LEGACY_TREESITTER_COMPATIBILITY_PATTERNS: &[LegacyTreeSitterPattern] = &[
{legacy_pattern_structs}
];

/// Returns the snake_case name for a syntax kind if it's part of the legacy set.
pub fn legacy_treesitter_name_for_kind(kind: {syntax_kind_type}) -> Option<&'static str> {{
    LEGACY_TREESITTER_COMPATIBILITY_PATTERNS
        .iter()
        .find(|p| p.kind == kind)
        .map(|p| p.name)
}}

/// Returns the slot mappings for a syntax kind if it's part of the legacy set.
pub fn legacy_treesitter_slots_for_kind(kind: {syntax_kind_type}) -> &'static [(&'static str, u32)] {{
    LEGACY_TREESITTER_COMPATIBILITY_PATTERNS
        .iter()
        .find(|p| p.kind == kind)
        .map_or(&[], |p| p.slots)
}}"#,
            syntax_kind_type = lang.syntax_kind_type,
            legacy_pattern_structs = legacy_patterns
                .iter()
                .map(|pattern| {
                    let slots = pattern
                        .slots
                        .iter()
                        .map(|(name, index)| format!(r#"("{}", {})"#, name, index))
                        .collect::<Vec<_>>()
                        .join(", ");

                    format!(
                        "    LegacyTreeSitterPattern {{ name: \"{name}\", kind: {syntax_kind_type}::{biome_kind}, slots: &[{slots}] }},",
                        name = pattern.name,
                        biome_kind = pattern.biome_kind,
                        slots = slots,
                        syntax_kind_type = lang.syntax_kind_type,
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        )
    } else {
        String::new()
    };

    let legacy_kind_mappings = if has_legacy_patterns {
        format!(
            "        // Legacy TreeSitter patterns\n{}",
            legacy_patterns
                .iter()
                .map(|p| {
                    format!(
                        r#"        "{}" => Some({syntax_kind_type}::{}),"#,
                        p.name,
                        p.biome_kind,
                        syntax_kind_type = lang.syntax_kind_type
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    } else {
        String::new()
    };

    let legacy_kind_by_name = if has_legacy_patterns {
        format!(
            r#"/// Returns the syntax kind for a legacy TreeSitter node name.
pub fn legacy_kind_by_name(node_name: &str) -> Option<{syntax_kind_type}> {{
    match node_name {{
{legacy_kind_mappings}

        _ => None,
    }}
}}"#,
            syntax_kind_type = lang.syntax_kind_type,
            legacy_kind_mappings = legacy_kind_mappings,
        )
    } else {
        format!(
            r#"/// Returns the syntax kind for a legacy TreeSitter node name.
pub fn legacy_kind_by_name(_node_name: &str) -> Option<{syntax_kind_type}> {{
    None
}}"#,
            syntax_kind_type = lang.syntax_kind_type,
        )
    };

    let native_slots_section = if has_native_slot_mappings {
        format!(
            r#"
/// Returns the native Biome slot mappings for a node name.
pub fn native_slots_for_name(node_name: &str) -> &'static [(&'static str, u32)] {{
    match node_name {{
{native_slot_mappings}
        _ => &[],
    }}
}}"#,
            native_slot_mappings = native_slot_mappings,
        )
    } else {
        String::new()
    };

    let result = format!(
        r#"
{header}
use biome_rowan::AstNode;
use {syntax_module} as lang;
use lang::{syntax_kind_type};

{legacy_section}


{legacy_kind_by_name}

/// Returns the syntax kind for a native Biome node name.
pub fn native_kind_by_name(node_name: &str) -> Option<{syntax_kind_type}> {{
    match node_name {{

        // Native Biome AST patterns
{native_patterns}
        _ => None,
    }}
}}

/// Returns the syntax kind for a legacy or native node name.
pub fn kind_by_name(node_name: &str) -> Option<{syntax_kind_type}> {{
    legacy_kind_by_name(node_name).or_else(|| native_kind_by_name(node_name))
}}
{native_slots_section}
"#,
        header = header,
        syntax_module = lang.syntax_module,
        syntax_kind_type = lang.syntax_kind_type,
        legacy_section = legacy_section,
        legacy_kind_by_name = legacy_kind_by_name,
        native_patterns = native_patterns,
        native_slots_section = native_slots_section,
    );

    xtask_glue::reformat(result)
}

struct LanguageConfig {
    syntax_kind_type: &'static str,
    syntax_module: &'static str,
    legacy_patterns: &'static [TreeSitterPattern],
}

impl LanguageConfig {
    fn new(language_kind: LanguageKind) -> Self {
        match language_kind {
            LanguageKind::Js => Self {
                syntax_kind_type: "JsSyntaxKind",
                syntax_module: "biome_js_syntax",
                legacy_patterns: JS_TREESITTER_PATTERNS,
            },
            LanguageKind::Css => Self {
                syntax_kind_type: "CssSyntaxKind",
                syntax_module: "biome_css_syntax",
                legacy_patterns: &[],
            },
            LanguageKind::Json => Self {
                syntax_kind_type: "JsonSyntaxKind",
                syntax_module: "biome_json_syntax",
                legacy_patterns: JSON_TREESITTER_PATTERNS,
            },
            _ => unimplemented!("Grit mappings are not supported for {:?}", language_kind),
        }
    }
}

#[derive(Debug, Clone)]
struct TreeSitterPattern {
    name: &'static str,
    biome_kind: &'static str,
    slots: &'static [(&'static str, u32)],
}

const JS_TREESITTER_PATTERNS: &[TreeSitterPattern] = &[
    TreeSitterPattern {
        name: "identifier",
        biome_kind: "JS_REFERENCE_IDENTIFIER",
        slots: &[],
    },
    TreeSitterPattern {
        name: "string",
        biome_kind: "JS_STRING_LITERAL_EXPRESSION",
        slots: &[],
    },
    TreeSitterPattern {
        name: "number",
        biome_kind: "JS_NUMBER_LITERAL_EXPRESSION",
        slots: &[],
    },
    TreeSitterPattern {
        name: "property_identifier",
        biome_kind: "JS_LITERAL_MEMBER_NAME",
        slots: &[],
    },
    TreeSitterPattern {
        name: "call_expression",
        biome_kind: "JS_CALL_EXPRESSION",
        slots: &[("function", 0), ("arguments", 3)],
    },
    TreeSitterPattern {
        name: "member_expression",
        biome_kind: "JS_STATIC_MEMBER_EXPRESSION",
        slots: &[("object", 0), ("property", 2)],
    },
    TreeSitterPattern {
        name: "subscript_expression",
        biome_kind: "JS_COMPUTED_MEMBER_EXPRESSION",
        slots: &[("object", 0), ("index", 3)],
    },
    TreeSitterPattern {
        name: "binary_expression",
        biome_kind: "JS_BINARY_EXPRESSION",
        slots: &[("left", 0), ("right", 2)],
    },
    TreeSitterPattern {
        name: "assignment_expression",
        biome_kind: "JS_ASSIGNMENT_EXPRESSION",
        slots: &[("left", 0), ("right", 2)],
    },
    TreeSitterPattern {
        name: "conditional_expression",
        biome_kind: "JS_CONDITIONAL_EXPRESSION",
        slots: &[("condition", 0), ("consequence", 2), ("alternative", 4)],
    },
    TreeSitterPattern {
        name: "arrow_function",
        biome_kind: "JS_ARROW_FUNCTION_EXPRESSION",
        slots: &[("parameters", 2), ("body", 5)],
    },
    TreeSitterPattern {
        name: "object",
        biome_kind: "JS_OBJECT_EXPRESSION",
        slots: &[("properties", 1)],
    },
    TreeSitterPattern {
        name: "array",
        biome_kind: "JS_ARRAY_EXPRESSION",
        slots: &[],
    },
    TreeSitterPattern {
        name: "pair",
        biome_kind: "JS_PROPERTY_OBJECT_MEMBER",
        slots: &[("key", 0), ("value", 2)],
    },
    TreeSitterPattern {
        name: "if_statement",
        biome_kind: "JS_IF_STATEMENT",
        slots: &[("condition", 2), ("consequence", 4)],
    },
    TreeSitterPattern {
        name: "for_statement",
        biome_kind: "JS_FOR_STATEMENT",
        slots: &[("initializer", 2), ("condition", 4), ("body", 8)],
    },
    TreeSitterPattern {
        name: "while_statement",
        biome_kind: "JS_WHILE_STATEMENT",
        slots: &[("condition", 2), ("body", 4)],
    },
    TreeSitterPattern {
        name: "function_declaration",
        biome_kind: "JS_FUNCTION_DECLARATION",
        slots: &[("name", 2), ("body", 7)],
    },
    TreeSitterPattern {
        name: "return_statement",
        biome_kind: "JS_RETURN_STATEMENT",
        slots: &[],
    },
    TreeSitterPattern {
        name: "variable_declaration",
        biome_kind: "JS_VARIABLE_DECLARATION",
        slots: &[],
    },
    TreeSitterPattern {
        name: "expression_statement",
        biome_kind: "JS_EXPRESSION_STATEMENT",
        slots: &[],
    },
    TreeSitterPattern {
        name: "jsx_expression",
        biome_kind: "JSX_EXPRESSION_CHILD",
        slots: &[],
    },
    TreeSitterPattern {
        name: "jsx_attribute",
        biome_kind: "JSX_ATTRIBUTE",
        slots: &[("name", 0), ("value", 1)],
    },
    TreeSitterPattern {
        name: "jsx_element",
        biome_kind: "JSX_ELEMENT",
        slots: &[],
    },
    TreeSitterPattern {
        name: "jsx_self_closing_element",
        biome_kind: "JSX_SELF_CLOSING_ELEMENT",
        slots: &[("name", 1), ("type_arguments", 2), ("attributes", 3)],
    },
    TreeSitterPattern {
        name: "jsx_opening_element",
        biome_kind: "JSX_OPENING_ELEMENT",
        slots: &[("name", 1), ("type_arguments", 2), ("attributes", 3)],
    },
    TreeSitterPattern {
        name: "jsx_closing_element",
        biome_kind: "JSX_CLOSING_ELEMENT",
        slots: &[],
    },
    TreeSitterPattern {
        name: "jsx_text",
        biome_kind: "JSX_TEXT",
        slots: &[],
    },
    TreeSitterPattern {
        name: "jsx_namespace_name",
        biome_kind: "JSX_NAMESPACE_NAME",
        slots: &[],
    },
];

const JSON_TREESITTER_PATTERNS: &[TreeSitterPattern] = &[
    TreeSitterPattern {
        name: "object",
        biome_kind: "JSON_OBJECT_VALUE",
        slots: &[("members", 1)],
    },
    TreeSitterPattern {
        name: "array",
        biome_kind: "JSON_ARRAY_VALUE",
        slots: &[("elements", 1)],
    },
    TreeSitterPattern {
        name: "pair",
        biome_kind: "JSON_MEMBER",
        slots: &[("key", 0), ("value", 2)],
    },
    TreeSitterPattern {
        name: "string",
        biome_kind: "JSON_STRING_VALUE",
        slots: &[],
    },
    TreeSitterPattern {
        name: "number",
        biome_kind: "JSON_NUMBER_VALUE",
        slots: &[],
    },
    TreeSitterPattern {
        name: "true",
        biome_kind: "JSON_BOOLEAN_VALUE",
        slots: &[],
    },
    TreeSitterPattern {
        name: "false",
        biome_kind: "JSON_BOOLEAN_VALUE",
        slots: &[],
    },
    TreeSitterPattern {
        name: "null",
        biome_kind: "JSON_NULL_VALUE",
        slots: &[],
    },
    TreeSitterPattern {
        name: "document",
        biome_kind: "JSON_ROOT",
        slots: &[("value", 1)],
    },
];
