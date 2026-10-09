use biome_tailwind_logic::use_tailwind_sorted_classes::{TailwindDesignSystem, sort_class_list};
use biome_tailwind_parser::parse_tailwind;
use json_comments::StripComments;

#[test]
fn fixtures() {
    insta::glob!("sort_v4/*.jsonc", |path| {
        let raw = std::fs::read_to_string(path).expect("read fixture");
        let cases: Vec<String> =
            serde_json::from_reader(StripComments::new(raw.as_bytes())).expect("parse jsonc array");
        let rendered = cases
            .iter()
            .map(|input| {
                let parsed = parse_tailwind(input);
                let sorted = sort_class_list(&parsed.tree(), &TailwindDesignSystem::default());
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
