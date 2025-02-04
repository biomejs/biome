#[cfg(feature = "schema")]
mod generate_bindings;
#[cfg(feature = "configuration")]
mod generate_configuration;
#[cfg(feature = "license")]
mod generate_license;
#[cfg(feature = "configuration")]
mod generate_migrate_eslint;
#[cfg(feature = "schema")]
mod generate_schema;
mod promote_rule;
use xtask::{project_root, pushd, Result};

#[cfg(feature = "schema")]
use crate::generate_bindings::generate_workspace_bindings;
#[cfg(feature = "configuration")]
use crate::generate_configuration::generate_rules_configuration;
#[cfg(feature = "license")]
use crate::generate_license::generate_license;
#[cfg(feature = "configuration")]
use crate::generate_migrate_eslint::generate_migrate_eslint;
#[cfg(feature = "schema")]
use crate::generate_schema::generate_configuration_schema;
use crate::promote_rule::promote_rule;

use xtask::Mode::Overwrite;
use xtask_codegen::{
    generate_analyzer, generate_ast, generate_formatters, generate_new_analyzer_rule,
    generate_tables, task_command, TaskCommand,
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
        TaskCommand::MigrateEslint => {
            #[cfg(feature = "configuration")]
            generate_migrate_eslint(Overwrite)?;
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
        TaskCommand::Unicode => {
            generate_tables()?;
        }
        TaskCommand::NewRule {
            category,
            name,
            kind,
        } => {
            generate_new_analyzer_rule(kind, category, &name);
        }
        TaskCommand::PromoteRule { name, group } => {
            promote_rule(&name, &group);
        }
        TaskCommand::All => {
            generate_tables()?;
            generate_ast(Overwrite, vec![])?;
            generate_formatters();
            generate_analyzer()?;
            #[cfg(feature = "configuration")]
            generate_rules_configuration(Overwrite)?;
            #[cfg(feature = "schema")]
            generate_configuration_schema(Overwrite)?;
            #[cfg(feature = "schema")]
            generate_workspace_bindings(Overwrite)?;
        }
    }

    Ok(())
}
