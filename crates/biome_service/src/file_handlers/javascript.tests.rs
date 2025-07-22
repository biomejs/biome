use biome_configuration::bool::Bool;
use biome_configuration::javascript::JsParserConfiguration;
use biome_configuration::{Configuration, JsConfiguration};
use biome_fs::BiomePath;

use crate::settings::Settings;

use super::*;

#[test]
fn correctly_parses_ts_generics_with_jsx_everywhere() {
    let js_conf = JsConfiguration {
        parser: Some(JsParserConfiguration {
            jsx_everywhere: Some(Bool(true)),
            ..Default::default()
        }),
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

    let source = r#"
const f = <T1>(arg1: T1) => <T2>(arg2: T2) => {
    return { arg1, arg2 };
}
"#;
    let result = parse(
        &BiomePath::new("file.test"),
        DocumentFileSource::Js(JsFileSource::ts()),
        source,
        &settings,
        &mut NodeCache::default(),
    );
    assert!(!result.any_parse.has_errors());
}
