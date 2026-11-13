use biome_analyze::RuleFilter;
use biome_configuration::analyzer::RuleGroupExt;
use biome_configuration::analyzer::linter::{
    A11y, Complexity, Correctness, Nursery, Performance, Security, Style, Suspicious,
};
use biome_deserialize::{Deserializable, json::deserialize_from_json_str};
use biome_json_parser::JsonParserOptions;

fn check_rule_filters<G: RuleGroupExt + Deserializable>() {
    let filters = G::all_rules_as_filters();
    assert!(filters.windows(2).all(|pair| pair[0] < pair[1]));

    for &filter in filters {
        let RuleFilter::Rule(_, rule_name) = filter else {
            panic!("Expected a rule filter, got {filter:?}");
        };
        for level in ["on", "off"] {
            let source = format!(r#"{{"{rule_name}": "{level}"}}"#);
            let result = deserialize_from_json_str::<G>(&source, JsonParserOptions::default(), "");
            assert!(result.diagnostics().is_empty(), "{source}");
            let group = result.into_deserialized().unwrap();
            let expected = [filter].into_iter().collect();
            if level == "on" {
                assert_eq!(group.get_enabled_rules(), expected, "{source}");
                assert!(group.get_disabled_rules().is_empty(), "{source}");
            } else {
                assert_eq!(group.get_disabled_rules(), expected, "{source}");
                assert!(group.get_enabled_rules().is_empty(), "{source}");
            }
        }
    }
}

#[test]
fn rule_filters_match_configuration_names() {
    check_rule_filters::<A11y>();
    check_rule_filters::<Complexity>();
    check_rule_filters::<Correctness>();
    check_rule_filters::<Nursery>();
    check_rule_filters::<Performance>();
    check_rule_filters::<Security>();
    check_rule_filters::<Style>();
    check_rule_filters::<Suspicious>();
}
