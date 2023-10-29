//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::configuration::linter::*;
use crate::configuration::parse::json::linter::are_recommended_and_all_correct;
use crate::Rules;
use biome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, VisitNode};
use biome_json_syntax::JsonLanguage;
use biome_rowan::SyntaxNode;
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
                    visitor.map_to_object(&value, name_text, diagnostics)?;
                    self.a11y = Some(visitor);
                }
            }
            "complexity" => {
                let mut visitor = Complexity::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    visitor.map_to_object(&value, name_text, diagnostics)?;
                    self.complexity = Some(visitor);
                }
            }
            "correctness" => {
                let mut visitor = Correctness::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    visitor.map_to_object(&value, name_text, diagnostics)?;
                    self.correctness = Some(visitor);
                }
            }
            "nursery" => {
                let mut visitor = Nursery::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    visitor.map_to_object(&value, name_text, diagnostics)?;
                    self.nursery = Some(visitor);
                }
            }
            "performance" => {
                let mut visitor = Performance::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    visitor.map_to_object(&value, name_text, diagnostics)?;
                    self.performance = Some(visitor);
                }
            }
            "security" => {
                let mut visitor = Security::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    visitor.map_to_object(&value, name_text, diagnostics)?;
                    self.security = Some(visitor);
                }
            }
            "style" => {
                let mut visitor = Style::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    visitor.map_to_object(&value, name_text, diagnostics)?;
                    self.style = Some(visitor);
                }
            }
            "suspicious" => {
                let mut visitor = Suspicious::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    visitor.map_to_object(&value, name_text, diagnostics)?;
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
                "noAriaUnsupportedElements",
                "noAutofocus",
                "noBlankTarget",
                "noDistractingElements",
                "noHeaderScope",
                "noNoninteractiveElementToInteractiveRole",
                "noNoninteractiveTabindex",
                "noPositiveTabindex",
                "noRedundantAlt",
                "noRedundantRoles",
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
                "useValidAriaValues",
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
            "noAccessKey" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noAccessKey", diagnostics)?;
                self.no_access_key = Some(configuration);
            }
            "noAriaUnsupportedElements" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noAriaUnsupportedElements",
                    diagnostics,
                )?;
                self.no_aria_unsupported_elements = Some(configuration);
            }
            "noAutofocus" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noAutofocus", diagnostics)?;
                self.no_autofocus = Some(configuration);
            }
            "noBlankTarget" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noBlankTarget", diagnostics)?;
                self.no_blank_target = Some(configuration);
            }
            "noDistractingElements" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noDistractingElements",
                    diagnostics,
                )?;
                self.no_distracting_elements = Some(configuration);
            }
            "noHeaderScope" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noHeaderScope", diagnostics)?;
                self.no_header_scope = Some(configuration);
            }
            "noNoninteractiveElementToInteractiveRole" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noNoninteractiveElementToInteractiveRole",
                    diagnostics,
                )?;
                self.no_noninteractive_element_to_interactive_role = Some(configuration);
            }
            "noNoninteractiveTabindex" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noNoninteractiveTabindex",
                    diagnostics,
                )?;
                self.no_noninteractive_tabindex = Some(configuration);
            }
            "noPositiveTabindex" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noPositiveTabindex", diagnostics)?;
                self.no_positive_tabindex = Some(configuration);
            }
            "noRedundantAlt" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noRedundantAlt", diagnostics)?;
                self.no_redundant_alt = Some(configuration);
            }
            "noRedundantRoles" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noRedundantRoles", diagnostics)?;
                self.no_redundant_roles = Some(configuration);
            }
            "noSvgWithoutTitle" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noSvgWithoutTitle", diagnostics)?;
                self.no_svg_without_title = Some(configuration);
            }
            "useAltText" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useAltText", diagnostics)?;
                self.use_alt_text = Some(configuration);
            }
            "useAnchorContent" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useAnchorContent", diagnostics)?;
                self.use_anchor_content = Some(configuration);
            }
            "useAriaPropsForRole" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useAriaPropsForRole", diagnostics)?;
                self.use_aria_props_for_role = Some(configuration);
            }
            "useButtonType" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useButtonType", diagnostics)?;
                self.use_button_type = Some(configuration);
            }
            "useHeadingContent" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useHeadingContent", diagnostics)?;
                self.use_heading_content = Some(configuration);
            }
            "useHtmlLang" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useHtmlLang", diagnostics)?;
                self.use_html_lang = Some(configuration);
            }
            "useIframeTitle" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useIframeTitle", diagnostics)?;
                self.use_iframe_title = Some(configuration);
            }
            "useKeyWithClickEvents" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "useKeyWithClickEvents",
                    diagnostics,
                )?;
                self.use_key_with_click_events = Some(configuration);
            }
            "useKeyWithMouseEvents" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "useKeyWithMouseEvents",
                    diagnostics,
                )?;
                self.use_key_with_mouse_events = Some(configuration);
            }
            "useMediaCaption" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useMediaCaption", diagnostics)?;
                self.use_media_caption = Some(configuration);
            }
            "useValidAnchor" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useValidAnchor", diagnostics)?;
                self.use_valid_anchor = Some(configuration);
            }
            "useValidAriaProps" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useValidAriaProps", diagnostics)?;
                self.use_valid_aria_props = Some(configuration);
            }
            "useValidAriaValues" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useValidAriaValues", diagnostics)?;
                self.use_valid_aria_values = Some(configuration);
            }
            "useValidLang" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useValidLang", diagnostics)?;
                self.use_valid_lang = Some(configuration);
            }
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
                "noBannedTypes",
                "noExcessiveCognitiveComplexity",
                "noExtraBooleanCast",
                "noForEach",
                "noMultipleSpacesInRegularExpressionLiterals",
                "noStaticOnlyClass",
                "noUselessCatch",
                "noUselessConstructor",
                "noUselessEmptyExport",
                "noUselessFragments",
                "noUselessLabel",
                "noUselessRename",
                "noUselessSwitchCase",
                "noUselessThisAlias",
                "noUselessTypeConstraint",
                "noVoid",
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
            "noBannedTypes" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noBannedTypes", diagnostics)?;
                self.no_banned_types = Some(configuration);
            }
            "noExcessiveCognitiveComplexity" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noExcessiveCognitiveComplexity",
                    diagnostics,
                )?;
                self.no_excessive_cognitive_complexity = Some(configuration);
            }
            "noExtraBooleanCast" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noExtraBooleanCast", diagnostics)?;
                self.no_extra_boolean_cast = Some(configuration);
            }
            "noForEach" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noForEach", diagnostics)?;
                self.no_for_each = Some(configuration);
            }
            "noMultipleSpacesInRegularExpressionLiterals" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noMultipleSpacesInRegularExpressionLiterals",
                    diagnostics,
                )?;
                self.no_multiple_spaces_in_regular_expression_literals = Some(configuration);
            }
            "noStaticOnlyClass" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noStaticOnlyClass", diagnostics)?;
                self.no_static_only_class = Some(configuration);
            }
            "noUselessCatch" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noUselessCatch", diagnostics)?;
                self.no_useless_catch = Some(configuration);
            }
            "noUselessConstructor" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noUselessConstructor",
                    diagnostics,
                )?;
                self.no_useless_constructor = Some(configuration);
            }
            "noUselessEmptyExport" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noUselessEmptyExport",
                    diagnostics,
                )?;
                self.no_useless_empty_export = Some(configuration);
            }
            "noUselessFragments" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noUselessFragments", diagnostics)?;
                self.no_useless_fragments = Some(configuration);
            }
            "noUselessLabel" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noUselessLabel", diagnostics)?;
                self.no_useless_label = Some(configuration);
            }
            "noUselessRename" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noUselessRename", diagnostics)?;
                self.no_useless_rename = Some(configuration);
            }
            "noUselessSwitchCase" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noUselessSwitchCase", diagnostics)?;
                self.no_useless_switch_case = Some(configuration);
            }
            "noUselessThisAlias" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noUselessThisAlias", diagnostics)?;
                self.no_useless_this_alias = Some(configuration);
            }
            "noUselessTypeConstraint" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noUselessTypeConstraint",
                    diagnostics,
                )?;
                self.no_useless_type_constraint = Some(configuration);
            }
            "noVoid" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noVoid", diagnostics)?;
                self.no_void = Some(configuration);
            }
            "noWith" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noWith", diagnostics)?;
                self.no_with = Some(configuration);
            }
            "useFlatMap" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useFlatMap", diagnostics)?;
                self.use_flat_map = Some(configuration);
            }
            "useLiteralKeys" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useLiteralKeys", diagnostics)?;
                self.use_literal_keys = Some(configuration);
            }
            "useOptionalChain" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useOptionalChain", diagnostics)?;
                self.use_optional_chain = Some(configuration);
            }
            "useSimpleNumberKeys" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useSimpleNumberKeys", diagnostics)?;
                self.use_simple_number_keys = Some(configuration);
            }
            "useSimplifiedLogicExpression" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "useSimplifiedLogicExpression",
                    diagnostics,
                )?;
                self.use_simplified_logic_expression = Some(configuration);
            }
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
                "noConstantCondition",
                "noConstructorReturn",
                "noEmptyPattern",
                "noGlobalObjectCalls",
                "noInnerDeclarations",
                "noInvalidConstructorSuper",
                "noNewSymbol",
                "noNonoctalDecimalEscape",
                "noPrecisionLoss",
                "noRenderReturnValue",
                "noSelfAssign",
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
                "useExhaustiveDependencies",
                "useHookAtTopLevel",
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
            "noChildrenProp" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noChildrenProp", diagnostics)?;
                self.no_children_prop = Some(configuration);
            }
            "noConstAssign" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noConstAssign", diagnostics)?;
                self.no_const_assign = Some(configuration);
            }
            "noConstantCondition" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noConstantCondition", diagnostics)?;
                self.no_constant_condition = Some(configuration);
            }
            "noConstructorReturn" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noConstructorReturn", diagnostics)?;
                self.no_constructor_return = Some(configuration);
            }
            "noEmptyPattern" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noEmptyPattern", diagnostics)?;
                self.no_empty_pattern = Some(configuration);
            }
            "noGlobalObjectCalls" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noGlobalObjectCalls", diagnostics)?;
                self.no_global_object_calls = Some(configuration);
            }
            "noInnerDeclarations" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noInnerDeclarations", diagnostics)?;
                self.no_inner_declarations = Some(configuration);
            }
            "noInvalidConstructorSuper" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noInvalidConstructorSuper",
                    diagnostics,
                )?;
                self.no_invalid_constructor_super = Some(configuration);
            }
            "noNewSymbol" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noNewSymbol", diagnostics)?;
                self.no_new_symbol = Some(configuration);
            }
            "noNonoctalDecimalEscape" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noNonoctalDecimalEscape",
                    diagnostics,
                )?;
                self.no_nonoctal_decimal_escape = Some(configuration);
            }
            "noPrecisionLoss" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noPrecisionLoss", diagnostics)?;
                self.no_precision_loss = Some(configuration);
            }
            "noRenderReturnValue" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noRenderReturnValue", diagnostics)?;
                self.no_render_return_value = Some(configuration);
            }
            "noSelfAssign" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noSelfAssign", diagnostics)?;
                self.no_self_assign = Some(configuration);
            }
            "noSetterReturn" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noSetterReturn", diagnostics)?;
                self.no_setter_return = Some(configuration);
            }
            "noStringCaseMismatch" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noStringCaseMismatch",
                    diagnostics,
                )?;
                self.no_string_case_mismatch = Some(configuration);
            }
            "noSwitchDeclarations" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noSwitchDeclarations",
                    diagnostics,
                )?;
                self.no_switch_declarations = Some(configuration);
            }
            "noUndeclaredVariables" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noUndeclaredVariables",
                    diagnostics,
                )?;
                self.no_undeclared_variables = Some(configuration);
            }
            "noUnnecessaryContinue" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noUnnecessaryContinue",
                    diagnostics,
                )?;
                self.no_unnecessary_continue = Some(configuration);
            }
            "noUnreachable" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noUnreachable", diagnostics)?;
                self.no_unreachable = Some(configuration);
            }
            "noUnreachableSuper" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noUnreachableSuper", diagnostics)?;
                self.no_unreachable_super = Some(configuration);
            }
            "noUnsafeFinally" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noUnsafeFinally", diagnostics)?;
                self.no_unsafe_finally = Some(configuration);
            }
            "noUnsafeOptionalChaining" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noUnsafeOptionalChaining",
                    diagnostics,
                )?;
                self.no_unsafe_optional_chaining = Some(configuration);
            }
            "noUnusedLabels" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noUnusedLabels", diagnostics)?;
                self.no_unused_labels = Some(configuration);
            }
            "noUnusedVariables" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noUnusedVariables", diagnostics)?;
                self.no_unused_variables = Some(configuration);
            }
            "noVoidElementsWithChildren" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noVoidElementsWithChildren",
                    diagnostics,
                )?;
                self.no_void_elements_with_children = Some(configuration);
            }
            "noVoidTypeReturn" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noVoidTypeReturn", diagnostics)?;
                self.no_void_type_return = Some(configuration);
            }
            "useExhaustiveDependencies" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "useExhaustiveDependencies",
                    diagnostics,
                )?;
                self.use_exhaustive_dependencies = Some(configuration);
            }
            "useHookAtTopLevel" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useHookAtTopLevel", diagnostics)?;
                self.use_hook_at_top_level = Some(configuration);
            }
            "useIsNan" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useIsNan", diagnostics)?;
                self.use_is_nan = Some(configuration);
            }
            "useValidForDirection" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "useValidForDirection",
                    diagnostics,
                )?;
                self.use_valid_for_direction = Some(configuration);
            }
            "useYield" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useYield", diagnostics)?;
                self.use_yield = Some(configuration);
            }
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
                "noApproximativeNumericConstant",
                "noDuplicateJsonKeys",
                "noEmptyBlockStatements",
                "noEmptyCharacterClassInRegex",
                "noInteractiveElementToNoninteractiveRole",
                "noInvalidNewBuiltin",
                "noMisleadingInstantiator",
                "noMisrefactoredShorthandAssign",
                "noThisInStatic",
                "noUnusedImports",
                "noUselessElse",
                "noUselessLoneBlockStatements",
                "useAriaActivedescendantWithTabindex",
                "useArrowFunction",
                "useAsConstAssertion",
                "useGroupedTypeImport",
                "useImportRestrictions",
                "useShorthandAssign",
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
            "noApproximativeNumericConstant" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noApproximativeNumericConstant",
                    diagnostics,
                )?;
                self.no_approximative_numeric_constant = Some(configuration);
            }
            "noDuplicateJsonKeys" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noDuplicateJsonKeys", diagnostics)?;
                self.no_duplicate_json_keys = Some(configuration);
            }
            "noEmptyBlockStatements" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noEmptyBlockStatements",
                    diagnostics,
                )?;
                self.no_empty_block_statements = Some(configuration);
            }
            "noEmptyCharacterClassInRegex" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noEmptyCharacterClassInRegex",
                    diagnostics,
                )?;
                self.no_empty_character_class_in_regex = Some(configuration);
            }
            "noInteractiveElementToNoninteractiveRole" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noInteractiveElementToNoninteractiveRole",
                    diagnostics,
                )?;
                self.no_interactive_element_to_noninteractive_role = Some(configuration);
            }
            "noInvalidNewBuiltin" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noInvalidNewBuiltin", diagnostics)?;
                self.no_invalid_new_builtin = Some(configuration);
            }
            "noMisleadingInstantiator" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noMisleadingInstantiator",
                    diagnostics,
                )?;
                self.no_misleading_instantiator = Some(configuration);
            }
            "noMisrefactoredShorthandAssign" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noMisrefactoredShorthandAssign",
                    diagnostics,
                )?;
                self.no_misrefactored_shorthand_assign = Some(configuration);
            }
            "noThisInStatic" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noThisInStatic", diagnostics)?;
                self.no_this_in_static = Some(configuration);
            }
            "noUnusedImports" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noUnusedImports", diagnostics)?;
                self.no_unused_imports = Some(configuration);
            }
            "noUselessElse" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noUselessElse", diagnostics)?;
                self.no_useless_else = Some(configuration);
            }
            "noUselessLoneBlockStatements" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noUselessLoneBlockStatements",
                    diagnostics,
                )?;
                self.no_useless_lone_block_statements = Some(configuration);
            }
            "useAriaActivedescendantWithTabindex" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "useAriaActivedescendantWithTabindex",
                    diagnostics,
                )?;
                self.use_aria_activedescendant_with_tabindex = Some(configuration);
            }
            "useArrowFunction" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useArrowFunction", diagnostics)?;
                self.use_arrow_function = Some(configuration);
            }
            "useAsConstAssertion" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useAsConstAssertion", diagnostics)?;
                self.use_as_const_assertion = Some(configuration);
            }
            "useGroupedTypeImport" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "useGroupedTypeImport",
                    diagnostics,
                )?;
                self.use_grouped_type_import = Some(configuration);
            }
            "useImportRestrictions" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "useImportRestrictions",
                    diagnostics,
                )?;
                self.use_import_restrictions = Some(configuration);
            }
            "useShorthandAssign" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useShorthandAssign", diagnostics)?;
                self.use_shorthand_assign = Some(configuration);
            }
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
        has_only_known_keys(
            node,
            &["recommended", "all", "noAccumulatingSpread", "noDelete"],
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
            "noAccumulatingSpread" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noAccumulatingSpread",
                    diagnostics,
                )?;
                self.no_accumulating_spread = Some(configuration);
            }
            "noDelete" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noDelete", diagnostics)?;
                self.no_delete = Some(configuration);
            }
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
            "noDangerouslySetInnerHtml" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noDangerouslySetInnerHtml",
                    diagnostics,
                )?;
                self.no_dangerously_set_inner_html = Some(configuration);
            }
            "noDangerouslySetInnerHtmlWithChildren" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noDangerouslySetInnerHtmlWithChildren",
                    diagnostics,
                )?;
                self.no_dangerously_set_inner_html_with_children = Some(configuration);
            }
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
                "useCollapsedElseIf",
                "useConst",
                "useDefaultParameterLast",
                "useEnumInitializers",
                "useExponentiationOperator",
                "useFragmentSyntax",
                "useLiteralEnumMembers",
                "useNamingConvention",
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
            "noArguments" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noArguments", diagnostics)?;
                self.no_arguments = Some(configuration);
            }
            "noCommaOperator" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noCommaOperator", diagnostics)?;
                self.no_comma_operator = Some(configuration);
            }
            "noImplicitBoolean" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noImplicitBoolean", diagnostics)?;
                self.no_implicit_boolean = Some(configuration);
            }
            "noInferrableTypes" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noInferrableTypes", diagnostics)?;
                self.no_inferrable_types = Some(configuration);
            }
            "noNamespace" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noNamespace", diagnostics)?;
                self.no_namespace = Some(configuration);
            }
            "noNegationElse" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noNegationElse", diagnostics)?;
                self.no_negation_else = Some(configuration);
            }
            "noNonNullAssertion" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noNonNullAssertion", diagnostics)?;
                self.no_non_null_assertion = Some(configuration);
            }
            "noParameterAssign" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noParameterAssign", diagnostics)?;
                self.no_parameter_assign = Some(configuration);
            }
            "noParameterProperties" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noParameterProperties",
                    diagnostics,
                )?;
                self.no_parameter_properties = Some(configuration);
            }
            "noRestrictedGlobals" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noRestrictedGlobals", diagnostics)?;
                self.no_restricted_globals = Some(configuration);
            }
            "noShoutyConstants" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noShoutyConstants", diagnostics)?;
                self.no_shouty_constants = Some(configuration);
            }
            "noUnusedTemplateLiteral" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noUnusedTemplateLiteral",
                    diagnostics,
                )?;
                self.no_unused_template_literal = Some(configuration);
            }
            "noVar" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noVar", diagnostics)?;
                self.no_var = Some(configuration);
            }
            "useBlockStatements" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useBlockStatements", diagnostics)?;
                self.use_block_statements = Some(configuration);
            }
            "useCollapsedElseIf" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useCollapsedElseIf", diagnostics)?;
                self.use_collapsed_else_if = Some(configuration);
            }
            "useConst" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useConst", diagnostics)?;
                self.use_const = Some(configuration);
            }
            "useDefaultParameterLast" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "useDefaultParameterLast",
                    diagnostics,
                )?;
                self.use_default_parameter_last = Some(configuration);
            }
            "useEnumInitializers" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useEnumInitializers", diagnostics)?;
                self.use_enum_initializers = Some(configuration);
            }
            "useExponentiationOperator" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "useExponentiationOperator",
                    diagnostics,
                )?;
                self.use_exponentiation_operator = Some(configuration);
            }
            "useFragmentSyntax" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useFragmentSyntax", diagnostics)?;
                self.use_fragment_syntax = Some(configuration);
            }
            "useLiteralEnumMembers" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "useLiteralEnumMembers",
                    diagnostics,
                )?;
                self.use_literal_enum_members = Some(configuration);
            }
            "useNamingConvention" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useNamingConvention", diagnostics)?;
                self.use_naming_convention = Some(configuration);
            }
            "useNumericLiterals" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useNumericLiterals", diagnostics)?;
                self.use_numeric_literals = Some(configuration);
            }
            "useSelfClosingElements" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "useSelfClosingElements",
                    diagnostics,
                )?;
                self.use_self_closing_elements = Some(configuration);
            }
            "useShorthandArrayType" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "useShorthandArrayType",
                    diagnostics,
                )?;
                self.use_shorthand_array_type = Some(configuration);
            }
            "useSingleCaseStatement" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "useSingleCaseStatement",
                    diagnostics,
                )?;
                self.use_single_case_statement = Some(configuration);
            }
            "useSingleVarDeclarator" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "useSingleVarDeclarator",
                    diagnostics,
                )?;
                self.use_single_var_declarator = Some(configuration);
            }
            "useTemplate" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useTemplate", diagnostics)?;
                self.use_template = Some(configuration);
            }
            "useWhile" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useWhile", diagnostics)?;
                self.use_while = Some(configuration);
            }
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
                "noConfusingVoidType",
                "noConsoleLog",
                "noConstEnum",
                "noControlCharactersInRegex",
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
                "noFallthroughSwitchClause",
                "noFunctionAssign",
                "noGlobalIsFinite",
                "noGlobalIsNan",
                "noImportAssign",
                "noLabelVar",
                "noPrototypeBuiltins",
                "noRedeclare",
                "noRedundantUseStrict",
                "noSelfCompare",
                "noShadowRestrictedNames",
                "noSparseArray",
                "noUnsafeDeclarationMerging",
                "noUnsafeNegation",
                "useDefaultSwitchClauseLast",
                "useGetterReturn",
                "useIsArray",
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
            "noArrayIndexKey" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noArrayIndexKey", diagnostics)?;
                self.no_array_index_key = Some(configuration);
            }
            "noAssignInExpressions" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noAssignInExpressions",
                    diagnostics,
                )?;
                self.no_assign_in_expressions = Some(configuration);
            }
            "noAsyncPromiseExecutor" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noAsyncPromiseExecutor",
                    diagnostics,
                )?;
                self.no_async_promise_executor = Some(configuration);
            }
            "noCatchAssign" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noCatchAssign", diagnostics)?;
                self.no_catch_assign = Some(configuration);
            }
            "noClassAssign" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noClassAssign", diagnostics)?;
                self.no_class_assign = Some(configuration);
            }
            "noCommentText" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noCommentText", diagnostics)?;
                self.no_comment_text = Some(configuration);
            }
            "noCompareNegZero" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noCompareNegZero", diagnostics)?;
                self.no_compare_neg_zero = Some(configuration);
            }
            "noConfusingLabels" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noConfusingLabels", diagnostics)?;
                self.no_confusing_labels = Some(configuration);
            }
            "noConfusingVoidType" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noConfusingVoidType", diagnostics)?;
                self.no_confusing_void_type = Some(configuration);
            }
            "noConsoleLog" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noConsoleLog", diagnostics)?;
                self.no_console_log = Some(configuration);
            }
            "noConstEnum" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noConstEnum", diagnostics)?;
                self.no_const_enum = Some(configuration);
            }
            "noControlCharactersInRegex" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noControlCharactersInRegex",
                    diagnostics,
                )?;
                self.no_control_characters_in_regex = Some(configuration);
            }
            "noDebugger" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noDebugger", diagnostics)?;
                self.no_debugger = Some(configuration);
            }
            "noDoubleEquals" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noDoubleEquals", diagnostics)?;
                self.no_double_equals = Some(configuration);
            }
            "noDuplicateCase" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noDuplicateCase", diagnostics)?;
                self.no_duplicate_case = Some(configuration);
            }
            "noDuplicateClassMembers" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noDuplicateClassMembers",
                    diagnostics,
                )?;
                self.no_duplicate_class_members = Some(configuration);
            }
            "noDuplicateJsxProps" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noDuplicateJsxProps", diagnostics)?;
                self.no_duplicate_jsx_props = Some(configuration);
            }
            "noDuplicateObjectKeys" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noDuplicateObjectKeys",
                    diagnostics,
                )?;
                self.no_duplicate_object_keys = Some(configuration);
            }
            "noDuplicateParameters" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noDuplicateParameters",
                    diagnostics,
                )?;
                self.no_duplicate_parameters = Some(configuration);
            }
            "noEmptyInterface" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noEmptyInterface", diagnostics)?;
                self.no_empty_interface = Some(configuration);
            }
            "noExplicitAny" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noExplicitAny", diagnostics)?;
                self.no_explicit_any = Some(configuration);
            }
            "noExtraNonNullAssertion" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noExtraNonNullAssertion",
                    diagnostics,
                )?;
                self.no_extra_non_null_assertion = Some(configuration);
            }
            "noFallthroughSwitchClause" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noFallthroughSwitchClause",
                    diagnostics,
                )?;
                self.no_fallthrough_switch_clause = Some(configuration);
            }
            "noFunctionAssign" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noFunctionAssign", diagnostics)?;
                self.no_function_assign = Some(configuration);
            }
            "noGlobalIsFinite" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noGlobalIsFinite", diagnostics)?;
                self.no_global_is_finite = Some(configuration);
            }
            "noGlobalIsNan" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noGlobalIsNan", diagnostics)?;
                self.no_global_is_nan = Some(configuration);
            }
            "noImportAssign" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noImportAssign", diagnostics)?;
                self.no_import_assign = Some(configuration);
            }
            "noLabelVar" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noLabelVar", diagnostics)?;
                self.no_label_var = Some(configuration);
            }
            "noPrototypeBuiltins" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noPrototypeBuiltins", diagnostics)?;
                self.no_prototype_builtins = Some(configuration);
            }
            "noRedeclare" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noRedeclare", diagnostics)?;
                self.no_redeclare = Some(configuration);
            }
            "noRedundantUseStrict" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noRedundantUseStrict",
                    diagnostics,
                )?;
                self.no_redundant_use_strict = Some(configuration);
            }
            "noSelfCompare" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noSelfCompare", diagnostics)?;
                self.no_self_compare = Some(configuration);
            }
            "noShadowRestrictedNames" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noShadowRestrictedNames",
                    diagnostics,
                )?;
                self.no_shadow_restricted_names = Some(configuration);
            }
            "noSparseArray" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noSparseArray", diagnostics)?;
                self.no_sparse_array = Some(configuration);
            }
            "noUnsafeDeclarationMerging" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noUnsafeDeclarationMerging",
                    diagnostics,
                )?;
                self.no_unsafe_declaration_merging = Some(configuration);
            }
            "noUnsafeNegation" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noUnsafeNegation", diagnostics)?;
                self.no_unsafe_negation = Some(configuration);
            }
            "useDefaultSwitchClauseLast" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "useDefaultSwitchClauseLast",
                    diagnostics,
                )?;
                self.use_default_switch_clause_last = Some(configuration);
            }
            "useGetterReturn" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useGetterReturn", diagnostics)?;
                self.use_getter_return = Some(configuration);
            }
            "useIsArray" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useIsArray", diagnostics)?;
                self.use_is_array = Some(configuration);
            }
            "useNamespaceKeyword" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useNamespaceKeyword", diagnostics)?;
                self.use_namespace_keyword = Some(configuration);
            }
            "useValidTypeof" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useValidTypeof", diagnostics)?;
                self.use_valid_typeof = Some(configuration);
            }
            _ => {}
        }
        Some(())
    }
}
