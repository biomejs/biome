pub mod builder;
pub mod db;
pub mod model;
pub mod specificity;

use biome_css_syntax::AnyCssRoot;
use biome_rowan::AstNode;
use builder::SemanticModelBuilder;
use model::SemanticModel;

use crate::events::SemanticEventExtractor;

pub fn semantic_model(root: &AnyCssRoot) -> SemanticModel {
    let mut extractor = SemanticEventExtractor::default();
    let mut builder = SemanticModelBuilder::new(root.clone());

    let root = root.syntax();
    for node in root.preorder() {
        match node {
            biome_css_syntax::WalkEvent::Enter(node) => {
                extractor.enter(&node);
            }
            biome_css_syntax::WalkEvent::Leave(node) => extractor.leave(&node),
        }
    }

    while let Some(e) = extractor.pop() {
        builder.push_event(e);
    }

    builder.build()
}

#[cfg(test)]
mod tests {
    use biome_css_parser::{CssParserOptions, parse_css};
    use biome_css_syntax::{
        CssPropertyAtRule,
        property_syntax::{
            PropertySyntax, PropertySyntaxComponentName, PropertySyntaxErrorKind,
            PropertySyntaxResult, PropertySyntaxType,
        },
    };
    use biome_languages::CssFileSource;
    use biome_rowan::{AstNode, TextRange, TextSize};

    #[test]
    fn test_simple_ruleset() {
        let parse = parse_css(
            r#"p {
  font-family: verdana;
  font-size: 20px;
}"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let model = super::semantic_model(&root);
        let rules = model.rules();
        let rule = rules.first().unwrap();

        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(rule.declarations.len(), 2);
        assert_eq!(rule.child_ids.len(), 0);
        assert_eq!(rule.parent_id, None);
    }

    #[test]
    fn test_composes_property_with_multiple_values() {
        let parse = parse_css(
            r#".foo {
  composes: classA from "./a.css", classB from "./b.css";
}"#,
            CssFileSource::new_css_modules(),
            CssParserOptions::default().allow_css_modules(),
        );

        let root = parse.tree();
        let model = super::semantic_model(&root);
        let rules = model.rules();
        let rule = rules.first().unwrap();

        assert_eq!(rule.declarations.len(), 2);
        assert!(rule.declarations[0].value().is_composes());
        assert!(rule.declarations[1].value().is_composes());
    }

    #[test]
    fn test_nested_selector() {
        let parse = parse_css(
            r#".parent {
  color: blue;

  .child {
    color: red;
  }
}"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let model = super::semantic_model(&root);
        let rules = model.rules();
        let rule = rules.first().unwrap();
        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(rule.declarations.len(), 1);
        assert_eq!(rule.child_ids.len(), 1);

        let child_id = rule.child_ids.first().unwrap();
        let child = model.get_rule_by_id(child_id).unwrap();

        assert_eq!(child.selectors.len(), 1);
        assert_eq!(child.declarations.len(), 1);
        assert_eq!(child.child_ids.len(), 0);
        assert_eq!(child.parent_id, Some(rule.id));
    }

    #[test]
    fn test_nested_sub_selector() {
        let parse = parse_css(
            r#"a {
        &:hover {
            color: orange;
        }
}"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let model = super::semantic_model(&root);
        let rules = model.rules();
        let rule = rules.first().unwrap();

        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(rule.declarations.len(), 0);
        assert_eq!(rule.child_ids.len(), 1);

        let child_id = rule.child_ids.first().unwrap();
        let child = model.get_rule_by_id(child_id).unwrap();
        assert_eq!(child.selectors.len(), 1);
        assert_eq!(child.declarations.len(), 1);
        assert_eq!(child.child_ids.len(), 0);
        assert_eq!(child.parent_id, Some(rule.id));
    }

    #[test]
    fn test_nested_at_media() {
        let parse = parse_css(
            r#"a {
        @media {
            color: orange;
        }
}"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let model = super::semantic_model(&root);
        let rules = model.rules();
        let rule = rules.first().unwrap();

        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(rule.declarations.len(), 0);
        assert_eq!(rule.child_ids.len(), 1);

        let child_id = rule.child_ids.first().unwrap();
        let child = model.get_rule_by_id(child_id).unwrap();
        assert_eq!(child.selectors.len(), 0);
        assert_eq!(child.declarations.len(), 1);
        assert_eq!(child.child_ids.len(), 0);
        assert_eq!(child.parent_id, Some(rule.id));
    }

    #[test]
    fn test_global_custom_variables() {
        let parse = parse_css(
            r#"@property --item-size {
  syntax: "<percentage>";
  inherits: true;
  initial-value: 40%;
}

:root {
  --custom-color: red;
  --custom-size: 20px;
}
  "#,
            CssFileSource::css(),
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let model = super::semantic_model(&root);
        let global_custom_variables = model.global_custom_variables();

        assert_eq!(global_custom_variables.len(), 3);

        let item_size = global_custom_variables.contains_key("--item-size");
        let custom_color = global_custom_variables.contains_key("--custom-color");
        let custom_size = global_custom_variables.contains_key("--custom-size");

        assert!(item_size);
        assert!(custom_color);
        assert!(custom_size);
    }

    #[test]
    fn test_scss_global_custom_variables() {
        let parse = parse_css(
            r#":root {
  --literal: $gap;
  --computed: #{$gap};
}"#,
            CssFileSource::scss(),
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let model = super::semantic_model(&root);
        let global_custom_variables = model.global_custom_variables();

        assert_eq!(global_custom_variables.len(), 2);
        assert!(global_custom_variables.contains_key("--literal"));
        assert!(global_custom_variables.contains_key("--computed"));
    }

    #[test]
    fn test_empty_at_property() {
        let parse = parse_css(
            r#"@property --item-size {}"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let model = super::semantic_model(&root);
        let global_custom_variables = model.global_custom_variables();

        assert_eq!(global_custom_variables.len(), 1);

        let item_size = global_custom_variables.contains_key("--item-size");

        assert!(item_size);
    }

    #[test]
    fn test_at_property_semantic_data() {
        let parse = parse_css(
            r#"@property --item-size {
  SYNTAX: "<percentage> | auto";
  InHeRiTs: false;
  INITIAL-VALUE: 40%;
}"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        );

        let model = super::semantic_model(&parse.tree());
        let variable = model
            .global_custom_variables()
            .get("--item-size")
            .expect("expected custom property");
        let at_property = variable.at_property().expect("expected at-property data");

        assert_eq!(at_property.name().text(), "--item-size");
        assert_eq!(at_property.inherits(), Some(false));
        assert!(at_property.initial_value().is_some());
        assert_eq!(
            at_property
                .name_node()
                .value_token()
                .unwrap()
                .text_trimmed(),
            "--item-size"
        );
        let PropertySyntaxResult::Value(PropertySyntax::Components(components)) =
            at_property.syntax()
        else {
            panic!("expected parsed property syntax");
        };
        assert_eq!(components.len(), 2);
        assert_eq!(
            components[0].name,
            PropertySyntaxComponentName::Type(PropertySyntaxType::Percentage)
        );
        assert_eq!(
            components[1].name,
            PropertySyntaxComponentName::CustomIdentifier("auto".into())
        );
    }

    #[test]
    fn test_at_property_syntax_states() {
        let parse = parse_css(
            r#"@property --invalid {
  syntax: "<unknown>";
  inherits: true;
}
@property --missing {
  inherits: true;
}
@property --unquoted {
  syntax: color;
  inherits: yes;
}
@property --invalid-last {
  syntax: "<length>";
  syntax:;
  inherits: true;
}"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let rules = root
            .syntax()
            .descendants()
            .filter_map(CssPropertyAtRule::cast)
            .collect::<Vec<_>>();
        let model = super::semantic_model(&root);
        let variables = model.global_custom_variables();
        let invalid = variables.at_property_by_range(rules[0].range()).unwrap();
        let missing = variables.at_property_by_range(rules[1].range()).unwrap();
        let unquoted = variables.at_property_by_range(rules[2].range()).unwrap();
        let invalid_last = variables.at_property_by_range(rules[3].range()).unwrap();

        let PropertySyntaxResult::Error(diagnostic) = invalid.syntax() else {
            panic!("expected invalid syntax");
        };
        assert_eq!(diagnostic.kind(), PropertySyntaxErrorKind::ExpectedTypeName);
        assert_eq!(missing.syntax(), &PropertySyntaxResult::Missing);
        let PropertySyntaxResult::Error(diagnostic) = unquoted.syntax() else {
            panic!("expected a string diagnostic, got {:#?}", unquoted.syntax());
        };
        assert_eq!(diagnostic.kind(), PropertySyntaxErrorKind::ExpectedString);
        let PropertySyntaxResult::Error(diagnostic) = invalid_last.syntax() else {
            panic!("expected the final syntax descriptor to be invalid");
        };
        assert_eq!(diagnostic.kind(), PropertySyntaxErrorKind::ExpectedString);
        assert_eq!(unquoted.inherits(), None);
    }

    #[test]
    fn test_at_property_decodes_css_string_escapes() {
        let source = r#"@property --escaped {
  syntax: "\3c color\3e ";
  inherits: true;
  initial-value: red;
}
@property --continued {
  syntax: "<col\
or>";
  inherits: true;
  initial-value: red;
}
@property --invalid-escaped {
  syntax: "\3c unknown\3e ";
  inherits: true;
}
@property --empty {
  syntax: "";
  inherits: true;
}
@property --identifier {
  syntax: "\\66 oo";
  inherits: true;
}"#;
        let parse = parse_css(source, CssFileSource::css(), CssParserOptions::default());

        let root = parse.tree();
        let rules = root
            .syntax()
            .descendants()
            .filter_map(CssPropertyAtRule::cast)
            .collect::<Vec<_>>();
        let model = super::semantic_model(&root);
        let variables = model.global_custom_variables();
        for (index, name) in ["--escaped", "--continued"].into_iter().enumerate() {
            let at_property = variables
                .at_property_by_range(rules[index].range())
                .unwrap();
            let PropertySyntaxResult::Value(PropertySyntax::Components(components)) =
                at_property.syntax()
            else {
                panic!("expected a parsed syntax for {name}");
            };
            assert_eq!(
                components[0].name,
                PropertySyntaxComponentName::Type(PropertySyntaxType::Color)
            );
        }

        let escaped = variables.at_property_by_range(rules[0].range()).unwrap();
        let PropertySyntaxResult::Value(PropertySyntax::Components(components)) = escaped.syntax()
        else {
            unreachable!();
        };
        let start = source.find("\\3c color\\3e ").unwrap() as u32;
        let end = start + "\\3c color\\3e ".len() as u32;
        assert_eq!(
            components[0].range,
            TextRange::new(start.into(), end.into())
        );

        let invalid = variables.at_property_by_range(rules[2].range()).unwrap();
        let PropertySyntaxResult::Error(diagnostic) = invalid.syntax() else {
            panic!("expected invalid escaped syntax");
        };
        let start = source.find("\\3c unknown\\3e ").unwrap() as u32;
        let end = start + "\\3c unknown\\3e ".len() as u32;
        assert_eq!(diagnostic.kind(), PropertySyntaxErrorKind::ExpectedTypeName);
        assert_eq!(diagnostic.range(), TextRange::new(start.into(), end.into()));

        let empty = variables.at_property_by_range(rules[3].range()).unwrap();
        let PropertySyntaxResult::Error(diagnostic) = empty.syntax() else {
            panic!("expected empty syntax");
        };
        assert_eq!(diagnostic.kind(), PropertySyntaxErrorKind::Empty);

        let identifier = variables.at_property_by_range(rules[4].range()).unwrap();
        let PropertySyntaxResult::Value(PropertySyntax::Components(components)) =
            identifier.syntax()
        else {
            panic!("expected an escaped custom identifier");
        };
        assert_eq!(
            components[0].name,
            PropertySyntaxComponentName::CustomIdentifier("foo".into())
        );
    }

    #[test]
    fn test_root_declaration_and_at_property_coexist() {
        let parse = parse_css(
            r#"@property --color {
  syntax: "<color>";
  inherits: true;
  initial-value: black;
}
:root {
  --color: red;
}"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        );

        let model = super::semantic_model(&parse.tree());
        let variable = model.global_custom_variables().get("--color").unwrap();

        assert!(variable.is_at_property());
        assert!(variable.is_root());
        assert!(variable.at_property().is_some());
    }

    #[test]
    fn test_last_valid_at_property_rule_wins() {
        let parse = parse_css(
            r#"@property --color {
  syntax: "<color>";
  inherits: true;
  initial-value: black;
}
@property --color {
  syntax: "<unknown>";
  inherits: true;
}"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let rules = root
            .syntax()
            .descendants()
            .filter_map(CssPropertyAtRule::cast)
            .collect::<Vec<_>>();
        let model = super::semantic_model(&root);
        let at_property = model
            .global_custom_variables()
            .get("--color")
            .unwrap()
            .at_property()
            .unwrap();
        let PropertySyntaxResult::Value(PropertySyntax::Components(components)) =
            at_property.syntax()
        else {
            panic!("expected the last valid rule");
        };

        assert_eq!(
            components[0].name,
            PropertySyntaxComponentName::Type(PropertySyntaxType::Color)
        );
        assert_eq!(model.global_custom_variables().at_properties().count(), 1);
        let invalid = model
            .global_custom_variables()
            .at_property_by_range(rules[1].range())
            .unwrap();
        let PropertySyntaxResult::Error(diagnostic) = invalid.syntax() else {
            panic!("expected the authored invalid rule");
        };
        assert_eq!(diagnostic.kind(), PropertySyntaxErrorKind::ExpectedTypeName);
    }

    #[test]
    fn test_at_property_registration_requires_valid_descriptors() {
        let parse = parse_css(
            r#"@property --missing-syntax { inherits: true; }
@property --invalid-syntax { syntax: "<unknown>"; inherits: true; }
@property --missing-inherits { syntax: "*"; }
@property --invalid-inherits { syntax: "*"; inherits: yes; }
@property --missing-initial { syntax: "<length>"; inherits: true; }
@property --universal { syntax: "*"; inherits: false; }"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        );

        let model = super::semantic_model(&parse.tree());
        let variables = model.global_custom_variables();
        for name in [
            "--missing-syntax",
            "--invalid-syntax",
            "--missing-inherits",
            "--invalid-inherits",
            "--missing-initial",
        ] {
            let variable = variables.get(name).unwrap();
            assert!(!variable.is_at_property(), "{name}");
            assert!(variable.at_property().is_none(), "{name}");
        }
        assert!(variables.get("--universal").unwrap().is_at_property());
        assert_eq!(variables.at_properties().count(), 1);
    }

    #[test]
    fn test_mismatched_initial_value_does_not_hide_valid_registration() {
        let parse = parse_css(
            r#"@property --value {
  syntax: "<color>";
  inherits: true;
  initial-value: red;
}
@property --value {
  syntax: "<length>";
  inherits: true;
  initial-value: red;
}"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        );

        let model = super::semantic_model(&parse.tree());
        let property = model
            .global_custom_variables()
            .get("--value")
            .unwrap()
            .at_property()
            .unwrap();
        let PropertySyntaxResult::Value(PropertySyntax::Components(components)) = property.syntax()
        else {
            panic!("expected the valid color registration");
        };
        assert_eq!(
            components[0].name,
            PropertySyntaxComponentName::Type(PropertySyntaxType::Color)
        );
    }

    #[test]
    fn test_context_dependent_initial_value_does_not_hide_valid_registration() {
        let parse = parse_css(
            r#"@property --value {
  syntax: "<length>";
  inherits: true;
  initial-value: 10px;
}
@property --value {
  syntax: "<length>";
  inherits: true;
  initial-value: 3em;
}"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        );

        let model = super::semantic_model(&parse.tree());
        let property = model
            .global_custom_variables()
            .get("--value")
            .unwrap()
            .at_property()
            .unwrap();
        assert!(property.range().end() < TextSize::from(100));
    }

    #[test]
    fn test_effective_at_properties_follow_last_definition_order() {
        let parse = parse_css(
            r#"@property --b { syntax: "<color>"; inherits: true; initial-value: red; }
@property --a { syntax: "<length>"; inherits: true; initial-value: 1px; }
@property --b { syntax: "<number>"; inherits: true; initial-value: 1; }
@property --c { syntax: "<percentage>"; inherits: true; initial-value: 1%; }"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        );

        let model = super::semantic_model(&parse.tree());
        let properties = model
            .global_custom_variables()
            .at_properties()
            .collect::<Vec<_>>();
        let names = properties
            .iter()
            .map(|property| property.name().text())
            .collect::<Vec<_>>();

        assert_eq!(names, ["--a", "--b", "--c"]);
        let PropertySyntaxResult::Value(PropertySyntax::Components(components)) =
            properties[1].syntax()
        else {
            panic!("expected the last --b syntax");
        };
        assert_eq!(
            components[0].name,
            PropertySyntaxComponentName::Type(PropertySyntaxType::Number)
        );
    }

    #[test]
    fn test_get_rule_by_range() {
        let parse = parse_css(
            r#"p {color: red; font-size: 12px;}"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        );
        let root = parse.tree();
        let model = super::semantic_model(&root);

        // range of the declaration 'red'
        let range = TextRange::new(10.into(), 13.into());
        let rule = model.get_rule_by_range(range).unwrap();

        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(rule.declarations.len(), 2);
        assert_eq!(rule.selectors[0].resolved().to_string(), "p");

        let range = TextRange::new(0.into(), 1.into());
        let rule = model.get_rule_by_range(range).unwrap();

        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(rule.declarations.len(), 2);
        assert_eq!(rule.selectors[0].resolved().to_string(), "p");
    }

    #[test]
    fn test_nested_get_rule_by_range() {
        let parse = parse_css(
            r#"p { --foo: red; font-size: 12px;
            .child { color: var(--foo)}
            }"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        );
        let root = parse.tree();
        let model = super::semantic_model(&root);

        // range of the declaration 'blue' in '.child'
        let range = TextRange::new(60.into(), 64.into());
        let rule = model.get_rule_by_range(range).unwrap();

        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(rule.declarations.len(), 1);
        assert_eq!(rule.selectors[0].resolved().to_string(), "p .child");

        let parent = model.get_rule_by_id(&rule.parent_id.unwrap()).unwrap();
        assert_eq!(parent.selectors.len(), 1);
        assert_eq!(parent.declarations.len(), 2);
        assert_eq!(parent.selectors[0].resolved().to_string(), "p");
    }

    #[ignore]
    #[test]
    fn quick_test() {
        let parse = parse_css(
            r#".parent {
  color: blue;

  .child {
    color: red;
  }
}"#,
            CssFileSource::css(),
            CssParserOptions::default(),
        );

        let root = parse.tree();
        let model = super::semantic_model(&root);
        dbg!(&model.rules());
        dbg!(&model.global_custom_variables());
    }
}

#[cfg(test)]
mod specificity_tests {
    use crate::model::{SemanticModel, Specificity};
    use biome_css_parser::{CssParserOptions, parse_css};
    use biome_languages::CssFileSource;

    fn to_semantic_model(source: &str) -> SemanticModel {
        let parse = parse_css(source, CssFileSource::css(), CssParserOptions::default());
        let root = parse.tree();
        super::semantic_model(&root)
    }

    #[test]
    fn selector() {
        let source = "div";
        let model = to_semantic_model(source);

        let mut specificity = model.specificity_of_rules();

        assert_eq!(specificity.next().unwrap(), Specificity(0, 0, 1))
    }

    #[test]
    fn selector_id() {
        let source = "#div";
        let model = to_semantic_model(source);

        let mut specificity = model.specificity_of_rules();

        assert_eq!(specificity.next().unwrap(), Specificity(1, 0, 0))
    }

    #[test]
    fn selector_class() {
        let source = ".div";
        let model = to_semantic_model(source);

        let mut specificity = model.specificity_of_rules();

        assert_eq!(specificity.next().unwrap(), Specificity(0, 1, 0))
    }

    #[test]
    fn selector_combinations() {
        let source = "#div .div {} #div .div div {} .div .div {}";
        let model = to_semantic_model(source);

        let mut specificity = model.specificity_of_rules();

        assert_eq!(
            specificity.next().unwrap(),
            Specificity(1, 1, 0),
            "#div .div"
        );

        assert_eq!(
            specificity.next().unwrap(),
            Specificity(1, 1, 1),
            "#div .div div"
        );
        assert_eq!(
            specificity.next().unwrap(),
            Specificity(0, 2, 0),
            ".div .div"
        );
    }

    #[test]
    fn nested_selector() {
        let source = r#"div {
        & > span {}
    } "#;
        let model = to_semantic_model(source);

        let mut specificity = model.specificity_of_rules();

        assert_eq!(specificity.next().unwrap(), Specificity(0, 0, 1), "div");
        assert_eq!(
            specificity.next().unwrap(),
            Specificity(0, 0, 2),
            "& > span"
        );
    }

    #[test]
    fn nested_selectors_with_media_query() {
        let source = r#"div {
  display: flex;

  & > p {
    justify-content: start;
  }

  @media (orientation: portrait) {
    & > p {
      justify-content: center;
    }
  }
}"#;
        let model = to_semantic_model(source);

        let mut specificity = model.specificity_of_rules();

        assert_eq!(specificity.next().unwrap(), Specificity(0, 0, 1), "div");
        assert_eq!(specificity.next().unwrap(), Specificity(0, 0, 2), "& > p");
        assert_eq!(
            specificity.next().unwrap(),
            Specificity(0, 0, 2),
            "@media (orientation: portrait) & > p"
        );
    }

    #[test]
    fn nested_selectors() {
        let source = r#"
#div {
    .div {
        div {
        }
    }
}"#;
        let model = to_semantic_model(source);

        let specificity = model.specificity_of_rules().collect::<Vec<_>>();

        let mut specificity = specificity.into_iter();

        assert_eq!(specificity.next().unwrap(), Specificity(1, 0, 0), "#div");
        assert_eq!(specificity.next().unwrap(), Specificity(1, 1, 0), ".div");
        assert_eq!(specificity.next().unwrap(), Specificity(1, 1, 1), "div");
    }

    #[test]
    fn nested_selectors_multiple_parents() {
        let source = r#"
#div {
    .div {
        div {
            & > p {}
            & & > p {}
            & & & > p {}
        }
    }
}"#;
        let model = to_semantic_model(source);

        let specificity = model.specificity_of_rules().collect::<Vec<_>>();

        let mut specificity = specificity.into_iter();

        assert_eq!(specificity.next().unwrap(), Specificity(1, 0, 0), "#div");
        assert_eq!(specificity.next().unwrap(), Specificity(1, 1, 0), ".div");
        assert_eq!(specificity.next().unwrap(), Specificity(1, 1, 1), "div");
        assert_eq!(specificity.next().unwrap(), Specificity(1, 1, 2), "& > p");
        assert_eq!(specificity.next().unwrap(), Specificity(1, 1, 1), "& & > p");
        assert_eq!(
            specificity.next().unwrap(),
            Specificity(1, 0, 1),
            "& & & > p"
        );
    }

    #[test]
    fn comma_separated_with_parent() {
        let source = r#"div, span { & > p {} }}"#;
        let model = to_semantic_model(source);

        let specificity = model.specificity_of_rules().collect::<Vec<_>>();

        // The child selector `& > p` is expanded to 2 selectors (one for each parent)
        assert_eq!(specificity.len(), 4);

        let mut specificity = specificity.into_iter();

        assert_eq!(specificity.next().unwrap(), Specificity(0, 0, 1), "div");
        assert_eq!(specificity.next().unwrap(), Specificity(0, 0, 1), "span");
        assert_eq!(specificity.next().unwrap(), Specificity(0, 0, 2), "div > p");
        assert_eq!(
            specificity.next().unwrap(),
            Specificity(0, 0, 2),
            "span > p"
        );
    }

    #[test]
    fn pseudo_selector() {
        let source = r#":is(#fake#fake#fake#fake#fake#fake, *) g {}"#;
        let model = to_semantic_model(source);

        let specificity = model.specificity_of_rules().collect::<Vec<_>>();

        let mut specificity = specificity.into_iter();

        assert_eq!(specificity.next().unwrap(), Specificity(6, 0, 1));
    }
}
