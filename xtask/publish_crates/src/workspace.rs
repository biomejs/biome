use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, bail};
use cargo_metadata::{DependencyKind, Metadata, MetadataCommand, Package, semver::Version};
use ignore::gitignore::GitignoreBuilder;

type TomlTable = toml::map::Map<String, toml::Value>;

pub(crate) const README_PREFIX: &str = r#"<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-dark-transparent.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg">
    <img alt="Shows the banner of Biome, with its logo and the phrase 'Biome - Toolchain of the web'." src="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg" width="400">
  </picture>
</p>
"#;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PublishPackage {
    pub(crate) name: String,
    pub(crate) version: Version,
    pub(crate) dependencies: BTreeSet<String>,
    pub(crate) dev_dependencies: BTreeSet<String>,
}

impl PublishPackage {
    #[cfg(test)]
    pub(crate) fn new(name: &str, dependencies: &[&str]) -> Self {
        Self {
            name: name.to_string(),
            version: Version::new(1, 0, 0),
            dependencies: dependencies
                .iter()
                .map(|dependency| (*dependency).to_string())
                .collect(),
            dev_dependencies: BTreeSet::new(),
        }
    }
}

#[derive(Clone, Debug)]
struct IncludePolicy {
    directory: PathBuf,
    patterns: Option<Vec<String>>,
    always_included: BTreeSet<PathBuf>,
}

impl IncludePolicy {
    fn from_package(root: &Path, package: &Package, patterns: Option<Vec<String>>) -> Result<Self> {
        let package_directory = package
            .manifest_path
            .parent()
            .with_context(|| format!("{} has no parent directory", package.manifest_path))?;
        let directory = package_directory
            .as_std_path()
            .strip_prefix(root)
            .with_context(|| format!("{} is outside the workspace", package.manifest_path))?
            .to_path_buf();
        let mut always_included = BTreeSet::from([directory.join("Cargo.toml")]);
        for path in [package.readme.as_ref(), package.license_file.as_ref()]
            .into_iter()
            .flatten()
        {
            let path = if path.is_absolute() {
                path.clone()
            } else {
                package_directory.join(path)
            };
            always_included.insert(
                normalize_path(path.as_std_path())
                    .strip_prefix(root)
                    .with_context(|| {
                        format!("always-included package input {path} is outside the workspace")
                    })?
                    .to_path_buf(),
            );
        }
        Ok(Self {
            directory,
            patterns,
            always_included,
        })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Workspace {
    pub(crate) root: PathBuf,
    pub(crate) target_directory: PathBuf,
    pub(crate) version: Version,
    pub(crate) packages: Vec<PublishPackage>,
    fingerprint: GitFingerprint,
    include_policies: Vec<IncludePolicy>,
}

pub(crate) fn workspace_metadata(directory: &Path) -> Result<Metadata> {
    MetadataCommand::new()
        .current_dir(directory)
        .no_deps()
        .other_options(vec!["--locked".to_string()])
        .exec()
        .context("failed to read Cargo metadata")
}

impl Workspace {
    pub(crate) fn from_directory(directory: &Path, allow_dirty: bool) -> Result<Self> {
        let metadata = workspace_metadata(directory)?;
        let root = metadata.workspace_root.clone().into_std_path_buf();
        let target_directory = metadata.target_directory.clone().into_std_path_buf();
        let fingerprint = GitFingerprint::from_repository(&root, allow_dirty)?;
        let root_manifest = read_toml(&root.join("Cargo.toml"))?;
        let version = workspace_version(&root_manifest)?;
        let workspace_dependencies = workspace_dependencies(&root_manifest)?;
        let workspace_packages = metadata.workspace_packages();
        let packages_by_name = workspace_packages
            .iter()
            .map(|package| (package.name.as_str(), *package))
            .collect::<BTreeMap<_, _>>();
        let manifests = workspace_packages
            .iter()
            .map(|package| {
                PackageManifest::from_package(package)
                    .map(|manifest| (package.name.as_str(), manifest))
            })
            .collect::<Result<BTreeMap<_, _>>>()?;
        let mut errors = Vec::new();
        let mut public_names = BTreeSet::new();
        let mut include_policies = Vec::new();

        for package in &workspace_packages {
            let manifest = manifests
                .get(package.name.as_str())
                .with_context(|| format!("missing manifest data for {}", package.name))?;
            match manifest.publish {
                Some(true) => {
                    public_names.insert(package.name.as_str());
                    include_policies.push(IncludePolicy::from_package(
                        &root,
                        package,
                        manifest.include.clone(),
                    )?);
                    if !manifest.version_workspace {
                        errors.push(format!(
                            "{}: public crate must inherit the workspace version",
                            relative_manifest(&root, package)
                        ));
                    }
                    if package.version != version {
                        errors.push(format!(
                            "{}: expected version {version}, found {}",
                            relative_manifest(&root, package),
                            package.version
                        ));
                    }
                    if package.description.as_deref().is_none_or(str::is_empty) {
                        errors.push(format!(
                            "{}: public crate is missing a description",
                            relative_manifest(&root, package)
                        ));
                    }
                    let readme = package
                        .manifest_path
                        .parent()
                        .with_context(|| {
                            format!("{} has no parent directory", package.manifest_path)
                        })?
                        .join("README.md");
                    match fs::read_to_string(&readme) {
                        Ok(contents) if contents.starts_with(README_PREFIX) => {}
                        Ok(_) => errors.push(format!(
                            "{}: README.md is missing the shared image",
                            relative_manifest(&root, package)
                        )),
                        Err(error) if error.kind() == std::io::ErrorKind::NotFound => errors.push(
                            format!("{}: missing README.md", relative_manifest(&root, package)),
                        ),
                        Err(error) => errors.push(format!(
                            "{}: failed to read README.md: {error}",
                            relative_manifest(&root, package)
                        )),
                    }
                    if let Some(dependency) = workspace_dependencies.get(package.name.as_str()) {
                        let dependency_version =
                            dependency.get("version").and_then(toml::Value::as_str);
                        match dependency_version {
                            Some(dependency_version) if dependency_version == version.to_string() => {}
                            Some(dependency_version) => errors.push(format!(
                                "Cargo.toml: {} workspace dependency must use {version}, found {dependency_version}",
                                package.name
                            )),
                            None => errors.push(format!(
                                "Cargo.toml: missing versioned workspace dependency for {}",
                                package.name
                            )),
                        }
                    }
                }
                Some(false) => {
                    if manifest.version_workspace {
                        errors.push(format!(
                            "{}: private crate inherits the workspace release version",
                            relative_manifest(&root, package)
                        ));
                    }
                }
                None => errors.push(format!(
                    "{}: missing explicit publish field",
                    relative_manifest(&root, package)
                )),
            }
        }

        let mut publish_packages = Vec::new();
        for package in workspace_packages
            .iter()
            .filter(|package| public_names.contains(package.name.as_str()))
        {
            let mut dependencies = BTreeSet::new();
            let mut dev_dependencies = BTreeSet::new();
            for dependency in &package.dependencies {
                if dependency.path.is_none() {
                    continue;
                }
                if dependency.req == cargo_metadata::semver::VersionReq::STAR {
                    if dependency.kind != DependencyKind::Development {
                        errors.push(format!(
                            "{}: path dependency {} is missing a version requirement",
                            package.name, dependency.name
                        ));
                    }
                    continue;
                }
                let Some(workspace_dependency) = packages_by_name.get(dependency.name.as_str())
                else {
                    continue;
                };
                if !public_names.contains(workspace_dependency.name.as_str()) {
                    errors.push(format!(
                        "{}: publishable crate depends on private workspace crate {}",
                        package.name, workspace_dependency.name
                    ));
                    continue;
                }
                if !dependency.req.matches(&workspace_dependency.version) {
                    errors.push(format!(
                        "{}: dependency requirement {} for {} does not include {}",
                        package.name,
                        dependency.req,
                        workspace_dependency.name,
                        workspace_dependency.version
                    ));
                    continue;
                }
                let dependency_name = workspace_dependency.name.to_string();
                if dependency.kind == DependencyKind::Development {
                    dev_dependencies.insert(dependency_name);
                } else {
                    dependencies.insert(dependency_name);
                }
            }
            publish_packages.push(PublishPackage {
                name: package.name.to_string(),
                version: package.version.clone(),
                dependencies,
                dev_dependencies,
            });
        }
        publish_packages.sort_by(|left, right| left.name.cmp(&right.name));

        if !errors.is_empty() {
            bail!(errors.join("\n"));
        }
        if !allow_dirty {
            ensure_no_ignored_package_inputs(&root, &include_policies)?;
        }

        Ok(Self {
            root,
            target_directory,
            version,
            packages: publish_packages,
            fingerprint,
            include_policies,
        })
    }

    pub(crate) fn ensure_unchanged(&self) -> Result<()> {
        self.fingerprint.ensure_unchanged(&self.root)?;
        ensure_no_ignored_package_inputs(&self.root, &self.include_policies)
    }

    pub(crate) fn ensure_verified_revision(&self, revision: &str) -> Result<()> {
        let verified_revision = format!("{revision}^{{tree}}");
        let verified_tree = git_output(&self.root, &["rev-parse", &verified_revision])?;
        let current_tree = git_output(&self.root, &["rev-parse", "HEAD^{tree}"])?;
        if verified_tree != current_tree {
            bail!(
                "verified revision {revision} has tree {verified_tree}, but publication uses {current_tree}"
            );
        }
        Ok(())
    }

    pub(crate) fn current_tree(&self) -> Result<String> {
        git_output(&self.root, &["rev-parse", "HEAD^{tree}"])
    }

    #[cfg(test)]
    pub(crate) fn new(packages: Vec<PublishPackage>) -> Self {
        Self {
            root: PathBuf::new(),
            target_directory: PathBuf::new(),
            version: Version::new(1, 0, 0),
            packages,
            fingerprint: GitFingerprint {
                head: String::new(),
            },
            include_policies: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
struct GitFingerprint {
    head: String,
}

impl GitFingerprint {
    fn from_repository(root: &Path, allow_dirty: bool) -> Result<Self> {
        if !allow_dirty {
            ensure_clean(root)?;
        }
        Ok(Self {
            head: git_output(root, &["rev-parse", "HEAD"])?,
        })
    }

    fn ensure_unchanged(&self, root: &Path) -> Result<()> {
        ensure_clean(root)?;
        let head = git_output(root, &["rev-parse", "HEAD"])?;
        if head != self.head {
            bail!("repository HEAD changed during crate publication");
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct PackageManifest {
    publish: Option<bool>,
    version_workspace: bool,
    include: Option<Vec<String>>,
}

impl PackageManifest {
    fn from_package(package: &Package) -> Result<Self> {
        let manifest = read_toml(package.manifest_path.as_std_path())?;
        let package_table = manifest
            .get("package")
            .and_then(toml::Value::as_table)
            .with_context(|| format!("{}: missing [package]", package.manifest_path))?;
        let publish = match package_table.get("publish") {
            Some(toml::Value::Boolean(value)) => Some(*value),
            Some(_) => bail!(
                "{}: publish must be an explicit boolean",
                package.manifest_path
            ),
            None => None,
        };
        let version_workspace = package_table
            .get("version")
            .and_then(toml::Value::as_table)
            .and_then(|version| version.get("workspace"))
            .and_then(toml::Value::as_bool)
            .unwrap_or(false);
        let include = package_table
            .get("include")
            .map(|include| {
                include
                    .as_array()
                    .context("package.include must be an array")?
                    .iter()
                    .map(|pattern| {
                        pattern
                            .as_str()
                            .context("package.include patterns must be strings")
                            .map(str::to_string)
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .transpose()?;
        Ok(Self {
            publish,
            version_workspace,
            include,
        })
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            component => normalized.push(component.as_os_str()),
        }
    }
    normalized
}

fn read_toml(path: &Path) -> Result<TomlTable> {
    let source =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    toml::from_str(&source).with_context(|| format!("failed to parse {}", path.display()))
}

fn workspace_version(manifest: &TomlTable) -> Result<Version> {
    manifest
        .get("workspace")
        .and_then(toml::Value::as_table)
        .and_then(|workspace| workspace.get("package"))
        .and_then(toml::Value::as_table)
        .and_then(|package| package.get("version"))
        .and_then(toml::Value::as_str)
        .context("missing [workspace.package].version")?
        .parse()
        .context("invalid [workspace.package].version")
}

fn workspace_dependencies(manifest: &TomlTable) -> Result<&TomlTable> {
    manifest
        .get("workspace")
        .and_then(toml::Value::as_table)
        .and_then(|workspace| workspace.get("dependencies"))
        .and_then(toml::Value::as_table)
        .context("missing [workspace.dependencies]")
}

fn relative_manifest(root: &Path, package: &Package) -> String {
    package
        .manifest_path
        .strip_prefix(root)
        .unwrap_or(&package.manifest_path)
        .to_string()
}

fn ensure_no_ignored_package_inputs(root: &Path, policies: &[IncludePolicy]) -> Result<()> {
    if policies.is_empty() {
        return Ok(());
    }
    let mut arguments = vec![
        "ls-files",
        "-z",
        "--others",
        "--ignored",
        "--exclude-standard",
        "--",
    ];
    for policy in policies {
        arguments.push(policy.directory.to_str().with_context(|| {
            format!("package path is not UTF-8: {}", policy.directory.display())
        })?);
        for path in &policy.always_included {
            arguments.push(
                path.to_str()
                    .with_context(|| format!("package input is not UTF-8: {}", path.display()))?,
            );
        }
    }
    let ignored = git_path_output(root, &arguments)?;
    let mut selected_ignored = BTreeSet::new();
    for policy in policies {
        if !ignored.iter().any(|path| {
            path.starts_with(&policy.directory) || policy.always_included.contains(path)
        }) {
            continue;
        }
        let matcher = if let Some(patterns) = &policy.patterns {
            let package_root = root.join(&policy.directory);
            let mut builder = GitignoreBuilder::new(&package_root);
            for pattern in patterns {
                builder
                    .add_line(None, pattern)
                    .with_context(|| format!("invalid package.include pattern {pattern:?}"))?;
            }
            Some(
                builder
                    .build()
                    .context("invalid package.include patterns")?,
            )
        } else {
            None
        };
        for path in ignored.iter().filter(|path| {
            path.starts_with(&policy.directory) || policy.always_included.contains(*path)
        }) {
            if policy.always_included.contains(path)
                || path.starts_with(&policy.directory)
                    && matcher.as_ref().is_some_and(|matcher| {
                        matcher
                            .matched_path_or_any_parents(root.join(path), false)
                            .is_ignore()
                    })
            {
                selected_ignored.insert(path.clone());
            }
        }
    }
    if !selected_ignored.is_empty() {
        bail!(
            "ignored package inputs are not covered by the verified Git tree:\n{}",
            selected_ignored
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
    Ok(())
}

fn ensure_clean(root: &Path) -> Result<()> {
    let status = git_output(root, &["status", "--porcelain", "--untracked-files=normal"])?;
    if !status.is_empty() {
        bail!("crate publication requires a clean worktree:\n{status}");
    }
    Ok(())
}

fn git_path_output(root: &Path, args: &[&str]) -> Result<Vec<PathBuf>> {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .with_context(|| format!("failed to run git {}", args.join(" ")))?;
    if !output.status.success() {
        bail!(
            "git {} failed with status {}",
            args.join(" "),
            output.status
        );
    }
    let output = String::from_utf8(output.stdout).context("git returned non-UTF-8 paths")?;
    Ok(output.split_terminator('\0').map(PathBuf::from).collect())
}

fn git_output(root: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .with_context(|| format!("failed to run git {}", args.join(" ")))?;
    if !output.status.success() {
        bail!(
            "git {} failed with status {}",
            args.join(" "),
            output.status
        );
    }
    String::from_utf8(output.stdout)
        .context("git returned non-UTF-8 output")
        .map(|output| output.trim().to_string())
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        process::Command,
        sync::atomic::{AtomicU64, Ordering},
    };

    use super::*;

    static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

    struct Fixture {
        root: PathBuf,
    }

    impl Fixture {
        fn new(root_manifest: &str, packages: &[(&str, String)]) -> Self {
            let root = std::env::temp_dir().join(format!(
                "biome-crate-publisher-{}-{}",
                std::process::id(),
                NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed)
            ));
            let _ = fs::remove_dir_all(&root);
            fs::create_dir_all(&root).unwrap();
            write_file(&root, "Cargo.toml", root_manifest);
            for (name, manifest) in packages {
                write_file(&root, &format!("crates/{name}/Cargo.toml"), manifest);
                write_file(&root, &format!("crates/{name}/src/lib.rs"), "");
                write_file(
                    &root,
                    &format!("crates/{name}/README.md"),
                    &format!("{README_PREFIX}\n# {name}\n"),
                );
            }
            run(&root, "cargo", &["generate-lockfile"]);
            run(&root, "git", &["init", "--quiet"]);
            run(&root, "git", &["add", "."]);
            run(
                &root,
                "git",
                &[
                    "-c",
                    "user.name=Biome",
                    "-c",
                    "user.email=biome@example.com",
                    "commit",
                    "--quiet",
                    "-m",
                    "fixture",
                ],
            );
            Self { root }
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.root).unwrap();
        }
    }

    fn root_manifest(dependencies: &[(&str, &str)]) -> String {
        let dependencies = dependencies
            .iter()
            .map(|(name, version)| {
                format!("{name} = {{ path = \"crates/{name}\", version = \"{version}\" }}")
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"2\"\n\n[workspace.package]\nversion = \"1.0.0\"\nedition = \"2024\"\n\n[workspace.dependencies]\n{dependencies}\n"
        )
    }

    fn package_manifest(name: &str, version: &str, publish: Option<bool>, extra: &str) -> String {
        let publish = publish.map_or_default(|publish| format!("publish = {publish}\n"));
        format!(
            "[package]\nname = \"{name}\"\nversion{version}\nedition.workspace = true\ndescription = \"{name}\"\n{publish}\n{extra}"
        )
    }

    fn write_file(root: &Path, relative: &str, contents: &str) {
        let path = root.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, contents).unwrap();
    }

    fn run(root: &Path, program: &str, args: &[&str]) {
        let status = Command::new(program)
            .current_dir(root)
            .args(args)
            .status()
            .unwrap();
        assert!(status.success(), "{program} {} failed", args.join(" "));
    }

    #[test]
    fn verifies_a_staged_bundle_without_changing_its_source_files() {
        let source = Fixture::new(
            &root_manifest(&[("parser", "1.0.0"), ("syntax", "1.0.0")]),
            &[
                (
                    "parser",
                    package_manifest("parser", ".workspace = true\n", Some(true), ""),
                ),
                (
                    "syntax",
                    package_manifest("syntax", ".workspace = true\n", Some(true), ""),
                ),
            ],
        );
        write_file(
            &source.root,
            "xtask/publish_crates/bundle-plan.toml",
            "roots = ['parser']\nprivate = []\n[bundles]\nbundle = ['parser', 'syntax']\n",
        );
        fs::remove_file(source.root.join("crates/syntax/README.md")).unwrap();
        let staged = Fixture::new(
            &root_manifest(&[]),
            &[(
                "bundle",
                package_manifest(
                    "bundle",
                    ".workspace = true\n",
                    Some(true),
                    "include = ['src/**']\n",
                ),
            )],
        );
        let staged_crate = staged.root.join("crates/bundle");
        let readme_source = source.root.join("crates/parser/README.md");
        let original_readme = fs::read(&readme_source).unwrap();
        let staged_manifest = staged_crate.join("Cargo.toml");
        let manifest_source = source.root.join("bundle-template.toml");
        let original_manifest = fs::read(&staged_manifest).unwrap();
        fs::rename(&staged_manifest, &manifest_source).unwrap();
        fs::hard_link(&manifest_source, &staged_manifest).unwrap();
        fs::remove_file(staged_crate.join("README.md")).unwrap();
        fs::hard_link(&readme_source, staged_crate.join("README.md")).unwrap();

        crate::bundle_plan::verify_staged(&source.root, &staged_crate).unwrap();

        assert_eq!(fs::read(readme_source).unwrap(), original_readme);
        assert_eq!(fs::read(manifest_source).unwrap(), original_manifest);

        let readme = fs::read_to_string(staged_crate.join("README.md")).unwrap();
        assert!(readme.starts_with(README_PREFIX));
        assert!(readme.contains("# `bundle`\n"));
        assert!(readme.contains("- `parser`\n- `syntax`\n"));
        let manifest = fs::read_to_string(staged_crate.join("Cargo.toml")).unwrap();
        assert!(manifest.contains("readme = \"README.md\""));
        assert!(
            staged_crate
                .join("target/package/bundle-1.0.0.crate")
                .is_file()
        );
    }

    #[test]
    fn rejects_a_package_file_list_without_the_root_readme() {
        let staged = Fixture::new(
            &root_manifest(&[]),
            &[(
                "bundle",
                package_manifest(
                    "bundle",
                    ".workspace = true\n",
                    Some(true),
                    "readme = false\ninclude = ['src/**', 'docs/**']\n",
                ),
            )],
        );
        let staged_crate = staged.root.join("crates/bundle");
        write_file(&staged_crate, "docs/README.md", README_PREFIX);
        let error = crate::cargo::SystemCargo::new()
            .verify_staged_package(&staged_crate, "bundle")
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "cargo would not include README.md in bundle"
        );
        assert!(
            !staged_crate
                .join("target/package/bundle-1.0.0.crate")
                .exists()
        );
    }

    #[test]
    fn bundle_plan_check_rejects_dependencies_marked_for_exclusion() {
        let fixture = Fixture::new(
            &root_manifest(&[("parser", "1.0.0"), ("internal", "1.0.0")]),
            &[
                (
                    "parser",
                    package_manifest(
                        "parser",
                        ".workspace = true\n",
                        Some(true),
                        "\n[dependencies]\ninternal.workspace = true\n",
                    ),
                ),
                (
                    "internal",
                    package_manifest("internal", ".workspace = true\n", Some(true), ""),
                ),
            ],
        );
        let config = "roots = ['parser']\n[bundles]\n";
        let path = "xtask/publish_crates/bundle-plan.toml";

        write_file(&fixture.root, path, &format!("private = []\n{config}"));
        crate::bundle_plan::print(&fixture.root, true).unwrap();

        write_file(
            &fixture.root,
            path,
            &format!("private = ['internal']\n{config}"),
        );
        crate::bundle_plan::print(&fixture.root, false).unwrap();
        let error = crate::bundle_plan::print(&fixture.root, true).unwrap_err();
        assert_eq!(
            error.to_string(),
            "bundle plan still needs crates marked for exclusion"
        );
    }

    #[test]
    fn selects_only_explicit_public_crates() {
        let fixture = Fixture::new(
            &root_manifest(&[("public", "1.0.0")]),
            &[
                (
                    "public",
                    package_manifest("public", ".workspace = true\n", Some(true), ""),
                ),
                (
                    "private",
                    package_manifest("private", " = \"0.0.0\"\n", Some(false), ""),
                ),
            ],
        );

        let workspace = Workspace::from_directory(&fixture.root, false).unwrap();

        assert_eq!(
            workspace
                .packages
                .iter()
                .map(|package| package.name.as_str())
                .collect::<Vec<_>>(),
            ["public"]
        );
    }

    #[test]
    fn verified_revision_requires_the_current_git_tree() {
        let fixture = Fixture::new(
            &root_manifest(&[("public", "1.0.0")]),
            &[(
                "public",
                package_manifest("public", ".workspace = true\n", Some(true), ""),
            )],
        );
        let verified_revision = git_output(&fixture.root, &["rev-parse", "HEAD"]).unwrap();
        run(
            &fixture.root,
            "git",
            &[
                "-c",
                "user.name=Biome",
                "-c",
                "user.email=biome@example.com",
                "commit",
                "--quiet",
                "--allow-empty",
                "-m",
                "same tree",
            ],
        );
        let workspace = Workspace::from_directory(&fixture.root, false).unwrap();

        workspace
            .ensure_verified_revision(&verified_revision)
            .unwrap();

        write_file(
            &fixture.root,
            "crates/public/README.md",
            &format!("{README_PREFIX}\n# public\nchanged\n"),
        );
        run(&fixture.root, "git", &["add", "crates/public/README.md"]);
        run(
            &fixture.root,
            "git",
            &[
                "-c",
                "user.name=Biome",
                "-c",
                "user.email=biome@example.com",
                "commit",
                "--quiet",
                "-m",
                "different tree",
            ],
        );
        let workspace = Workspace::from_directory(&fixture.root, false).unwrap();

        let error = workspace
            .ensure_verified_revision(&verified_revision)
            .unwrap_err()
            .to_string();

        assert!(error.contains("publication uses"));
        assert!(
            workspace
                .ensure_verified_revision("unknown-revision")
                .is_err()
        );
    }

    #[test]
    fn publishes_a_crate_without_a_shared_dependency_entry() {
        let fixture = Fixture::new(
            &root_manifest(&[]),
            &[(
                "public",
                package_manifest("public", ".workspace = true\n", Some(true), ""),
            )],
        );

        let workspace = Workspace::from_directory(&fixture.root, false).unwrap();
        assert_eq!(workspace.packages, [PublishPackage::new("public", &[])]);
    }

    #[test]
    fn rejects_an_unversioned_shared_dependency_for_a_public_crate() {
        let mut root = root_manifest(&[]);
        root.push_str("public = { path = 'crates/public' }\n");
        let fixture = Fixture::new(
            &root,
            &[(
                "public",
                package_manifest("public", ".workspace = true\n", Some(true), ""),
            )],
        );

        let error = Workspace::from_directory(&fixture.root, false).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Cargo.toml: missing versioned workspace dependency for public"
        );
    }

    #[test]
    fn rejects_a_missing_publish_setting() {
        let fixture = Fixture::new(
            &root_manifest(&[("missing", "1.0.0")]),
            &[(
                "missing",
                package_manifest("missing", ".workspace = true\n", None, ""),
            )],
        );

        let error = Workspace::from_directory(&fixture.root, false)
            .unwrap_err()
            .to_string();

        let manifest = Path::new("crates").join("missing").join("Cargo.toml");
        assert!(
            error.contains(&format!(
                "{}: missing explicit publish field",
                manifest.display()
            )),
            "{error}"
        );
    }

    #[test]
    fn rejects_a_public_crate_outside_the_shared_version() {
        let fixture = Fixture::new(
            &root_manifest(&[("public", "0.5.0")]),
            &[(
                "public",
                package_manifest("public", " = \"0.5.0\"\n", Some(true), ""),
            )],
        );

        let error = Workspace::from_directory(&fixture.root, false)
            .unwrap_err()
            .to_string();

        assert!(error.contains("public crate must inherit the workspace version"));
        assert!(error.contains("expected version 1.0.0, found 0.5.0"));
    }

    #[test]
    fn rejects_ignored_files_that_package_include_can_select() {
        let fixture = Fixture::new(
            &root_manifest(&[("public", "1.0.0")]),
            &[(
                "public",
                package_manifest(
                    "public",
                    ".workspace = true\n",
                    Some(true),
                    "include = [\"src/**/*\", \"README.md\"]\n",
                ),
            )],
        );
        write_file(
            &fixture.root,
            ".gitignore",
            "crates/public/src/ignored.rs\n",
        );
        run(&fixture.root, "git", &["add", ".gitignore"]);
        run(
            &fixture.root,
            "git",
            &[
                "-c",
                "user.name=Biome",
                "-c",
                "user.email=biome@example.com",
                "commit",
                "--quiet",
                "-m",
                "ignore package input",
            ],
        );
        write_file(&fixture.root, "crates/public/src/ignored.rs", "");

        let error = Workspace::from_directory(&fixture.root, false)
            .unwrap_err()
            .to_string();

        assert!(error.contains("ignored package inputs"));
        assert!(error.contains("crates/public/src/ignored.rs"));
    }

    #[test]
    fn allows_ignored_files_excluded_by_package_include() {
        let fixture = Fixture::new(
            &root_manifest(&[("public", "1.0.0")]),
            &[(
                "public",
                package_manifest(
                    "public",
                    ".workspace = true\n",
                    Some(true),
                    "include = [\"src/**/*\", \"README.md\"]\n",
                ),
            )],
        );
        write_file(
            &fixture.root,
            ".gitignore",
            "crates/public/tests/ignored.rs\n",
        );
        run(&fixture.root, "git", &["add", ".gitignore"]);
        run(
            &fixture.root,
            "git",
            &[
                "-c",
                "user.name=Biome",
                "-c",
                "user.email=biome@example.com",
                "commit",
                "--quiet",
                "-m",
                "ignore excluded input",
            ],
        );
        write_file(&fixture.root, "crates/public/tests/ignored.rs", "");

        let workspace = Workspace::from_directory(&fixture.root, false).unwrap();

        assert_eq!(workspace.packages.len(), 1);
    }

    #[test]
    fn rejects_ignored_always_included_files_without_package_include() {
        let fixture = Fixture::new(
            &root_manifest(&[("public", "1.0.0")]),
            &[(
                "public",
                package_manifest("public", ".workspace = true\n", Some(true), ""),
            )],
        );
        write_file(&fixture.root, ".gitignore", "crates/public/README.md\n");
        run(
            &fixture.root,
            "git",
            &["rm", "--cached", "--quiet", "crates/public/README.md"],
        );
        run(&fixture.root, "git", &["add", ".gitignore"]);
        run(
            &fixture.root,
            "git",
            &[
                "-c",
                "user.name=Biome",
                "-c",
                "user.email=biome@example.com",
                "commit",
                "--quiet",
                "-m",
                "ignore readme",
            ],
        );

        let error = Workspace::from_directory(&fixture.root, false)
            .unwrap_err()
            .to_string();

        assert!(error.contains("ignored package inputs"));
        assert!(error.contains("crates/public/README.md"));
    }

    #[test]
    fn collects_publication_edges_by_dependency_kind() {
        let names = [
            "app",
            "build_dep",
            "dev_dep",
            "normal",
            "optional",
            "target",
        ];
        let root = root_manifest(
            &names
                .iter()
                .map(|name| (*name, "1.0.0"))
                .collect::<Vec<_>>(),
        );
        let app_dependencies = "[dependencies]\nnormal.workspace = true\noptional = { workspace = true, optional = true }\n\n[build-dependencies]\nbuild_dep.workspace = true\n\n[dev-dependencies]\ndev_dep.workspace = true\n\n[target.'cfg(unix)'.dependencies]\ntarget.workspace = true\n";
        let packages = names
            .iter()
            .map(|name| {
                (
                    *name,
                    package_manifest(
                        name,
                        ".workspace = true\n",
                        Some(true),
                        if *name == "app" { app_dependencies } else { "" },
                    ),
                )
            })
            .collect::<Vec<_>>();
        let fixture = Fixture::new(&root, &packages);

        let workspace = Workspace::from_directory(&fixture.root, false).unwrap();
        let app = workspace
            .packages
            .iter()
            .find(|package| package.name == "app")
            .unwrap();

        assert_eq!(
            app.dependencies,
            BTreeSet::from([
                "build_dep".to_string(),
                "normal".to_string(),
                "optional".to_string(),
                "target".to_string(),
            ])
        );
        assert_eq!(
            app.dev_dependencies,
            BTreeSet::from(["dev_dep".to_string()])
        );
    }
}
