#[path = "../benches/css_parser/suites.rs"]
mod suites;

use biome_languages::CssFileSource;
use suites::parser_benchmark_suites;

const CORPUS_REVISION: &str = "b5812cb1bd3cbbe79c485b28c721a43fc3bbda9d";
const CORPUS_FILES: [&str; 6] = [
    "full-spectrum.scss",
    "hot-ambiguous-nested-rules.scss",
    "hot-url-interpolation.scss",
    "hot-interpolated-strings.scss",
    "hot-tight-binary-expressions.scss",
    "hot-lists-maps-arguments.scss",
];

#[test]
fn scss_benchmark_corpus_is_commit_pinned() {
    let [css, scss] = parser_benchmark_suites();
    assert_eq!(css.group_name, "css_parser");
    assert_eq!(css.source_type, CssFileSource::css());
    assert!(!css.require_all_libraries);
    assert!(!css.require_clean_parse);

    assert_eq!(scss.group_name, "scss_parser");
    assert_eq!(scss.source_type, CssFileSource::scss());
    assert!(scss.require_all_libraries);
    assert!(scss.require_clean_parse);

    let actual_urls = scss.libraries.lines().collect::<Vec<_>>();
    let expected_urls = CORPUS_FILES
        .map(|file| {
            format!(
                "https://raw.githubusercontent.com/biomejs/benchmark-fixtures/{CORPUS_REVISION}/generated/{file}"
            )
        })
        .to_vec();

    assert_eq!(actual_urls, expected_urls);
}

#[test]
fn scss_benchmark_corpus_requires_every_remote_file() {
    let [css, scss] = parser_benchmark_suites();
    css.handle_load_error("https://example.com/css.css", "offline");

    let failure = std::panic::catch_unwind(|| {
        scss.handle_load_error("https://example.com/scss.scss", "offline");
    })
    .expect_err("SCSS corpus acquisition failures must stop the benchmark");
    let message = failure
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| failure.downcast_ref::<&str>().copied())
        .expect("panic payload should contain the load failure message");

    assert_eq!(
        message,
        "failed to load scss_parser benchmark fixture https://example.com/scss.scss: offline"
    );
}
