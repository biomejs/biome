use biome_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleFilter};
use biome_json_analyze::JsonAnalyzeServices;
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_json_syntax::{AnyJsonValue, JsonFileSource, JsonObjectValue, JsonStringValue};
use biome_rowan::{AstNode, AstSeparatedList};
use std::path::PathBuf;
use std::slice;

/// Extract the text of a JSON value node from the data.json source, trimmed of surrounding
/// whitespace/trivia, so it can be used as a standalone JSON document.
fn value_to_string(value: &AnyJsonValue) -> String {
    value.syntax().text_trimmed().to_string()
}

/// Recursively compare two `AnyJsonValue` nodes for structural equality, including key order
/// in objects. Used instead of serde for semantic comparison.
fn json_values_equal(a: &AnyJsonValue, b: &AnyJsonValue) -> bool {
    match (a, b) {
        (AnyJsonValue::JsonObjectValue(a_obj), AnyJsonValue::JsonObjectValue(b_obj)) => {
            json_objects_equal(a_obj, b_obj)
        }
        (AnyJsonValue::JsonArrayValue(a_arr), AnyJsonValue::JsonArrayValue(b_arr)) => {
            let a_elems: Vec<_> = a_arr.elements().iter().filter_map(|e| e.ok()).collect();
            let b_elems: Vec<_> = b_arr.elements().iter().filter_map(|e| e.ok()).collect();
            if a_elems.len() != b_elems.len() {
                return false;
            }
            a_elems
                .iter()
                .zip(b_elems.iter())
                .all(|(a_v, b_v)| json_values_equal(a_v, b_v))
        }
        (AnyJsonValue::JsonStringValue(a_s), AnyJsonValue::JsonStringValue(b_s)) => {
            json_strings_equal(a_s, b_s)
        }
        // Numbers, booleans, null: compare raw token text
        _ => a.syntax().text_trimmed().to_string() == b.syntax().text_trimmed().to_string(),
    }
}

fn json_objects_equal(a: &JsonObjectValue, b: &JsonObjectValue) -> bool {
    let a_members: Vec<_> = a.json_member_list().iter().filter_map(|m| m.ok()).collect();
    let b_members: Vec<_> = b.json_member_list().iter().filter_map(|m| m.ok()).collect();

    if a_members.len() != b_members.len() {
        return false;
    }

    a_members.iter().zip(b_members.iter()).all(|(a_m, b_m)| {
        let a_key = a_m
            .name()
            .ok()
            .and_then(|n| n.inner_string_text())
            .map(|t| t.to_string());
        let b_key = b_m
            .name()
            .ok()
            .and_then(|n| n.inner_string_text())
            .map(|t| t.to_string());
        if a_key != b_key {
            return false;
        }
        match (a_m.value(), b_m.value()) {
            (Ok(a_v), Ok(b_v)) => json_values_equal(&a_v, &b_v),
            _ => false,
        }
    })
}

fn json_strings_equal(a: &JsonStringValue, b: &JsonStringValue) -> bool {
    match (a.inner_string_text(), b.inner_string_text()) {
        (Ok(a_t), Ok(b_t)) => a_t == b_t,
        _ => false,
    }
}

/// Run the `useSortedPackageJson` assist action on the given JSON text.
/// Returns `Ok(new_text)` if a transformation was applied, `Err` otherwise.
fn apply_organize_action(input_code: &str) -> Result<String, String> {
    let parsed = parse_json(input_code, JsonParserOptions::default());

    if parsed.has_errors() {
        return Err(format!(
            "Parse errors in input: {}",
            parsed
                .diagnostics()
                .iter()
                .map(|d| format!("{d:?}"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    let root = parsed.tree();

    let rule_filter = RuleFilter::Rule("source", "useSortedPackageJson");
    let filter = AnalysisFilter {
        enabled_rules: Some(slice::from_ref(&rule_filter)),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default().with_file_path("package.json");
    let services = JsonAnalyzeServices {
        file_source: JsonFileSource::json(),
        configuration_provider: None,
    };

    let mut result_code = None;

    let (_, _) = biome_json_analyze::analyze(&root, filter, &options, services, &[], |event| {
        for action in event.actions() {
            if !action.is_suppression() {
                let (_, text_edit) = action
                    .mutation
                    .clone()
                    .commit_with_text_range_and_edit(true);

                if let Some((_, edit)) = text_edit {
                    result_code = Some(edit.new_string(input_code));
                }
                break;
            }
        }

        ControlFlow::<Never>::Continue(())
    });

    result_code.ok_or_else(|| "No transformation applied".to_string())
}

/// Load test cases from data.json using Biome's own JSON parser.
/// Returns a list of `(test_name, input_text, output_text)` tuples.
fn load_test_cases() -> Vec<(String, String, String)> {
    let test_file =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/compat_sortpkg/data.json");

    let content = std::fs::read_to_string(&test_file).expect("Failed to read data.json");

    let parsed = parse_json(&content, JsonParserOptions::default());
    assert!(
        !parsed.has_errors(),
        "data.json has parse errors: {:?}",
        parsed.diagnostics()
    );

    let root = parsed.tree();
    let root_value = root.value().expect("data.json must have a root value");

    let array = match root_value {
        AnyJsonValue::JsonArrayValue(a) => a,
        _ => panic!("data.json root must be a JSON array"),
    };

    array
        .elements()
        .iter()
        .filter_map(|elem| elem.ok())
        .map(|elem| {
            let obj = match elem {
                AnyJsonValue::JsonObjectValue(o) => o,
                _ => panic!("Each test case must be a JSON object"),
            };

            let mut test_name = String::new();
            let mut input_text: Option<String> = None;
            let mut output_text = String::new();

            for member in obj.json_member_list().iter().filter_map(|m| m.ok()) {
                let key = member
                    .name()
                    .ok()
                    .and_then(|n| n.inner_string_text())
                    .map(|t| t.to_string())
                    .unwrap_or_default();

                match key.as_str() {
                    "testName" => {
                        if let Ok(AnyJsonValue::JsonStringValue(s)) = member.value() {
                            test_name = s
                                .inner_string_text()
                                .map(|t| t.to_string())
                                .unwrap_or_default();
                        }
                    }
                    "input" => {
                        if let Ok(v) = member.value() {
                            // Treat explicit JSON null the same as absent input (idempotency test)
                            if !matches!(v, AnyJsonValue::JsonNullValue(_)) {
                                input_text = Some(value_to_string(&v));
                            }
                        }
                    }
                    "output" => {
                        if let Ok(v) = member.value() {
                            output_text = value_to_string(&v);
                        }
                    }
                    _ => {}
                }
            }

            // When input is absent, use output as input (idempotency test)
            let input = input_text.unwrap_or_else(|| output_text.clone());

            (test_name, input, output_text)
        })
        .collect()
}

#[test]
fn test_compat_with_sortpkg() {
    let test_cases = load_test_cases();
    let total = test_cases.len();
    let mut failures: Vec<String> = Vec::new();

    for (i, (test_name, input_str, expected_str)) in test_cases.iter().enumerate() {
        let result_str = match apply_organize_action(input_str) {
            Ok(result) => result,
            Err(_) => {
                // No transformation applied — use original input as result
                input_str.clone()
            }
        };

        // Parse both result and expected with Biome's parser for semantic comparison
        let result_parsed = parse_json(&result_str, JsonParserOptions::default());
        let expected_parsed = parse_json(expected_str, JsonParserOptions::default());

        if result_parsed.has_errors() {
            failures.push(format!(
                "Test #{} '{}': transformation produced invalid JSON:\n{result_str}",
                i + 1,
                test_name,
            ));
            continue;
        }
        if expected_parsed.has_errors() {
            failures.push(format!(
                "Test #{} '{}': expected JSON in data.json is invalid:\n{expected_str}",
                i + 1,
                test_name,
            ));
            continue;
        }

        let result_value = result_parsed
            .tree()
            .value()
            .expect("result must have value");
        let expected_value = expected_parsed
            .tree()
            .value()
            .expect("expected must have value");

        if !json_values_equal(&result_value, &expected_value) {
            failures.push(format!(
                "Test #{} failed: {}\n\nInput:\n{}\n\nExpected:\n{}\n\nGot:\n{}",
                i + 1,
                test_name,
                input_str,
                expected_str,
                result_str,
            ));
        }
    }

    if !failures.is_empty() {
        panic!(
            "{}/{} tests failed:\n\n{}",
            failures.len(),
            total,
            failures.join("\n\n---\n\n")
        );
    }
}
