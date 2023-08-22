use rome_console::{Markup, MarkupElement};

#[test]
fn test_macro() {
    let category = "test";

    match
    // Due to how MarkupNode is implemented, the result of the markup macro
    // cannot be stored in a binding and must be matched upon immediately
    rome_markup::markup! {
        <Info><Emphasis>{category}</Emphasis>" Commands"</Info>
    }
    {
        Markup(markup) => {
            let node_0 = &markup[0];
            assert_eq!(&node_0.elements, &[MarkupElement::Info, MarkupElement::Emphasis]);
            // assert_eq!(node_0.content.to_string(), category.to_string());

            let node_1 = &markup[1];
            assert_eq!(&node_1.elements, &[MarkupElement::Info]);
            // assert_eq!(node_1.content.to_string(), " Commands".to_string());
        }
    }
}

#[test]
fn test_macro_attributes() {
    rome_markup::markup! {
        <Hyperlink href="https://biomejs.dev/">"link"</Hyperlink>
    };
}

#[test]
fn test_macro_errors() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/markup/*.rs");
}
