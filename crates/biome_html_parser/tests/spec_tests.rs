mod spec_test;

use biome_html_factory::syntax::HtmlElement;
use biome_html_parser::{HtmlParseOptions, parse_html};
use biome_html_syntax::ScriptType;
use biome_rowan::AstNode;

mod ok {
    tests_macros::gen_tests! {"tests/html_specs/ok/**/*.{html,astro,vue,svelte}", crate::spec_test::run, "ok"}
}

mod error {
    tests_macros::gen_tests! {"tests/html_specs/error/**/*.{html,astro,vue,svelte}", crate::spec_test::run, "error"}
}

#[test]
fn test_is_javascript_tag() {
    let html = r#"
        <script type="text/javascript">
        </script>
        "#;
    let syntax = parse_html(html, HtmlParseOptions::default());
    let element = syntax
        .tree()
        .syntax()
        .descendants()
        .find_map(HtmlElement::cast)
        .unwrap();

    assert!(element.is_javascript_tag());

    let html = r#"
        <script type="application/javascript">
        </script>
        "#;
    let syntax = parse_html(html, HtmlParseOptions::default());
    let element = syntax
        .tree()
        .syntax()
        .descendants()
        .find_map(HtmlElement::cast)
        .unwrap();

    assert!(element.is_javascript_tag());
    assert_eq!(element.get_script_type(), Some(ScriptType::Classic));

    let html = r#"
        <script type="application/ecmascript">
        </script>
        "#;
    let syntax = parse_html(html, HtmlParseOptions::default());
    let element = syntax
        .tree()
        .syntax()
        .descendants()
        .find_map(HtmlElement::cast)
        .unwrap();

    assert!(element.is_javascript_tag());
    assert_eq!(element.get_script_type(), Some(ScriptType::Classic));

    let html = r#"
        <script type="module">
        </script>
        "#;
    let syntax = parse_html(html, HtmlParseOptions::default());
    let element = syntax
        .tree()
        .syntax()
        .descendants()
        .find_map(HtmlElement::cast)
        .unwrap();

    assert!(element.is_javascript_tag());
    assert_eq!(element.get_script_type(), Some(ScriptType::Module));

    // FIXME: Uncomment when the parser supports unquoted attribute values.
    //let html = r#"
    //    <script type=module></script>
    //    "#;
    //let syntax = parse_html(html, HtmlParseOptions::default());
    //let element = syntax
    //    .tree()
    //    .syntax()
    //    .descendants()
    //    .find_map(HtmlElement::cast)
    //    .unwrap();
    //
    //assert!(element.is_javascript_tag());
    //assert_eq!(element.get_script_type(), Some(ScriptType::Module));

    let html = r#"
        <script></script>
        "#;
    let syntax = parse_html(html, HtmlParseOptions::default());
    let element = syntax
        .tree()
        .syntax()
        .descendants()
        .find_map(HtmlElement::cast)
        .unwrap();

    assert!(element.is_javascript_tag());
    assert_eq!(element.get_script_type(), Some(ScriptType::Classic));

    let html = r#"
        <script type="importmap"></script>
        "#;
    let syntax = parse_html(html, HtmlParseOptions::default());
    let element = syntax
        .tree()
        .syntax()
        .descendants()
        .find_map(HtmlElement::cast)
        .unwrap();

    assert!(!element.is_javascript_tag());
    assert_eq!(element.get_script_type(), Some(ScriptType::ImportMap));
}
