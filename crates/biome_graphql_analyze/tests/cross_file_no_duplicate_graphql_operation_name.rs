use biome_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleFilter};
use biome_graphql_parser::parse_graphql;
use biome_module_graph::{GraphqlModuleInfo, ModuleInfoKind};
use biome_workspace_db::WorkspaceDb;
use rustc_hash::FxHashSet;
use std::slice;

#[test]
fn flags_duplicate_operation_defined_in_another_file() {
    let mut db = WorkspaceDb::default();

    let mut operation_names = FxHashSet::default();
    operation_names.insert("user".to_string());

    db.update_or_insert_module(
        "schema_a.graphql".into(),
        ModuleInfoKind::Graphql(GraphqlModuleInfo {
            operation_names,
            ..GraphqlModuleInfo::default()
        }),
    );

    let parsed = parse_graphql(
        r#"
        query user {
          me {
            id
          }
        }
        "#,
    );

    let rule_filter = RuleFilter::Rule("suspicious", "noDuplicateGraphqlOperationName");
    let filter = AnalysisFilter {
        enabled_rules: Some(slice::from_ref(&rule_filter)),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default().with_file_path("schema_b.graphql");
    let services =
        biome_graphql_analyze::GraphqlAnalyzerServices::default().with_module_db(db.rc_module_db());

    let mut has_diagnostic = false;
    let (_, errors) =
        biome_graphql_analyze::analyze(&parsed.tree(), filter, &options, services, |signal| {
            if signal.diagnostic().is_some() {
                has_diagnostic = true;
            }
            ControlFlow::<Never>::Continue(())
        });

    assert!(errors.is_empty());
    assert!(has_diagnostic);
}
