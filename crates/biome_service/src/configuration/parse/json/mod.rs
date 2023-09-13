//! This module is responsible to parse the configuration from a JSON format
//!

mod configuration;
mod files;
mod formatter;
mod javascript;
mod json_impl;
mod linter;
mod organize_imports;
mod rules;
mod vcs;

use crate::Configuration;
use biome_deserialize::json::{JsonDeserialize, VisitJsonNode};
use biome_deserialize::DeserializationDiagnostic;
use biome_json_syntax::{AnyJsonValue, JsonRoot};
use biome_rowan::AstNode;

impl JsonDeserialize for Configuration {
    fn deserialize_from_ast(
        root: &JsonRoot,
        visitor: &mut impl VisitJsonNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let value = root.value().ok()?;
        match value {
            AnyJsonValue::JsonObjectValue(node) => {
                for element in node.json_member_list() {
                    let element = element.ok()?;
                    let member_name = element.name().ok()?;
                    let member_value = element.value().ok()?;
                    visitor.visit_map(member_name.syntax(), member_value.syntax(), diagnostics)?;
                }
                Some(())
            }
            _ => {
                diagnostics.push(
                    DeserializationDiagnostic::new("The configuration should be an object")
                        .with_range(root.range()),
                );
                None
            }
        }
    }
}
