use crate::settings::{LanguageSettings, ServiceLanguage, Settings};
use crate::workspace::DocumentFileSource;
use biome_analyze::RuleFilter;
use biome_configuration::analyzer::{GroupPlainConfiguration, Nursery, SeverityOrGroup};
use biome_configuration::javascript::JsxRuntime;
use biome_configuration::max_size::MaxSize;
use biome_configuration::{
    Configuration, JsConfiguration, LinterConfiguration, OverrideFilesConfiguration, OverrideGlobs,
    OverrideLinterConfiguration, OverridePattern, Overrides, RuleConfiguration,
    RulePlainConfiguration, Rules,
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
        .merge_with_configuration(configuration, None)
        .expect("valid configuration");
    let environment = JsLanguage::resolve_environment(Some(&settings));

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
        .merge_with_configuration(configuration, None)
        .expect("valid configuration");
    let environment = JsLanguage::resolve_environment(Some(&settings));
    let language = JsLanguage::lookup_settings(&settings.languages);
    let options = JsLanguage::resolve_analyzer_options(
        Some(&settings),
        Some(&language.linter),
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
                    nursery: Some(SeverityOrGroup::Group(Nursery {
                        use_explicit_type: Some(RuleConfiguration::Plain(
                            RulePlainConfiguration::Off,
                        )),
                        ..Nursery::default()
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
        .merge_with_configuration(configuration, None)
        .expect("valid configuration");

    let disabled_rules = settings
        .as_linter_rules(Utf8Path::new("path/to/file.ts"))
        .unwrap()
        .as_disabled_rules();

    assert_eq!(
        disabled_rules,
        FxHashSet::from_iter([RuleFilter::Rule("nursery", "useExplicitType")])
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
        .merge_with_configuration(configuration, None)
        .expect("valid configuration");

    assert_eq!(
        settings.override_settings.patterns[0].files.max_size,
        Some(MaxSize(NonZeroU64::new(1024).unwrap()))
    );
}
