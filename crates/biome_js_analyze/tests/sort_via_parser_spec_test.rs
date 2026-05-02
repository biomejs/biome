use biome_js_analyze::lint::nursery::use_sorted_classes::sort_via_parser::sort_class_list;
use biome_tailwind_parser::parse_tailwind;
use std::fmt::Write;
use std::fs;

pub fn run(test_case: &str, _snapshot_name: &str, test_directory: &str, _outcome_str: &str) {
    let content = fs::read_to_string(test_case).expect("readable UTF-8 file");
    let mut snapshot = String::new();
    let mut current_comment = String::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            current_comment = trimmed[1..].trim().to_string();
        } else if trimmed.is_empty() {
            // separator
        } else {
            let parsed = parse_tailwind(trimmed);
            let sorted = sort_class_list(&parsed.tree());
            if !current_comment.is_empty() {
                writeln!(snapshot, "## {current_comment}").unwrap();
                current_comment.clear();
            }
            writeln!(snapshot, "Input:  {trimmed}").unwrap();
            writeln!(snapshot, "Sorted: {sorted}").unwrap();
            writeln!(snapshot).unwrap();
        }
    }

    let file_name = std::path::Path::new(test_case)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => test_directory,
    }, {
        insta::assert_snapshot!(file_name, snapshot);
    });
}
