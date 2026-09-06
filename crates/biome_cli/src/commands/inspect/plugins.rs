use super::{AdviceLine, InspectionAdvice, InspectionDiagnostic, display_path};
use crate::{
    CliDiagnostic, CliSession, cli_options::CliOptions,
    commands::validate_configuration_diagnostics,
};
use biome_configuration::{BiomeDiagnostic, OverrideGlobs};
use biome_console::{ConsoleExt, MarkupBuf, fmt::Formatter, markup};
use biome_diagnostics::{Diagnostic, MessageAndDescription, PrintDiagnostic};
use biome_fs::normalize_path;
use biome_glob::NormalizedGlob;
use biome_plugin_loader::{ResolvedPluginKind, resolve_plugin};
use biome_service::{WorkspaceError, configuration::load_configuration, settings::Settings};
use camino::Utf8PathBuf;
use serde::Serialize;
use serde_json::json;
use std::{borrow::Cow, collections::BTreeMap, sync::Arc};

/// Include globs attached to one occurrence of a plugin import.
#[derive(Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct ImportSelection<'configuration> {
    #[serde(skip)]
    selected: bool,
    includes: Option<&'configuration [NormalizedGlob]>,
    override_includes: Option<&'configuration OverrideGlobs>,
}

impl ImportSelection<'_> {
    fn is_unfiltered(&self) -> bool {
        self.includes.is_none() && self.override_includes.is_none()
    }

    fn describe(&self) -> String {
        let mut conditions = Vec::new();
        if let Some(globs) = self.includes {
            let globs = globs
                .iter()
                .map(|glob| glob.as_ref().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            conditions.push(format!("includes: [{globs}]"));
        }
        if let Some(globs) = self.override_includes {
            let globs = match globs {
                OverrideGlobs::Globs(globs) => globs
                    .iter()
                    .map(|glob| glob.as_ref().to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                OverrideGlobs::EditorconfigGlob(glob) => glob.to_string(),
            };
            conditions.push(format!("override includes: [{globs}]"));
        }
        conditions.join(" and ")
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

    fn import_names(&self, local: bool) -> String {
        self.imports
            .iter()
            .filter_map(|(name, selections)| {
                if selections.iter().any(ImportSelection::is_unfiltered) {
                    return (!local).then(|| name.clone());
                }
                let globs = selections
                    .iter()
                    .map(|selection| {
                        let globs = selection.describe();
                        if selections.len() > 1 {
                            format!("({globs})")
                        } else {
                            globs
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" or ");
                Some(if local {
                    globs
                } else {
                    format!("{name} [{globs}]")
                })
            })
            .collect::<Vec<_>>()
            .join(", ")
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

pub(super) fn inspect_plugins(
    session: CliSession,
    cli_options: &CliOptions,
    path: Option<&str>,
    json: bool,
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
    let mut plugins: BTreeMap<(Option<String>, Utf8PathBuf), PluginInventory<'_>> = BTreeMap::new();
    let mut errors = Vec::new();
    for (override_index, override_includes, declarations) in declarations {
        for configuration in declarations.into_iter().flat_map(|plugins| plugins.iter()) {
            let resolved = match resolve_plugin(
                fs,
                configuration.path(),
                base_path,
                configuration.resolved_package_specifier(),
            ) {
                Ok(resolved) => resolved,
                Err(error) => {
                    errors.push((
                        configuration
                            .resolved_package_specifier()
                            .unwrap_or(configuration.path())
                            .to_string(),
                        error,
                    ));
                    continue;
                }
            };
            let selected = if let Some(path) = &matched_path {
                if !configuration.matches_includes(path)
                    || override_index.is_some_and(|index| {
                        !matching_overrides
                            .as_ref()
                            .is_some_and(|matching| matching.contains(&index))
                    })
                {
                    continue;
                }
                true
            } else {
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
                .unwrap_or("direct import");
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

    let resolved_count = plugins
        .values()
        .map(|plugin| plugin.rules.len())
        .sum::<usize>();
    let rules = if resolved_count == 1 { "rule" } else { "rules" };
    if json {
        let json_errors = errors
            .iter()
            .map(|(reference, error)| {
                let causes = std::iter::successors(error.source(), |cause| cause.source())
                    .map(|cause| {
                        let mut message = MarkupBuf::default();
                        cause.message(&mut Formatter::new(&mut message))?;
                        Ok(MessageAndDescription::from(message).to_string())
                    })
                    .collect::<std::io::Result<Vec<_>>>()?;
                Ok(json!({
                    "reference": reference,
                    "message": error.to_string(),
                    "causes": causes,
                }))
            })
            .collect::<std::io::Result<Vec<_>>>()?;
        let output = json!({
            "path": matched_path.as_deref().map(display_path),
            "plugins": plugins.iter().map(|((name, path), plugin)| json!({
                "name": name.as_deref().map_or_else(|| display_path(path), Cow::Borrowed),
                "path": display_path(path),
                "imports": plugin.unknown_rules.as_ref().map(|rule| &rule.imports),
                "enabled": plugin.rules.values().any(RuleInventory::is_enabled)
                    || plugin.unknown_rules.as_ref().is_some_and(RuleInventory::is_enabled),
                "rules": plugin.rules.iter().map(|(name, rule)| json!({
                    "name": name,
                    "path": display_path(&rule.path),
                    "enabled": rule.is_enabled(),
                    "imports": rule.imports,
                })).collect::<Vec<_>>(),
                "ruleInventoryAvailable": plugin.unknown_rules.is_none(),
            })).collect::<Vec<_>>(),
            "errors": json_errors,
        });
        let output = serde_json::to_string_pretty(&output)
            .map_err(|_| WorkspaceError::from(BiomeDiagnostic::new_serialization_error()))?;
        session.app.console.log(markup! { {output} });
        if !errors.is_empty() {
            return Err(CliDiagnostic::inspection_error(
                markup! { {resolved_count}" "{rules}" resolved. Some plugin imports could not be resolved." }.to_owned(),
                InspectionAdvice(Vec::new()),
            ));
        }
        return Ok(());
    }

    let mut advice = Vec::new();
    if let Some(path) = &matched_path {
        advice.push(AdviceLine::Plain(
            markup! { "Target path: "{display_path(path)} }.to_owned(),
        ));
    }
    let mut filtered = Vec::new();
    let mut unknown = Vec::new();
    let mut local_rules = Vec::new();
    let mut local_name_counts = BTreeMap::new();
    for ((package, _), plugin) in &plugins {
        if package.is_none() {
            for (name, rule) in &plugin.rules {
                if rule.is_enabled() {
                    *local_name_counts.entry(name).or_insert(0usize) += 1;
                }
            }
        }
    }
    for ((package, path), plugin) in &plugins {
        let label = package
            .clone()
            .unwrap_or_else(|| display_path(path).into_owned());
        let mut enabled = Vec::new();
        let ambiguous_name =
            package.is_some() && plugins.keys().filter(|(other, _)| other == package).count() > 1;
        let label = if ambiguous_name {
            format!("{label} ({})", display_path(path))
        } else {
            label
        };
        for (name, rule) in &plugin.rules {
            let imports = rule.import_names(package.is_none());
            if rule.is_enabled() && package.is_none() {
                let details = if local_name_counts.get(name).is_some_and(|count| *count > 1) {
                    if imports.is_empty() {
                        display_path(&rule.path).into_owned()
                    } else {
                        format!("{}; {imports}", display_path(&rule.path))
                    }
                } else {
                    imports
                };
                if details.is_empty() {
                    enabled.push(markup! { {name} }.to_owned());
                } else {
                    enabled.push(markup! { {name}" ("{details}")" }.to_owned());
                }
                continue;
            }
            if rule.is_enabled() {
                enabled.push(markup! { {name}" ("{imports}")" }.to_owned());
            } else if package.is_some() {
                filtered.push(markup! { {label}"/"{name}" ("{imports}")" }.to_owned());
            } else if rule.path == *path && path.file_stem() == Some(name.as_str()) {
                filtered.push(markup! { {label}" ("{imports}")" }.to_owned());
            } else {
                filtered.push(markup! { {label}": "{name}" ("{imports}")" }.to_owned());
            }
        }
        if !enabled.is_empty() {
            if package.is_none() {
                local_rules.extend(enabled);
            } else {
                advice.push(AdviceLine::Plain(
                    markup! { <Emphasis>{label}</Emphasis> }.to_owned(),
                ));
                advice.push(AdviceLine::List(enabled));
            }
        }
        if let Some(rule) = &plugin.unknown_rules {
            let globs = rule.import_names(true);
            if globs.is_empty() {
                unknown.push(markup! { {label} }.to_owned());
            } else {
                unknown.push(markup! { {label}" ("{globs}")" }.to_owned());
            }
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
            markup! { <Emphasis>"Rules with include globs"</Emphasis> }.to_owned(),
        ));
        advice.push(AdviceLine::List(filtered));
    }
    if !unknown.is_empty() {
        advice.push(AdviceLine::Plain(
            markup! { <Emphasis>"JavaScript plugins"</Emphasis> }.to_owned(),
        ));
        advice.push(AdviceLine::List(unknown));
    }
    let has_errors = !errors.is_empty();
    if has_errors {
        advice.push(AdviceLine::Plain(
            markup! { <Emphasis>"Unresolved imports"</Emphasis> }.to_owned(),
        ));
        for (reference, error) in errors {
            advice.push(AdviceLine::Plain(markup! { {reference} }.to_owned()));
            advice.push(AdviceLine::Error(error.into()));
        }
    }
    let message: MarkupBuf = if has_errors {
        markup! { {resolved_count}" "{rules}" resolved. Some plugin imports could not be resolved." }.to_owned()
    } else if plugins.is_empty() {
        markup! { "No plugin rules resolved." }.to_owned()
    } else if plugins
        .values()
        .any(|plugin| plugin.unknown_rules.is_some())
    {
        markup! { {resolved_count}" "{rules}" resolved. Some JavaScript rule names are unavailable." }.to_owned()
    } else {
        markup! { {resolved_count}" "{rules}" resolved." }.to_owned()
    };
    let advice = InspectionAdvice(advice);
    if has_errors {
        return Err(CliDiagnostic::inspection_error(message, advice));
    }
    let diagnostic = InspectionDiagnostic {
        message,
        path: None,
        span: None,
        source_code: None,
        advice,
    };
    session
        .app
        .console
        .log(markup! { {PrintDiagnostic::simple(&diagnostic)} });
    Ok(())
}
