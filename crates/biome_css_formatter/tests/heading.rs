use biome_css_formatter::{context::CssFormatOptions, format_node};
use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_syntax::{CssGenericProperty, CssPseudoClassFunctionHeading, CssQualifiedRule};
use biome_languages::CssFileSource;
use biome_rowan::{AstNode, AstSeparatedList};
use biome_test_utils::has_bogus_nodes_or_empty_slots;

#[test]
fn heading_integer_tokens_survive_formatting() {
    let levels = [
        "0001",
        "-0",
        "+0",
        "+7",
        "-12",
        "999999999999999999999999999999",
    ];
    let source = format!(":heading( {} ){{color:red}}", levels.join(","));
    for source_type in [CssFileSource::css(), CssFileSource::scss()] {
        for width in [40, 80] {
            let parsed = parse_css(&source, source_type, CssParserOptions::default());
            assert!(!parsed.has_errors(), "{:?}", parsed.diagnostics());
            assert_eq!(parsed.syntax().to_string(), source);
            assert!(!has_bogus_nodes_or_empty_slots(&parsed.syntax()));
            let heading = parsed
                .syntax()
                .descendants()
                .find_map(CssPseudoClassFunctionHeading::cast)
                .unwrap();
            assert_eq!(heading.levels().len(), levels.len());
            for (level, expected) in heading.levels().iter().zip(levels) {
                assert_eq!(
                    level.unwrap().value_token().unwrap().text_trimmed(),
                    expected
                );
            }

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
            assert!(!has_bogus_nodes_or_empty_slots(&reparsed.syntax()));
            let heading = reparsed
                .syntax()
                .descendants()
                .find_map(CssPseudoClassFunctionHeading::cast)
                .unwrap();
            assert_eq!(heading.levels().len(), levels.len());
            for (level, expected) in heading.levels().iter().zip(levels) {
                assert_eq!(
                    level.unwrap().value_token().unwrap().text_trimmed(),
                    expected
                );
            }
            assert_eq!(
                formatted.as_code(),
                format_node(options, &reparsed.syntax())
                    .unwrap()
                    .print()
                    .unwrap()
                    .as_code()
            );
        }
    }
}

#[test]
fn heading_recovery_preserves_following_declarations_and_rules() {
    for source_type in [CssFileSource::css(), CssFileSource::scss()] {
        for source in [
            ".outer { &:heading(1; width:7px; } .after { color:green; }",
            ".outer { &:heading(1 } .after { width:7px; }",
        ] {
            let parsed = parse_css(source, source_type, CssParserOptions::default());
            assert!(parsed.has_errors());
            assert_eq!(parsed.syntax().to_string(), source);
            let formatted = format_node(CssFormatOptions::new(source_type), &parsed.syntax())
                .unwrap()
                .print()
                .unwrap();
            let reparsed = parse_css(
                formatted.as_code(),
                source_type,
                CssParserOptions::default(),
            );
            assert!(reparsed.has_errors());
            for syntax in [parsed.syntax(), reparsed.syntax()] {
                assert_eq!(
                    syntax
                        .descendants()
                        .filter_map(CssQualifiedRule::cast)
                        .count(),
                    2,
                    "{source}: {syntax:#?}"
                );
                assert!(
                    syntax
                        .descendants()
                        .filter_map(CssGenericProperty::cast)
                        .any(|property| {
                            property.name().unwrap().syntax().text_trimmed() == "width"
                        }),
                    "{source}: {syntax:#?}"
                );
            }
        }
    }
}

#[test]
fn malformed_heading_arguments_remain_invalid() {
    for source_type in [CssFileSource::css(), CssFileSource::scss()] {
        for arguments in [
            "", ",1", "1,", "1,,2", "1 2", "1/**/2", "1.0", "1e0", "1E+0", "+ 1", "- 1", "1px",
            "1%", "odd", "calc(1)", "(1)", "1/2", "1 of h2",
        ] {
            let selector = format!(":heading({arguments})");
            let source = format!("{selector} {{color:red}} .after {{color:green}}");
            let parsed = parse_css(&source, source_type, CssParserOptions::default());
            assert!(parsed.has_errors(), "{source}");
            assert_eq!(parsed.syntax().to_string(), source);
            assert!(
                !parsed
                    .syntax()
                    .descendants()
                    .any(|node| CssPseudoClassFunctionHeading::can_cast(node.kind())),
                "{source}"
            );
            let formatted = format_node(CssFormatOptions::new(source_type), &parsed.syntax())
                .unwrap()
                .print()
                .unwrap();
            assert!(
                formatted.as_code().contains(&selector),
                "{source}: {}",
                formatted.as_code()
            );
            let reparsed = parse_css(
                formatted.as_code(),
                source_type,
                CssParserOptions::default(),
            );
            assert!(reparsed.has_errors(), "{source}: {}", formatted.as_code());
        }
    }
}

#[test]
fn heading_comments_and_suppressions_survive_formatting() {
    let source = ":heading(/* first */ 1 /* before comma */, /* after comma */ 2 /* last */){color:red}\n:heading(\n/* biome-ignore format: keep the level */\n0001,2){color:blue}\n/* biome-ignore format: keep the rule */\n:heading( 1 ,2 ) { color : green; }\n";
    for source_type in [CssFileSource::css(), CssFileSource::scss()] {
        let parsed = parse_css(source, source_type, CssParserOptions::default());
        assert!(!parsed.has_errors(), "{:?}", parsed.diagnostics());
        assert_eq!(parsed.syntax().to_string(), source);
        let options = CssFormatOptions::new(source_type);
        let formatted = format_node(options.clone(), &parsed.syntax())
            .unwrap()
            .print()
            .unwrap();
        for preserved in [
            "/* first */",
            "/* before comma */",
            "/* after comma */",
            "/* last */",
            "/* biome-ignore format: keep the level */",
            "0001",
            ":heading( 1 ,2 ) { color : green; }",
        ] {
            assert!(
                formatted.as_code().contains(preserved),
                "{}",
                formatted.as_code()
            );
        }
        let reparsed = parse_css(
            formatted.as_code(),
            source_type,
            CssParserOptions::default(),
        );
        assert!(!reparsed.has_errors(), "{:?}", reparsed.diagnostics());
        assert!(!has_bogus_nodes_or_empty_slots(&reparsed.syntax()));
        assert_eq!(
            formatted.as_code(),
            format_node(options, &reparsed.syntax())
                .unwrap()
                .print()
                .unwrap()
                .as_code()
        );
    }
}
