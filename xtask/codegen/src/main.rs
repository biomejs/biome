#[cfg(feature = "schema")]
mod generate_bindings;
#[cfg(feature = "configuration")]
mod generate_configuration;
#[cfg(feature = "license")]
mod generate_license;
mod generate_new_lintrule;
#[cfg(feature = "schema")]
mod generate_schema;
#[cfg(feature = "website")]
mod generate_website;
mod promote_rule;

use pico_args::Arguments;
use xtask::{project_root, pushd, Mode, Result};

#[cfg(feature = "aria")]
use crate::generate_aria::generate_aria;
#[cfg(feature = "schema")]
use crate::generate_bindings::generate_workspace_bindings;
#[cfg(feature = "configuration")]
use crate::generate_configuration::generate_rules_configuration;
#[cfg(feature = "license")]
use crate::generate_license::generate_license;
#[cfg(feature = "schema")]
use crate::generate_schema::generate_configuration_schema;
#[cfg(feature = "website")]
use crate::generate_website::generate_files;
use crate::promote_rule::promote_rule;
use generate_new_lintrule::*;
use xtask_codegen::{
    generate_analyzer, generate_ast, generate_formatters, generate_parser_tests, generate_tables,
};

fn main() -> Result<()> {
    let _d = pushd(project_root());

    let mut args = Arguments::from_env();
    let command = args.subcommand()?.unwrap_or_default();
    match command.as_str() {
        "grammar" => {
            generate_grammar(args);
            Ok(())
        }
        "formatter" => {
            generate_formatters();
            Ok(())
        }
        "test" => {
            generate_parser_tests(Mode::Overwrite)?;
            Ok(())
        }
        "unicode" => {
            generate_tables()?;
            Ok(())
        }
        "analyzer" => {
            generate_analyzer()?;
            Ok(())
        }
        "newlintrule" => {
            let path: String = args.value_from_str("--path").unwrap();
            let rule_name: String = args.value_from_str("--name").unwrap();
            generate_new_lintrule(&path, &rule_name);
            Ok(())
        }
        "promoterule" => {
            let rule: String = args.value_from_str("--rule").unwrap();
            let group: String = args.value_from_str("--group").unwrap();
            promote_rule(&rule, &group);
            Ok(())
        }
        #[cfg(feature = "configuration")]
        "configuration" => {
            generate_rules_configuration(Mode::Overwrite)?;
            Ok(())
        }
        #[cfg(feature = "schema")]
        "schema" => {
            generate_configuration_schema(Mode::Overwrite)?;
            Ok(())
        }
        #[cfg(feature = "schema")]
        "bindings" => {
            generate_workspace_bindings(Mode::Overwrite)?;
            Ok(())
        }
        #[cfg(feature = "website")]
        "website" => {
            generate_files()?;
            Ok(())
        }
        #[cfg(feature = "license")]
        "license" => {
            generate_license(Mode::Overwrite)?;
            Ok(())
        }
        "all" => {
            generate_tables()?;
            generate_grammar(args);
            generate_parser_tests(Mode::Overwrite)?;
            generate_formatters();
            generate_analyzer()?;
            #[cfg(feature = "website")]
            generate_files()?;
            #[cfg(feature = "configuration")]
            generate_rules_configuration(Mode::Overwrite)?;
            #[cfg(feature = "schema")]
            generate_configuration_schema(Mode::Overwrite)?;
            #[cfg(feature = "schema")]
            generate_workspace_bindings(Mode::Overwrite)?;
            #[cfg(feature = "aria")]
            generate_aria(Mode::Overwrite)?;
            Ok(())
        }
        _ => {
            eprintln!(
                "\
cargo codegen
Run codegen command.
USAGE:
	cargo codegen <SUBCOMMAND> [option]
SUBCOMMANDS:
	aria            Generate aria bindings for lint rules
	analyzer        Generate factory functions for the analyzer and the configuration of the analyzers
	configuration    Generate the part of the configuration that depends on some metadata
	schema          Generate the JSON schema for the Biome configuration file format
	bindings        Generate TypeScript definitions for the JavaScript bindings to the Workspace API
    license         It updates the file that contains licenses
	grammar         Transforms ungram files into AST
	formatter       Generates formatters for each language
	test            Extracts parser inline comments into test files
	unicode         Generates unicode table inside lexer
    newlintrule     Generates a template for an empty lint rule
    all             Run all generators
			"
            );
            Ok(())
        }
    }
}

fn generate_grammar(args: Arguments) {
    let arg_list = args.finish();
    let language_list = arg_list
        .into_iter()
        .filter_map(|arg| arg.to_str().map(|item| item.to_string()))
        .collect::<Vec<_>>();
    let _ = generate_ast(Mode::Overwrite, language_list);
}
