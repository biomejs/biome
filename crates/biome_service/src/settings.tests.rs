use crate::scanner::ScanKind;
use crate::settings::{
    LanguageSettings, ModuleGraphResolutionKind, ServiceLanguage, Settings,
    to_json_language_settings,
};
use crate::workspace::DocumentFileSource;
use biome_analyze::RuleFilter;
use biome_configuration::analyzer::{GroupPlainConfiguration, SeverityOrGroup, Style};
use biome_configuration::javascript::JsxRuntime;
use biome_configuration::json::{JsonAssistConfiguration, JsonLinterConfiguration};
use biome_configuration::max_size::MaxSize;
use biome_configuration::{
    Configuration, FormatterConfiguration, JsConfiguration, JsonConfiguration, LinterConfiguration,
    OverrideFilesConfiguration, OverrideGlobs, OverrideLinterConfiguration, OverridePattern,
    Overrides, RuleConfiguration, RulePlainConfiguration, Rules,
};
use biome_fs::BiomePath;
use biome_js_syntax::JsLanguage;
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::FxHashSet;
use std::num::NonZeroU64;
use std::str::FromStr;

#[test]
fn correctly_passes_jsx_runtime() {
    let js_conf = JsConfiguration {
        jsx_runtime: Some(JsxRuntime::ReactClassic),
        ..Default::default()
    };

    let settings = LanguageSettings::<JsLanguage>::from(js_conf);

    assert_eq!(
        settings.environment.jsx_runtime,
        Some(JsxRuntime::ReactClassic)
    );
}

#[test]
fn correctly_lookups_environment_settings() {
    let js_conf = JsConfiguration {
        jsx_runtime: Some(JsxRuntime::ReactClassic),
        ..Default::default()
    };
    let configuration = Configuration {
        javascript: Some(js_conf),
        ..Default::default()
    };
    let mut settings = Settings::default();
    settings
        .merge_with_configuration(configuration, None, vec![])
        .expect("valid configuration");
    let environment = JsLanguage::resolve_environment(&settings);

    assert_eq!(
        environment.unwrap().jsx_runtime,
        Some(JsxRuntime::ReactClassic)
    );
}

#[test]
fn correctly_computes_analyzer_options() {
    let js_conf = JsConfiguration {
        jsx_runtime: Some(JsxRuntime::ReactClassic),
        ..Default::default()
    };
    let configuration = Configuration {
        javascript: Some(js_conf),
        ..Default::default()
    };
    let mut settings = Settings::default();
    settings
        .merge_with_configuration(configuration, None, vec![])
        .expect("valid configuration");
    let environment = JsLanguage::resolve_environment(&settings);
    let language = JsLanguage::lookup_settings(&settings.languages);
    let options = JsLanguage::resolve_analyzer_options(
        &settings,
        &language.linter,
        environment,
        &BiomePath::new(Utf8PathBuf::new()),
        &DocumentFileSource::from_language_id("javascript"),
        None,
    );

    assert_eq!(
        options.jsx_runtime(),
        Some(biome_analyze::options::JsxRuntime::ReactClassic)
    );
}

#[test]
fn merge_override_linter_group_rule() {
    let configuration = Configuration {
        linter: Some(LinterConfiguration {
            rules: Some(Rules {
                nursery: Some(SeverityOrGroup::Plain(GroupPlainConfiguration::On)),
                ..Rules::default()
            }),
            ..LinterConfiguration::default()
        }),
        overrides: Some(Overrides(vec![OverridePattern {
            includes: Some(OverrideGlobs::Globs(Box::new([
                biome_glob::NormalizedGlob::from_str("**/*").unwrap(),
            ]))),
            linter: Some(OverrideLinterConfiguration {
                rules: Some(Rules {
                    style: Some(SeverityOrGroup::Group(Style {
                        no_default_export: Some(RuleConfiguration::Plain(
                            RulePlainConfiguration::Off,
                        )),
                        ..Style::default()
                    })),
                    ..Rules::default()
                }),
                ..OverrideLinterConfiguration::default()
            }),
            ..OverridePattern::default()
        }])),
        ..Default::default()
    };

    let mut settings = Settings::default();

    settings
        .merge_with_configuration(configuration, None, vec![])
        .expect("valid configuration");

    let disabled_rules = settings
        .as_linter_rules(Utf8Path::new("path/to/file.ts"))
        .unwrap()
        .as_disabled_rules();

    assert_eq!(
        disabled_rules,
        FxHashSet::from_iter([RuleFilter::Rule("style", "noDefaultExport")])
    );
}

#[test]
fn merge_override_files_max_size_rule() {
    let configuration = Configuration {
        overrides: Some(Overrides(vec![OverridePattern {
            files: Some(OverrideFilesConfiguration {
                max_size: Some(MaxSize(NonZeroU64::new(1024).unwrap())),
            }),
            ..OverridePattern::default()
        }])),
        ..Default::default()
    };

    let mut settings = Settings::default();

    settings
        .merge_with_configuration(configuration, None, vec![])
        .expect("valid configuration");

    assert_eq!(
        settings.override_settings.patterns[0].files.max_size,
        Some(MaxSize(NonZeroU64::new(1024).unwrap()))
    );
}

#[test]
fn json_to_settings_includes_linter_and_assist() {
    let config = JsonConfiguration {
        linter: Some(JsonLinterConfiguration {
            enabled: Some(true.into()),
        }),
        assist: Some(JsonAssistConfiguration {
            enabled: Some(true.into()),
        }),
        ..Default::default()
    };
    let parent_settings = Settings::default();
    let settings = to_json_language_settings(config, &parent_settings.languages.json);

    assert_eq!(settings.linter.enabled, Some(true.into()));
    assert_eq!(settings.assist.enabled, Some(true.into()));
}

#[test]
fn override_inherits_global_formatter_when_not_specified() {
    // the formatter should inherit from global settings instead of being disabled
    let configuration = Configuration {
        formatter: Some(FormatterConfiguration {
            enabled: Some(true.into()),
            ..FormatterConfiguration::default()
        }),
        linter: Some(LinterConfiguration {
            enabled: Some(true.into()),
            ..LinterConfiguration::default()
        }),
        overrides: Some(Overrides(vec![OverridePattern {
            includes: Some(OverrideGlobs::Globs(Box::new([
                biome_glob::NormalizedGlob::from_str("*.vue").unwrap(),
            ]))),
            // Override only specifies linter, not formatter
            linter: Some(OverrideLinterConfiguration {
                enabled: Some(false.into()),
                ..OverrideLinterConfiguration::default()
            }),
            ..OverridePattern::default()
        }])),
        ..Default::default()
    };

    let mut settings = Settings::default();
    settings
        .merge_with_configuration(configuration, None, vec![])
        .expect("valid configuration");

    // For .vue files, linter should be disabled (from override)
    let linter_enabled =
        JsLanguage::linter_enabled_for_file_path(&settings, Utf8Path::new("test.vue"));
    assert!(!linter_enabled, "Linter should be disabled for .vue files");

    // For .vue files, formatter should be enabled (inherited from global)
    let formatter_enabled =
        JsLanguage::formatter_enabled_for_file_path(&settings, Utf8Path::new("test.vue"));
    assert!(
        formatter_enabled,
        "Formatter should be enabled for .vue files (inherited from global)"
    );

    // For non .vue files, both should be enabled (from global)
    let linter_enabled_js =
        JsLanguage::linter_enabled_for_file_path(&settings, Utf8Path::new("test.js"));
    assert!(linter_enabled_js, "Linter should be enabled for .js files");

    let formatter_enabled_js =
        JsLanguage::formatter_enabled_for_file_path(&settings, Utf8Path::new("test.js"));
    assert!(
        formatter_enabled_js,
        "Formatter should be enabled for .js files"
    );
}

#[test]
fn test_module_graph_resolution_kind_from_scan_kind() {
    // Test all ScanKind variants map to correct ModuleGraphResolutionKind
    assert_eq!(
        ModuleGraphResolutionKind::from(&ScanKind::NoScanner),
        ModuleGraphResolutionKind::None
    );

    assert_eq!(
        ModuleGraphResolutionKind::from(&ScanKind::KnownFiles),
        ModuleGraphResolutionKind::None
    );

    assert_eq!(
        ModuleGraphResolutionKind::from(&ScanKind::TargetedKnownFiles {
            target_paths: vec![],
            descend_from_targets: false,
        }),
        ModuleGraphResolutionKind::None
    );

    assert_eq!(
        ModuleGraphResolutionKind::from(&ScanKind::Project),
        ModuleGraphResolutionKind::Modules
    );

    assert_eq!(
        ModuleGraphResolutionKind::from(&ScanKind::TypeAware),
        ModuleGraphResolutionKind::ModulesAndTypes
    );
}

#[test]
fn test_module_graph_resolution_kind_is_modules_and_types() {
    // Test is_modules_and_types predicate
    assert!(!ModuleGraphResolutionKind::None.is_modules_and_types());
    assert!(!ModuleGraphResolutionKind::Modules.is_modules_and_types());
    assert!(ModuleGraphResolutionKind::ModulesAndTypes.is_modules_and_types());
}

#[test]
fn test_type_aware_scan_enables_module_graph_type_inference() {
    // This test verifies that TypeAware scan kind results in type inference being enabled
    let type_aware_kind = ModuleGraphResolutionKind::from(&ScanKind::TypeAware);
    assert!(
        type_aware_kind.is_modules_and_types(),
        "TypeAware scan should enable type inference"
    );
}

#[test]
fn test_project_scan_disables_module_graph_type_inference() {
    // This test verifies that Project scan kind does NOT enable type inference
    let project_kind = ModuleGraphResolutionKind::from(&ScanKind::Project);
    assert!(
        !project_kind.is_modules_and_types(),
        "Project scan should NOT enable type inference"
    );
}
