use crate::settings::{LanguageSettings, ServiceLanguage, Settings};
use crate::workspace::DocumentFileSource;
use biome_configuration::javascript::JsxRuntime;
use biome_configuration::{Configuration, JsConfiguration};
use biome_fs::BiomePath;
use biome_js_syntax::JsLanguage;
use camino::Utf8PathBuf;

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
