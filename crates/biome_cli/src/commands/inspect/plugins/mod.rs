use super::{AdviceLine, InspectionAdvice, InspectionDiagnostic, display_path};
use crate::{
    CliDiagnostic, CliSession, cli_options::CliOptions,
    commands::validate_configuration_diagnostics,
};
use biome_configuration::OverrideGlobs;
use biome_console::fmt::{self, Formatter};
use biome_console::{ConsoleExt, markup};
use biome_diagnostics::PrintDiagnostic;
use biome_fs::normalize_path;
use biome_glob::NormalizedGlob;
use biome_plugin_loader::{PluginDiagnostic, ResolvedPluginKind, resolve_plugin};
use biome_service::{configuration::load_configuration, settings::Settings};
use camino::{Utf8Path, Utf8PathBuf};
use std::{collections::BTreeMap, io, sync::Arc};

/// Import label for a plugin entry that names a file, a directory, or a single package rule,
/// as opposed to a package preset such as `presets/recommended`.
const LISTED_DIRECTLY: &str = "listed directly";

/// Include globs attached to one occurrence of a plugin import.
#[derive(Clone, PartialEq)]
struct ImportSelection<'configuration> {
    selected: bool,
    includes: Option<&'configuration [NormalizedGlob]>,
    override_includes: Option<&'configuration OverrideGlobs>,
}

impl ImportSelection<'_> {
    fn is_unfiltered(&self) -> bool {
        self.includes.is_none() && self.override_includes.is_none()
    }
}

/// Renders the override and plugin include globs that must both match, such as
/// `override includes: [tests/**] and includes: [**/*.test.ts]`.
impl fmt::Display for ImportSelection<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        match self.override_includes {
            Some(OverrideGlobs::Globs(globs)) => {
                fmt.write_markup(markup! { "override includes: "{GlobList(globs)} })?;
            }
            Some(OverrideGlobs::EditorconfigGlob(glob)) => {
                fmt.write_fmt(format_args!("override includes: [{glob}]"))?;
            }
            None => {}
        }
        if let Some(globs) = self.includes {
            if self.override_includes.is_some() {
                fmt.write_str(" and ")?;
            }
            fmt.write_markup(markup! { "includes: "{GlobList(globs)} })?;
        }
        Ok(())
    }
}

/// Renders globs as a bracketed list, such as `[src/**/*.ts, !**/*.test.ts]`.
struct GlobList<'a>(&'a [NormalizedGlob]);

impl fmt::Display for GlobList<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        fmt.write_str("[")?;
        for (index, glob) in self.0.iter().enumerate() {
            if index > 0 {
                fmt.write_str(", ")?;
            }
            fmt.write_fmt(format_args!("{}", glob.as_ref()))?;
        }
        fmt.write_str("]")
    }
}

struct RuleInventory<'configuration> {
    path: Utf8PathBuf,
    imports: BTreeMap<String, Vec<ImportSelection<'configuration>>>,
}

impl<'configuration> RuleInventory<'configuration> {
    fn include_import(&mut self, import: &str, selection: &ImportSelection<'configuration>) {
        let selections = self.imports.entry(import.to_string()).or_default();
        if selections.iter().any(ImportSelection::is_unfiltered) {
            return;
        }
        if selection.is_unfiltered() {
            selections.clear();
        }
        if !selections.contains(selection) {
            selections.push(selection.clone());
        }
    }

    fn is_enabled(&self) -> bool {
        self.imports
            .values()
            .flatten()
            .any(|selection| selection.selected)
    }
}

/// Renders the imports that select a rule, such as `listed directly, presets/strict`.
struct ImportList<'a>(&'a RuleInventory<'a>);

impl fmt::Display for ImportList<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        for (index, import) in self.0.imports.keys().enumerate() {
            if index > 0 {
                fmt.write_str(", ")?;
            }
            fmt.write_str(import)?;
        }
        Ok(())
    }
}

/// Renders the include conditions under which a rule applies, one per import occurrence.
///
/// A single condition stays on the list item's line. Multiple conditions become a nested
/// list because any one of them is enough for the rule to apply. With `with_imports`, each
/// condition names the import it comes from, so conditions contributed by different presets
/// of the same package stay distinguishable.
struct Conditions<'a> {
    rule: &'a RuleInventory<'a>,
    with_imports: bool,
}

impl fmt::Display for Conditions<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let with_imports = self.with_imports;
        let count = self.rule.imports.values().map(Vec::len).sum::<usize>();
        let conditions = self.rule.imports.iter().flat_map(|(import, selections)| {
            selections.iter().map(move |selection| Condition {
                selection,
                import: with_imports.then_some(import.as_str()),
            })
        });
        if count == 1 {
            for condition in conditions {
                fmt.write_markup(markup! { ", for files matching "{condition} })?;
            }
            return Ok(());
        }
        fmt.write_str(", for files matching any of:")?;
        for condition in conditions {
            fmt.write_markup(markup! { "\n- "{condition} })?;
        }
        Ok(())
    }
}

/// Renders one include condition, followed by the import that contributes it when known.
struct Condition<'a> {
    selection: &'a ImportSelection<'a>,
    import: Option<&'a str>,
}

impl fmt::Display for Condition<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        fmt.write_markup(markup! { {self.selection} })?;
        if let Some(import) = self.import {
            fmt.write_markup(markup! { " ("{import}")" })?;
        }
        Ok(())
    }
}

#[derive(Default)]
struct PluginInventory<'configuration> {
    rules: BTreeMap<String, RuleInventory<'configuration>>,
    unknown_rules: Option<RuleInventory<'configuration>>,
}

impl<'configuration> PluginInventory<'configuration> {
    fn include_rule(
        &mut self,
        name: String,
        path: Utf8PathBuf,
        import: &str,
        selection: &ImportSelection<'configuration>,
    ) {
        self.rules
            .entry(name)
            .or_insert_with(|| RuleInventory {
                path,
                imports: BTreeMap::new(),
            })
            .include_import(import, selection);
    }
}

/// Renders `path` relative to `base_path` when it is inside it.
#[derive(Clone, Copy)]
struct RelativePath<'a> {
    path: &'a Utf8Path,
    base_path: &'a Utf8Path,
}

impl fmt::Display for RelativePath<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let path = self.path.strip_prefix(self.base_path).unwrap_or(self.path);
        fmt.write_str(&display_path(path))
    }
}

/// Renders a rule from a local file or manifest, such as `before.grit (rule name: before)`.
///
/// Grit files don't declare a rule name, so the file path comes first and the name derived
/// from the file name follows. That name is the one used in suppression comments.
struct LocalRule<'a> {
    path: RelativePath<'a>,
    name: &'a str,
}

impl fmt::Display for LocalRule<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        fmt.write_markup(markup! { {self.path}" (rule name: "{self.name}")" })
    }
}

/// Renders a package name, followed by its manifest path when the configuration resolves the
/// same package from more than one location.
#[derive(Clone, Copy)]
struct PackageLabel<'a> {
    package: &'a str,
    manifest_path: Option<RelativePath<'a>>,
}

impl fmt::Display for PackageLabel<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        fmt.write_str(self.package)?;
        if let Some(manifest_path) = self.manifest_path {
            fmt.write_markup(markup! { " ("{manifest_path}")" })?;
        }
        Ok(())
    }
}

/// Renders a count followed by the singular or plural form of a noun.
struct Quantity {
    count: usize,
    singular: &'static str,
    plural: &'static str,
}

impl fmt::Display for Quantity {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let noun = if self.count == 1 {
            self.singular
        } else {
            self.plural
        };
        fmt.write_markup(markup! { {self.count}" "{noun} })
    }
}

/// Renders how many plugin rules and JavaScript plugins the configuration enables, optionally
/// for the `--path` target.
struct EnabledSummary<'a> {
    rules: usize,
    javascript_plugins: usize,
    target: Option<RelativePath<'a>>,
}

impl fmt::Display for EnabledSummary<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let rules = Quantity {
            count: self.rules,
            singular: "plugin rule",
            plural: "plugin rules",
        };
        let javascript_plugins = Quantity {
            count: self.javascript_plugins,
            singular: "JavaScript plugin",
            plural: "JavaScript plugins",
        };
        match (self.rules, self.javascript_plugins) {
            (0, 0) => fmt.write_str("The configuration doesn't enable any plugin rules")?,
            (_, 0) => fmt.write_markup(markup! { "The configuration enables "{rules} })?,
            (0, _) => {
                fmt.write_markup(markup! { "The configuration enables "{javascript_plugins} })?;
            }
            _ => fmt.write_markup(markup! {
                "The configuration enables "{rules}" and "{javascript_plugins}
            })?,
        }
        if let Some(target) = self.target {
            fmt.write_markup(markup! { " for "<Emphasis>{target}</Emphasis> })?;
        }
        fmt.write_str(".")
    }
}

/// A plugin entry from the configuration that failed to resolve.
struct PluginError<'configuration> {
    /// The plugin reference as it appears in the merged configuration.
    reference: &'configuration str,
    selection: ImportSelection<'configuration>,
    /// Whether the entry applies to the `--path` target. `None` without `--path`.
    applies_to_target: Option<bool>,
    error: PluginDiagnostic,
}

/// Renders a failing plugin reference, followed by its include conditions when it has any.
struct PluginReference<'a> {
    reference: RelativePath<'a>,
    selection: &'a ImportSelection<'a>,
}

impl fmt::Display for PluginReference<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        fmt.write_markup(markup! { {self.reference} })?;
        if !self.selection.is_unfiltered() {
            fmt.write_markup(markup! { " ("{self.selection}")" })?;
        }
        Ok(())
    }
}

/// Renders the number of plugin entries that failed to resolve.
struct ErrorSummary(usize);

impl fmt::Display for ErrorSummary {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        if self.0 == 1 {
            fmt.write_str("1 plugin in the configuration has an error.")
        } else {
            fmt.write_markup(markup! { {self.0}" plugins in the configuration have errors." })
        }
    }
}

pub(crate) fn inspect_plugins(
    session: CliSession,
    cli_options: &CliOptions,
    path: Option<&str>,
) -> Result<(), CliDiagnostic> {
    let fs = session.app.workspace.fs();
    let working_directory = fs.working_directory().unwrap_or_default();
    let loaded = load_configuration(
        fs,
        cli_options.as_configuration_path_hint(&working_directory),
    )?;
    validate_configuration_diagnostics(&loaded, session.app.console, cli_options.verbose)?;
    let source = Arc::new(loaded.source);
    let base_path = source
        .directory_path
        .as_deref()
        .unwrap_or(&working_directory);
    let configuration = source.resolve();
    let mut settings = Settings::default();
    settings.merge_with_configuration_source(source.clone())?;
    let matched_path = path.map(|path| normalize_path(&working_directory.join(path)));
    let target = matched_path
        .as_deref()
        .map(|path| RelativePath { path, base_path });
    let matching_overrides = matched_path.as_deref().map(|path| {
        settings
            .override_settings
            .matching_indices(path)
            .collect::<Vec<_>>()
    });
    let overrides = configuration
        .overrides
        .as_ref()
        .map(|overrides| overrides.0.as_slice())
        .unwrap_or_default();
    let declarations = std::iter::once((None, None, configuration.plugins.as_ref())).chain(
        overrides.iter().enumerate().map(|(index, pattern)| {
            (
                Some(index),
                pattern.includes.as_ref(),
                pattern.plugins.as_ref(),
            )
        }),
    );
    let mut plugins: BTreeMap<_, PluginInventory> = BTreeMap::new();
    let mut errors = Vec::new();
    for (override_index, override_includes, declarations) in declarations {
        for configuration in declarations.into_iter().flat_map(|plugins| plugins.iter()) {
            let applies_to_target = matched_path.as_deref().map(|path| {
                configuration.matches_includes(base_path, path)
                    && override_index.is_none_or(|index| {
                        matching_overrides
                            .as_ref()
                            .is_some_and(|matching| matching.contains(&index))
                    })
            });
            let resolved = match resolve_plugin(
                fs,
                configuration.path(),
                base_path,
                configuration.resolved_package_specifier(),
            ) {
                Ok(resolved) => resolved,
                Err(error) => {
                    errors.push(PluginError {
                        reference: configuration
                            .resolved_package_specifier()
                            .unwrap_or(configuration.path()),
                        selection: ImportSelection {
                            selected: false,
                            includes: configuration.includes(),
                            override_includes,
                        },
                        applies_to_target,
                        error,
                    });
                    continue;
                }
            };
            let selected = match applies_to_target {
                Some(false) => continue,
                Some(true) => true,
                None => {
                    if configuration
                        .includes()
                        .is_some_and(|includes| !includes.iter().any(|glob| !glob.is_negated()))
                    {
                        continue;
                    }
                    if override_index.is_some() {
                        let can_match = match override_includes {
                            Some(OverrideGlobs::Globs(globs)) => {
                                globs.iter().any(|glob| !glob.is_negated())
                            }
                            Some(OverrideGlobs::EditorconfigGlob(_)) => true,
                            None => false,
                        };
                        if !can_match {
                            continue;
                        }
                    }
                    override_index.is_none() && configuration.includes().is_none()
                }
            };
            let selection = ImportSelection {
                selected,
                includes: configuration.includes(),
                override_includes,
            };
            let import = resolved
                .selection
                .as_deref()
                .filter(|selection| selection.starts_with("presets/"))
                .unwrap_or(LISTED_DIRECTLY);
            match resolved.kind {
                ResolvedPluginKind::Grit => {
                    let name = resolved
                        .path
                        .file_stem()
                        .unwrap_or(resolved.path.as_str())
                        .to_string();
                    plugins
                        .entry((None, resolved.path.clone()))
                        .or_default()
                        .include_rule(name, resolved.path, import, &selection);
                }
                ResolvedPluginKind::JavaScript => {
                    let plugin = plugins.entry((None, resolved.path.clone())).or_default();
                    plugin
                        .unknown_rules
                        .get_or_insert_with(|| RuleInventory {
                            path: resolved.path,
                            imports: BTreeMap::new(),
                        })
                        .include_import(import, &selection);
                }
                ResolvedPluginKind::Manifest { rules, .. } => {
                    for rule in rules {
                        let identity = if rule.package.is_some() {
                            rule.exporting_manifest_path
                        } else {
                            rule.path.clone()
                        };
                        plugins
                            .entry((rule.package, identity))
                            .or_default()
                            .include_rule(rule.export_name, rule.path, import, &selection);
                    }
                }
            }
        }
    }

    let summary = EnabledSummary {
        rules: plugins.values().map(|plugin| plugin.rules.len()).sum(),
        javascript_plugins: plugins
            .values()
            .filter(|plugin| plugin.unknown_rules.is_some())
            .count(),
        target,
    };

    let mut advice = Vec::new();
    let mut filtered = Vec::new();
    let mut javascript_plugins = Vec::new();
    let mut local_rules = Vec::new();
    // Rules from local files and manifests share one namespace in suppression comments.
    let mut local_name_counts = BTreeMap::new();
    for ((package, path), plugin) in &plugins {
        let path = RelativePath { path, base_path };
        let Some(package) = package else {
            for (name, rule) in &plugin.rules {
                *local_name_counts.entry(name.as_str()).or_insert(0) += 1;
                let label = LocalRule {
                    path: RelativePath {
                        path: &rule.path,
                        base_path,
                    },
                    name,
                };
                if rule.is_enabled() {
                    local_rules.push(markup! { {label} }.to_owned());
                } else {
                    let conditions = Conditions {
                        rule,
                        with_imports: false,
                    };
                    filtered.push(markup! { {label}{conditions} }.to_owned());
                }
            }
            if let Some(rule) = &plugin.unknown_rules {
                if rule.is_enabled() {
                    javascript_plugins.push(markup! { {path} }.to_owned());
                } else {
                    let conditions = Conditions {
                        rule,
                        with_imports: false,
                    };
                    javascript_plugins.push(markup! { {path}{conditions} }.to_owned());
                }
            }
            continue;
        };
        let is_ambiguous = plugins
            .keys()
            .filter(|(other, _)| other.as_ref() == Some(package))
            .nth(1)
            .is_some();
        let label = PackageLabel {
            package,
            manifest_path: is_ambiguous.then_some(path),
        };
        let mut enabled = Vec::new();
        for (name, rule) in &plugin.rules {
            if rule.is_enabled() {
                enabled.push(markup! { {name}" ("{ImportList(rule)}")" }.to_owned());
            } else {
                let conditions = Conditions {
                    rule,
                    with_imports: true,
                };
                filtered.push(markup! { {label}"/"{name}{conditions} }.to_owned());
            }
        }
        if !enabled.is_empty() {
            advice.push(AdviceLine::Plain(
                markup! { <Emphasis>{label}</Emphasis> }.to_owned(),
            ));
            advice.push(AdviceLine::List(enabled));
        }
    }
    if !local_rules.is_empty() {
        advice.push(AdviceLine::Plain(
            markup! { <Emphasis>"Local plugins"</Emphasis> }.to_owned(),
        ));
        advice.push(AdviceLine::List(local_rules));
    }
    if !filtered.is_empty() {
        advice.push(AdviceLine::Plain(
            markup! { <Emphasis>"Rules that apply only to matching files"</Emphasis> }.to_owned(),
        ));
        advice.push(AdviceLine::List(filtered));
    }
    for (name, count) in local_name_counts {
        if count > 1 {
            advice.push(AdviceLine::Info(
                markup! {
                    {count}" plugin rules are named "<Emphasis>{name}</Emphasis>". A suppression comment for "
                    <Emphasis>"lint/plugin/"{name}</Emphasis>" applies to all of them."
                }
                .to_owned(),
            ));
        }
    }
    if !javascript_plugins.is_empty() {
        advice.push(AdviceLine::Plain(
            markup! { <Emphasis>"JavaScript plugins"</Emphasis> }.to_owned(),
        ));
        advice.push(AdviceLine::List(javascript_plugins));
        advice.push(AdviceLine::Info(
            markup! {
                "Biome doesn't run JavaScript plugins during inspection, so their rule names aren't listed."
            }
            .to_owned(),
        ));
    }

    if errors.is_empty() {
        let diagnostic = InspectionDiagnostic {
            message: markup! { {summary} }.to_owned(),
            path: None,
            span: None,
            source_code: None,
            advice: InspectionAdvice(advice),
        };
        session
            .app
            .console
            .log(markup! { {PrintDiagnostic::simple(&diagnostic)} });
        return Ok(());
    }

    advice.insert(0, AdviceLine::Plain(markup! { {summary} }.to_owned()));
    advice.push(AdviceLine::Plain(
        markup! { <Emphasis>"Plugins with errors"</Emphasis> }.to_owned(),
    ));
    let message = markup! { {ErrorSummary(errors.len())} }.to_owned();
    let reports_other_files = errors
        .iter()
        .any(|error| error.applies_to_target == Some(false));
    for error in errors {
        let reference = PluginReference {
            reference: RelativePath {
                path: Utf8Path::new(error.reference),
                base_path,
            },
            selection: &error.selection,
        };
        advice.push(AdviceLine::Plain(markup! { {reference} }.to_owned()));
        advice.push(AdviceLine::Error(error.error.into()));
    }
    if let Some(target) = target
        && reports_other_files
    {
        advice.push(AdviceLine::Info(
            markup! {
                "Plugins that don't apply to "<Emphasis>{target}</Emphasis>" are also reported, because Biome loads every plugin in the configuration."
            }
            .to_owned(),
        ));
    }
    Err(CliDiagnostic::inspection_error(
        message,
        InspectionAdvice(advice),
    ))
}
