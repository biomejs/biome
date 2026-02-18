use biome_analyze::{AnalysisFilter, ControlFlow, Never, RuleFilter};
use biome_json_analyze::JsonAnalyzeServices;
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_json_syntax::JsonFileSource;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::slice;

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    #[serde(rename = "testName")]
    test_name: String,
    input: Option<serde_json::Value>,
    output: serde_json::Value,
}

fn load_test_cases() -> Vec<TestCase> {
    let test_file =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/compat_sortpkg/data.json");

    let content = std::fs::read_to_string(&test_file).expect("Failed to read data.json");

    serde_json::from_str(&content).expect("Failed to parse data.json")
}

fn apply_organize_action(input_code: &str) -> Result<String, String> {
    let parsed = parse_json(input_code, JsonParserOptions::default());

    if parsed.has_errors() {
        return Err("Parse errors in input".to_string());
    }

    let root = parsed.tree();

    // Set up the analyzer with only the organizePackageJson rule
    let rule_filter = RuleFilter::Rule("source", "organizePackageJson");
    let filter = AnalysisFilter {
        enabled_rules: Some(slice::from_ref(&rule_filter)),
        ..AnalysisFilter::default()
    };

    let options = Default::default();
    let services = JsonAnalyzeServices {
        file_source: JsonFileSource::json(),
        configuration_source: None,
    };

    let mut result_code = None;

    let (_, _) = biome_json_analyze::analyze(&root, filter, &options, services, |event| {
        // Get the first code action (organize action)
        for action in event.actions() {
            if !action.is_suppression() {
                // Apply the mutation to get the transformed code
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

#[test]
fn test_compat_with_sortpkg() {
    let test_cases = load_test_cases();

    for (i, test) in test_cases.iter().enumerate() {
        // For tests without input, use output as input (idempotency test)
        let input = test.input.as_ref().unwrap_or(&test.output);

        let input_str = serde_json::to_string_pretty(input).unwrap();

        let result_json = match apply_organize_action(&input_str) {
            Ok(result) => {
                serde_json::from_str(&result).expect("Transformation produced invalid JSON")
            }
            Err(_) => {
                // No transformation applied - use original input
                input.clone()
            }
        };

        assert_eq!(
            result_json,
            test.output,
            "\n❌ Test #{} failed: {}\n\nInput:\n{}\n\nExpected:\n{}\n\nGot:\n{}\n",
            i + 1,
            test.test_name,
            input_str,
            serde_json::to_string_pretty(&test.output).unwrap(),
            serde_json::to_string_pretty(&result_json).unwrap()
        );
    }

    eprintln!(
        "\n✅ All {} sort-package-json compatibility tests passed\n",
        test_cases.len()
    );
}
