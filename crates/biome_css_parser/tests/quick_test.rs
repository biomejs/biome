use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_syntax::CssFileSource;
use biome_test_utils::has_bogus_nodes_or_empty_slots;

#[ignore]
#[test]
pub fn quick_test() {
    let code = r#"
  .card {
    label:hover {
      color: red;
    }
  }
"#;

    let root = parse_css(
        code,
        CssFileSource::scss(),
        CssParserOptions::default()
            .allow_wrong_line_comments()
            .allow_css_modules()
            .allow_metavariables()
            .allow_tailwind_directives(),
    );
    let syntax = root.syntax();
    dbg!(&syntax, root.diagnostics(), root.has_errors());
    if has_bogus_nodes_or_empty_slots(&syntax) {
        panic!("modified tree has bogus nodes or empty slots:\n{syntax:#?} \n\n {syntax}")
    }
}
