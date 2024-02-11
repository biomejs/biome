use biome_formatter::{AttributePosition, IndentStyle, LineWidth, QuoteStyle};
use biome_formatter_test::check_reformat::CheckReformat;
use biome_js_formatter::context::{ArrowParentheses, JsFormatOptions, Semicolons};
use biome_js_formatter::format_node;
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::JsFileSource;

mod language {
    include!("language.rs");
}

#[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"
   import React from "react";

const Component = () => (
  <div>
    <div data-a="1">
      Lorem ipsum dolor sit amet, consectetur adipiscing elit.
    </div>

    <div data-a="1" data-b="2" data-c="3">
      Lorem ipsum dolor sit amet, consectetur adipiscing elit.
    </div>

    <div data-a="Lorem ipsum dolor sit amet" data-b="Lorem ipsum dolor sit amet" data-c="Lorem ipsum dolor sit amet">
      Lorem ipsum dolor sit amet, consectetur adipiscing elit.
    </div>

    <div data-long-attribute-a="1" data-long-attribute-b="2" data-long-attribute-c="3">
      Lorem ipsum dolor sit amet, consectetur adipiscing elit.
    </div>

    <img src="/images/foo.png" />

    <img src="/images/foo.png" alt="bar" />

    <img src="/images/foo.png" alt="Lorem ipsum dolor sit amet, consectetur adipiscing elit." />
  </div>
);

    "#;
    let source_type = JsFileSource::tsx();
    let tree = parse(
        src,
        source_type,
        JsParserOptions::default().with_parse_class_parameter_decorators(),
    );
    let options = JsFormatOptions::new(source_type)
        .with_indent_style(IndentStyle::Space)
        .with_line_width(LineWidth::try_from(120).unwrap())
        .with_semicolons(Semicolons::Always)
        .with_quote_style(QuoteStyle::Double)
        .with_jsx_quote_style(QuoteStyle::Single)
        .with_arrow_parentheses(ArrowParentheses::Always)
        .with_attribute_position(AttributePosition::Multiline);

    let doc = format_node(options.clone(), &tree.syntax()).unwrap();
    let result = doc.print().unwrap();

    println!("{}", doc.into_document());
    eprintln!("{}", result.as_code());

    CheckReformat::new(
        &tree.syntax(),
        result.as_code(),
        "testing",
        &language::JsTestFormatLanguage::new(source_type),
        options,
    )
    .check_reformat();
}
