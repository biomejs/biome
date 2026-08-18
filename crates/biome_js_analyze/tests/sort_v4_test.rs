use biome_tailwind_logic::sorted_classes::{TailwindRegistry, sort_class_list};
use biome_tailwind_parser::parse_tailwind;
use biome_test_utils::scripts_from_json;

#[test]
fn fixtures() {
    let registry = TailwindRegistry::default();
    insta::glob!("sort_v4/*.jsonc", |path| {
        let raw = std::fs::read_to_string(path).expect("read fixture");
        let cases = scripts_from_json("jsonc", &raw).expect("parse jsonc array");
        let rendered = cases
            .iter()
            .map(|input| {
                let parsed = parse_tailwind(input);
                let sorted = sort_class_list(&parsed.tree(), &registry);
                format!("input:  {input}\nsorted: {sorted}")
            })
            .collect::<Vec<_>>()
            .join("\n---\n");
        let snap_name = path.file_stem().unwrap().to_str().unwrap();
        insta::with_settings!(
            {
                snapshot_path => path.parent().unwrap(),
                prepend_module_to_snapshot => false,
            },
            { insta::assert_snapshot!(snap_name, rendered) }
        );
    });
}
