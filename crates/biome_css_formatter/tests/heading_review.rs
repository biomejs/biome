use biome_css_formatter::{context::CssFormatOptions, format_node};
use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_syntax::{
    AnyCssRoot, AnyCssSelector, CssMediaAtRule, CssQualifiedRule, CssSyntaxKind, T,
};
use biome_languages::CssFileSource;
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, Direction};
use biome_test_utils::has_bogus_nodes_or_empty_slots;

fn only_selector(root: &AnyCssRoot) -> AnyCssSelector {
    let root = root.as_css_root().unwrap();
    assert_eq!(root.items().len(), 1);
    let item = root.items().iter().next().unwrap();
    let rule = CssQualifiedRule::cast(item.into_syntax()).unwrap();
    assert_eq!(rule.prelude().len(), 1);
    rule.prelude().iter().next().unwrap().unwrap()
}

fn assert_media_boundary(root: &AnyCssRoot) -> CssMediaAtRule {
    let root = root.as_css_root().unwrap();
    assert_eq!(root.items().len(), 2, "{root:#?}");
    let after = CssQualifiedRule::cast(root.items().iter().last().unwrap().into_syntax())
        .expect("the following rule must remain a top-level qualified rule");
    assert_eq!(after.prelude().syntax().text_trimmed(), ".after");
    let media = root
        .syntax()
        .descendants()
        .find_map(CssMediaAtRule::cast)
        .unwrap();
    assert!(
        media.block().unwrap().as_css_rule_block().is_some(),
        "{root:#?}"
    );
    media
}

#[test]
fn heading_review_tight_comments_css() {
    assert_tight_heading_comments(CssFileSource::css());
}

#[test]
fn heading_review_tight_comments_scss() {
    assert_tight_heading_comments(CssFileSource::scss());
}

fn assert_tight_heading_comments(source_type: CssFileSource) {
    for selector in [
        ":heading(1)/**/.foo",
        ":heading(1)/* boundary */.foo",
        ":heading(1)/* first *//* second */.foo",
        ":heading(1)/* line\nbreak */.foo",
        ":heading(1)/**/#target",
        ":heading(1)/**/[hidden]",
        ":heading(1)/**/:hover",
        ":heading(1)/**/::before",
        ".foo/**/:heading(1)",
    ] {
        let source = format!("{selector} {{ color:red; }}");
        let parsed = parse_css(&source, source_type, CssParserOptions::default());
        assert!(!parsed.has_errors(), "{source}: {:?}", parsed.diagnostics());
        assert_eq!(parsed.syntax().to_string(), source);
        let before = only_selector(&parsed.tree());
        let before = before
            .as_css_compound_selector()
            .expect("the input is a compound selector");
        assert_eq!(before.sub_selectors().len(), 2, "{source}");
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
            assert!(
                !reparsed.has_errors(),
                "{source}: {:?}",
                reparsed.diagnostics()
            );
            assert!(!has_bogus_nodes_or_empty_slots(&reparsed.syntax()));
            let after = only_selector(&reparsed.tree());
            assert!(
                after.as_css_compound_selector().is_some(),
                "{source_type:?}: {source} became {}",
                formatted.as_code()
            );
            assert_eq!(
                after
                    .as_css_compound_selector()
                    .unwrap()
                    .sub_selectors()
                    .len(),
                2
            );
            assert!(
                !after
                    .syntax()
                    .descendants_tokens(Direction::Next)
                    .any(|token| token.kind() == CssSyntaxKind::CSS_SPACE_LITERAL),
                "{}",
                formatted.as_code()
            );
            for comment in ["/**/", "/* boundary */", "/* first */", "/* second */"] {
                if selector.contains(comment) {
                    assert_eq!(
                        formatted.as_code().matches(comment).count(),
                        selector.matches(comment).count()
                    );
                }
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
fn heading_review_real_combinators_are_preserved() {
    for source_type in [CssFileSource::css(), CssFileSource::scss()] {
        for (selector, kind) in [
            (":heading(1) .foo", CssSyntaxKind::CSS_SPACE_LITERAL),
            (":heading(1)/**/ .foo", CssSyntaxKind::CSS_SPACE_LITERAL),
            (":heading(1) /**/.foo", CssSyntaxKind::CSS_SPACE_LITERAL),
            (":heading(1)/**/\n.foo", CssSyntaxKind::CSS_SPACE_LITERAL),
            (":heading(1)/**/>.foo", T![>]),
        ] {
            let source = format!("{selector} {{ color:red; }}");
            let parsed = parse_css(&source, source_type, CssParserOptions::default());
            assert!(!parsed.has_errors(), "{source}: {:?}", parsed.diagnostics());
            let before = only_selector(&parsed.tree());
            assert_eq!(
                before
                    .as_css_complex_selector()
                    .unwrap()
                    .combinator()
                    .unwrap()
                    .kind(),
                kind
            );
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
            assert!(
                !reparsed.has_errors(),
                "{source}: {:?}",
                reparsed.diagnostics()
            );
            let after = only_selector(&reparsed.tree());
            assert_eq!(
                after
                    .as_css_complex_selector()
                    .unwrap()
                    .combinator()
                    .unwrap()
                    .kind(),
                kind,
                "{}",
                formatted.as_code()
            );
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
fn heading_review_suppressed_compound_remains_unchanged() {
    let source =
        "/* biome-ignore format: preserve the rule */\n:heading( 1 )/**/.foo { color:red; }\n";
    for source_type in [CssFileSource::css(), CssFileSource::scss()] {
        let parsed = parse_css(source, source_type, CssParserOptions::default());
        assert!(!parsed.has_errors());
        assert!(
            only_selector(&parsed.tree())
                .as_css_compound_selector()
                .is_some()
        );
        let formatted = format_node(CssFormatOptions::new(source_type), &parsed.syntax())
            .unwrap()
            .print()
            .unwrap();
        assert_eq!(formatted.as_code(), source);
    }
}

#[test]
fn heading_review_missing_close_keeps_media_sibling_css() {
    assert_heading_media_recovery(CssFileSource::css());
}

#[test]
fn heading_review_missing_close_keeps_media_sibling_scss() {
    assert_heading_media_recovery(CssFileSource::scss());
}

fn assert_heading_media_recovery(source_type: CssFileSource) {
    for selector in [":heading(1", ":heading(odd", ":heading(", ":heading(1, 2"] {
        let source = format!("@media all {{ {selector} }} .after {{ color:green; }}");
        let parsed = parse_css(&source, source_type, CssParserOptions::default());
        assert!(parsed.has_errors(), "{source}");
        assert_eq!(parsed.syntax().to_string(), source);
        let media = assert_media_boundary(&parsed.tree());
        let first_close = parsed
            .syntax()
            .descendants_tokens(Direction::Next)
            .find(|token| token.kind() == T!['}'])
            .unwrap();
        assert_eq!(
            media
                .block()
                .unwrap()
                .as_css_rule_block()
                .unwrap()
                .r_curly_token()
                .unwrap(),
            first_close
        );
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
        assert!(reparsed.has_errors(), "{source}: {}", formatted.as_code());
        assert_media_boundary(&reparsed.tree());
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

#[test]
fn heading_review_complete_media_rule_is_unchanged() {
    for source_type in [CssFileSource::css(), CssFileSource::scss()] {
        let source = "@media all { :heading(1) { color:red; } } .after { color:green; }";
        let parsed = parse_css(source, source_type, CssParserOptions::default());
        assert!(!parsed.has_errors());
        assert_eq!(parsed.syntax().to_string(), source);
        assert_media_boundary(&parsed.tree());
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
        assert!(!reparsed.has_errors());
        assert_media_boundary(&reparsed.tree());
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

#[test]
fn heading_review_boundary_suppression_keeps_its_child_target() {
    for source_type in [CssFileSource::css(), CssFileSource::scss()] {
        for (selector, expected) in [
            (
                ":HEADING( 1 )/* biome-ignore format: x */:HEADING(2,3)",
                ":HEADING( 1 )/* biome-ignore format: x */:heading(2, 3)",
            ),
            (
                ".foo/* biome-ignore format: x */:HEADING( 1,2 )",
                ".foo/* biome-ignore format: x */:heading(1, 2)",
            ),
        ] {
            let source = format!("{selector} {{ color:red; }}");
            let parsed = parse_css(&source, source_type, CssParserOptions::default());
            assert!(!parsed.has_errors());
            assert!(
                only_selector(&parsed.tree())
                    .as_css_compound_selector()
                    .is_some()
            );
            let options = CssFormatOptions::new(source_type);
            let formatted = format_node(options.clone(), &parsed.syntax())
                .unwrap()
                .print()
                .unwrap();
            assert!(
                formatted.as_code().contains(expected),
                "{}",
                formatted.as_code()
            );
            let reparsed = parse_css(
                formatted.as_code(),
                source_type,
                CssParserOptions::default(),
            );
            assert!(!reparsed.has_errors());
            assert!(
                only_selector(&reparsed.tree())
                    .as_css_compound_selector()
                    .is_some()
            );
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
fn heading_review_colon_comments_css() {
    assert_heading_colon_comments(CssFileSource::css());
}

#[test]
fn heading_review_colon_comments_scss() {
    assert_heading_colon_comments(CssFileSource::scss());
}

fn assert_heading_colon_comments(source_type: CssFileSource) {
    for selector in [
        ":/**/heading(1)",
        ":/* first *//* second */heading(1,2)",
        ":/* line\nbreak */heading(1)",
        r":/**/h\65 ading(1)",
        ":/**/heading(1)/**/.foo",
        ":/* biome-ignore format: keep function */HEADING( 1,2 )",
    ] {
        let source = format!("{selector} {{ color:red; }}");
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
            let name_end = selector.find('(').unwrap();
            assert!(
                formatted.as_code().starts_with(&selector[..=name_end]),
                "{source} became {}",
                formatted.as_code()
            );
            let reparsed = parse_css(
                formatted.as_code(),
                source_type,
                CssParserOptions::default(),
            );
            assert!(
                !reparsed.has_errors(),
                "{source} became {}: {:?}",
                formatted.as_code(),
                reparsed.diagnostics()
            );
            assert!(!has_bogus_nodes_or_empty_slots(&reparsed.syntax()));
            let after = only_selector(&reparsed.tree());
            assert!(after.as_css_compound_selector().is_some());
            assert!(
                !after
                    .syntax()
                    .descendants_tokens(Direction::Next)
                    .any(|token| token.kind() == CssSyntaxKind::CSS_SPACE_LITERAL)
            );
            assert_eq!(
                formatted.as_code().matches("/*").count(),
                selector.matches("/*").count()
            );
            assert!(formatted.as_code().contains("color: red;"));
            if selector.contains("biome-ignore") {
                assert!(
                    formatted.as_code().contains("HEADING( 1,2 )"),
                    "{}",
                    formatted.as_code()
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
fn heading_review_colon_whitespace_is_not_removed() {
    for source_type in [CssFileSource::css(), CssFileSource::scss()] {
        for selector in [":/**/ heading(1)", ": /**/heading(1)", ":/**/\nheading(1)"] {
            let source = format!("{selector} {{ color:red; }}");
            let parsed = parse_css(&source, source_type, CssParserOptions::default());
            assert_eq!(parsed.syntax().to_string(), source);
            let options = CssFormatOptions::new(source_type);
            let formatted = format_node(options.clone(), &parsed.syntax())
                .unwrap()
                .print()
                .unwrap();
            assert!(
                formatted.as_code().starts_with(":/**/ heading(1)"),
                "{source} became {}",
                formatted.as_code()
            );
            let reparsed = parse_css(
                formatted.as_code(),
                source_type,
                CssParserOptions::default(),
            );
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
