use biome_test_utils::analyze_with_workspace;
use camino::Utf8Path;

tests_macros::gen_tests! {"tests/js_embeds/*/*.jsx", crate::run_test, "module"}

fn run_test(input: &'static str, _: &str, _: &str, _: &str) {
    let input_file = Utf8Path::new(input);
    let input_code = std::fs::read_to_string(input_file).expect("read fixture");
    let rule = input_file.parent().unwrap().file_name().unwrap();
    let snapshot = analyze_with_workspace(input_file, input_code, "suspicious", rule);
    insta::with_settings!({
        snapshot_path => input_file.parent().unwrap(),
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(input_file.file_name().unwrap(), snapshot);
    });
}
