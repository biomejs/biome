mod ast;
mod generate_macros;
mod generate_node_factory;
mod generate_nodes;
mod generate_nodes_mut;
mod generate_syntax_factory;
mod generate_syntax_kinds;
mod kind;

use crate::ast::{generate_syntax, load_ungrammar_file};
use crate::kind::KindsSrc;
use anyhow::Result;
use std::path::PathBuf;

pub struct Options<K> {
    syntax_path: PathBuf,
    syntax_factory_path: PathBuf,
    syntax_crate_name: String,
    language_kind: K,
    ungrammar_file_path: PathBuf,
    check_unions: bool,
}

pub fn generate_ast<'a, K>(options: Options<K>) -> Result<()>
where
    K: KindsSrc<'a>,
{
    let ast = load_ungrammar_file(options.ungrammar_file_path.as_path(), options.check_unions)?;
    generate_syntax(ast, options)?;

    Ok(())
}
