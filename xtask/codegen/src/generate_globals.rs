use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::{AnyJsModuleItem, AnyJsRoot, JsFileSource, ModuleKind};
use biome_rowan::AstNode;
use std::{fs, u8};
use std::path::Path;
use xtask::{project_root, Result};
use xtask_codegen::update;

struct GlobalType {
    name: String,
    kind: String,
    source_file: String,
}

pub(crate) fn generate_globals() -> Result<()> {
    let types_dir = project_root().join("xtask/codegen/src/types");
    let mut global_types = Vec::new();
    let files = fs::read_dir(&types_dir)?;
    for file in files {
        let path = file?.path();
        let source_code = fs::read_to_string(&path)?;

        // use biome parser to parse the TypeScript definition file
        let parsed = parse(
            &source_code, 
            JsFileSource::ts().with_module_kind(ModuleKind::Module),
            JsParserOptions::default());

        if parsed.has_errors() {
            eprintln!("Parse errors in {}: {:?}", path.display(), parsed.diagnostics());
            continue;
        }

        process_root(&parsed.tree(), path.display().to_string().as_str(), &mut global_types);
    }

    // Deduplicate types by name (keep first occurrence)
    // TODO: Support declaration merging
    let mut seen_names = std::collections::HashSet::new();
    global_types.retain(|gt| seen_names.insert(gt.name.clone()));

    // Sort types by source file, then by name
    global_types.sort_by(|a, b| {
        a.source_file.cmp(&b.source_file)
            .then_with(|| a.name.cmp(&b.name))
    });

    println!("Found {} unique types (after deduplication)", global_types.len());

    // Generate the output file
    let output = generate_output(&global_types);
    let output_path = project_root().join("crates/biome_js_type_info/src/generated_globals.rs");
    update(&output_path, &output, &xtask::Mode::Overwrite)?;

    println!("Generated generated_globals.rs with {} types", global_types.len());

    Ok(())
}

fn process_root(root: &AnyJsRoot, file_name: &str, global_types: &mut Vec<GlobalType>) {
    // Try to cast to JsModule
    if let AnyJsRoot::JsModule(module) = root {
        for item in module.items() {
            process_module_item(&item, file_name, global_types);
        }
    }
}

fn process_module_item(item: &AnyJsModuleItem, file_name: &str, global_types: &mut Vec<GlobalType>) {
    match item {
        AnyJsModuleItem::AnyJsStatement(stmt) => {
            // Check for TypeScript declare statements
            let stmt_text = stmt.syntax().text_trimmed().to_string();
            if stmt_text.starts_with("declare") || stmt_text.starts_with("interface") {
                // Extract interface or type name
                // Here, we only handle interfaces, type aliases, and classes
                if let Some(name) = extract_type_name(&stmt_text) {
                    let kind = if stmt_text.contains("interface") {
                        "interface"
                    } else if stmt_text.contains("type ") {
                        "type alias"
                    } else if stmt_text.contains("class ") {
                        "class"
                    } else {
                        "declaration"
                    };

                    global_types.push(GlobalType {
                        name: name.to_string(),
                        kind: kind.to_string(),
                        source_file: file_name.to_string(),
                    });
                }
            }
        }
        _ => {}
    }
}

fn extract_type_name(text: &str) -> Option<&str> {
    // Simple extraction of type/interface name
    // Look for patterns like "interface Name", "type Name", "class Name", etc.
    let patterns = ["interface ", "type ", "class ", "declare interface ", "declare type ", "declare class "];
    
    for pattern in &patterns {
        if let Some(pos) = text.find(pattern) {
            let after_keyword = &text[pos + pattern.len()..];
            if let Some(name_end) = after_keyword.find(|c: char| !c.is_alphanumeric() && c != '_') {
                return Some(after_keyword[..name_end].trim());
            } else {
                return Some(after_keyword.trim());
            }
        }
    }
    
    None
}

fn generate_output(global_types: &[GlobalType]) -> String {
    let mut output = String::new();

    // Add file header
    output.push_str("\
//! This file is auto-generated. Do not edit manually.
// Run `just gen-globals` to regenerate this file.

use crate::{
    TypeId, ResolvedTypeId, TypeResolverLevel,
};

");

    // Generate TypeId constants for each type
    // This is temporary to distinguish between hardcoded and generated IDs in globals.rs
    const IDX_OFFSET: usize = 100;
    output.push_str("// Type ID constants\n");
    for (idx, global_type) in global_types.iter().enumerate() {
        let const_name = type_name_to_const(&global_type.name);
        output.push_str(&format!(
            "pub const {}_ID: TypeId = TypeId::new({});\n",
            const_name, idx + IDX_OFFSET
        ));
    }
    output.push_str("\n");

    output.push_str("pub const GLOBAL_LEVEL: TypeResolverLevel = TypeResolverLevel::Global;\n");
    for global_type in global_types.iter() {
        let const_name = type_name_to_const(&global_type.name);
        output.push_str(&format!(
            "pub const GLOBAL_{}_ID: ResolvedTypeId = ResolvedTypeId::new(GLOBAL_LEVEL, {}_ID);\n",
            const_name, const_name
        ));
    }
    output.push_str("\n");

    output.push_str("// TODO: Modify xtask to generate global resolver");
    
    output
}

// Convert a type name to a constant name (e.g., "Array" -> "ARRAY", "ArrayLike" -> "ARRAY_LIKE")
// TODO: Check to see if this implementation exists in the codebase already
fn type_name_to_const(name: &str) -> String {
    let mut result = String::new();
    let mut prev_was_lowercase = false;
    
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() && i > 0 && prev_was_lowercase {
            result.push('_');
        }
        
        if c.is_alphanumeric() {
            result.push(c.to_ascii_uppercase());
            prev_was_lowercase = c.is_lowercase();
        } else {
            result.push('_');
            prev_was_lowercase = false;
        }
    }
    
    result.trim_end_matches('_').to_string()
}