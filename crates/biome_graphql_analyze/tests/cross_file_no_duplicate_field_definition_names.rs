use biome_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleFilter};
use biome_graphql_parser::parse_graphql;
use biome_module_graph::{GraphqlModuleInfo, GraphqlTypeKey, GraphqlTypeKind, ModuleInfoKind};
use biome_workspace_db::WorkspaceDb;
use rustc_hash::{FxHashMap, FxHashSet};
use std::slice;

#[test]
fn flags_duplicate_field_defined_in_another_schema_file() {
    let mut db = WorkspaceDb::default();

    let mut fields_by_type = FxHashMap::default();
    let mut user_fields = FxHashSet::default();
    user_fields.insert("id".to_string());

    fields_by_type.insert(
        GraphqlTypeKey {
            kind: GraphqlTypeKind::Object,
            name: "User".to_string(),
        },
        user_fields,
    );

    db.update_or_insert_module(
        "schema_a.graphql".into(),
        ModuleInfoKind::Graphql(GraphqlModuleInfo {
            type_fields: fields_by_type,
            ..GraphqlModuleInfo::default()
        }),
    );

    let parsed = parse_graphql(
        r#"
        extend type User {
          id: ID
        }
        "#,
    );

    let rule_filter = RuleFilter::Rule("nursery", "noDuplicateFieldDefinitionNames");
    let filter = AnalysisFilter {
        enabled_rules: Some(slice::from_ref(&rule_filter)),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default().with_file_path("schema_b.graphql");
    let services = biome_graphql_analyze::GraphqlAnalyzerServices::default()
        .with_module_db(db.rc_module_db());

    let mut has_diagnostic = false;
    let (_, errors) = biome_graphql_analyze::analyze(
        &parsed.tree(),
        filter,
        &options,
        services,
        |signal| {
            if signal.diagnostic().is_some() {
                has_diagnostic = true;
            }
            ControlFlow::<Never>::Continue(())
        },
    );

    assert!(errors.is_empty());
    assert!(has_diagnostic);
}
