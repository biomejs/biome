use biome_test_utils::has_bogus_nodes_or_empty_slots;
use biome_yaml_parser::parse_yaml;

#[ignore]
#[test]
pub fn quick_test() {
    let code = r#"- foo
- bar
"#;

    let root = parse_yaml(code);
    let syntax = root.syntax();
    dbg!(&syntax, root.diagnostics(), root.has_errors());
    if has_bogus_nodes_or_empty_slots(&syntax) {
        panic!("modified tree has bogus nodes or empty slots:\n{syntax:#?} \n\n {syntax}")
    }
}
