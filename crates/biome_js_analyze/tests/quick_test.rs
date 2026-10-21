use biome_analyze::options::JsxRuntime;
use biome_analyze::{
    ActionFilter, AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never,
    RuleFilter,
};
use biome_db::Source;
use biome_deserialize::TextRange;
use biome_diagnostics::{Diagnostic, DiagnosticExt, Severity, print_diagnostic_to_string};
use biome_fs::TemporaryFs;
use biome_js_analyze::{JsAnalyzerServices, analyze};
use biome_js_parser::{JsParserOptions, Parse, parse};
use biome_js_syntax::AnyJsRoot;
use biome_languages::{DocumentFileSource, JsFileSource, LanguageDb};
use biome_package::{Dependencies, PackageJson};
use biome_parser::AnyParse;
use biome_parser::db::{ParseDb, SharedParse, SourceParse};
use biome_project_layout::ProjectLayout;
use biome_test_utils::module_graph_for_test_file;
use camino::{Utf8Path, Utf8PathBuf};
use salsa::Storage;
use std::rc::Rc;
use std::slice;
use std::sync::Arc;

#[salsa::db]
struct TestDb {
    parsed: Option<Source>,
    parse: SharedParse,
    storage: Storage<Self>,
}

/// Returns the parse that the test made of the text, which is the tree the
/// analyzer analyzes.
#[salsa::db]
impl ParseDb for TestDb {
    fn parse_source(&self, _source: Source) -> SourceParse {
        SourceParse {
            parse: self.parse.clone(),
            document_source_index: 0,
            snippets: Vec::new(),
        }
    }
}

#[salsa::db]
impl LanguageDb for TestDb {
    fn source_from_index(&self, _index: usize) -> Option<DocumentFileSource> {
        Some(DocumentFileSource::Js(JsFileSource::tsx()))
    }
}

#[salsa::db]
impl biome_db::Db for TestDb {
    fn parsed_source_for_path(&self, _path: &Utf8Path) -> Option<Source> {
        self.parsed
    }
}

#[salsa::db]
impl salsa::Database for TestDb {}

fn embedded_db(parsed: &Parse<AnyJsRoot>) -> Rc<dyn LanguageDb> {
    let mut db = TestDb {
        parsed: None,
        parse: AnyParse::from(parsed.syntax().as_send().unwrap()).into(),
        storage: Storage::default(),
    };
    let text = parsed.syntax().text_with_trivia().to_string();
    db.parsed = Some(Source::new(&db, Utf8PathBuf::new(), text.into(), 0));
    Rc::new(db)
}

fn project_layout_with_top_level_dependencies(dependencies: Dependencies) -> Arc<ProjectLayout> {
    let manifest = PackageJson::default().with_dependencies(dependencies);

    let project_layout = ProjectLayout::default();
    project_layout.insert_node_manifest("/".into(), manifest);

    Arc::new(project_layout)
}

// use this test check if your snippet produces the diagnostics you wish, without using a snapshot
#[test]
fn quick_test() {
    const FILENAME: &str = "dummyFile.ts";
    const SOURCE: &str = r#"export function foo(_arg: string) {
  const { bar, ...params } = something();
  return console.log(bar, params);

  function something() {
    const obj: Record<string, string> = { bar: "bar" };
    const { bar, baz } = obj;
    return { bar, baz };
  }
}
"#;

    let parsed = parse(SOURCE, JsFileSource::tsx(), JsParserOptions::default());

    let mut fs = TemporaryFs::new("quick_test");
    fs.create_file("sleep.ts", "export const sleep = async (ms = 1000): Promise<void> => new Promise((resolve) => setTimeout(resolve, ms));");
    fs.create_file(FILENAME, SOURCE);

    let file_path = Utf8PathBuf::from(format!("{}/{FILENAME}", fs.cli_path()));

    let mut error_ranges: Vec<TextRange> = Vec::new();
    let options = AnalyzerOptions::default()
        .with_file_path(file_path.clone())
        .with_configuration(
            AnalyzerConfiguration::default().with_jsx_runtime(JsxRuntime::ReactClassic),
        )
        .with_working_directory(fs.working_directory.clone());
    let rule_filter = RuleFilter::Rule("correctness", "noUnusedImports");

    let dependencies = Dependencies(Box::new([("buffer".into(), "latest".into())]));

    let project_layout = project_layout_with_top_level_dependencies(dependencies);
    let db = module_graph_for_test_file(file_path.as_path(), project_layout.as_ref());
    let services =
        crate::JsAnalyzerServices::from((db.rc_module_db(), project_layout, JsFileSource::tsx()))
            .with_language_db(embedded_db(&parsed));

    analyze(
        &parsed.tree(),
        AnalysisFilter {
            enabled_rules: Some(slice::from_ref(&rule_filter)),
            ..AnalysisFilter::default()
        },
        &options,
        &[],
        services,
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                error_ranges.push(diag.location().span.unwrap());
                let error = diag
                    .with_severity(Severity::Warning)
                    .with_file_path(FILENAME)
                    .with_file_source_code(SOURCE);
                let text = print_diagnostic_to_string(&error);
                eprintln!("{text}");
            }

            for action in signal.actions(ActionFilter::all()) {
                let new_code = action.mutation.commit();
                eprintln!("new code!!!");
                eprintln!("{new_code}");
            }

            ControlFlow::<Never>::Continue(())
        },
    );

    // assert_eq!(error_ranges.as_slice(), &[]);
}

#[test]
fn test_jsx_factory_from_tsconfig() {
    const FILENAME: &str = "test.jsx";
    const SOURCE: &str = r#"import { h, Fragment } from 'preact';

function App() {
  return (
    <>
      <div>Hello World</div>
    </>
  );
}
"#;

    let parsed = parse(SOURCE, JsFileSource::jsx(), JsParserOptions::default());

    let file_path = Utf8PathBuf::from(FILENAME);

    let mut error_ranges: Vec<TextRange> = Vec::new();
    let options = AnalyzerOptions::default()
        .with_file_path(file_path.clone())
        .with_configuration(
            AnalyzerConfiguration::default()
                .with_jsx_runtime(JsxRuntime::ReactClassic)
                .with_jsx_factory(Some("h".into()))
                .with_jsx_fragment_factory(Some("Fragment".into())),
        );
    let rule_filter = RuleFilter::Rule("correctness", "noUnusedImports");

    let services = JsAnalyzerServices::default().with_language_db(embedded_db(&parsed));

    analyze(
        &parsed.tree(),
        AnalysisFilter {
            enabled_rules: Some(slice::from_ref(&rule_filter)),
            ..AnalysisFilter::default()
        },
        &options,
        &[],
        services,
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                error_ranges.push(diag.location().span.unwrap());
                let error = diag
                    .with_severity(Severity::Warning)
                    .with_file_path(FILENAME)
                    .with_file_source_code(SOURCE);
                let text = print_diagnostic_to_string(&error);
                eprintln!("{text}");
            }

            ControlFlow::<Never>::Continue(())
        },
    );

    // Should not report any errors because h and Fragment are used as JSX factory functions
    assert_eq!(error_ranges.as_slice(), &[]);
}
