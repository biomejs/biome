#[cfg(feature = "schema")]
mod generate_bindings;
#[cfg(feature = "configuration")]
mod generate_configuration;
#[cfg(feature = "configuration")]
mod generate_eslint_migrate;
#[cfg(feature = "license")]
mod generate_license;
#[cfg(feature = "schema")]
mod generate_schema;
#[cfg(feature = "website")]
mod generate_website;
mod promote_rule;
use xtask::{project_root, pushd, Result};

#[cfg(feature = "schema")]
use crate::generate_bindings::generate_workspace_bindings;
#[cfg(feature = "configuration")]
use crate::generate_configuration::generate_rules_configuration;
#[cfg(feature = "configuration")]
use crate::generate_eslint_migrate::generate_eslint_migrate;
#[cfg(feature = "license")]
use crate::generate_license::generate_license;
#[cfg(feature = "schema")]
use crate::generate_schema::generate_configuration_schema;
#[cfg(feature = "website")]
use crate::generate_website::generate_files;
use crate::promote_rule::promote_rule;

use xtask::Mode::Overwrite;
use xtask_codegen::{
    generate_analyzer, generate_ast, generate_crate, generate_formatters, generate_new_lintrule,
    generate_parser_tests, generate_tables, task_command, TaskCommand,
};

fn main() -> Result<()> {
    let _d = pushd(project_root());
    let result = task_command().fallback_to_usage().run();

    match result {
        TaskCommand::Formatter => {
            generate_formatters();
        }
        TaskCommand::Analyzer => {
            generate_analyzer()?;
        }
        TaskCommand::Configuration => {
            #[cfg(feature = "configuration")]
            generate_rules_configuration(Overwrite)?;
        }
        TaskCommand::EslintMigrate => {
            #[cfg(feature = "configuration")]
            generate_eslint_migrate(Overwrite)?;
        }
        TaskCommand::Schema => {
            #[cfg(feature = "schema")]
            generate_configuration_schema(Overwrite)?;
        }
        TaskCommand::Bindings => {
            #[cfg(feature = "schema")]
            generate_workspace_bindings(Overwrite)?;
        }
        TaskCommand::License => {
            #[cfg(feature = "license")]
            generate_license(Overwrite)?;
        }
        TaskCommand::Grammar(language_list) => {
            generate_ast(Overwrite, language_list)?;
        }
        TaskCommand::Test => {
            generate_parser_tests(Overwrite)?;
        }
        TaskCommand::Unicode => {
            generate_tables()?;
        }
        TaskCommand::NewLintRule(new_rule_kind, rule_name) => {
            generate_new_lintrule(new_rule_kind, &rule_name);
        }
        TaskCommand::PromoteRule { name, group } => {
            promote_rule(&name, &group);
        }
        TaskCommand::Website => {
            #[cfg(feature = "website")]
            generate_files()?;
        }
        TaskCommand::All => {
            generate_tables()?;
            generate_ast(Overwrite, vec![])?;
            generate_parser_tests(Overwrite)?;
            generate_formatters();
            generate_analyzer()?;
            #[cfg(feature = "website")]
            generate_files()?;
            #[cfg(feature = "configuration")]
            generate_rules_configuration(Overwrite)?;
            #[cfg(feature = "schema")]
            generate_configuration_schema(Overwrite)?;
            #[cfg(feature = "schema")]
            generate_workspace_bindings(Overwrite)?;
        }
        TaskCommand::NewCrate { name } => {
            generate_crate(name)?;
        }
    }

    Ok(())
}
