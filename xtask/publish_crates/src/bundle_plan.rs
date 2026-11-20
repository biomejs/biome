use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
    process,
    sync::atomic::{AtomicU64, Ordering},
};

use anyhow::{Context, Result, ensure};
use serde::Deserialize;

use crate::{
    cargo::SystemCargo,
    plan::topological_order,
    workspace::{PublishPackage, README_PREFIX, Workspace, workspace_metadata},
};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BundleConfig {
    roots: Vec<String>,
    private: BTreeSet<String>,
    bundles: BTreeMap<String, Vec<String>>,
}

#[derive(Debug)]
struct BundlePlan {
    packages: Vec<(String, BTreeSet<String>)>,
    blockers: BTreeMap<String, BTreeSet<String>>,
}

impl BundlePlan {
    fn new(config: &BundleConfig, packages: &[PublishPackage]) -> Result<Self> {
        ensure!(
            !config.roots.is_empty(),
            "bundle plan needs at least one root"
        );
        let sources: BTreeMap<_, _> = packages
            .iter()
            .map(|package| (package.name.as_str(), package))
            .collect();
        let mut owners = BTreeMap::new();
        for (name, members) in &config.bundles {
            ensure!(
                !sources.contains_key(name.as_str()),
                "bundle name {name} is already a publishable crate"
            );
            ensure!(
                members.len() >= 2,
                "bundle {name} needs at least two source crates"
            );
            for member in members {
                ensure!(
                    sources.contains_key(member.as_str()),
                    "bundle {name} refers to unknown publishable crate {member}"
                );
                if let Some(previous) = owners.insert(member.as_str(), name.as_str()) {
                    anyhow::bail!("crate {member} is assigned to both {previous} and {name}");
                }
            }
        }

        let owner = |name| owners.get(name).copied().unwrap_or(name);
        let mut groups: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
        for name in sources.keys() {
            groups.entry(owner(name)).or_default().insert(*name);
        }

        let mut pending = Vec::new();
        for root in &config.roots {
            ensure!(
                sources.contains_key(root.as_str()),
                "unknown publishable root {root}"
            );
            pending.push(owner(root.as_str()));
        }
        let mut projected = BTreeMap::new();
        let mut blockers = BTreeMap::<String, BTreeSet<String>>::new();
        while let Some(name) = pending.pop() {
            if projected.contains_key(name) {
                continue;
            }
            let members = groups
                .get(name)
                .with_context(|| format!("no source crates for package {name}"))?;
            if config.private.contains(name) {
                blockers.entry(name.to_string()).or_default();
            }
            let mut closure_package = None;
            for member in members {
                let source = sources
                    .get(member)
                    .with_context(|| format!("missing publication manifest for {member}"))?;
                let package = closure_package.get_or_insert_with(|| PublishPackage {
                    name: name.to_string(),
                    version: source.version.clone(),
                    dependencies: BTreeSet::new(),
                    dev_dependencies: BTreeSet::new(),
                });
                for (from, to) in [
                    (&source.dependencies, &mut package.dependencies),
                    (&source.dev_dependencies, &mut package.dev_dependencies),
                ] {
                    for dependency in from {
                        let dependency_owner = owner(dependency.as_str());
                        if dependency_owner != name || !config.bundles.contains_key(name) {
                            if config.private.contains(dependency_owner) {
                                blockers
                                    .entry(dependency_owner.to_string())
                                    .or_default()
                                    .insert((*member).to_string());
                            }
                            to.insert(dependency_owner.to_string());
                            pending.push(dependency_owner);
                        }
                    }
                }
            }
            let package =
                closure_package.with_context(|| format!("package {name} has no source crates"))?;
            projected.insert(name, package);
        }
        for bundle in config.bundles.keys() {
            ensure!(
                projected.contains_key(bundle.as_str()),
                "bundle {bundle} is not reachable from the publish roots"
            );
        }

        let publishing = projected.keys().copied().collect();
        let packages: Vec<_> = projected.into_values().collect();
        let order = topological_order(&packages, &publishing)?;
        let packages = order
            .into_iter()
            .map(|package| {
                let members = groups
                    .remove(package.name.as_str())
                    .with_context(|| format!("missing source crates for {}", package.name))?;
                Ok((
                    package.name.clone(),
                    members.into_iter().map(str::to_string).collect(),
                ))
            })
            .collect::<Result<_>>()?;
        Ok(Self { packages, blockers })
    }
}

pub(crate) fn print(directory: &Path, check: bool) -> Result<()> {
    let workspace = Workspace::from_directory(directory, true)?;
    let config = read_config(&workspace.root)?;
    let plan = BundlePlan::new(&config, &workspace.packages)?;
    let blockers = plan.blockers.len();
    write_report(plan, workspace.packages.len(), io::stdout().lock())?;
    ensure!(
        !check || blockers == 0,
        "bundle plan still needs crates marked for exclusion"
    );
    Ok(())
}

pub(crate) fn verify_staged(directory: &Path, staging_dir: &Path) -> Result<()> {
    let root = workspace_metadata(directory)?.workspace_root;
    let staging_dir = staging_dir
        .canonicalize()
        .context("opening the staging directory")?;
    ensure!(
        !staging_dir.starts_with(root.canonicalize()?),
        "the staged bundle must be outside the Biome workspace"
    );
    let config = read_config(root.as_std_path())?;

    let manifest_path = staging_dir.join("Cargo.toml");
    ensure!(
        fs::symlink_metadata(&manifest_path)?.is_file(),
        "expected a regular staged Cargo.toml"
    );
    let mut manifest: toml::Table = toml::from_str(&fs::read_to_string(&manifest_path)?)?;
    let package = manifest
        .get_mut("package")
        .and_then(toml::Value::as_table_mut)
        .context("staged Cargo.toml has no [package] table")?;
    let name = package
        .get("name")
        .and_then(toml::Value::as_str)
        .context("staged Cargo.toml has no package name")?
        .to_string();
    let members = config
        .bundles
        .get(&name)
        .with_context(|| format!("{name} is not a configured bundle"))?;

    let readme_path = staging_dir.join("README.md");
    match fs::symlink_metadata(&readme_path) {
        Ok(metadata) => ensure!(metadata.is_file(), "expected a regular staged README.md"),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }

    let mut readme =
        format!("{README_PREFIX}\n# `{name}`\n\nThis package combines these Biome crates:\n");
    for member in members {
        readme.push_str("\n- `");
        readme.push_str(member);
        readme.push('`');
    }
    readme.push('\n');
    package.insert(
        "readme".to_string(),
        toml::Value::String("README.md".to_string()),
    );
    let manifest = toml::to_string(&manifest)?;
    replace_staged_file(&staging_dir, &readme_path, &readme)
        .with_context(|| format!("writing {}", readme_path.display()))?;
    replace_staged_file(&staging_dir, &manifest_path, &manifest)
        .with_context(|| format!("writing {}", manifest_path.display()))?;

    ensure!(
        fs::read_to_string(&readme_path)?.starts_with(README_PREFIX),
        "staged README.md is missing the shared image"
    );
    SystemCargo::new().verify_staged_package(&staging_dir, &name)
}

fn replace_staged_file(directory: &Path, path: &Path, contents: &str) -> io::Result<()> {
    static NEXT_FILE: AtomicU64 = AtomicU64::new(0);
    let (temporary, mut file) = loop {
        let id = NEXT_FILE.fetch_add(1, Ordering::Relaxed);
        let temporary = directory.join(format!(".biome-bundle-{}-{id}", process::id()));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
        {
            Ok(file) => break (temporary, file),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error),
        }
    };
    let written = file.write_all(contents.as_bytes());
    drop(file);
    let replaced = written.and_then(|()| fs::rename(&temporary, path));
    if replaced.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    replaced
}

fn read_config(root: &Path) -> Result<BundleConfig> {
    let path = root.join("xtask/publish_crates/bundle-plan.toml");
    let source =
        fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    toml::from_str(&source).with_context(|| format!("parsing {}", path.display()))
}

fn write_report(plan: BundlePlan, source_count: usize, mut output: impl Write) -> io::Result<()> {
    writeln!(
        output,
        "Bundle plan: {} packages from {source_count} currently publishable crates (graph only)",
        plan.packages.len()
    )?;
    for (name, members) in plan.packages {
        if members.len() > 1 {
            writeln!(
                output,
                "{name} <- {}",
                members.into_iter().collect::<Vec<_>>().join(", ")
            )?;
        } else {
            writeln!(output, "{name}")?;
        }
    }
    if !plan.blockers.is_empty() {
        writeln!(
            output,
            "\nPlanned private crates still needed by this release set:"
        )?;
        for (name, dependents) in plan.blockers {
            if dependents.is_empty() {
                writeln!(output, "{name}: selected by the plan")?;
            } else {
                writeln!(
                    output,
                    "{name}: required by {}",
                    dependents.into_iter().collect::<Vec<_>>().join(", ")
                )?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(source: &str) -> BundleConfig {
        toml::from_str(source).unwrap()
    }

    #[test]
    fn includes_dependencies_from_every_member_and_orders_packages() {
        let config = config(
            r#"
            roots = ["parser"]
            private = ["internal", "syntax", "unused"]
            [bundles]
            language = ["parser", "syntax", "formatter"]
        "#,
        );
        let mut formatter = PublishPackage::new("formatter", &["parser", "internal"]);
        formatter.dev_dependencies.insert("z_fixture".to_string());
        let packages = [
            formatter,
            PublishPackage::new("parser", &["syntax"]),
            PublishPackage::new("syntax", &[]),
            PublishPackage::new("internal", &[]),
            PublishPackage::new("z_fixture", &[]),
            PublishPackage::new("unused", &[]),
        ];

        let plan = BundlePlan::new(&config, &packages).unwrap();
        assert_eq!(
            plan.packages
                .iter()
                .map(|(name, _)| name.as_str())
                .collect::<Vec<_>>(),
            ["internal", "z_fixture", "language"]
        );
        assert_eq!(
            plan.blockers,
            BTreeMap::from([(
                "internal".to_string(),
                BTreeSet::from(["formatter".to_string()])
            )])
        );
    }

    #[test]
    fn propagates_report_write_errors() {
        let plan = BundlePlan {
            packages: Vec::new(),
            blockers: BTreeMap::new(),
        };
        let output: &mut [u8] = &mut [];
        assert_eq!(
            write_report(plan, 0, output).unwrap_err().kind(),
            io::ErrorKind::WriteZero
        );
    }

    #[test]
    fn rejects_cycles_introduced_by_bundling() {
        let config = config(
            r#"
            roots = ["formatter"]
            private = []
            [bundles]
            language = ["formatter", "parser"]
        "#,
        );
        let packages = [
            PublishPackage::new("formatter", &["deserialize"]),
            PublishPackage::new("deserialize", &["parser"]),
            PublishPackage::new("parser", &[]),
        ];

        let error = BundlePlan::new(&config, &packages).unwrap_err();
        assert_eq!(
            error.to_string(),
            "circular publish dependency detected among: deserialize, language"
        );
    }

    #[test]
    fn rejects_overlapping_bundles() {
        let config = config(
            r#"
            roots = ["parser"]
            private = []
            [bundles]
            first = ["parser", "syntax"]
            second = ["formatter", "syntax"]
        "#,
        );
        let packages = [
            PublishPackage::new("parser", &[]),
            PublishPackage::new("syntax", &[]),
            PublishPackage::new("formatter", &[]),
        ];

        let error = BundlePlan::new(&config, &packages).unwrap_err();
        assert_eq!(
            error.to_string(),
            "crate syntax is assigned to both first and second"
        );
    }
}
