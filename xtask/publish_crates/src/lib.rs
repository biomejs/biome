mod bundle_plan;
mod cargo;
mod plan;
mod publisher;
mod rate_limit;
mod registry;
mod workspace;

use std::env;

use anyhow::Result;
use bpaf::Bpaf;

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]
pub enum Command {
    #[bpaf(command)]
    BundlePlan,
    #[bpaf(command)]
    Verify {
        #[bpaf(switch)]
        allow_dirty: bool,
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
        Command::BundlePlan => bundle_plan::print(&directory),
        Command::Verify { allow_dirty } => publisher::verify(&directory, allow_dirty),
        Command::Publish {
            execute,
            verified_revision,
        } => publisher::publish(&directory, execute, verified_revision.as_deref()),
    }
}
