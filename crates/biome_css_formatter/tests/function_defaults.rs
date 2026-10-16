use biome_css_formatter::{context::CssFormatOptions, format_node};
use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_syntax::{
    AnyCssRoot, CssFunctionAtRule, CssFunctionParameter, CssQualifiedRule, CssUrlValueRaw,
};
use biome_languages::CssFileSource;
use biome_rowan::{AstNode, Direction};
use biome_test_utils::has_bogus_nodes_or_empty_slots;

#[test]
fn native_function_defaults_round_trip() {
    for (source, source_type) in [
        (
            include_str!("specs/css/atrule/function-default-values.css"),
            CssFileSource::css(),
        ),
        (
            include_str!("specs/scss/at-rule/function-default-values.scss"),
            CssFileSource::scss(),
        ),
        (
            include_str!("specs/css/atrule/function-default-blocks.css"),
            CssFileSource::css(),
        ),
        (
            include_str!("specs/scss/at-rule/function-default-blocks.scss"),
            CssFileSource::scss(),
        ),
    ] {
        let parsed = parse_css(source, source_type, CssParserOptions::default());
        assert!(!parsed.has_errors(), "{:?}", parsed.diagnostics());
        assert_eq!(parsed.syntax().to_string(), source);
        assert!(!has_bogus_nodes_or_empty_slots(&parsed.syntax()));
        assert!(!format!("{:#?}", parsed.tree()).contains("missing (required)"));
        let parameters = parameter_tokens(&parsed.tree());

        for width in [40, 80] {
            let options =
                CssFormatOptions::new(source_type).with_line_width(width.try_into().unwrap());
            let formatted = format_node(options.clone(), &parsed.syntax())
                .unwrap()
                .print()
                .unwrap();
            let reparsed = parse_css(
                formatted.as_code(),
                source_type,
                CssParserOptions::default(),
            );
            assert!(!reparsed.has_errors(), "{:?}", reparsed.diagnostics());
            assert_eq!(reparsed.syntax().to_string(), formatted.as_code());
            assert!(!has_bogus_nodes_or_empty_slots(&reparsed.syntax()));
            assert!(!format!("{:#?}", reparsed.tree()).contains("missing (required)"));
            assert_eq!(parameters, parameter_tokens(&reparsed.tree()));
            let second = format_node(options, &reparsed.syntax())
                .unwrap()
                .print()
                .unwrap();
            assert_eq!(formatted.as_code(), second.as_code());
        }
    }
}

fn parameter_tokens(root: &AnyCssRoot) -> Vec<Vec<String>> {
    root.syntax()
        .descendants()
        .filter_map(CssFunctionParameter::cast)
        .map(|parameter| {
            parameter
                .syntax()
                .descendants_tokens(Direction::Next)
                .map(|token| token.text_trimmed().to_owned())
                .collect()
        })
        .collect()
}

#[test]
fn malformed_native_defaults_preserve_following_rules() {
    for source_type in [CssFileSource::css(), CssFileSource::scss()] {
        for prelude in [
            "--value: , --after: 4px)",
            "--value: 8px; 16px, --after: 4px)",
            "--value: 8px !important, --after: 4px)",
            "--value: fn(8px], --after: 4px)",
            "--value: [8px)",
            "--value: 8px 16px",
        ] {
            let source =
                format!("@function --f({prelude} {{result:var(--value)}}\n.after {{color:red}}");
            let parsed = parse_css(&source, source_type, CssParserOptions::default());
            assert!(parsed.has_errors(), "{source}");
            assert_eq!(parsed.syntax().to_string(), source);
            let function = parsed
                .syntax()
                .descendants()
                .find_map(CssFunctionAtRule::cast)
                .unwrap();
            assert_eq!(
                function.block().unwrap().syntax().text_trimmed(),
                "{result:var(--value)}"
            );
            let after = parsed
                .syntax()
                .descendants()
                .find_map(CssQualifiedRule::cast)
                .unwrap();
            assert_eq!(after.syntax().text_trimmed(), ".after {color:red}");
        }
    }
}

#[test]
fn native_function_default_urls_preserve_payloads() {
    for source_type in [CssFileSource::css(), CssFileSource::scss()] {
        for (url, payload) in [
            ("url(a,b)", "a,b"),
            ("url(../images/a,b.png)", "../images/a,b.png"),
            (
                "url(data:image/png;base64,AAAA)",
                "data:image/png;base64,AAAA",
            ),
            (
                "url(data:image/svg+xml,%3Csvg%3E%3C/svg%3E)",
                "data:image/svg+xml,%3Csvg%3E%3C/svg%3E",
            ),
            ("url(a/*payload*/,b)", "a/*payload*/,b"),
            ("URL(a,b)", "a,b"),
            ("u\\72l(a,b)", "a,b"),
            ("url(a\u{2028}b,a\u{2029}b)", "a\u{2028}b,a\u{2029}b"),
            ("url(\r\na,b\r\n)", "a,b"),
            ("url(\ra,b\r)", "a,b"),
            ("url(a{b)", "a{b"),
            ("url(a}b)", "a}b"),
            ("url(a[b)", "a[b"),
            ("url(a]b)", "a]b"),
            ("url([a)", "[a"),
            ("url({a)", "{a"),
            ("url(]a)", "]a"),
            ("url(}a)", "}a"),
        ] {
            let source = format!(
                "@function --f(--x: {url}, --nested: var(--image, {url}), --other: red) {{result:var(--x)}}"
            );
            let parsed = parse_css(&source, source_type, CssParserOptions::default());
            assert!(!parsed.has_errors(), "{source}: {:?}", parsed.diagnostics());
            assert_eq!(parsed.syntax().to_string(), source);
            assert!(!has_bogus_nodes_or_empty_slots(&parsed.syntax()));
            for width in [40, 80] {
                let options =
                    CssFormatOptions::new(source_type).with_line_width(width.try_into().unwrap());
                let formatted = format_node(options.clone(), &parsed.syntax())
                    .unwrap()
                    .print()
                    .unwrap();
                assert_eq!(
                    formatted.as_code().matches(payload).count(),
                    2,
                    "{source}: {}",
                    formatted.as_code()
                );
                assert!(!formatted.as_code().contains('\r'));
                let reparsed = parse_css(
                    formatted.as_code(),
                    source_type,
                    CssParserOptions::default(),
                );
                assert!(!reparsed.has_errors(), "{:?}", reparsed.diagnostics());
                assert!(!has_bogus_nodes_or_empty_slots(&reparsed.syntax()));
                let second = format_node(options, &reparsed.syntax())
                    .unwrap()
                    .print()
                    .unwrap();
                assert_eq!(formatted.as_code(), second.as_code());
            }
        }
    }
}

#[test]
fn missing_native_default_parenthesis_preserves_selector_lists() {
    for source_type in [CssFileSource::css(), CssFileSource::scss()] {
        for following in [
            ".a, .b {color:red}",
            ".a, button {color:red}",
            ".a,\n.b, .c {color:red}",
            ".a, :is(.b) {color:blue;}",
            ".a, :not(:is(.b, .c)) {color:blue;}",
        ] {
            let source = format!("@function --f(--x: red {{result:var(--x)}}\n{following}");
            let parsed = parse_css(&source, source_type, CssParserOptions::default());
            assert!(parsed.has_errors(), "{source}");
            assert_eq!(parsed.syntax().to_string(), source);
            let function = parsed
                .syntax()
                .descendants()
                .find_map(CssFunctionAtRule::cast)
                .unwrap();
            assert_eq!(
                function.block().unwrap().syntax().text_trimmed(),
                "{result:var(--x)}"
            );
            let after = parsed
                .syntax()
                .descendants()
                .find_map(CssQualifiedRule::cast)
                .unwrap();
            assert_eq!(after.syntax().text_trimmed(), following);
            assert!(!has_bogus_nodes_or_empty_slots(after.syntax()));
        }
    }
}

#[test]
fn native_function_default_url_boundaries_are_raw() {
    let mut failures = Vec::new();
    for source_type in [CssFileSource::css(), CssFileSource::scss()] {
        for payload in ["a{b", "a}b", "a[b", "a]b", "[a", "{a", "]a", "}a"] {
            let source = format!(
                "@function --f(--x: url({payload}), --other: 8px 16px) {{ result: var(--x); }}"
            );
            let parsed = parse_css(&source, source_type, CssParserOptions::default());
            assert_eq!(parsed.syntax().to_string(), source);
            if parsed.has_errors() {
                failures.push(format!(
                    "{source_type:?}: {payload}: {:?}",
                    parsed.diagnostics()
                ));
                continue;
            }
            assert!(!has_bogus_nodes_or_empty_slots(&parsed.syntax()));
            assert!(!format!("{:#?}", parsed.tree()).contains("missing (required)"));
            assert_eq!(parameter_tokens(&parsed.tree()).len(), 2);
            let urls: Vec<_> = parsed
                .syntax()
                .descendants()
                .filter_map(CssUrlValueRaw::cast)
                .map(|url| url.value_token().unwrap().text_trimmed().to_owned())
                .collect();
            assert_eq!(urls, [payload], "{source}");
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn ordinary_urls_with_leading_delimiters_round_trip() {
    for source_type in [CssFileSource::css(), CssFileSource::scss()] {
        for payload in ["[a", "{a", "]a", "}a"] {
            let source = format!(".a {{background:url({payload}); color:red;}}");
            let parsed = parse_css(&source, source_type, CssParserOptions::default());
            assert!(!parsed.has_errors(), "{source}: {:?}", parsed.diagnostics());
            assert_eq!(parsed.syntax().to_string(), source);
            assert!(!has_bogus_nodes_or_empty_slots(&parsed.syntax()));
            let url = parsed
                .syntax()
                .descendants()
                .find_map(CssUrlValueRaw::cast)
                .unwrap();
            assert_eq!(url.value_token().unwrap().text_trimmed(), payload);
            let options = CssFormatOptions::new(source_type);
            let formatted = format_node(options.clone(), &parsed.syntax())
                .unwrap()
                .print()
                .unwrap();
            let reparsed = parse_css(
                formatted.as_code(),
                source_type,
                CssParserOptions::default(),
            );
            assert!(!reparsed.has_errors(), "{:?}", reparsed.diagnostics());
            assert!(!has_bogus_nodes_or_empty_slots(&reparsed.syntax()));
            let second = format_node(options, &reparsed.syntax())
                .unwrap()
                .print()
                .unwrap();
            assert_eq!(formatted.as_code(), second.as_code());
            assert!(formatted.as_code().contains(&format!("url({payload})")));
        }
    }
}

#[test]
fn native_default_validation_preserves_header_ownership() {
    let mut failures = Vec::new();
    for source_type in [CssFileSource::css(), CssFileSource::scss()] {
        let source = "@function --f(--x: , --y: {a:b}) {result:red;}\n.after {color:blue;}";
        let parsed = parse_css(source, source_type, CssParserOptions::default());
        assert!(parsed.has_errors());
        assert_eq!(parsed.syntax().to_string(), source);
        let function = parsed
            .syntax()
            .descendants()
            .find_map(CssFunctionAtRule::cast)
            .unwrap();
        let body = function.block().unwrap();
        if body.syntax().text_trimmed() != "{result:red;}" {
            failures.push(format!("{source_type:?}: {}", body.syntax().text_trimmed()));
            continue;
        }
        assert_eq!(parsed.diagnostics().len(), 1);
        let parameters: Vec<_> = function
            .syntax()
            .descendants()
            .filter_map(CssFunctionParameter::cast)
            .collect();
        assert_eq!(parameters.len(), 2);
        assert_eq!(parameters[1].syntax().text_trimmed(), "--y: {a:b}");
        assert!(!has_bogus_nodes_or_empty_slots(parameters[1].syntax()));
        let after = parsed
            .syntax()
            .descendants()
            .find_map(CssQualifiedRule::cast)
            .unwrap();
        assert_eq!(after.syntax().text_trimmed(), ".after {color:blue;}");
        assert!(!has_bogus_nodes_or_empty_slots(after.syntax()));
    }
    assert!(
        failures.is_empty(),
        "braced defaults became bodies: {failures:#?}"
    );
}

#[test]
fn malformed_native_defaults_preserve_later_braced_defaults() {
    for source_type in [CssFileSource::css(), CssFileSource::scss()] {
        for name in ["--f", "--f ", "--#{$name}", "--#{$name} "] {
            if source_type == CssFileSource::css() && name.contains('#') {
                continue;
            }
            for boundary in ["", "] ", "} "] {
                let malformed = format!("@function {name}(--x:red {{result:red;}}\n");
                let valid = "@function --good(--x: {a; b !important} @raw(value), --y: [c, d]) {result:var(--x);}";
                let source = format!("{}{boundary}{valid}", malformed.repeat(16));
                let parsed = parse_css(&source, source_type, CssParserOptions::default());
                assert!(parsed.has_errors());
                assert_eq!(parsed.syntax().to_string(), source);
                let functions: Vec<_> = parsed
                    .syntax()
                    .descendants()
                    .filter_map(CssFunctionAtRule::cast)
                    .collect();
                assert_eq!(functions.len(), 17);
                for function in &functions[..16] {
                    assert_eq!(
                        function.block().unwrap().syntax().text_trimmed(),
                        "{result:red;}"
                    );
                }
                let good = functions.last().unwrap();
                assert_eq!(good.syntax().text_trimmed(), &valid[1..]);
                assert!(!has_bogus_nodes_or_empty_slots(good.syntax()));
            }
        }
    }
}
