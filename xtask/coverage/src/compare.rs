use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use xtask::project_root;

use crate::results::emit_compare;
use crate::util::decode_maybe_utf16_string;
use crate::TestResults;

// this is the filename of the results coming from `main` branch
const BASE_RESULT_FILE: &str = "base_results.json";

// this is the filename of the results coming from the current PR
const NEW_RESULT_FILE: &str = "new_results.json";

pub fn coverage_compare(
    base_result_path: Option<&str>,
    new_result_path: Option<&str>,
    markdown: bool,
) {
    // resolve the path passed as argument, or retrieve the default one
    let base_result_dir = if let Some(base_result_path) = base_result_path {
        PathBuf::from(base_result_path)
    } else {
        project_root().join(BASE_RESULT_FILE)
    };

    // resolve the path passed as argument, or retrieve the default one
    let new_result_dir = if let Some(new_result_path) = new_result_path {
        PathBuf::from(new_result_path)
    } else {
        project_root().join(NEW_RESULT_FILE)
    };

    if !base_result_dir.exists() {
        panic!("The path to the base results doesn't exist: {base_result_dir:?}");
    }

    if !&new_result_dir.exists() {
        panic!("The path to the new results doesn't exist: {new_result_dir:?}");
    }

    let mut base_results = read_test_results(base_result_dir.as_path(), "base")
        .into_iter()
        .collect::<Vec<_>>();

    // Sort suite names to get a stable result in CI comments
    base_results.sort_unstable_by(|(suite_a, _), (suite_b, _)| suite_a.cmp(suite_b));

    let mut new_results = read_test_results(new_result_dir.as_path(), "new");

    for (suite, base) in base_results {
        let new_result = new_results.remove(&suite).unwrap_or_default();

        emit_compare(&base, &new_result, suite.as_str(), markdown);
    }

    for (suite, new) in new_results.drain() {
        emit_compare(&TestResults::new(), &new, suite.as_str(), markdown);
    }
}

fn read_test_results(path: &Path, name: &'static str) -> HashMap<String, TestResults> {
    let buffer = fs::read(path)
        .unwrap_or_else(|err| panic!("Can't read the file of the {name} results: {err:?}"));

    let content = decode_maybe_utf16_string(&buffer)
        .unwrap_or_else(|err| panic!("Can't read the file of the {name} results: {err:?}"));

    serde_json::from_str(&content)
        .unwrap_or_else(|err| panic!("Can't parse the JSON file of the {name} results: {err:?}"))
}
