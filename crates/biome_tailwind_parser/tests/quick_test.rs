use biome_tailwind_parser::parse_tailwind;
use biome_test_utils::has_bogus_nodes_or_empty_slots;

#[ignore]
#[test]
pub fn quick_test() {
    let code = r#"text-sm"#;

    let root = parse_tailwind(code);
    let syntax = root.syntax();
    dbg!(&syntax, root.diagnostics(), root.has_errors());
    if has_bogus_nodes_or_empty_slots(&syntax) {
        panic!("modified tree has bogus nodes or empty slots:\n{syntax:#?} \n\n {syntax}")
    }
}
