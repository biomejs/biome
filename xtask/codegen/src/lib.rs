//! Codegen tools for generating Syntax and AST definitions. Derived from Rust analyzer's codegen
//!
mod ast;
mod css_kinds_src;
mod formatter;
mod generate_analyzer;
mod generate_macros;
pub mod generate_new_lintrule;
mod generate_node_factory;
mod generate_nodes;
mod generate_nodes_mut;
mod generate_syntax_factory;
mod generate_syntax_kinds;
mod grit_kinds_src;
mod js_kinds_src;
mod json_kinds_src;

mod generate_crate;
mod html_kinds_src;
mod kind_src;
mod language_kind;
mod parser_tests;
pub mod promote_rule;
mod termcolorful;
mod unicode;

use bpaf::Bpaf;
use std::path::Path;

use xtask::{glue::fs2, Mode, Result};

pub use self::ast::generate_ast;
pub use self::formatter::generate_formatters;
pub use self::generate_analyzer::generate_analyzer;
pub use self::generate_crate::generate_crate;
pub use self::generate_new_lintrule::{generate_new_lintrule, RuleKind};
pub use self::parser_tests::generate_parser_tests;
pub use self::unicode::generate_tables;

pub enum UpdateResult {
    NotUpdated,
    Updated,
}

/// A helper to update file on disk if it has changed.
/// With verify = false,
pub fn update(path: &Path, contents: &str, mode: &Mode) -> Result<UpdateResult> {
    match fs2::read_to_string(path) {
        Ok(old_contents) if old_contents == contents => {
            return Ok(UpdateResult::NotUpdated);
        }
        _ => (),
    }

    if *mode == Mode::Verify {
        anyhow::bail!("`{}` is not up-to-date", path.display());
    }

    eprintln!("updating {}", path.display());
    fs2::write(path, contents)?;
    Ok(UpdateResult::Updated)
}

pub fn to_capitalized(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub enum TaskCommand {
    /// Generates formatters for each language
    #[bpaf(command)]
    Formatter,
    /// Generate factory functions for the analyzer and the configuration of the analyzers
    #[bpaf(command)]
    Analyzer,
    /// Generate the part of the configuration that depends on some metadata
    #[bpaf(command)]
    Configuration,
    /// Generate the JSON schema for the Biome configuration file format
    #[bpaf(command)]
    Schema,
    /// Generate TypeScript definitions for the JavaScript bindings to the Workspace API
    #[bpaf(command)]
    Bindings,
    /// It updates the file that contains licenses
    #[bpaf(command)]
    License,
    /// Transforms ungram files into AST
    #[bpaf(command)]
    Grammar(Vec<String>),
    /// Extracts parser inline comments into test files
    #[bpaf(command)]
    Test,
    /// Generates unicode table inside lexer
    #[bpaf(command)]
    Unicode,
    /// Creates a new lint rule
    #[bpaf(command, long("new-lintrule"))]
    NewLintRule(
        /// Path of the rule
        #[bpaf(long("kind"))]
        RuleKind,
        /// Name of the rule
        #[bpaf(long("name"))]
        String,
    ),
    /// Promotes a nursery rule
    #[bpaf(command, long("promote-rule"))]
    PromoteRule {
        /// Path of the rule
        #[bpaf(long("name"), argument("STRING"))]
        name: String,
        /// Name of the rule
        #[bpaf(long("group"), argument("STRING"))]
        group: String,
    },
    /// Generates website files
    #[bpaf(command)]
    Website,
    /// Runs ALL the codegen
    #[bpaf(command)]
    All,
    /// Creates a new crate
    #[bpaf(command, long("new-crate"))]
    NewCrate {
        /// The name of the crate
        #[bpaf(long("name"), argument("STRING"))]
        name: String,
    },
}
