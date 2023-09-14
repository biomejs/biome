//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::configuration::linter::*;
use crate::configuration::parse::json::linter::are_recommended_and_all_correct;
use crate::Rules;
use biome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, VisitNode};
use biome_json_syntax::{AnyJsonValue, JsonLanguage};
use biome_rowan::{AstNode, SyntaxNode};
impl VisitNode<JsonLanguage> for Rules {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "all",
                "a11y",
                "complexity",
                "correctness",
                "nursery",
                "performance",
                "security",
                "style",
                "suspicious",
            ],
            diagnostics,
        )
    }
    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "a11y" => {
                let mut visitor = A11y::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                    self.a11y = Some(visitor);
                }
            }
            "complexity" => {
                let mut visitor = Complexity::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                    self.complexity = Some(visitor);
                }
            }
            "correctness" => {
                let mut visitor = Correctness::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                    self.correctness = Some(visitor);
                }
            }
            "nursery" => {
                let mut visitor = Nursery::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                    self.nursery = Some(visitor);
                }
            }
            "performance" => {
                let mut visitor = Performance::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                    self.performance = Some(visitor);
                }
            }
            "security" => {
                let mut visitor = Security::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                    self.security = Some(visitor);
                }
            }
            "style" => {
                let mut visitor = Style::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                    self.style = Some(visitor);
                }
            }
            "suspicious" => {
                let mut visitor = Suspicious::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                    self.suspicious = Some(visitor);
                }
            }
            _ => {}
        }
        Some(())
    }
}
impl VisitNode<JsonLanguage> for A11y {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "all",
                "noAccessKey",
                "noAutofocus",
                "noBlankTarget",
                "noDistractingElements",
                "noHeaderScope",
                "noNoninteractiveElementToInteractiveRole",
                "noPositiveTabindex",
                "noRedundantAlt",
                "noSvgWithoutTitle",
                "useAltText",
                "useAnchorContent",
                "useAriaPropsForRole",
                "useButtonType",
                "useHeadingContent",
                "useHtmlLang",
                "useIframeTitle",
                "useKeyWithClickEvents",
                "useKeyWithMouseEvents",
                "useMediaCaption",
                "useValidAnchor",
                "useValidAriaProps",
                "useValidLang",
            ],
            diagnostics,
        )
    }
    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noAccessKey" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_access_key = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noAccessKey",
                        diagnostics,
                    )?;
                    self.no_access_key = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noAutofocus" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_autofocus = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noAutofocus",
                        diagnostics,
                    )?;
                    self.no_autofocus = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noBlankTarget" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_blank_target = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noBlankTarget",
                        diagnostics,
                    )?;
                    self.no_blank_target = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDistractingElements" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_distracting_elements = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noDistractingElements",
                        diagnostics,
                    )?;
                    self.no_distracting_elements = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noHeaderScope" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_header_scope = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noHeaderScope",
                        diagnostics,
                    )?;
                    self.no_header_scope = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noNoninteractiveElementToInteractiveRole" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_noninteractive_element_to_interactive_role = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noNoninteractiveElementToInteractiveRole",
                        diagnostics,
                    )?;
                    self.no_noninteractive_element_to_interactive_role = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noPositiveTabindex" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_positive_tabindex = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noPositiveTabindex",
                        diagnostics,
                    )?;
                    self.no_positive_tabindex = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noRedundantAlt" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_redundant_alt = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noRedundantAlt",
                        diagnostics,
                    )?;
                    self.no_redundant_alt = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noSvgWithoutTitle" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_svg_without_title = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noSvgWithoutTitle",
                        diagnostics,
                    )?;
                    self.no_svg_without_title = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useAltText" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_alt_text = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useAltText",
                        diagnostics,
                    )?;
                    self.use_alt_text = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useAnchorContent" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_anchor_content = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useAnchorContent",
                        diagnostics,
                    )?;
                    self.use_anchor_content = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useAriaPropsForRole" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_aria_props_for_role = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useAriaPropsForRole",
                        diagnostics,
                    )?;
                    self.use_aria_props_for_role = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useButtonType" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_button_type = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useButtonType",
                        diagnostics,
                    )?;
                    self.use_button_type = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useHeadingContent" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_heading_content = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useHeadingContent",
                        diagnostics,
                    )?;
                    self.use_heading_content = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useHtmlLang" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_html_lang = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useHtmlLang",
                        diagnostics,
                    )?;
                    self.use_html_lang = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useIframeTitle" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_iframe_title = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useIframeTitle",
                        diagnostics,
                    )?;
                    self.use_iframe_title = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useKeyWithClickEvents" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_key_with_click_events = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useKeyWithClickEvents",
                        diagnostics,
                    )?;
                    self.use_key_with_click_events = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useKeyWithMouseEvents" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_key_with_mouse_events = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useKeyWithMouseEvents",
                        diagnostics,
                    )?;
                    self.use_key_with_mouse_events = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useMediaCaption" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_media_caption = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useMediaCaption",
                        diagnostics,
                    )?;
                    self.use_media_caption = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useValidAnchor" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_valid_anchor = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useValidAnchor",
                        diagnostics,
                    )?;
                    self.use_valid_anchor = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useValidAriaProps" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_valid_aria_props = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useValidAriaProps",
                        diagnostics,
                    )?;
                    self.use_valid_aria_props = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useValidLang" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_valid_lang = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useValidLang",
                        diagnostics,
                    )?;
                    self.use_valid_lang = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            _ => {}
        }
        Some(())
    }
}
impl VisitNode<JsonLanguage> for Complexity {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "all",
                "noExtraBooleanCast",
                "noForEach",
                "noMultipleSpacesInRegularExpressionLiterals",
                "noUselessCatch",
                "noUselessConstructor",
                "noUselessFragments",
                "noUselessLabel",
                "noUselessRename",
                "noUselessSwitchCase",
                "noUselessTypeConstraint",
                "noWith",
                "useFlatMap",
                "useLiteralKeys",
                "useOptionalChain",
                "useSimpleNumberKeys",
                "useSimplifiedLogicExpression",
            ],
            diagnostics,
        )
    }
    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noExtraBooleanCast" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_extra_boolean_cast = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noExtraBooleanCast",
                        diagnostics,
                    )?;
                    self.no_extra_boolean_cast = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noForEach" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_for_each = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noForEach",
                        diagnostics,
                    )?;
                    self.no_for_each = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noMultipleSpacesInRegularExpressionLiterals" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_multiple_spaces_in_regular_expression_literals = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noMultipleSpacesInRegularExpressionLiterals",
                        diagnostics,
                    )?;
                    self.no_multiple_spaces_in_regular_expression_literals =
                        Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUselessCatch" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_useless_catch = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUselessCatch",
                        diagnostics,
                    )?;
                    self.no_useless_catch = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUselessConstructor" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_useless_constructor = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUselessConstructor",
                        diagnostics,
                    )?;
                    self.no_useless_constructor = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUselessFragments" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_useless_fragments = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUselessFragments",
                        diagnostics,
                    )?;
                    self.no_useless_fragments = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUselessLabel" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_useless_label = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUselessLabel",
                        diagnostics,
                    )?;
                    self.no_useless_label = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUselessRename" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_useless_rename = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUselessRename",
                        diagnostics,
                    )?;
                    self.no_useless_rename = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUselessSwitchCase" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_useless_switch_case = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUselessSwitchCase",
                        diagnostics,
                    )?;
                    self.no_useless_switch_case = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUselessTypeConstraint" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_useless_type_constraint = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUselessTypeConstraint",
                        diagnostics,
                    )?;
                    self.no_useless_type_constraint = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noWith" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_with = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noWith",
                        diagnostics,
                    )?;
                    self.no_with = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useFlatMap" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_flat_map = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useFlatMap",
                        diagnostics,
                    )?;
                    self.use_flat_map = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useLiteralKeys" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_literal_keys = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useLiteralKeys",
                        diagnostics,
                    )?;
                    self.use_literal_keys = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useOptionalChain" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_optional_chain = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useOptionalChain",
                        diagnostics,
                    )?;
                    self.use_optional_chain = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useSimpleNumberKeys" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_simple_number_keys = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useSimpleNumberKeys",
                        diagnostics,
                    )?;
                    self.use_simple_number_keys = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useSimplifiedLogicExpression" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_simplified_logic_expression = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useSimplifiedLogicExpression",
                        diagnostics,
                    )?;
                    self.use_simplified_logic_expression = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            _ => {}
        }
        Some(())
    }
}
impl VisitNode<JsonLanguage> for Correctness {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "all",
                "noChildrenProp",
                "noConstAssign",
                "noConstructorReturn",
                "noEmptyPattern",
                "noGlobalObjectCalls",
                "noInnerDeclarations",
                "noInvalidConstructorSuper",
                "noNewSymbol",
                "noPrecisionLoss",
                "noRenderReturnValue",
                "noSetterReturn",
                "noStringCaseMismatch",
                "noSwitchDeclarations",
                "noUndeclaredVariables",
                "noUnnecessaryContinue",
                "noUnreachable",
                "noUnreachableSuper",
                "noUnsafeFinally",
                "noUnsafeOptionalChaining",
                "noUnusedLabels",
                "noUnusedVariables",
                "noVoidElementsWithChildren",
                "noVoidTypeReturn",
                "useIsNan",
                "useValidForDirection",
                "useYield",
            ],
            diagnostics,
        )
    }
    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noChildrenProp" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_children_prop = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noChildrenProp",
                        diagnostics,
                    )?;
                    self.no_children_prop = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noConstAssign" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_const_assign = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noConstAssign",
                        diagnostics,
                    )?;
                    self.no_const_assign = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noConstructorReturn" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_constructor_return = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noConstructorReturn",
                        diagnostics,
                    )?;
                    self.no_constructor_return = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noEmptyPattern" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_empty_pattern = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noEmptyPattern",
                        diagnostics,
                    )?;
                    self.no_empty_pattern = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noGlobalObjectCalls" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_global_object_calls = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noGlobalObjectCalls",
                        diagnostics,
                    )?;
                    self.no_global_object_calls = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noInnerDeclarations" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_inner_declarations = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noInnerDeclarations",
                        diagnostics,
                    )?;
                    self.no_inner_declarations = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noInvalidConstructorSuper" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_invalid_constructor_super = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noInvalidConstructorSuper",
                        diagnostics,
                    )?;
                    self.no_invalid_constructor_super = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noNewSymbol" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_new_symbol = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noNewSymbol",
                        diagnostics,
                    )?;
                    self.no_new_symbol = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noPrecisionLoss" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_precision_loss = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noPrecisionLoss",
                        diagnostics,
                    )?;
                    self.no_precision_loss = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noRenderReturnValue" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_render_return_value = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noRenderReturnValue",
                        diagnostics,
                    )?;
                    self.no_render_return_value = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noSetterReturn" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_setter_return = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noSetterReturn",
                        diagnostics,
                    )?;
                    self.no_setter_return = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noStringCaseMismatch" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_string_case_mismatch = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noStringCaseMismatch",
                        diagnostics,
                    )?;
                    self.no_string_case_mismatch = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noSwitchDeclarations" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_switch_declarations = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noSwitchDeclarations",
                        diagnostics,
                    )?;
                    self.no_switch_declarations = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUndeclaredVariables" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_undeclared_variables = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUndeclaredVariables",
                        diagnostics,
                    )?;
                    self.no_undeclared_variables = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnnecessaryContinue" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unnecessary_continue = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUnnecessaryContinue",
                        diagnostics,
                    )?;
                    self.no_unnecessary_continue = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnreachable" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unreachable = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUnreachable",
                        diagnostics,
                    )?;
                    self.no_unreachable = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnreachableSuper" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unreachable_super = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUnreachableSuper",
                        diagnostics,
                    )?;
                    self.no_unreachable_super = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnsafeFinally" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unsafe_finally = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUnsafeFinally",
                        diagnostics,
                    )?;
                    self.no_unsafe_finally = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnsafeOptionalChaining" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unsafe_optional_chaining = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUnsafeOptionalChaining",
                        diagnostics,
                    )?;
                    self.no_unsafe_optional_chaining = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnusedLabels" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unused_labels = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUnusedLabels",
                        diagnostics,
                    )?;
                    self.no_unused_labels = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnusedVariables" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unused_variables = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUnusedVariables",
                        diagnostics,
                    )?;
                    self.no_unused_variables = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noVoidElementsWithChildren" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_void_elements_with_children = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noVoidElementsWithChildren",
                        diagnostics,
                    )?;
                    self.no_void_elements_with_children = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noVoidTypeReturn" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_void_type_return = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noVoidTypeReturn",
                        diagnostics,
                    )?;
                    self.no_void_type_return = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useIsNan" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_is_nan = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useIsNan",
                        diagnostics,
                    )?;
                    self.use_is_nan = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useValidForDirection" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_valid_for_direction = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useValidForDirection",
                        diagnostics,
                    )?;
                    self.use_valid_for_direction = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useYield" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_yield = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useYield",
                        diagnostics,
                    )?;
                    self.use_yield = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            _ => {}
        }
        Some(())
    }
}
impl VisitNode<JsonLanguage> for Nursery {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "all",
                "noAccumulatingSpread",
                "noAriaUnsupportedElements",
                "noBannedTypes",
                "noConfusingVoidType",
                "noConstantCondition",
                "noControlCharactersInRegex",
                "noDuplicateJsonKeys",
                "noExcessiveComplexity",
                "noFallthroughSwitchClause",
                "noGlobalIsFinite",
                "noGlobalIsNan",
                "noNoninteractiveTabindex",
                "noNonoctalDecimalEscape",
                "noRedundantRoles",
                "noSelfAssign",
                "noStaticOnlyClass",
                "noUnsafeDeclarationMerging",
                "noUselessEmptyExport",
                "noUselessThisAlias",
                "noVoid",
                "useAriaPropTypes",
                "useArrowFunction",
                "useCollapsedElseIf",
                "useExhaustiveDependencies",
                "useGetterReturn",
                "useGroupedTypeImport",
                "useHookAtTopLevel",
                "useImportRestrictions",
                "useIsArray",
                "useLiteralEnumMembers",
                "useNamingConvention",
            ],
            diagnostics,
        )
    }
    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noAccumulatingSpread" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_accumulating_spread = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noAccumulatingSpread",
                        diagnostics,
                    )?;
                    self.no_accumulating_spread = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noAriaUnsupportedElements" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_aria_unsupported_elements = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noAriaUnsupportedElements",
                        diagnostics,
                    )?;
                    self.no_aria_unsupported_elements = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noBannedTypes" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_banned_types = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noBannedTypes",
                        diagnostics,
                    )?;
                    self.no_banned_types = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noConfusingVoidType" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_confusing_void_type = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noConfusingVoidType",
                        diagnostics,
                    )?;
                    self.no_confusing_void_type = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noConstantCondition" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_constant_condition = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noConstantCondition",
                        diagnostics,
                    )?;
                    self.no_constant_condition = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noControlCharactersInRegex" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_control_characters_in_regex = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noControlCharactersInRegex",
                        diagnostics,
                    )?;
                    self.no_control_characters_in_regex = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDuplicateJsonKeys" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_duplicate_json_keys = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noDuplicateJsonKeys",
                        diagnostics,
                    )?;
                    self.no_duplicate_json_keys = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noExcessiveComplexity" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_excessive_complexity = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noExcessiveComplexity",
                        diagnostics,
                    )?;
                    self.no_excessive_complexity = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noFallthroughSwitchClause" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_fallthrough_switch_clause = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noFallthroughSwitchClause",
                        diagnostics,
                    )?;
                    self.no_fallthrough_switch_clause = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noGlobalIsFinite" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_global_is_finite = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noGlobalIsFinite",
                        diagnostics,
                    )?;
                    self.no_global_is_finite = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noGlobalIsNan" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_global_is_nan = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noGlobalIsNan",
                        diagnostics,
                    )?;
                    self.no_global_is_nan = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noNoninteractiveTabindex" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_noninteractive_tabindex = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noNoninteractiveTabindex",
                        diagnostics,
                    )?;
                    self.no_noninteractive_tabindex = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noNonoctalDecimalEscape" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_nonoctal_decimal_escape = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noNonoctalDecimalEscape",
                        diagnostics,
                    )?;
                    self.no_nonoctal_decimal_escape = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noRedundantRoles" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_redundant_roles = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noRedundantRoles",
                        diagnostics,
                    )?;
                    self.no_redundant_roles = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noSelfAssign" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_self_assign = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noSelfAssign",
                        diagnostics,
                    )?;
                    self.no_self_assign = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noStaticOnlyClass" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_static_only_class = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noStaticOnlyClass",
                        diagnostics,
                    )?;
                    self.no_static_only_class = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnsafeDeclarationMerging" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unsafe_declaration_merging = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUnsafeDeclarationMerging",
                        diagnostics,
                    )?;
                    self.no_unsafe_declaration_merging = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUselessEmptyExport" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_useless_empty_export = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUselessEmptyExport",
                        diagnostics,
                    )?;
                    self.no_useless_empty_export = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUselessThisAlias" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_useless_this_alias = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUselessThisAlias",
                        diagnostics,
                    )?;
                    self.no_useless_this_alias = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noVoid" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_void = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noVoid",
                        diagnostics,
                    )?;
                    self.no_void = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useAriaPropTypes" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_aria_prop_types = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useAriaPropTypes",
                        diagnostics,
                    )?;
                    self.use_aria_prop_types = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useArrowFunction" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_arrow_function = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useArrowFunction",
                        diagnostics,
                    )?;
                    self.use_arrow_function = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useCollapsedElseIf" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_collapsed_else_if = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useCollapsedElseIf",
                        diagnostics,
                    )?;
                    self.use_collapsed_else_if = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useExhaustiveDependencies" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_exhaustive_dependencies = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useExhaustiveDependencies",
                        diagnostics,
                    )?;
                    self.use_exhaustive_dependencies = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useGetterReturn" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_getter_return = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useGetterReturn",
                        diagnostics,
                    )?;
                    self.use_getter_return = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useGroupedTypeImport" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_grouped_type_import = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useGroupedTypeImport",
                        diagnostics,
                    )?;
                    self.use_grouped_type_import = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useHookAtTopLevel" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_hook_at_top_level = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useHookAtTopLevel",
                        diagnostics,
                    )?;
                    self.use_hook_at_top_level = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useImportRestrictions" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_import_restrictions = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useImportRestrictions",
                        diagnostics,
                    )?;
                    self.use_import_restrictions = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useIsArray" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_is_array = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useIsArray",
                        diagnostics,
                    )?;
                    self.use_is_array = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useLiteralEnumMembers" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_literal_enum_members = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useLiteralEnumMembers",
                        diagnostics,
                    )?;
                    self.use_literal_enum_members = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useNamingConvention" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_naming_convention = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useNamingConvention",
                        diagnostics,
                    )?;
                    self.use_naming_convention = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            _ => {}
        }
        Some(())
    }
}
impl VisitNode<JsonLanguage> for Performance {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, &["recommended", "all", "noDelete"], diagnostics)
    }
    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noDelete" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_delete = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noDelete",
                        diagnostics,
                    )?;
                    self.no_delete = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            _ => {}
        }
        Some(())
    }
}
impl VisitNode<JsonLanguage> for Security {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "all",
                "noDangerouslySetInnerHtml",
                "noDangerouslySetInnerHtmlWithChildren",
            ],
            diagnostics,
        )
    }
    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noDangerouslySetInnerHtml" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_dangerously_set_inner_html = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noDangerouslySetInnerHtml",
                        diagnostics,
                    )?;
                    self.no_dangerously_set_inner_html = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDangerouslySetInnerHtmlWithChildren" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_dangerously_set_inner_html_with_children = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noDangerouslySetInnerHtmlWithChildren",
                        diagnostics,
                    )?;
                    self.no_dangerously_set_inner_html_with_children = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            _ => {}
        }
        Some(())
    }
}
impl VisitNode<JsonLanguage> for Style {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "all",
                "noArguments",
                "noCommaOperator",
                "noImplicitBoolean",
                "noInferrableTypes",
                "noNamespace",
                "noNegationElse",
                "noNonNullAssertion",
                "noParameterAssign",
                "noParameterProperties",
                "noRestrictedGlobals",
                "noShoutyConstants",
                "noUnusedTemplateLiteral",
                "noVar",
                "useBlockStatements",
                "useConst",
                "useDefaultParameterLast",
                "useEnumInitializers",
                "useExponentiationOperator",
                "useFragmentSyntax",
                "useNumericLiterals",
                "useSelfClosingElements",
                "useShorthandArrayType",
                "useSingleCaseStatement",
                "useSingleVarDeclarator",
                "useTemplate",
                "useWhile",
            ],
            diagnostics,
        )
    }
    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noArguments" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_arguments = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noArguments",
                        diagnostics,
                    )?;
                    self.no_arguments = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noCommaOperator" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_comma_operator = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noCommaOperator",
                        diagnostics,
                    )?;
                    self.no_comma_operator = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noImplicitBoolean" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_implicit_boolean = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noImplicitBoolean",
                        diagnostics,
                    )?;
                    self.no_implicit_boolean = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noInferrableTypes" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_inferrable_types = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noInferrableTypes",
                        diagnostics,
                    )?;
                    self.no_inferrable_types = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noNamespace" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_namespace = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noNamespace",
                        diagnostics,
                    )?;
                    self.no_namespace = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noNegationElse" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_negation_else = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noNegationElse",
                        diagnostics,
                    )?;
                    self.no_negation_else = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noNonNullAssertion" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_non_null_assertion = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noNonNullAssertion",
                        diagnostics,
                    )?;
                    self.no_non_null_assertion = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noParameterAssign" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_parameter_assign = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noParameterAssign",
                        diagnostics,
                    )?;
                    self.no_parameter_assign = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noParameterProperties" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_parameter_properties = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noParameterProperties",
                        diagnostics,
                    )?;
                    self.no_parameter_properties = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noRestrictedGlobals" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_restricted_globals = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noRestrictedGlobals",
                        diagnostics,
                    )?;
                    self.no_restricted_globals = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noShoutyConstants" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_shouty_constants = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noShoutyConstants",
                        diagnostics,
                    )?;
                    self.no_shouty_constants = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnusedTemplateLiteral" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unused_template_literal = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUnusedTemplateLiteral",
                        diagnostics,
                    )?;
                    self.no_unused_template_literal = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noVar" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_var = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noVar",
                        diagnostics,
                    )?;
                    self.no_var = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useBlockStatements" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_block_statements = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useBlockStatements",
                        diagnostics,
                    )?;
                    self.use_block_statements = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useConst" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_const = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useConst",
                        diagnostics,
                    )?;
                    self.use_const = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useDefaultParameterLast" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_default_parameter_last = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useDefaultParameterLast",
                        diagnostics,
                    )?;
                    self.use_default_parameter_last = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useEnumInitializers" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_enum_initializers = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useEnumInitializers",
                        diagnostics,
                    )?;
                    self.use_enum_initializers = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useExponentiationOperator" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_exponentiation_operator = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useExponentiationOperator",
                        diagnostics,
                    )?;
                    self.use_exponentiation_operator = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useFragmentSyntax" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_fragment_syntax = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useFragmentSyntax",
                        diagnostics,
                    )?;
                    self.use_fragment_syntax = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useNumericLiterals" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_numeric_literals = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useNumericLiterals",
                        diagnostics,
                    )?;
                    self.use_numeric_literals = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useSelfClosingElements" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_self_closing_elements = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useSelfClosingElements",
                        diagnostics,
                    )?;
                    self.use_self_closing_elements = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useShorthandArrayType" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_shorthand_array_type = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useShorthandArrayType",
                        diagnostics,
                    )?;
                    self.use_shorthand_array_type = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useSingleCaseStatement" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_single_case_statement = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useSingleCaseStatement",
                        diagnostics,
                    )?;
                    self.use_single_case_statement = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useSingleVarDeclarator" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_single_var_declarator = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useSingleVarDeclarator",
                        diagnostics,
                    )?;
                    self.use_single_var_declarator = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useTemplate" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_template = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useTemplate",
                        diagnostics,
                    )?;
                    self.use_template = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useWhile" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_while = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useWhile",
                        diagnostics,
                    )?;
                    self.use_while = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            _ => {}
        }
        Some(())
    }
}
impl VisitNode<JsonLanguage> for Suspicious {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "all",
                "noArrayIndexKey",
                "noAssignInExpressions",
                "noAsyncPromiseExecutor",
                "noCatchAssign",
                "noClassAssign",
                "noCommentText",
                "noCompareNegZero",
                "noConfusingLabels",
                "noConsoleLog",
                "noConstEnum",
                "noDebugger",
                "noDoubleEquals",
                "noDuplicateCase",
                "noDuplicateClassMembers",
                "noDuplicateJsxProps",
                "noDuplicateObjectKeys",
                "noDuplicateParameters",
                "noEmptyInterface",
                "noExplicitAny",
                "noExtraNonNullAssertion",
                "noFunctionAssign",
                "noImportAssign",
                "noLabelVar",
                "noPrototypeBuiltins",
                "noRedeclare",
                "noRedundantUseStrict",
                "noSelfCompare",
                "noShadowRestrictedNames",
                "noSparseArray",
                "noUnsafeNegation",
                "useDefaultSwitchClauseLast",
                "useNamespaceKeyword",
                "useValidTypeof",
            ],
            diagnostics,
        )
    }
    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noArrayIndexKey" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_array_index_key = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noArrayIndexKey",
                        diagnostics,
                    )?;
                    self.no_array_index_key = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noAssignInExpressions" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_assign_in_expressions = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noAssignInExpressions",
                        diagnostics,
                    )?;
                    self.no_assign_in_expressions = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noAsyncPromiseExecutor" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_async_promise_executor = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noAsyncPromiseExecutor",
                        diagnostics,
                    )?;
                    self.no_async_promise_executor = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noCatchAssign" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_catch_assign = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noCatchAssign",
                        diagnostics,
                    )?;
                    self.no_catch_assign = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noClassAssign" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_class_assign = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noClassAssign",
                        diagnostics,
                    )?;
                    self.no_class_assign = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noCommentText" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_comment_text = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noCommentText",
                        diagnostics,
                    )?;
                    self.no_comment_text = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noCompareNegZero" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_compare_neg_zero = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noCompareNegZero",
                        diagnostics,
                    )?;
                    self.no_compare_neg_zero = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noConfusingLabels" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_confusing_labels = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noConfusingLabels",
                        diagnostics,
                    )?;
                    self.no_confusing_labels = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noConsoleLog" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_console_log = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noConsoleLog",
                        diagnostics,
                    )?;
                    self.no_console_log = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noConstEnum" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_const_enum = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noConstEnum",
                        diagnostics,
                    )?;
                    self.no_const_enum = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDebugger" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_debugger = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noDebugger",
                        diagnostics,
                    )?;
                    self.no_debugger = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDoubleEquals" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_double_equals = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noDoubleEquals",
                        diagnostics,
                    )?;
                    self.no_double_equals = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDuplicateCase" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_duplicate_case = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noDuplicateCase",
                        diagnostics,
                    )?;
                    self.no_duplicate_case = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDuplicateClassMembers" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_duplicate_class_members = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noDuplicateClassMembers",
                        diagnostics,
                    )?;
                    self.no_duplicate_class_members = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDuplicateJsxProps" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_duplicate_jsx_props = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noDuplicateJsxProps",
                        diagnostics,
                    )?;
                    self.no_duplicate_jsx_props = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDuplicateObjectKeys" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_duplicate_object_keys = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noDuplicateObjectKeys",
                        diagnostics,
                    )?;
                    self.no_duplicate_object_keys = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDuplicateParameters" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_duplicate_parameters = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noDuplicateParameters",
                        diagnostics,
                    )?;
                    self.no_duplicate_parameters = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noEmptyInterface" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_empty_interface = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noEmptyInterface",
                        diagnostics,
                    )?;
                    self.no_empty_interface = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noExplicitAny" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_explicit_any = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noExplicitAny",
                        diagnostics,
                    )?;
                    self.no_explicit_any = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noExtraNonNullAssertion" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_extra_non_null_assertion = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noExtraNonNullAssertion",
                        diagnostics,
                    )?;
                    self.no_extra_non_null_assertion = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noFunctionAssign" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_function_assign = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noFunctionAssign",
                        diagnostics,
                    )?;
                    self.no_function_assign = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noImportAssign" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_import_assign = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noImportAssign",
                        diagnostics,
                    )?;
                    self.no_import_assign = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noLabelVar" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_label_var = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noLabelVar",
                        diagnostics,
                    )?;
                    self.no_label_var = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noPrototypeBuiltins" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_prototype_builtins = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noPrototypeBuiltins",
                        diagnostics,
                    )?;
                    self.no_prototype_builtins = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noRedeclare" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_redeclare = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noRedeclare",
                        diagnostics,
                    )?;
                    self.no_redeclare = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noRedundantUseStrict" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_redundant_use_strict = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noRedundantUseStrict",
                        diagnostics,
                    )?;
                    self.no_redundant_use_strict = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noSelfCompare" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_self_compare = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noSelfCompare",
                        diagnostics,
                    )?;
                    self.no_self_compare = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noShadowRestrictedNames" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_shadow_restricted_names = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noShadowRestrictedNames",
                        diagnostics,
                    )?;
                    self.no_shadow_restricted_names = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noSparseArray" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_sparse_array = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noSparseArray",
                        diagnostics,
                    )?;
                    self.no_sparse_array = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnsafeNegation" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unsafe_negation = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "noUnsafeNegation",
                        diagnostics,
                    )?;
                    self.no_unsafe_negation = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useDefaultSwitchClauseLast" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_default_switch_clause_last = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useDefaultSwitchClauseLast",
                        diagnostics,
                    )?;
                    self.use_default_switch_clause_last = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useNamespaceKeyword" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_namespace_keyword = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useNamespaceKeyword",
                        diagnostics,
                    )?;
                    self.use_namespace_keyword = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useValidTypeof" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_valid_typeof = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(
                        &value,
                        name_text,
                        "useValidTypeof",
                        diagnostics,
                    )?;
                    self.use_valid_typeof = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            _ => {}
        }
        Some(())
    }
}
