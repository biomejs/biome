use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_syntax::{CssKeyframesItem, CssSyntaxKind, ScssMixinAtRule};
use biome_languages::CssFileSource;
use biome_rowan::AstNode;
use biome_test_utils::{has_bogus_nodes_or_empty_slots, validate_eof_token};

#[test]
fn percentage_template_definition_is_independent_of_invocation() {
    let definition = "@mixin frames { 0% { opacity: 0; } 100% { opacity: 1; } }";
    let uncalled = parse_css(
        definition,
        CssFileSource::scss(),
        CssParserOptions::default(),
    );
    assert!(!uncalled.has_errors(), "{:?}", uncalled.diagnostics());
    assert!(!has_bogus_nodes_or_empty_slots(&uncalled.syntax()));
    assert_eq!(uncalled.syntax().text_with_trivia().to_string(), definition);
    validate_eof_token(uncalled.syntax());
    let invoked = parse_css(
        &format!("{definition} @keyframes fade {{ @include frames; }}"),
        CssFileSource::scss(),
        CssParserOptions::default(),
    );
    let mixin = |root: biome_css_syntax::CssSyntaxNode| {
        root.descendants().find_map(ScssMixinAtRule::cast).unwrap()
    };
    let uncalled_mixin = mixin(uncalled.syntax());
    let invoked_mixin = mixin(invoked.syntax());
    assert_eq!(
        structural_tree(uncalled_mixin.syntax()),
        structural_tree(invoked_mixin.syntax())
    );
    let items: Vec<_> = uncalled_mixin
        .syntax()
        .descendants()
        .filter_map(CssKeyframesItem::cast)
        .collect();
    assert_eq!(items.len(), 2);
    for (item, label) in items.iter().zip(["0%", "100%"]) {
        assert_eq!(item.selectors().syntax().text_trimmed(), label);
        assert!(item.block().is_ok());
        assert_eq!(
            item.syntax().parent().unwrap().kind(),
            CssSyntaxKind::CSS_DECLARATION_OR_RULE_LIST
        );
    }
}

#[test]
fn percentage_templates_do_not_enable_native_css_steps_in_style_rules() {
    let parsed = parse_css(
        ".a { 0% { opacity: 0; } }",
        CssFileSource::css(),
        CssParserOptions::default(),
    );
    assert_eq!(parsed.diagnostics().len(), 1);
    assert!(
        !parsed
            .syntax()
            .descendants()
            .any(|node| node.kind() == CssSyntaxKind::CSS_KEYFRAMES_ITEM)
    );
}

fn structural_tree(node: &biome_css_syntax::CssSyntaxNode) -> String {
    let children = node
        .children_with_tokens()
        .map(|child| match child {
            biome_rowan::NodeOrToken::Node(node) => structural_tree(&node),
            biome_rowan::NodeOrToken::Token(token) => {
                format!("{:?}:{:?}", token.kind(), token.text_trimmed())
            }
        })
        .collect::<Vec<_>>();
    format!("{:?}({})", node.kind(), children.join(","))
}
