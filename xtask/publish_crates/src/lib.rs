mod bundle_plan;
mod cargo;
mod plan;
mod publisher;
mod rate_limit;
mod registry;
mod workspace;

use std::{env, path::PathBuf};

use anyhow::{Result, ensure};
use bpaf::Bpaf;

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]
pub enum Command {
    #[bpaf(command)]
    BundlePlan {
        #[bpaf(switch)]
        check: bool,
    },
    #[bpaf(command)]
    Verify {
        #[bpaf(switch)]
        allow_dirty: bool,
        #[bpaf(long("staged-bundle"), argument("PATH"))]
        staged_bundle: Option<PathBuf>,
    },
    #[bpaf(command)]
    Publish {
        #[bpaf(switch)]
        execute: bool,
        #[bpaf(long("verified-revision"), argument("REVISION"))]
        verified_revision: Option<String>,
    },
}

pub fn run(command: Command) -> Result<()> {
    let directory = env::current_dir()?;
    match command {
        Command::BundlePlan { check } => bundle_plan::print(&directory, check),
        Command::Verify {
            allow_dirty,
            staged_bundle,
        } => match staged_bundle {
            Some(staging_dir) => {
                ensure!(
                    !allow_dirty,
                    "--allow-dirty cannot be used with --staged-bundle"
                );
                bundle_plan::verify_staged(&directory, &staging_dir)
            }
            None => publisher::verify(&directory, allow_dirty),
        },
        Command::Publish {
            execute,
            verified_revision,
        } => publisher::publish(&directory, execute, verified_revision.as_deref()),
    }
}
