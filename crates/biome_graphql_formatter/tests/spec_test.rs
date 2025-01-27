use biome_configuration::graphql::GraphqlFormatterConfiguration;
use biome_configuration::{Configuration, GraphqlConfiguration};
use biome_formatter_test::spec::{SpecSnapshot, SpecTestFile};
use biome_graphql_formatter::{context::GraphqlFormatOptions, GraphqlFormatLanguage};
use biome_service::workspace::UpdateSettingsParams;
use camino::Utf8Path;

mod language {
    include!("language.rs");
}

/// [insta.rs](https://insta.rs/docs) snapshot testing
///
/// For better development workflow, run
/// `cargo watch -i '*.new' -x 'test -p biome_graphql_formatter formatter'`
///
/// To review and commit the snapshots, `cargo install cargo-insta`, and run
/// `cargo insta review` or `cargo insta accept`
///
/// The input and the expected output are stored as dedicated files in the `tests/specs` directory where
/// the input file name is `{spec_name}.graphql` and the output file name is `{spec_name}.graphql.snap`.
///
/// Specs can be grouped in directories by specifying the directory name in the spec name. Examples:
///
/// # Examples
///
/// * `graphql/null` -> input: `tests/specs/graphql/null.graphql`, expected output: `tests/specs/graphql/null.graphql.snap`
/// * `null` -> input: `tests/specs/null.graphql`, expected output: `tests/specs/null.graphql.snap`
pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, _file_type: &str) {
    let root_path = Utf8Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/specs/"));
    let settings = |project_key| {
        Some(UpdateSettingsParams {
            project_key,
            configuration: Configuration {
                graphql: Some(GraphqlConfiguration {
                    formatter: Some(GraphqlFormatterConfiguration {
                        enabled: Some(true.into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            vcs_base_path: None,
            gitignore_matches: vec![],
            workspace_directory: None,
        })
    };

    let Some(test_file) = SpecTestFile::try_from_file(spec_input_file, root_path, settings) else {
        return;
    };

    let options = GraphqlFormatOptions::default();
    let language = language::GraphqlTestFormatLanguage::default();

    let snapshot = SpecSnapshot::new(
        test_file,
        test_directory,
        language,
        GraphqlFormatLanguage::new(options),
    );

    snapshot.test()
}
