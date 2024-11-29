//! Biome utility crate to generate a grammar that can be used with `biome_parser`
//! The utility is based on internal fork of `ungrammar`, and it uses a **strict** naming convention
//! to generate utility nodes such as union nodes, list nodes and more.
//!
//! Check the `examples/` folder to understand how to write one.
//!
//! To run the example, execute:
//! ```shell
//! cargo run --example minigrammar
//! ```

mod ast;
mod generate_macros;
mod generate_node_factory;
mod generate_nodes;
mod generate_nodes_mut;
mod generate_syntax_factory;
mod generate_syntax_kinds;
mod language_src;

use crate::ast::{generate_syntax, load_ungrammar_file};
pub use crate::language_src::LanguageSrc;
use anyhow::Result;
use std::path::PathBuf;

/// Required options to generate the grammar
pub struct GrammarOptions {
    /// A path to a **directory** where to save code generated files for the syntax.
    /// Ideally, you'd want to point this directory to a crate
    pub syntax_dir_path: PathBuf,
    /// A path to a **directory** where to save code generated files for syntax factory.
    /// Ideally, you'd want to point this directory to a crate
    pub factory_dir_path: PathBuf,
    /// The files saved inside the directory `factory_dir_path` will import files coming from `syntax_dir_path`.
    /// Use this option in case you plan to generate these files in different directories.
    /// If you plan to generate these files in the same crate, use `"crate"` instead.  
    pub syntax_crate_name: String,
    /// The path to the **file** `.ungram`
    pub ungrammar_file_path: PathBuf,
    /// Adds a BFS search implementation to check if unions trees contain circular dependencies or reuse types.
    /// The goal is to prevent potential union circular dependencies as well as unions having the same type in multiple branches.
    pub check_unions: bool,
}

/// Generates the grammar
pub fn generate_grammar<K>(language_src: K, options: GrammarOptions) -> Result<()>
where
    K: LanguageSrc,
{
    let ast = load_ungrammar_file(options.ungrammar_file_path.as_path(), options.check_unions)?;
    generate_syntax(language_src, ast, options)?;

    Ok(())
}
