//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::configuration::linter::*;
use crate::Rules;
use biome_console::markup;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    ExpectedType,
};
use biome_rowan::{TextRange, TokenText};
impl Deserializable for Rules {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Rules;
            const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
                range: TextRange,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut recommended_is_set = false;
                let mut result = Self::Output::default();
                for (key, value) in members {
                    let key_range = key.range();
                    let Some (key) = TokenText :: deserialize (key , diagnostics) else { continue ; } ;
                    match key.text() {
                        "recommended" => {
                            recommended_is_set = true;
                            result.recommended = Deserializable::deserialize(value, diagnostics);
                        }
                        "all" => {
                            result.all = Deserializable::deserialize(value, diagnostics);
                        }
                        "a11y" => {
                            result.a11y = Deserializable::deserialize(value, diagnostics);
                        }
                        "complexity" => {
                            result.complexity = Deserializable::deserialize(value, diagnostics);
                        }
                        "correctness" => {
                            result.correctness = Deserializable::deserialize(value, diagnostics);
                        }
                        "nursery" => {
                            result.nursery = Deserializable::deserialize(value, diagnostics);
                        }
                        "performance" => {
                            result.performance = Deserializable::deserialize(value, diagnostics);
                        }
                        "security" => {
                            result.security = Deserializable::deserialize(value, diagnostics);
                        }
                        "style" => {
                            result.style = Deserializable::deserialize(value, diagnostics);
                        }
                        "suspicious" => {
                            result.suspicious = Deserializable::deserialize(value, diagnostics);
                        }
                        _ => {
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                key.text(),
                                key_range,
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
                            ));
                        }
                    }
                }
                if recommended_is_set
                    && matches!(result.recommended, Some(true))
                    && matches!(result.all, Some(true))
                {
                    diagnostics . push (DeserializationDiagnostic :: new (markup ! (< Emphasis > "'recommended'" < / Emphasis > " and " < Emphasis > "'all'" < / Emphasis > " can't be both " < Emphasis > "'true'" < / Emphasis > ". You should choose only one of them.")) . with_range (range) . with_note (markup ! ("Biome will fallback to its defaults for this section."))) ;
                    return Some(Self::Output::default());
                }
                Some(result)
            }
        }
        value.deserialize(Visitor, diagnostics)
    }
}
impl Deserializable for A11y {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = A11y;
            const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
                range: TextRange,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut recommended_is_set = false;
                let mut result = Self::Output::default();
                for (key, value) in members {
                    let key_range = key.range();
                    let Some (key) = TokenText :: deserialize (key , diagnostics) else { continue ; } ;
                    match key.text() {
                        "recommended" => {
                            recommended_is_set = true;
                            result.recommended = Deserializable::deserialize(value, diagnostics);
                        }
                        "all" => {
                            result.all = Deserializable::deserialize(value, diagnostics);
                        }
                        "noAccessKey" => {
                            result.no_access_key = RuleConfiguration::deserialize_from_rule_name(
                                "noAccessKey",
                                value,
                                diagnostics,
                            );
                        }
                        "noAriaUnsupportedElements" => {
                            result.no_aria_unsupported_elements =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noAriaUnsupportedElements",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noAutofocus" => {
                            result.no_autofocus = RuleConfiguration::deserialize_from_rule_name(
                                "noAutofocus",
                                value,
                                diagnostics,
                            );
                        }
                        "noBlankTarget" => {
                            result.no_blank_target = RuleConfiguration::deserialize_from_rule_name(
                                "noBlankTarget",
                                value,
                                diagnostics,
                            );
                        }
                        "noDistractingElements" => {
                            result.no_distracting_elements =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noDistractingElements",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noHeaderScope" => {
                            result.no_header_scope = RuleConfiguration::deserialize_from_rule_name(
                                "noHeaderScope",
                                value,
                                diagnostics,
                            );
                        }
                        "noNoninteractiveElementToInteractiveRole" => {
                            result.no_noninteractive_element_to_interactive_role =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noNoninteractiveElementToInteractiveRole",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noNoninteractiveTabindex" => {
                            result.no_noninteractive_tabindex =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noNoninteractiveTabindex",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noPositiveTabindex" => {
                            result.no_positive_tabindex =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noPositiveTabindex",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noRedundantAlt" => {
                            result.no_redundant_alt = RuleConfiguration::deserialize_from_rule_name(
                                "noRedundantAlt",
                                value,
                                diagnostics,
                            );
                        }
                        "noRedundantRoles" => {
                            result.no_redundant_roles =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noRedundantRoles",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noSvgWithoutTitle" => {
                            result.no_svg_without_title =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noSvgWithoutTitle",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useAltText" => {
                            result.use_alt_text = RuleConfiguration::deserialize_from_rule_name(
                                "useAltText",
                                value,
                                diagnostics,
                            );
                        }
                        "useAnchorContent" => {
                            result.use_anchor_content =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useAnchorContent",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useAriaPropsForRole" => {
                            result.use_aria_props_for_role =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useAriaPropsForRole",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useButtonType" => {
                            result.use_button_type = RuleConfiguration::deserialize_from_rule_name(
                                "useButtonType",
                                value,
                                diagnostics,
                            );
                        }
                        "useHeadingContent" => {
                            result.use_heading_content =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useHeadingContent",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useHtmlLang" => {
                            result.use_html_lang = RuleConfiguration::deserialize_from_rule_name(
                                "useHtmlLang",
                                value,
                                diagnostics,
                            );
                        }
                        "useIframeTitle" => {
                            result.use_iframe_title = RuleConfiguration::deserialize_from_rule_name(
                                "useIframeTitle",
                                value,
                                diagnostics,
                            );
                        }
                        "useKeyWithClickEvents" => {
                            result.use_key_with_click_events =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useKeyWithClickEvents",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useKeyWithMouseEvents" => {
                            result.use_key_with_mouse_events =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useKeyWithMouseEvents",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useMediaCaption" => {
                            result.use_media_caption =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useMediaCaption",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useValidAnchor" => {
                            result.use_valid_anchor = RuleConfiguration::deserialize_from_rule_name(
                                "useValidAnchor",
                                value,
                                diagnostics,
                            );
                        }
                        "useValidAriaProps" => {
                            result.use_valid_aria_props =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useValidAriaProps",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useValidAriaValues" => {
                            result.use_valid_aria_values =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useValidAriaValues",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useValidLang" => {
                            result.use_valid_lang = RuleConfiguration::deserialize_from_rule_name(
                                "useValidLang",
                                value,
                                diagnostics,
                            );
                        }
                        _ => {
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                key.text(),
                                key_range,
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
                            ));
                        }
                    }
                }
                if recommended_is_set
                    && matches!(result.recommended, Some(true))
                    && matches!(result.all, Some(true))
                {
                    diagnostics . push (DeserializationDiagnostic :: new (markup ! (< Emphasis > "'recommended'" < / Emphasis > " and " < Emphasis > "'all'" < / Emphasis > " can't be both " < Emphasis > "'true'" < / Emphasis > ". You should choose only one of them.")) . with_range (range) . with_note (markup ! ("Biome will fallback to its defaults for this section."))) ;
                    return Some(Self::Output::default());
                }
                Some(result)
            }
        }
        value.deserialize(Visitor, diagnostics)
    }
}
impl Deserializable for Complexity {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Complexity;
            const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
                range: TextRange,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut recommended_is_set = false;
                let mut result = Self::Output::default();
                for (key, value) in members {
                    let key_range = key.range();
                    let Some (key) = TokenText :: deserialize (key , diagnostics) else { continue ; } ;
                    match key.text() {
                        "recommended" => {
                            recommended_is_set = true;
                            result.recommended = Deserializable::deserialize(value, diagnostics);
                        }
                        "all" => {
                            result.all = Deserializable::deserialize(value, diagnostics);
                        }
                        "noBannedTypes" => {
                            result.no_banned_types = RuleConfiguration::deserialize_from_rule_name(
                                "noBannedTypes",
                                value,
                                diagnostics,
                            );
                        }
                        "noExcessiveCognitiveComplexity" => {
                            result.no_excessive_cognitive_complexity =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noExcessiveCognitiveComplexity",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noExtraBooleanCast" => {
                            result.no_extra_boolean_cast =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noExtraBooleanCast",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noForEach" => {
                            result.no_for_each = RuleConfiguration::deserialize_from_rule_name(
                                "noForEach",
                                value,
                                diagnostics,
                            );
                        }
                        "noMultipleSpacesInRegularExpressionLiterals" => {
                            result.no_multiple_spaces_in_regular_expression_literals =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noMultipleSpacesInRegularExpressionLiterals",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noStaticOnlyClass" => {
                            result.no_static_only_class =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noStaticOnlyClass",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noUselessCatch" => {
                            result.no_useless_catch = RuleConfiguration::deserialize_from_rule_name(
                                "noUselessCatch",
                                value,
                                diagnostics,
                            );
                        }
                        "noUselessConstructor" => {
                            result.no_useless_constructor =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUselessConstructor",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noUselessEmptyExport" => {
                            result.no_useless_empty_export =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUselessEmptyExport",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noUselessFragments" => {
                            result.no_useless_fragments =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUselessFragments",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noUselessLabel" => {
                            result.no_useless_label = RuleConfiguration::deserialize_from_rule_name(
                                "noUselessLabel",
                                value,
                                diagnostics,
                            );
                        }
                        "noUselessRename" => {
                            result.no_useless_rename =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUselessRename",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noUselessSwitchCase" => {
                            result.no_useless_switch_case =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUselessSwitchCase",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noUselessThisAlias" => {
                            result.no_useless_this_alias =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUselessThisAlias",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noUselessTypeConstraint" => {
                            result.no_useless_type_constraint =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUselessTypeConstraint",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noVoid" => {
                            result.no_void = RuleConfiguration::deserialize_from_rule_name(
                                "noVoid",
                                value,
                                diagnostics,
                            );
                        }
                        "noWith" => {
                            result.no_with = RuleConfiguration::deserialize_from_rule_name(
                                "noWith",
                                value,
                                diagnostics,
                            );
                        }
                        "useFlatMap" => {
                            result.use_flat_map = RuleConfiguration::deserialize_from_rule_name(
                                "useFlatMap",
                                value,
                                diagnostics,
                            );
                        }
                        "useLiteralKeys" => {
                            result.use_literal_keys = RuleConfiguration::deserialize_from_rule_name(
                                "useLiteralKeys",
                                value,
                                diagnostics,
                            );
                        }
                        "useOptionalChain" => {
                            result.use_optional_chain =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useOptionalChain",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useSimpleNumberKeys" => {
                            result.use_simple_number_keys =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useSimpleNumberKeys",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useSimplifiedLogicExpression" => {
                            result.use_simplified_logic_expression =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useSimplifiedLogicExpression",
                                    value,
                                    diagnostics,
                                );
                        }
                        _ => {
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                key.text(),
                                key_range,
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
                            ));
                        }
                    }
                }
                if recommended_is_set
                    && matches!(result.recommended, Some(true))
                    && matches!(result.all, Some(true))
                {
                    diagnostics . push (DeserializationDiagnostic :: new (markup ! (< Emphasis > "'recommended'" < / Emphasis > " and " < Emphasis > "'all'" < / Emphasis > " can't be both " < Emphasis > "'true'" < / Emphasis > ". You should choose only one of them.")) . with_range (range) . with_note (markup ! ("Biome will fallback to its defaults for this section."))) ;
                    return Some(Self::Output::default());
                }
                Some(result)
            }
        }
        value.deserialize(Visitor, diagnostics)
    }
}
impl Deserializable for Correctness {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Correctness;
            const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
                range: TextRange,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut recommended_is_set = false;
                let mut result = Self::Output::default();
                for (key, value) in members {
                    let key_range = key.range();
                    let Some (key) = TokenText :: deserialize (key , diagnostics) else { continue ; } ;
                    match key.text() {
                        "recommended" => {
                            recommended_is_set = true;
                            result.recommended = Deserializable::deserialize(value, diagnostics);
                        }
                        "all" => {
                            result.all = Deserializable::deserialize(value, diagnostics);
                        }
                        "noChildrenProp" => {
                            result.no_children_prop = RuleConfiguration::deserialize_from_rule_name(
                                "noChildrenProp",
                                value,
                                diagnostics,
                            );
                        }
                        "noConstAssign" => {
                            result.no_const_assign = RuleConfiguration::deserialize_from_rule_name(
                                "noConstAssign",
                                value,
                                diagnostics,
                            );
                        }
                        "noConstantCondition" => {
                            result.no_constant_condition =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noConstantCondition",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noConstructorReturn" => {
                            result.no_constructor_return =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noConstructorReturn",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noEmptyPattern" => {
                            result.no_empty_pattern = RuleConfiguration::deserialize_from_rule_name(
                                "noEmptyPattern",
                                value,
                                diagnostics,
                            );
                        }
                        "noGlobalObjectCalls" => {
                            result.no_global_object_calls =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noGlobalObjectCalls",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noInnerDeclarations" => {
                            result.no_inner_declarations =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noInnerDeclarations",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noInvalidConstructorSuper" => {
                            result.no_invalid_constructor_super =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noInvalidConstructorSuper",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noNewSymbol" => {
                            result.no_new_symbol = RuleConfiguration::deserialize_from_rule_name(
                                "noNewSymbol",
                                value,
                                diagnostics,
                            );
                        }
                        "noNonoctalDecimalEscape" => {
                            result.no_nonoctal_decimal_escape =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noNonoctalDecimalEscape",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noPrecisionLoss" => {
                            result.no_precision_loss =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noPrecisionLoss",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noRenderReturnValue" => {
                            result.no_render_return_value =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noRenderReturnValue",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noSelfAssign" => {
                            result.no_self_assign = RuleConfiguration::deserialize_from_rule_name(
                                "noSelfAssign",
                                value,
                                diagnostics,
                            );
                        }
                        "noSetterReturn" => {
                            result.no_setter_return = RuleConfiguration::deserialize_from_rule_name(
                                "noSetterReturn",
                                value,
                                diagnostics,
                            );
                        }
                        "noStringCaseMismatch" => {
                            result.no_string_case_mismatch =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noStringCaseMismatch",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noSwitchDeclarations" => {
                            result.no_switch_declarations =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noSwitchDeclarations",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noUndeclaredVariables" => {
                            result.no_undeclared_variables =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUndeclaredVariables",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noUnnecessaryContinue" => {
                            result.no_unnecessary_continue =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUnnecessaryContinue",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noUnreachable" => {
                            result.no_unreachable = RuleConfiguration::deserialize_from_rule_name(
                                "noUnreachable",
                                value,
                                diagnostics,
                            );
                        }
                        "noUnreachableSuper" => {
                            result.no_unreachable_super =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUnreachableSuper",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noUnsafeFinally" => {
                            result.no_unsafe_finally =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUnsafeFinally",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noUnsafeOptionalChaining" => {
                            result.no_unsafe_optional_chaining =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUnsafeOptionalChaining",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noUnusedLabels" => {
                            result.no_unused_labels = RuleConfiguration::deserialize_from_rule_name(
                                "noUnusedLabels",
                                value,
                                diagnostics,
                            );
                        }
                        "noUnusedVariables" => {
                            result.no_unused_variables =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUnusedVariables",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noVoidElementsWithChildren" => {
                            result.no_void_elements_with_children =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noVoidElementsWithChildren",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noVoidTypeReturn" => {
                            result.no_void_type_return =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noVoidTypeReturn",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useExhaustiveDependencies" => {
                            result.use_exhaustive_dependencies =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useExhaustiveDependencies",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useHookAtTopLevel" => {
                            result.use_hook_at_top_level =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useHookAtTopLevel",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useIsNan" => {
                            result.use_is_nan = RuleConfiguration::deserialize_from_rule_name(
                                "useIsNan",
                                value,
                                diagnostics,
                            );
                        }
                        "useValidForDirection" => {
                            result.use_valid_for_direction =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useValidForDirection",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useYield" => {
                            result.use_yield = RuleConfiguration::deserialize_from_rule_name(
                                "useYield",
                                value,
                                diagnostics,
                            );
                        }
                        _ => {
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                key.text(),
                                key_range,
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
                            ));
                        }
                    }
                }
                if recommended_is_set
                    && matches!(result.recommended, Some(true))
                    && matches!(result.all, Some(true))
                {
                    diagnostics . push (DeserializationDiagnostic :: new (markup ! (< Emphasis > "'recommended'" < / Emphasis > " and " < Emphasis > "'all'" < / Emphasis > " can't be both " < Emphasis > "'true'" < / Emphasis > ". You should choose only one of them.")) . with_range (range) . with_note (markup ! ("Biome will fallback to its defaults for this section."))) ;
                    return Some(Self::Output::default());
                }
                Some(result)
            }
        }
        value.deserialize(Visitor, diagnostics)
    }
}
impl Deserializable for Nursery {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Nursery;
            const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
                range: TextRange,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut recommended_is_set = false;
                let mut result = Self::Output::default();
                for (key, value) in members {
                    let key_range = key.range();
                    let Some (key) = TokenText :: deserialize (key , diagnostics) else { continue ; } ;
                    match key.text() {
                        "recommended" => {
                            recommended_is_set = true;
                            result.recommended = Deserializable::deserialize(value, diagnostics);
                        }
                        "all" => {
                            result.all = Deserializable::deserialize(value, diagnostics);
                        }
                        "noApproximativeNumericConstant" => {
                            result.no_approximative_numeric_constant =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noApproximativeNumericConstant",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noDuplicateJsonKeys" => {
                            result.no_duplicate_json_keys =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noDuplicateJsonKeys",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noEmptyBlockStatements" => {
                            result.no_empty_block_statements =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noEmptyBlockStatements",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noEmptyCharacterClassInRegex" => {
                            result.no_empty_character_class_in_regex =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noEmptyCharacterClassInRegex",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noInteractiveElementToNoninteractiveRole" => {
                            result.no_interactive_element_to_noninteractive_role =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noInteractiveElementToNoninteractiveRole",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noInvalidNewBuiltin" => {
                            result.no_invalid_new_builtin =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noInvalidNewBuiltin",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noMisleadingInstantiator" => {
                            result.no_misleading_instantiator =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noMisleadingInstantiator",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noMisrefactoredShorthandAssign" => {
                            result.no_misrefactored_shorthand_assign =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noMisrefactoredShorthandAssign",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noThisInStatic" => {
                            result.no_this_in_static =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noThisInStatic",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noUnusedImports" => {
                            result.no_unused_imports =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUnusedImports",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noUnusedPrivateClassMembers" => {
                            result.no_unused_private_class_members =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUnusedPrivateClassMembers",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noUselessElse" => {
                            result.no_useless_else = RuleConfiguration::deserialize_from_rule_name(
                                "noUselessElse",
                                value,
                                diagnostics,
                            );
                        }
                        "noUselessLoneBlockStatements" => {
                            result.no_useless_lone_block_statements =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUselessLoneBlockStatements",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useAriaActivedescendantWithTabindex" => {
                            result.use_aria_activedescendant_with_tabindex =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useAriaActivedescendantWithTabindex",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useArrowFunction" => {
                            result.use_arrow_function =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useArrowFunction",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useAsConstAssertion" => {
                            result.use_as_const_assertion =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useAsConstAssertion",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useGroupedTypeImport" => {
                            result.use_grouped_type_import =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useGroupedTypeImport",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useImportRestrictions" => {
                            result.use_import_restrictions =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useImportRestrictions",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useShorthandAssign" => {
                            result.use_shorthand_assign =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useShorthandAssign",
                                    value,
                                    diagnostics,
                                );
                        }
                        _ => {
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                key.text(),
                                key_range,
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
                                    "noUnusedPrivateClassMembers",
                                    "noUselessElse",
                                    "noUselessLoneBlockStatements",
                                    "useAriaActivedescendantWithTabindex",
                                    "useArrowFunction",
                                    "useAsConstAssertion",
                                    "useGroupedTypeImport",
                                    "useImportRestrictions",
                                    "useShorthandAssign",
                                ],
                            ));
                        }
                    }
                }
                if recommended_is_set
                    && matches!(result.recommended, Some(true))
                    && matches!(result.all, Some(true))
                {
                    diagnostics . push (DeserializationDiagnostic :: new (markup ! (< Emphasis > "'recommended'" < / Emphasis > " and " < Emphasis > "'all'" < / Emphasis > " can't be both " < Emphasis > "'true'" < / Emphasis > ". You should choose only one of them.")) . with_range (range) . with_note (markup ! ("Biome will fallback to its defaults for this section."))) ;
                    return Some(Self::Output::default());
                }
                Some(result)
            }
        }
        value.deserialize(Visitor, diagnostics)
    }
}
impl Deserializable for Performance {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Performance;
            const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
                range: TextRange,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut recommended_is_set = false;
                let mut result = Self::Output::default();
                for (key, value) in members {
                    let key_range = key.range();
                    let Some (key) = TokenText :: deserialize (key , diagnostics) else { continue ; } ;
                    match key.text() {
                        "recommended" => {
                            recommended_is_set = true;
                            result.recommended = Deserializable::deserialize(value, diagnostics);
                        }
                        "all" => {
                            result.all = Deserializable::deserialize(value, diagnostics);
                        }
                        "noAccumulatingSpread" => {
                            result.no_accumulating_spread =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noAccumulatingSpread",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noDelete" => {
                            result.no_delete = RuleConfiguration::deserialize_from_rule_name(
                                "noDelete",
                                value,
                                diagnostics,
                            );
                        }
                        _ => {
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                key.text(),
                                key_range,
                                &["recommended", "all", "noAccumulatingSpread", "noDelete"],
                            ));
                        }
                    }
                }
                if recommended_is_set
                    && matches!(result.recommended, Some(true))
                    && matches!(result.all, Some(true))
                {
                    diagnostics . push (DeserializationDiagnostic :: new (markup ! (< Emphasis > "'recommended'" < / Emphasis > " and " < Emphasis > "'all'" < / Emphasis > " can't be both " < Emphasis > "'true'" < / Emphasis > ". You should choose only one of them.")) . with_range (range) . with_note (markup ! ("Biome will fallback to its defaults for this section."))) ;
                    return Some(Self::Output::default());
                }
                Some(result)
            }
        }
        value.deserialize(Visitor, diagnostics)
    }
}
impl Deserializable for Security {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Security;
            const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
                range: TextRange,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut recommended_is_set = false;
                let mut result = Self::Output::default();
                for (key, value) in members {
                    let key_range = key.range();
                    let Some (key) = TokenText :: deserialize (key , diagnostics) else { continue ; } ;
                    match key.text() {
                        "recommended" => {
                            recommended_is_set = true;
                            result.recommended = Deserializable::deserialize(value, diagnostics);
                        }
                        "all" => {
                            result.all = Deserializable::deserialize(value, diagnostics);
                        }
                        "noDangerouslySetInnerHtml" => {
                            result.no_dangerously_set_inner_html =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noDangerouslySetInnerHtml",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noDangerouslySetInnerHtmlWithChildren" => {
                            result.no_dangerously_set_inner_html_with_children =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noDangerouslySetInnerHtmlWithChildren",
                                    value,
                                    diagnostics,
                                );
                        }
                        _ => {
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                key.text(),
                                key_range,
                                &[
                                    "recommended",
                                    "all",
                                    "noDangerouslySetInnerHtml",
                                    "noDangerouslySetInnerHtmlWithChildren",
                                ],
                            ));
                        }
                    }
                }
                if recommended_is_set
                    && matches!(result.recommended, Some(true))
                    && matches!(result.all, Some(true))
                {
                    diagnostics . push (DeserializationDiagnostic :: new (markup ! (< Emphasis > "'recommended'" < / Emphasis > " and " < Emphasis > "'all'" < / Emphasis > " can't be both " < Emphasis > "'true'" < / Emphasis > ". You should choose only one of them.")) . with_range (range) . with_note (markup ! ("Biome will fallback to its defaults for this section."))) ;
                    return Some(Self::Output::default());
                }
                Some(result)
            }
        }
        value.deserialize(Visitor, diagnostics)
    }
}
impl Deserializable for Style {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value)?;
        let name_text = name.inner_string_text().ok()?;
        let name_text = name_text.text();
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
            "noDefaultExport" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "noDefaultExport", diagnostics)?;
                self.no_default_export = Some(configuration);
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
            "noUnusedPrivateClassMembers" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(
                    &value,
                    "noUnusedPrivateClassMembers",
                    diagnostics,
                )?;
                self.no_unused_private_class_members = Some(configuration);
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
            "useAwait" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useAwait", diagnostics)?;
                self.use_await = Some(configuration);
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
            "useValidAriaRole" => {
                let mut configuration = RuleConfiguration::default();
                configuration.map_rule_configuration(&value, "useValidAriaRole", diagnostics)?;
                self.use_valid_aria_role = Some(configuration);
            }
            _ => {
                report_unknown_map_key(
                    &name,
                    &[
                        "recommended",
                        "all",
                        "noApproximativeNumericConstant",
                        "noDefaultExport",
                        "noDuplicateJsonKeys",
                        "noEmptyBlockStatements",
                        "noEmptyCharacterClassInRegex",
                        "noInteractiveElementToNoninteractiveRole",
                        "noInvalidNewBuiltin",
                        "noMisleadingInstantiator",
                        "noMisrefactoredShorthandAssign",
                        "noThisInStatic",
                        "noUnusedImports",
                        "noUnusedPrivateClassMembers",
                        "noUselessElse",
                        "noUselessLoneBlockStatements",
                        "useAriaActivedescendantWithTabindex",
                        "useArrowFunction",
                        "useAsConstAssertion",
                        "useAwait",
                        "useGroupedTypeImport",
                        "useImportRestrictions",
                        "useShorthandAssign",
                        "useValidAriaRole",
                    ],
                    diagnostics,
                );
            }
        }
        value.deserialize(Visitor, diagnostics)
    }
}
impl Deserializable for Suspicious {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Suspicious;
            const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
                range: TextRange,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut recommended_is_set = false;
                let mut result = Self::Output::default();
                for (key, value) in members {
                    let key_range = key.range();
                    let Some (key) = TokenText :: deserialize (key , diagnostics) else { continue ; } ;
                    match key.text() {
                        "recommended" => {
                            recommended_is_set = true;
                            result.recommended = Deserializable::deserialize(value, diagnostics);
                        }
                        "all" => {
                            result.all = Deserializable::deserialize(value, diagnostics);
                        }
                        "noArrayIndexKey" => {
                            result.no_array_index_key =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noArrayIndexKey",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noAssignInExpressions" => {
                            result.no_assign_in_expressions =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noAssignInExpressions",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noAsyncPromiseExecutor" => {
                            result.no_async_promise_executor =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noAsyncPromiseExecutor",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noCatchAssign" => {
                            result.no_catch_assign = RuleConfiguration::deserialize_from_rule_name(
                                "noCatchAssign",
                                value,
                                diagnostics,
                            );
                        }
                        "noClassAssign" => {
                            result.no_class_assign = RuleConfiguration::deserialize_from_rule_name(
                                "noClassAssign",
                                value,
                                diagnostics,
                            );
                        }
                        "noCommentText" => {
                            result.no_comment_text = RuleConfiguration::deserialize_from_rule_name(
                                "noCommentText",
                                value,
                                diagnostics,
                            );
                        }
                        "noCompareNegZero" => {
                            result.no_compare_neg_zero =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noCompareNegZero",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noConfusingLabels" => {
                            result.no_confusing_labels =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noConfusingLabels",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noConfusingVoidType" => {
                            result.no_confusing_void_type =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noConfusingVoidType",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noConsoleLog" => {
                            result.no_console_log = RuleConfiguration::deserialize_from_rule_name(
                                "noConsoleLog",
                                value,
                                diagnostics,
                            );
                        }
                        "noConstEnum" => {
                            result.no_const_enum = RuleConfiguration::deserialize_from_rule_name(
                                "noConstEnum",
                                value,
                                diagnostics,
                            );
                        }
                        "noControlCharactersInRegex" => {
                            result.no_control_characters_in_regex =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noControlCharactersInRegex",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noDebugger" => {
                            result.no_debugger = RuleConfiguration::deserialize_from_rule_name(
                                "noDebugger",
                                value,
                                diagnostics,
                            );
                        }
                        "noDoubleEquals" => {
                            result.no_double_equals = RuleConfiguration::deserialize_from_rule_name(
                                "noDoubleEquals",
                                value,
                                diagnostics,
                            );
                        }
                        "noDuplicateCase" => {
                            result.no_duplicate_case =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noDuplicateCase",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noDuplicateClassMembers" => {
                            result.no_duplicate_class_members =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noDuplicateClassMembers",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noDuplicateJsxProps" => {
                            result.no_duplicate_jsx_props =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noDuplicateJsxProps",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noDuplicateObjectKeys" => {
                            result.no_duplicate_object_keys =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noDuplicateObjectKeys",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noDuplicateParameters" => {
                            result.no_duplicate_parameters =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noDuplicateParameters",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noEmptyInterface" => {
                            result.no_empty_interface =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noEmptyInterface",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noExplicitAny" => {
                            result.no_explicit_any = RuleConfiguration::deserialize_from_rule_name(
                                "noExplicitAny",
                                value,
                                diagnostics,
                            );
                        }
                        "noExtraNonNullAssertion" => {
                            result.no_extra_non_null_assertion =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noExtraNonNullAssertion",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noFallthroughSwitchClause" => {
                            result.no_fallthrough_switch_clause =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noFallthroughSwitchClause",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noFunctionAssign" => {
                            result.no_function_assign =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noFunctionAssign",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noGlobalIsFinite" => {
                            result.no_global_is_finite =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noGlobalIsFinite",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noGlobalIsNan" => {
                            result.no_global_is_nan = RuleConfiguration::deserialize_from_rule_name(
                                "noGlobalIsNan",
                                value,
                                diagnostics,
                            );
                        }
                        "noImportAssign" => {
                            result.no_import_assign = RuleConfiguration::deserialize_from_rule_name(
                                "noImportAssign",
                                value,
                                diagnostics,
                            );
                        }
                        "noLabelVar" => {
                            result.no_label_var = RuleConfiguration::deserialize_from_rule_name(
                                "noLabelVar",
                                value,
                                diagnostics,
                            );
                        }
                        "noPrototypeBuiltins" => {
                            result.no_prototype_builtins =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noPrototypeBuiltins",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noRedeclare" => {
                            result.no_redeclare = RuleConfiguration::deserialize_from_rule_name(
                                "noRedeclare",
                                value,
                                diagnostics,
                            );
                        }
                        "noRedundantUseStrict" => {
                            result.no_redundant_use_strict =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noRedundantUseStrict",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noSelfCompare" => {
                            result.no_self_compare = RuleConfiguration::deserialize_from_rule_name(
                                "noSelfCompare",
                                value,
                                diagnostics,
                            );
                        }
                        "noShadowRestrictedNames" => {
                            result.no_shadow_restricted_names =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noShadowRestrictedNames",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noSparseArray" => {
                            result.no_sparse_array = RuleConfiguration::deserialize_from_rule_name(
                                "noSparseArray",
                                value,
                                diagnostics,
                            );
                        }
                        "noUnsafeDeclarationMerging" => {
                            result.no_unsafe_declaration_merging =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUnsafeDeclarationMerging",
                                    value,
                                    diagnostics,
                                );
                        }
                        "noUnsafeNegation" => {
                            result.no_unsafe_negation =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "noUnsafeNegation",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useDefaultSwitchClauseLast" => {
                            result.use_default_switch_clause_last =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useDefaultSwitchClauseLast",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useGetterReturn" => {
                            result.use_getter_return =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useGetterReturn",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useIsArray" => {
                            result.use_is_array = RuleConfiguration::deserialize_from_rule_name(
                                "useIsArray",
                                value,
                                diagnostics,
                            );
                        }
                        "useNamespaceKeyword" => {
                            result.use_namespace_keyword =
                                RuleConfiguration::deserialize_from_rule_name(
                                    "useNamespaceKeyword",
                                    value,
                                    diagnostics,
                                );
                        }
                        "useValidTypeof" => {
                            result.use_valid_typeof = RuleConfiguration::deserialize_from_rule_name(
                                "useValidTypeof",
                                value,
                                diagnostics,
                            );
                        }
                        _ => {
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                key.text(),
                                key_range,
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
                            ));
                        }
                    }
                }
                if recommended_is_set
                    && matches!(result.recommended, Some(true))
                    && matches!(result.all, Some(true))
                {
                    diagnostics . push (DeserializationDiagnostic :: new (markup ! (< Emphasis > "'recommended'" < / Emphasis > " and " < Emphasis > "'all'" < / Emphasis > " can't be both " < Emphasis > "'true'" < / Emphasis > ". You should choose only one of them.")) . with_range (range) . with_note (markup ! ("Biome will fallback to its defaults for this section."))) ;
                    return Some(Self::Output::default());
                }
                Some(result)
            }
        }
        value.deserialize(Visitor, diagnostics)
    }
}
