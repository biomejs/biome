use std::{
    collections::BTreeMap,
    env,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};

use crate::workspace::{PublishPackage, Workspace};

pub(crate) trait CargoRunner {
    fn verify(&mut self, workspace: &Workspace, allow_dirty: bool) -> Result<()>;
    fn package(&mut self, workspace: &Workspace) -> Result<BTreeMap<String, String>>;
    fn publish(&mut self, workspace: &Workspace, package: &PublishPackage) -> Result<bool>;
}

pub(crate) struct SystemCargo {
    executable: String,
}

impl SystemCargo {
    pub(crate) fn new() -> Self {
        Self {
            executable: env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()),
        }
    }

    fn command(&self, root: &Path) -> Command {
        let mut command = Command::new(&self.executable);
        command.current_dir(root);
        command
    }

    fn run_checked(&self, root: &Path, args: &[String]) -> Result<()> {
        let status = self
            .command(root)
            .args(args)
            .status()
            .with_context(|| format!("failed to run cargo {}", args.join(" ")))?;
        if !status.success() {
            bail!("cargo {} failed with status {status}", args.join(" "));
        }
        Ok(())
    }
}

impl CargoRunner for SystemCargo {
    fn verify(&mut self, workspace: &Workspace, allow_dirty: bool) -> Result<()> {
        let mut args = vec![
            "publish".to_string(),
            "--dry-run".to_string(),
            "--locked".to_string(),
            "--registry".to_string(),
            "crates-io".to_string(),
        ];
        if allow_dirty {
            args.push("--allow-dirty".to_string());
        }
        push_packages(&mut args, &workspace.packages);
        self.run_checked(&workspace.root, &args)
    }

    fn package(&mut self, workspace: &Workspace) -> Result<BTreeMap<String, String>> {
        let mut args = vec![
            "package".to_string(),
            "--locked".to_string(),
            "--no-verify".to_string(),
            "--registry".to_string(),
            "crates-io".to_string(),
        ];
        push_packages(&mut args, &workspace.packages);
        self.run_checked(&workspace.root, &args)?;

        workspace
            .packages
            .iter()
            .map(|package| {
                let archive = archive_path(&workspace.target_directory, package);
                sha256(&archive)
                    .with_context(|| format!("failed to hash {}", archive.display()))
                    .map(|checksum| (package.name.clone(), checksum))
            })
            .collect()
    }

    fn publish(&mut self, workspace: &Workspace, package: &PublishPackage) -> Result<bool> {
        self.command(&workspace.root)
            .args([
                "publish",
                "--package",
                package.name.as_str(),
                "--locked",
                "--no-verify",
                "--registry",
                "crates-io",
            ])
            .status()
            .with_context(|| format!("failed to publish {}", package.name))
            .map(|status| status.success())
    }
}

fn push_packages(args: &mut Vec<String>, packages: &[PublishPackage]) {
    for package in packages {
        args.push("--package".to_string());
        args.push(package.name.clone());
    }
}

fn archive_path(target_directory: &Path, package: &PublishPackage) -> PathBuf {
    target_directory
        .join("package")
        .join(format!("{}-{}.crate", package.name, package.version))
}

fn sha256(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut digest = Sha256::new();
    let mut buffer = [0; 8 * 1024];
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    let digest = digest.finalize();
    Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
}
