use crate::{
    Activation, ConfigValue, Contribution, Declaration, Facet, FrameworkLoader, InstalledPackage,
    LoaderContext, LoaderHost,
};
use biome_glob::Glob;
use biome_json_value::JsonValue;
use biome_package::PackageJson;
use biome_package::node_semver::{Range, Version};
use biome_rowan::Text;
use camino::{Utf8Path, Utf8PathBuf};
use indexmap::IndexSet;
use rustc_hash::FxHashSet;
use std::collections::{BTreeSet, VecDeque};
use std::str::FromStr;

/// The maximum nesting of loaders, such as Astro, which registers Starlight,
/// which registers a Starlight plugin.
///
/// Integrations that loaders of the deepest level contribute are ignored.
pub const MAX_INTEGRATION_DEPTH: usize = 3;

/// What the frameworks of a package inject into it.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FrameworkContributions {
    /// Files to visit, in the order loaders contributed them.
    pub visits: IndexSet<Utf8PathBuf>,

    pub declarations: Vec<Declaration>,

    /// Facets that are incomplete for the package.
    pub incomplete: BTreeSet<Facet>,
}

/// Runs `loaders` for the package in `package_path`, whose manifest is
/// `manifest`, and collects their contributions.
///
/// A loader with [Activation::Dependency] runs when the manifest depends on
/// its package. Each [Contribution::Integration] then runs the loader of the
/// integration, or visits the stylesheets that the integration package
/// exports when no loader handles it. A package is loaded once, and loaders
/// nest at most [MAX_INTEGRATION_DEPTH] levels deep.
///
/// The package is incomplete for every [Facet] when a framework that a loader
/// handles isn't installed, or when its installed version is outside the
/// versions that the loader supports.
pub fn load_frameworks(
    host: &dyn LoaderHost,
    loaders: &[&dyn FrameworkLoader],
    package_path: &Utf8Path,
    manifest: &PackageJson,
) -> FrameworkContributions {
    let mut driver = Driver {
        host,
        loaders,
        package_path,
        contributions: FrameworkContributions::default(),
        loaded: FxHashSet::default(),
        integrations: VecDeque::new(),
    };

    for loader in loaders {
        let Activation::Dependency { config_files } = loader.activation() else {
            continue;
        };
        if !manifest.contains_dependency(loader.package_name()) {
            continue;
        }
        let config_path = config_files
            .iter()
            .map(|file_name| package_path.join(file_name))
            .find(|path| host.is_file(path));
        let options = match &config_path {
            Some(path) => host
                .js_file(path)
                .map_or(ConfigValue::Unknown, |(root, model)| {
                    ConfigValue::from_default_export(&root, &model)
                }),
            None => ConfigValue::new_empty_object(),
        };
        driver.run(*loader, &options, config_path.as_deref(), 1);
    }

    while let Some(integration) = driver.integrations.pop_front() {
        driver.run_integration(integration);
    }

    driver.contributions
}

/// An integration that a loader contributed and that hasn't run yet.
struct PendingIntegration {
    package: Text,
    options: ConfigValue,

    /// The configuration file of the loader that contributed the
    /// integration, which `options` come from.
    config_path: Option<Utf8PathBuf>,

    depth: usize,
}

struct Driver<'a> {
    host: &'a dyn LoaderHost,
    loaders: &'a [&'a dyn FrameworkLoader],
    package_path: &'a Utf8Path,
    contributions: FrameworkContributions,

    /// The packages that a loader already ran for.
    loaded: FxHashSet<Text>,

    integrations: VecDeque<PendingIntegration>,
}

impl Driver<'_> {
    fn run(
        &mut self,
        loader: &dyn FrameworkLoader,
        options: &ConfigValue,
        config_path: Option<&Utf8Path>,
        depth: usize,
    ) {
        if !self.loaded.insert(Text::new_static(loader.package_name())) {
            return;
        }
        let Some(framework) = self.installed_package(loader.package_name()) else {
            self.contributions.incomplete.extend(Facet::ALL);
            return;
        };
        if !is_supported_version(&framework, loader.supported_versions()) {
            self.contributions.incomplete.extend(Facet::ALL);
            return;
        }

        let ctx = LoaderContext {
            host: self.host,
            package_path: self.package_path,
            framework: &framework,
            config_path,
        };
        for contribution in loader.load(&ctx, options) {
            match contribution {
                Contribution::Visit(path) => {
                    self.contributions.visits.insert(path);
                }
                Contribution::Declare(declaration) => {
                    self.contributions.declarations.push(declaration);
                }
                Contribution::Incomplete(facet) => {
                    self.contributions.incomplete.insert(facet);
                }
                Contribution::Integration { package, options } => {
                    if depth < MAX_INTEGRATION_DEPTH {
                        self.integrations.push_back(PendingIntegration {
                            package,
                            options,
                            config_path: config_path.map(Utf8Path::to_path_buf),
                            depth: depth + 1,
                        });
                    } else if !self.loaded.contains(&package) {
                        // The integration that isn't loaded can inject
                        // anything. A package that already ran was loaded.
                        self.contributions.incomplete.extend(Facet::ALL);
                    }
                }
            }
        }
    }

    fn run_integration(&mut self, integration: PendingIntegration) {
        let loader = self.loaders.iter().find(|loader| {
            loader.activation() == Activation::Integration
                && loader.package_name() == integration.package.text()
        });
        match loader {
            Some(loader) => self.run(
                *loader,
                &integration.options,
                integration.config_path.as_deref(),
                integration.depth,
            ),
            None => self.visit_exported_stylesheets(integration.package),
        }
    }

    /// Visits the stylesheets that `package` exports, assuming that an
    /// integration exports stylesheets so that pages use them.
    ///
    /// Like in Node.js, a target must start with `./` and can't have an
    /// empty, `.`, `..`, or `node_modules` segment, where both `/` and `\`
    /// separate segments. Each `*` in a target matches the same characters,
    /// separators included, except such segments.
    fn visit_exported_stylesheets(&mut self, package: Text) {
        if !self.loaded.insert(package.clone()) {
            return;
        }
        let Some(installed) = self.installed_package(package.text()) else {
            self.contributions.incomplete.extend(Facet::ALL);
            return;
        };
        let Some(exports) = &installed.manifest.exports else {
            return;
        };

        let mut targets = Vec::new();
        collect_stylesheet_targets(exports, &mut targets);
        for target in targets {
            let target = target.replace('\\', "/");
            let Some(target) = target.strip_prefix("./") else {
                continue;
            };
            if has_invalid_segment(target) {
                continue;
            }
            if target.contains('*') {
                let paths = self.matching_files(&installed.path, target);
                self.contributions.visits.extend(paths);
            } else {
                self.contributions
                    .visits
                    .insert(installed.path.join(target));
            }
        }
    }

    /// Returns the files in `directory` whose path relative to `directory`
    /// is `pattern` after replacing each `*` with the same characters,
    /// without invalid segments.
    fn matching_files(&self, directory: &Utf8Path, pattern: &str) -> Vec<Utf8PathBuf> {
        let Ok(glob) = Glob::from_str("**") else {
            return Vec::new();
        };
        self.host
            .glob(directory, &glob)
            .into_iter()
            .filter(|path| {
                path.strip_prefix(directory).is_ok_and(|relative| {
                    let relative = relative
                        .components()
                        .map(|component| component.as_str())
                        .collect::<Vec<_>>()
                        .join("/");
                    pattern_match(pattern, &relative)
                        .is_some_and(|matched| !has_invalid_segment(matched))
                })
            })
            .collect()
    }

    fn installed_package(&self, name: &str) -> Option<InstalledPackage> {
        self.host.installed_package(self.package_path, name)
    }
}

/// Returns whether `path` has a segment that Node.js rejects in the target
/// of an export: an empty, `.`, `..`, or `node_modules` segment.
fn has_invalid_segment(path: &str) -> bool {
    path.split(['/', '\\']).any(|segment| {
        matches!(segment, "" | "." | "..") || segment.eq_ignore_ascii_case("node_modules")
    })
}

/// Returns the characters that replace each `*` of `pattern` to give `path`,
/// or `None` when no replacement does.
///
/// For example, `themes/*/*.css` gives `themes/dark/dark.css` with `dark`.
fn pattern_match<'a>(pattern: &str, path: &'a str) -> Option<&'a str> {
    let parts: Vec<_> = pattern.split('*').collect();
    let (first, rest) = parts.split_first()?;
    let stars = rest.len();
    let fixed_length: usize = parts.iter().map(|part| part.len()).sum();
    let matched_length = path.len().checked_sub(fixed_length)?;
    if stars == 0 || matched_length % stars != 0 {
        return None;
    }
    let matched = path.get(first.len()..first.len() + matched_length / stars)?;
    (pattern.replace('*', matched) == path).then_some(matched)
}

/// Returns whether the installed version of `package` is in `range`.
fn is_supported_version(package: &InstalledPackage, range: &str) -> bool {
    let Some(version) = package.manifest.version.as_deref() else {
        return false;
    };
    match (Version::from_str(version), Range::from_str(range)) {
        (Ok(version), Ok(range)) => range.includes(&version),
        _ => false,
    }
}

/// Collects the targets of `exports` that are stylesheets, whatever their
/// subpath or condition.
fn collect_stylesheet_targets<'a>(exports: &'a JsonValue, targets: &mut Vec<&'a str>) {
    match exports {
        JsonValue::String(target) if target.as_str().ends_with(".css") => {
            targets.push(target.as_str());
        }
        JsonValue::Object(object) => {
            for value in object.values() {
                collect_stylesheet_targets(value, targets);
            }
        }
        JsonValue::Array(array) => {
            for value in array.iter() {
                collect_stylesheet_targets(value, targets);
            }
        }
        _ => {}
    }
}
