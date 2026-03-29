use biome_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleFilter};
use biome_diagnostics::{DiagnosticExt, Severity, print_diagnostic_to_string};
use biome_graphql_analyze::{GraphqlAnalyzerServices, GraphqlProjectIndex, analyze_with_services};
use biome_graphql_parser::parse_graphql;
use camino::Utf8PathBuf;
use std::slice;
use std::sync::Arc;

#[test]
fn detects_duplicate_operation_name_from_project_index() {
    const FILENAME: &str = "query.graphql";
    const SOURCE: &str = r#"
        query user {
            me {
                id
            }
        }
    "#;

    let parsed = parse_graphql(SOURCE);
    let rule_filter = RuleFilter::Rule("nursery", "noDuplicateGraphqlOperationName");
    let options = AnalyzerOptions::default().with_file_path(Utf8PathBuf::from(FILENAME));
    let mut project_index = GraphqlProjectIndex::default();
    project_index.insert_operation_name("user", "shared/other.graphql");
    let services = GraphqlAnalyzerServices::default().with_project_index(Arc::new(project_index));

    let mut diagnostics = Vec::new();
    let (_, errors) = analyze_with_services(
        &parsed.tree(),
        AnalysisFilter {
            enabled_rules: Some(slice::from_ref(&rule_filter)),
            ..AnalysisFilter::default()
        },
        &options,
        services,
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                let error = diag
                    .with_severity(Severity::Warning)
                    .with_file_path(FILENAME)
                    .with_file_source_code(SOURCE);
                diagnostics.push(print_diagnostic_to_string(&error));
            }

            ControlFlow::<Never>::Continue(())
        },
    );

    assert!(errors.is_empty());
    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics[0].contains("shared/other.graphql"));
}

#[test]
fn ignores_project_index_entry_for_current_file() {
    const FILENAME: &str = "query.graphql";
    const SOURCE: &str = r#"
        query user {
            me {
                id
            }
        }
    "#;

    let parsed = parse_graphql(SOURCE);
    let rule_filter = RuleFilter::Rule("nursery", "noDuplicateGraphqlOperationName");
    let options = AnalyzerOptions::default().with_file_path(Utf8PathBuf::from(FILENAME));
    let mut project_index = GraphqlProjectIndex::default();
    project_index.insert_operation_name("user", FILENAME);
    let services = GraphqlAnalyzerServices::default().with_project_index(Arc::new(project_index));

    let mut diagnostics = 0;
    let (_, errors) = analyze_with_services(
        &parsed.tree(),
        AnalysisFilter {
            enabled_rules: Some(slice::from_ref(&rule_filter)),
            ..AnalysisFilter::default()
        },
        &options,
        services,
        |signal| {
            if signal.diagnostic().is_some() {
                diagnostics += 1;
            }

            ControlFlow::<Never>::Continue(())
        },
    );

    assert!(errors.is_empty());
    assert_eq!(diagnostics, 0);
}
