//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::configuration::linter::*;
use crate::Rules;
use biome_console::markup;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor, Text,
    VisitableType,
};
use biome_rowan::TextRange;
impl Deserializable for Rules {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Rules;
            const EXPECTED_TYPE: VisitableType = VisitableType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                range: TextRange,
                _name: &str,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut recommended_is_set = false;
                let mut result = Self::Output::default();
                for (key, value) in members.flatten() {
                    let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                        continue;
                    };
                    match key_text.text() {
                        "recommended" => {
                            recommended_is_set = true;
                            result.recommended =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "all" => {
                            result.all =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "a11y" => {
                            result.a11y =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "complexity" => {
                            result.complexity =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "correctness" => {
                            result.correctness =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "nursery" => {
                            result.nursery =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "performance" => {
                            result.performance =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "security" => {
                            result.security =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "style" => {
                            result.style =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "suspicious" => {
                            result.suspicious =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        unknown_key => {
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                unknown_key,
                                key.range(),
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
        value.deserialize(Visitor, name, diagnostics)
    }
}
impl Deserializable for A11y {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = A11y;
            const EXPECTED_TYPE: VisitableType = VisitableType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                range: TextRange,
                _name: &str,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut recommended_is_set = false;
                let mut result = Self::Output::default();
                for (key, value) in members.flatten() {
                    let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                        continue;
                    };
                    match key_text.text() {
                        "recommended" => {
                            recommended_is_set = true;
                            result.recommended =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "all" => {
                            result.all =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "noAccessKey" => {
                            result.no_access_key =
                                Deserializable::deserialize(&value, "noAccessKey", diagnostics);
                        }
                        "noAriaUnsupportedElements" => {
                            result.no_aria_unsupported_elements = Deserializable::deserialize(
                                &value,
                                "noAriaUnsupportedElements",
                                diagnostics,
                            );
                        }
                        "noAutofocus" => {
                            result.no_autofocus =
                                Deserializable::deserialize(&value, "noAutofocus", diagnostics);
                        }
                        "noBlankTarget" => {
                            result.no_blank_target =
                                Deserializable::deserialize(&value, "noBlankTarget", diagnostics);
                        }
                        "noDistractingElements" => {
                            result.no_distracting_elements = Deserializable::deserialize(
                                &value,
                                "noDistractingElements",
                                diagnostics,
                            );
                        }
                        "noHeaderScope" => {
                            result.no_header_scope =
                                Deserializable::deserialize(&value, "noHeaderScope", diagnostics);
                        }
                        "noInteractiveElementToNoninteractiveRole" => {
                            result.no_interactive_element_to_noninteractive_role =
                                Deserializable::deserialize(
                                    &value,
                                    "noInteractiveElementToNoninteractiveRole",
                                    diagnostics,
                                );
                        }
                        "noNoninteractiveElementToInteractiveRole" => {
                            result.no_noninteractive_element_to_interactive_role =
                                Deserializable::deserialize(
                                    &value,
                                    "noNoninteractiveElementToInteractiveRole",
                                    diagnostics,
                                );
                        }
                        "noNoninteractiveTabindex" => {
                            result.no_noninteractive_tabindex = Deserializable::deserialize(
                                &value,
                                "noNoninteractiveTabindex",
                                diagnostics,
                            );
                        }
                        "noPositiveTabindex" => {
                            result.no_positive_tabindex = Deserializable::deserialize(
                                &value,
                                "noPositiveTabindex",
                                diagnostics,
                            );
                        }
                        "noRedundantAlt" => {
                            result.no_redundant_alt =
                                Deserializable::deserialize(&value, "noRedundantAlt", diagnostics);
                        }
                        "noRedundantRoles" => {
                            result.no_redundant_roles = Deserializable::deserialize(
                                &value,
                                "noRedundantRoles",
                                diagnostics,
                            );
                        }
                        "noSvgWithoutTitle" => {
                            result.no_svg_without_title = Deserializable::deserialize(
                                &value,
                                "noSvgWithoutTitle",
                                diagnostics,
                            );
                        }
                        "useAltText" => {
                            result.use_alt_text =
                                Deserializable::deserialize(&value, "useAltText", diagnostics);
                        }
                        "useAnchorContent" => {
                            result.use_anchor_content = Deserializable::deserialize(
                                &value,
                                "useAnchorContent",
                                diagnostics,
                            );
                        }
                        "useAriaActivedescendantWithTabindex" => {
                            result.use_aria_activedescendant_with_tabindex =
                                Deserializable::deserialize(
                                    &value,
                                    "useAriaActivedescendantWithTabindex",
                                    diagnostics,
                                );
                        }
                        "useAriaPropsForRole" => {
                            result.use_aria_props_for_role = Deserializable::deserialize(
                                &value,
                                "useAriaPropsForRole",
                                diagnostics,
                            );
                        }
                        "useButtonType" => {
                            result.use_button_type =
                                Deserializable::deserialize(&value, "useButtonType", diagnostics);
                        }
                        "useHeadingContent" => {
                            result.use_heading_content = Deserializable::deserialize(
                                &value,
                                "useHeadingContent",
                                diagnostics,
                            );
                        }
                        "useHtmlLang" => {
                            result.use_html_lang =
                                Deserializable::deserialize(&value, "useHtmlLang", diagnostics);
                        }
                        "useIframeTitle" => {
                            result.use_iframe_title =
                                Deserializable::deserialize(&value, "useIframeTitle", diagnostics);
                        }
                        "useKeyWithClickEvents" => {
                            result.use_key_with_click_events = Deserializable::deserialize(
                                &value,
                                "useKeyWithClickEvents",
                                diagnostics,
                            );
                        }
                        "useKeyWithMouseEvents" => {
                            result.use_key_with_mouse_events = Deserializable::deserialize(
                                &value,
                                "useKeyWithMouseEvents",
                                diagnostics,
                            );
                        }
                        "useMediaCaption" => {
                            result.use_media_caption =
                                Deserializable::deserialize(&value, "useMediaCaption", diagnostics);
                        }
                        "useValidAnchor" => {
                            result.use_valid_anchor =
                                Deserializable::deserialize(&value, "useValidAnchor", diagnostics);
                        }
                        "useValidAriaProps" => {
                            result.use_valid_aria_props = Deserializable::deserialize(
                                &value,
                                "useValidAriaProps",
                                diagnostics,
                            );
                        }
                        "useValidAriaValues" => {
                            result.use_valid_aria_values = Deserializable::deserialize(
                                &value,
                                "useValidAriaValues",
                                diagnostics,
                            );
                        }
                        "useValidLang" => {
                            result.use_valid_lang =
                                Deserializable::deserialize(&value, "useValidLang", diagnostics);
                        }
                        unknown_key => {
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                unknown_key,
                                key.range(),
                                &[
                                    "recommended",
                                    "all",
                                    "noAccessKey",
                                    "noAriaUnsupportedElements",
                                    "noAutofocus",
                                    "noBlankTarget",
                                    "noDistractingElements",
                                    "noHeaderScope",
                                    "noInteractiveElementToNoninteractiveRole",
                                    "noNoninteractiveElementToInteractiveRole",
                                    "noNoninteractiveTabindex",
                                    "noPositiveTabindex",
                                    "noRedundantAlt",
                                    "noRedundantRoles",
                                    "noSvgWithoutTitle",
                                    "useAltText",
                                    "useAnchorContent",
                                    "useAriaActivedescendantWithTabindex",
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
        value.deserialize(Visitor, name, diagnostics)
    }
}
impl Deserializable for Complexity {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Complexity;
            const EXPECTED_TYPE: VisitableType = VisitableType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                range: TextRange,
                _name: &str,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut recommended_is_set = false;
                let mut result = Self::Output::default();
                for (key, value) in members.flatten() {
                    let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                        continue;
                    };
                    match key_text.text() {
                        "recommended" => {
                            recommended_is_set = true;
                            result.recommended =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "all" => {
                            result.all =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "noBannedTypes" => {
                            result.no_banned_types =
                                Deserializable::deserialize(&value, "noBannedTypes", diagnostics);
                        }
                        "noExcessiveCognitiveComplexity" => {
                            result.no_excessive_cognitive_complexity = Deserializable::deserialize(
                                &value,
                                "noExcessiveCognitiveComplexity",
                                diagnostics,
                            );
                        }
                        "noExtraBooleanCast" => {
                            result.no_extra_boolean_cast = Deserializable::deserialize(
                                &value,
                                "noExtraBooleanCast",
                                diagnostics,
                            );
                        }
                        "noForEach" => {
                            result.no_for_each =
                                Deserializable::deserialize(&value, "noForEach", diagnostics);
                        }
                        "noMultipleSpacesInRegularExpressionLiterals" => {
                            result.no_multiple_spaces_in_regular_expression_literals =
                                Deserializable::deserialize(
                                    &value,
                                    "noMultipleSpacesInRegularExpressionLiterals",
                                    diagnostics,
                                );
                        }
                        "noStaticOnlyClass" => {
                            result.no_static_only_class = Deserializable::deserialize(
                                &value,
                                "noStaticOnlyClass",
                                diagnostics,
                            );
                        }
                        "noThisInStatic" => {
                            result.no_this_in_static =
                                Deserializable::deserialize(&value, "noThisInStatic", diagnostics);
                        }
                        "noUselessCatch" => {
                            result.no_useless_catch =
                                Deserializable::deserialize(&value, "noUselessCatch", diagnostics);
                        }
                        "noUselessConstructor" => {
                            result.no_useless_constructor = Deserializable::deserialize(
                                &value,
                                "noUselessConstructor",
                                diagnostics,
                            );
                        }
                        "noUselessEmptyExport" => {
                            result.no_useless_empty_export = Deserializable::deserialize(
                                &value,
                                "noUselessEmptyExport",
                                diagnostics,
                            );
                        }
                        "noUselessFragments" => {
                            result.no_useless_fragments = Deserializable::deserialize(
                                &value,
                                "noUselessFragments",
                                diagnostics,
                            );
                        }
                        "noUselessLabel" => {
                            result.no_useless_label =
                                Deserializable::deserialize(&value, "noUselessLabel", diagnostics);
                        }
                        "noUselessRename" => {
                            result.no_useless_rename =
                                Deserializable::deserialize(&value, "noUselessRename", diagnostics);
                        }
                        "noUselessSwitchCase" => {
                            result.no_useless_switch_case = Deserializable::deserialize(
                                &value,
                                "noUselessSwitchCase",
                                diagnostics,
                            );
                        }
                        "noUselessThisAlias" => {
                            result.no_useless_this_alias = Deserializable::deserialize(
                                &value,
                                "noUselessThisAlias",
                                diagnostics,
                            );
                        }
                        "noUselessTypeConstraint" => {
                            result.no_useless_type_constraint = Deserializable::deserialize(
                                &value,
                                "noUselessTypeConstraint",
                                diagnostics,
                            );
                        }
                        "noVoid" => {
                            result.no_void =
                                Deserializable::deserialize(&value, "noVoid", diagnostics);
                        }
                        "noWith" => {
                            result.no_with =
                                Deserializable::deserialize(&value, "noWith", diagnostics);
                        }
                        "useArrowFunction" => {
                            result.use_arrow_function = Deserializable::deserialize(
                                &value,
                                "useArrowFunction",
                                diagnostics,
                            );
                        }
                        "useFlatMap" => {
                            result.use_flat_map =
                                Deserializable::deserialize(&value, "useFlatMap", diagnostics);
                        }
                        "useLiteralKeys" => {
                            result.use_literal_keys =
                                Deserializable::deserialize(&value, "useLiteralKeys", diagnostics);
                        }
                        "useOptionalChain" => {
                            result.use_optional_chain = Deserializable::deserialize(
                                &value,
                                "useOptionalChain",
                                diagnostics,
                            );
                        }
                        "useSimpleNumberKeys" => {
                            result.use_simple_number_keys = Deserializable::deserialize(
                                &value,
                                "useSimpleNumberKeys",
                                diagnostics,
                            );
                        }
                        "useSimplifiedLogicExpression" => {
                            result.use_simplified_logic_expression = Deserializable::deserialize(
                                &value,
                                "useSimplifiedLogicExpression",
                                diagnostics,
                            );
                        }
                        unknown_key => {
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                unknown_key,
                                key.range(),
                                &[
                                    "recommended",
                                    "all",
                                    "noBannedTypes",
                                    "noExcessiveCognitiveComplexity",
                                    "noExtraBooleanCast",
                                    "noForEach",
                                    "noMultipleSpacesInRegularExpressionLiterals",
                                    "noStaticOnlyClass",
                                    "noThisInStatic",
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
                                    "useArrowFunction",
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
        value.deserialize(Visitor, name, diagnostics)
    }
}
impl Deserializable for Correctness {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Correctness;
            const EXPECTED_TYPE: VisitableType = VisitableType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                range: TextRange,
                _name: &str,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut recommended_is_set = false;
                let mut result = Self::Output::default();
                for (key, value) in members.flatten() {
                    let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                        continue;
                    };
                    match key_text.text() {
                        "recommended" => {
                            recommended_is_set = true;
                            result.recommended =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "all" => {
                            result.all =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "noChildrenProp" => {
                            result.no_children_prop =
                                Deserializable::deserialize(&value, "noChildrenProp", diagnostics);
                        }
                        "noConstAssign" => {
                            result.no_const_assign =
                                Deserializable::deserialize(&value, "noConstAssign", diagnostics);
                        }
                        "noConstantCondition" => {
                            result.no_constant_condition = Deserializable::deserialize(
                                &value,
                                "noConstantCondition",
                                diagnostics,
                            );
                        }
                        "noConstructorReturn" => {
                            result.no_constructor_return = Deserializable::deserialize(
                                &value,
                                "noConstructorReturn",
                                diagnostics,
                            );
                        }
                        "noEmptyCharacterClassInRegex" => {
                            result.no_empty_character_class_in_regex = Deserializable::deserialize(
                                &value,
                                "noEmptyCharacterClassInRegex",
                                diagnostics,
                            );
                        }
                        "noEmptyPattern" => {
                            result.no_empty_pattern =
                                Deserializable::deserialize(&value, "noEmptyPattern", diagnostics);
                        }
                        "noGlobalObjectCalls" => {
                            result.no_global_object_calls = Deserializable::deserialize(
                                &value,
                                "noGlobalObjectCalls",
                                diagnostics,
                            );
                        }
                        "noInnerDeclarations" => {
                            result.no_inner_declarations = Deserializable::deserialize(
                                &value,
                                "noInnerDeclarations",
                                diagnostics,
                            );
                        }
                        "noInvalidConstructorSuper" => {
                            result.no_invalid_constructor_super = Deserializable::deserialize(
                                &value,
                                "noInvalidConstructorSuper",
                                diagnostics,
                            );
                        }
                        "noInvalidNewBuiltin" => {
                            result.no_invalid_new_builtin = Deserializable::deserialize(
                                &value,
                                "noInvalidNewBuiltin",
                                diagnostics,
                            );
                        }
                        "noNewSymbol" => {
                            result.no_new_symbol =
                                Deserializable::deserialize(&value, "noNewSymbol", diagnostics);
                        }
                        "noNonoctalDecimalEscape" => {
                            result.no_nonoctal_decimal_escape = Deserializable::deserialize(
                                &value,
                                "noNonoctalDecimalEscape",
                                diagnostics,
                            );
                        }
                        "noPrecisionLoss" => {
                            result.no_precision_loss =
                                Deserializable::deserialize(&value, "noPrecisionLoss", diagnostics);
                        }
                        "noRenderReturnValue" => {
                            result.no_render_return_value = Deserializable::deserialize(
                                &value,
                                "noRenderReturnValue",
                                diagnostics,
                            );
                        }
                        "noSelfAssign" => {
                            result.no_self_assign =
                                Deserializable::deserialize(&value, "noSelfAssign", diagnostics);
                        }
                        "noSetterReturn" => {
                            result.no_setter_return =
                                Deserializable::deserialize(&value, "noSetterReturn", diagnostics);
                        }
                        "noStringCaseMismatch" => {
                            result.no_string_case_mismatch = Deserializable::deserialize(
                                &value,
                                "noStringCaseMismatch",
                                diagnostics,
                            );
                        }
                        "noSwitchDeclarations" => {
                            result.no_switch_declarations = Deserializable::deserialize(
                                &value,
                                "noSwitchDeclarations",
                                diagnostics,
                            );
                        }
                        "noUndeclaredVariables" => {
                            result.no_undeclared_variables = Deserializable::deserialize(
                                &value,
                                "noUndeclaredVariables",
                                diagnostics,
                            );
                        }
                        "noUnnecessaryContinue" => {
                            result.no_unnecessary_continue = Deserializable::deserialize(
                                &value,
                                "noUnnecessaryContinue",
                                diagnostics,
                            );
                        }
                        "noUnreachable" => {
                            result.no_unreachable =
                                Deserializable::deserialize(&value, "noUnreachable", diagnostics);
                        }
                        "noUnreachableSuper" => {
                            result.no_unreachable_super = Deserializable::deserialize(
                                &value,
                                "noUnreachableSuper",
                                diagnostics,
                            );
                        }
                        "noUnsafeFinally" => {
                            result.no_unsafe_finally =
                                Deserializable::deserialize(&value, "noUnsafeFinally", diagnostics);
                        }
                        "noUnsafeOptionalChaining" => {
                            result.no_unsafe_optional_chaining = Deserializable::deserialize(
                                &value,
                                "noUnsafeOptionalChaining",
                                diagnostics,
                            );
                        }
                        "noUnusedLabels" => {
                            result.no_unused_labels =
                                Deserializable::deserialize(&value, "noUnusedLabels", diagnostics);
                        }
                        "noUnusedVariables" => {
                            result.no_unused_variables = Deserializable::deserialize(
                                &value,
                                "noUnusedVariables",
                                diagnostics,
                            );
                        }
                        "noVoidElementsWithChildren" => {
                            result.no_void_elements_with_children = Deserializable::deserialize(
                                &value,
                                "noVoidElementsWithChildren",
                                diagnostics,
                            );
                        }
                        "noVoidTypeReturn" => {
                            result.no_void_type_return = Deserializable::deserialize(
                                &value,
                                "noVoidTypeReturn",
                                diagnostics,
                            );
                        }
                        "useExhaustiveDependencies" => {
                            result.use_exhaustive_dependencies = Deserializable::deserialize(
                                &value,
                                "useExhaustiveDependencies",
                                diagnostics,
                            );
                        }
                        "useHookAtTopLevel" => {
                            result.use_hook_at_top_level = Deserializable::deserialize(
                                &value,
                                "useHookAtTopLevel",
                                diagnostics,
                            );
                        }
                        "useIsNan" => {
                            result.use_is_nan =
                                Deserializable::deserialize(&value, "useIsNan", diagnostics);
                        }
                        "useValidForDirection" => {
                            result.use_valid_for_direction = Deserializable::deserialize(
                                &value,
                                "useValidForDirection",
                                diagnostics,
                            );
                        }
                        "useYield" => {
                            result.use_yield =
                                Deserializable::deserialize(&value, "useYield", diagnostics);
                        }
                        unknown_key => {
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                unknown_key,
                                key.range(),
                                &[
                                    "recommended",
                                    "all",
                                    "noChildrenProp",
                                    "noConstAssign",
                                    "noConstantCondition",
                                    "noConstructorReturn",
                                    "noEmptyCharacterClassInRegex",
                                    "noEmptyPattern",
                                    "noGlobalObjectCalls",
                                    "noInnerDeclarations",
                                    "noInvalidConstructorSuper",
                                    "noInvalidNewBuiltin",
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
        value.deserialize(Visitor, name, diagnostics)
    }
}
impl Deserializable for Nursery {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Nursery;
            const EXPECTED_TYPE: VisitableType = VisitableType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                range: TextRange,
                _name: &str,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut recommended_is_set = false;
                let mut result = Self::Output::default();
                for (key, value) in members.flatten() {
                    let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                        continue;
                    };
                    match key_text.text() {
                        "recommended" => {
                            recommended_is_set = true;
                            result.recommended =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "all" => {
                            result.all =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "noAriaHiddenOnFocusable" => {
                            result.no_aria_hidden_on_focusable = Deserializable::deserialize(
                                &value,
                                "noAriaHiddenOnFocusable",
                                diagnostics,
                            );
                        }
                        "noDefaultExport" => {
                            result.no_default_export =
                                Deserializable::deserialize(&value, "noDefaultExport", diagnostics);
                        }
                        "noDuplicateJsonKeys" => {
                            result.no_duplicate_json_keys = Deserializable::deserialize(
                                &value,
                                "noDuplicateJsonKeys",
                                diagnostics,
                            );
                        }
                        "noEmptyBlockStatements" => {
                            result.no_empty_block_statements = Deserializable::deserialize(
                                &value,
                                "noEmptyBlockStatements",
                                diagnostics,
                            );
                        }
                        "noImplicitAnyLet" => {
                            result.no_implicit_any_let = Deserializable::deserialize(
                                &value,
                                "noImplicitAnyLet",
                                diagnostics,
                            );
                        }
                        "noInvalidUseBeforeDeclaration" => {
                            result.no_invalid_use_before_declaration = Deserializable::deserialize(
                                &value,
                                "noInvalidUseBeforeDeclaration",
                                diagnostics,
                            );
                        }
                        "noMisleadingCharacterClass" => {
                            result.no_misleading_character_class = Deserializable::deserialize(
                                &value,
                                "noMisleadingCharacterClass",
                                diagnostics,
                            );
                        }
                        "noNodejsModules" => {
                            result.no_nodejs_modules =
                                Deserializable::deserialize(&value, "noNodejsModules", diagnostics);
                        }
                        "noUnusedImports" => {
                            result.no_unused_imports =
                                Deserializable::deserialize(&value, "noUnusedImports", diagnostics);
                        }
                        "noUnusedPrivateClassMembers" => {
                            result.no_unused_private_class_members = Deserializable::deserialize(
                                &value,
                                "noUnusedPrivateClassMembers",
                                diagnostics,
                            );
                        }
                        "noUselessLoneBlockStatements" => {
                            result.no_useless_lone_block_statements = Deserializable::deserialize(
                                &value,
                                "noUselessLoneBlockStatements",
                                diagnostics,
                            );
                        }
                        "noUselessTernary" => {
                            result.no_useless_ternary = Deserializable::deserialize(
                                &value,
                                "noUselessTernary",
                                diagnostics,
                            );
                        }
                        "useAwait" => {
                            result.use_await =
                                Deserializable::deserialize(&value, "useAwait", diagnostics);
                        }
                        "useExportType" => {
                            result.use_export_type =
                                Deserializable::deserialize(&value, "useExportType", diagnostics);
                        }
                        "useFilenamingConvention" => {
                            result.use_filenaming_convention = Deserializable::deserialize(
                                &value,
                                "useFilenamingConvention",
                                diagnostics,
                            );
                        }
                        "useForOf" => {
                            result.use_for_of =
                                Deserializable::deserialize(&value, "useForOf", diagnostics);
                        }
                        "useGroupedTypeImport" => {
                            result.use_grouped_type_import = Deserializable::deserialize(
                                &value,
                                "useGroupedTypeImport",
                                diagnostics,
                            );
                        }
                        "useImportRestrictions" => {
                            result.use_import_restrictions = Deserializable::deserialize(
                                &value,
                                "useImportRestrictions",
                                diagnostics,
                            );
                        }
                        "useNodeImportProtocol" => {
                            result.use_node_import_protocol = Deserializable::deserialize(
                                &value,
                                "useNodeImportProtocol",
                                diagnostics,
                            );
                        }
                        "useRegexLiterals" => {
                            result.use_regex_literals = Deserializable::deserialize(
                                &value,
                                "useRegexLiterals",
                                diagnostics,
                            );
                        }
                        "useShorthandFunctionType" => {
                            result.use_shorthand_function_type = Deserializable::deserialize(
                                &value,
                                "useShorthandFunctionType",
                                diagnostics,
                            );
                        }
                        "useValidAriaRole" => {
                            result.use_valid_aria_role = Deserializable::deserialize(
                                &value,
                                "useValidAriaRole",
                                diagnostics,
                            );
                        }
                        unknown_key => {
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                unknown_key,
                                key.range(),
                                &[
                                    "recommended",
                                    "all",
                                    "noAriaHiddenOnFocusable",
                                    "noDefaultExport",
                                    "noDuplicateJsonKeys",
                                    "noEmptyBlockStatements",
                                    "noImplicitAnyLet",
                                    "noInvalidUseBeforeDeclaration",
                                    "noMisleadingCharacterClass",
                                    "noNodejsModules",
                                    "noUnusedImports",
                                    "noUnusedPrivateClassMembers",
                                    "noUselessLoneBlockStatements",
                                    "noUselessTernary",
                                    "useAwait",
                                    "useExportType",
                                    "useFilenamingConvention",
                                    "useForOf",
                                    "useGroupedTypeImport",
                                    "useImportRestrictions",
                                    "useNodeImportProtocol",
                                    "useRegexLiterals",
                                    "useShorthandFunctionType",
                                    "useValidAriaRole",
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
        value.deserialize(Visitor, name, diagnostics)
    }
}
impl Deserializable for Performance {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Performance;
            const EXPECTED_TYPE: VisitableType = VisitableType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                range: TextRange,
                _name: &str,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut recommended_is_set = false;
                let mut result = Self::Output::default();
                for (key, value) in members.flatten() {
                    let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                        continue;
                    };
                    match key_text.text() {
                        "recommended" => {
                            recommended_is_set = true;
                            result.recommended =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "all" => {
                            result.all =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "noAccumulatingSpread" => {
                            result.no_accumulating_spread = Deserializable::deserialize(
                                &value,
                                "noAccumulatingSpread",
                                diagnostics,
                            );
                        }
                        "noDelete" => {
                            result.no_delete =
                                Deserializable::deserialize(&value, "noDelete", diagnostics);
                        }
                        unknown_key => {
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                unknown_key,
                                key.range(),
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
        value.deserialize(Visitor, name, diagnostics)
    }
}
impl Deserializable for Security {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Security;
            const EXPECTED_TYPE: VisitableType = VisitableType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                range: TextRange,
                _name: &str,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut recommended_is_set = false;
                let mut result = Self::Output::default();
                for (key, value) in members.flatten() {
                    let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                        continue;
                    };
                    match key_text.text() {
                        "recommended" => {
                            recommended_is_set = true;
                            result.recommended =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "all" => {
                            result.all =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "noDangerouslySetInnerHtml" => {
                            result.no_dangerously_set_inner_html = Deserializable::deserialize(
                                &value,
                                "noDangerouslySetInnerHtml",
                                diagnostics,
                            );
                        }
                        "noDangerouslySetInnerHtmlWithChildren" => {
                            result.no_dangerously_set_inner_html_with_children =
                                Deserializable::deserialize(
                                    &value,
                                    "noDangerouslySetInnerHtmlWithChildren",
                                    diagnostics,
                                );
                        }
                        unknown_key => {
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                unknown_key,
                                key.range(),
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
        value.deserialize(Visitor, name, diagnostics)
    }
}
impl Deserializable for Style {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Style;
            const EXPECTED_TYPE: VisitableType = VisitableType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                range: TextRange,
                _name: &str,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut recommended_is_set = false;
                let mut result = Self::Output::default();
                for (key, value) in members.flatten() {
                    let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                        continue;
                    };
                    match key_text.text() {
                        "recommended" => {
                            recommended_is_set = true;
                            result.recommended =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "all" => {
                            result.all =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "noArguments" => {
                            result.no_arguments =
                                Deserializable::deserialize(&value, "noArguments", diagnostics);
                        }
                        "noCommaOperator" => {
                            result.no_comma_operator =
                                Deserializable::deserialize(&value, "noCommaOperator", diagnostics);
                        }
                        "noImplicitBoolean" => {
                            result.no_implicit_boolean = Deserializable::deserialize(
                                &value,
                                "noImplicitBoolean",
                                diagnostics,
                            );
                        }
                        "noInferrableTypes" => {
                            result.no_inferrable_types = Deserializable::deserialize(
                                &value,
                                "noInferrableTypes",
                                diagnostics,
                            );
                        }
                        "noNamespace" => {
                            result.no_namespace =
                                Deserializable::deserialize(&value, "noNamespace", diagnostics);
                        }
                        "noNegationElse" => {
                            result.no_negation_else =
                                Deserializable::deserialize(&value, "noNegationElse", diagnostics);
                        }
                        "noNonNullAssertion" => {
                            result.no_non_null_assertion = Deserializable::deserialize(
                                &value,
                                "noNonNullAssertion",
                                diagnostics,
                            );
                        }
                        "noParameterAssign" => {
                            result.no_parameter_assign = Deserializable::deserialize(
                                &value,
                                "noParameterAssign",
                                diagnostics,
                            );
                        }
                        "noParameterProperties" => {
                            result.no_parameter_properties = Deserializable::deserialize(
                                &value,
                                "noParameterProperties",
                                diagnostics,
                            );
                        }
                        "noRestrictedGlobals" => {
                            result.no_restricted_globals = Deserializable::deserialize(
                                &value,
                                "noRestrictedGlobals",
                                diagnostics,
                            );
                        }
                        "noShoutyConstants" => {
                            result.no_shouty_constants = Deserializable::deserialize(
                                &value,
                                "noShoutyConstants",
                                diagnostics,
                            );
                        }
                        "noUnusedTemplateLiteral" => {
                            result.no_unused_template_literal = Deserializable::deserialize(
                                &value,
                                "noUnusedTemplateLiteral",
                                diagnostics,
                            );
                        }
                        "noUselessElse" => {
                            result.no_useless_else =
                                Deserializable::deserialize(&value, "noUselessElse", diagnostics);
                        }
                        "noVar" => {
                            result.no_var =
                                Deserializable::deserialize(&value, "noVar", diagnostics);
                        }
                        "useAsConstAssertion" => {
                            result.use_as_const_assertion = Deserializable::deserialize(
                                &value,
                                "useAsConstAssertion",
                                diagnostics,
                            );
                        }
                        "useBlockStatements" => {
                            result.use_block_statements = Deserializable::deserialize(
                                &value,
                                "useBlockStatements",
                                diagnostics,
                            );
                        }
                        "useCollapsedElseIf" => {
                            result.use_collapsed_else_if = Deserializable::deserialize(
                                &value,
                                "useCollapsedElseIf",
                                diagnostics,
                            );
                        }
                        "useConst" => {
                            result.use_const =
                                Deserializable::deserialize(&value, "useConst", diagnostics);
                        }
                        "useDefaultParameterLast" => {
                            result.use_default_parameter_last = Deserializable::deserialize(
                                &value,
                                "useDefaultParameterLast",
                                diagnostics,
                            );
                        }
                        "useEnumInitializers" => {
                            result.use_enum_initializers = Deserializable::deserialize(
                                &value,
                                "useEnumInitializers",
                                diagnostics,
                            );
                        }
                        "useExponentiationOperator" => {
                            result.use_exponentiation_operator = Deserializable::deserialize(
                                &value,
                                "useExponentiationOperator",
                                diagnostics,
                            );
                        }
                        "useFragmentSyntax" => {
                            result.use_fragment_syntax = Deserializable::deserialize(
                                &value,
                                "useFragmentSyntax",
                                diagnostics,
                            );
                        }
                        "useLiteralEnumMembers" => {
                            result.use_literal_enum_members = Deserializable::deserialize(
                                &value,
                                "useLiteralEnumMembers",
                                diagnostics,
                            );
                        }
                        "useNamingConvention" => {
                            result.use_naming_convention = Deserializable::deserialize(
                                &value,
                                "useNamingConvention",
                                diagnostics,
                            );
                        }
                        "useNumericLiterals" => {
                            result.use_numeric_literals = Deserializable::deserialize(
                                &value,
                                "useNumericLiterals",
                                diagnostics,
                            );
                        }
                        "useSelfClosingElements" => {
                            result.use_self_closing_elements = Deserializable::deserialize(
                                &value,
                                "useSelfClosingElements",
                                diagnostics,
                            );
                        }
                        "useShorthandArrayType" => {
                            result.use_shorthand_array_type = Deserializable::deserialize(
                                &value,
                                "useShorthandArrayType",
                                diagnostics,
                            );
                        }
                        "useShorthandAssign" => {
                            result.use_shorthand_assign = Deserializable::deserialize(
                                &value,
                                "useShorthandAssign",
                                diagnostics,
                            );
                        }
                        "useSingleCaseStatement" => {
                            result.use_single_case_statement = Deserializable::deserialize(
                                &value,
                                "useSingleCaseStatement",
                                diagnostics,
                            );
                        }
                        "useSingleVarDeclarator" => {
                            result.use_single_var_declarator = Deserializable::deserialize(
                                &value,
                                "useSingleVarDeclarator",
                                diagnostics,
                            );
                        }
                        "useTemplate" => {
                            result.use_template =
                                Deserializable::deserialize(&value, "useTemplate", diagnostics);
                        }
                        "useWhile" => {
                            result.use_while =
                                Deserializable::deserialize(&value, "useWhile", diagnostics);
                        }
                        unknown_key => {
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                unknown_key,
                                key.range(),
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
                                    "noUselessElse",
                                    "noVar",
                                    "useAsConstAssertion",
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
                                    "useShorthandAssign",
                                    "useSingleCaseStatement",
                                    "useSingleVarDeclarator",
                                    "useTemplate",
                                    "useWhile",
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
        value.deserialize(Visitor, name, diagnostics)
    }
}
impl Deserializable for Suspicious {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Suspicious;
            const EXPECTED_TYPE: VisitableType = VisitableType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                range: TextRange,
                _name: &str,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut recommended_is_set = false;
                let mut result = Self::Output::default();
                for (key, value) in members.flatten() {
                    let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                        continue;
                    };
                    match key_text.text() {
                        "recommended" => {
                            recommended_is_set = true;
                            result.recommended =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "all" => {
                            result.all =
                                Deserializable::deserialize(&value, &key_text, diagnostics);
                        }
                        "noApproximativeNumericConstant" => {
                            result.no_approximative_numeric_constant = Deserializable::deserialize(
                                &value,
                                "noApproximativeNumericConstant",
                                diagnostics,
                            );
                        }
                        "noArrayIndexKey" => {
                            result.no_array_index_key =
                                Deserializable::deserialize(&value, "noArrayIndexKey", diagnostics);
                        }
                        "noAssignInExpressions" => {
                            result.no_assign_in_expressions = Deserializable::deserialize(
                                &value,
                                "noAssignInExpressions",
                                diagnostics,
                            );
                        }
                        "noAsyncPromiseExecutor" => {
                            result.no_async_promise_executor = Deserializable::deserialize(
                                &value,
                                "noAsyncPromiseExecutor",
                                diagnostics,
                            );
                        }
                        "noCatchAssign" => {
                            result.no_catch_assign =
                                Deserializable::deserialize(&value, "noCatchAssign", diagnostics);
                        }
                        "noClassAssign" => {
                            result.no_class_assign =
                                Deserializable::deserialize(&value, "noClassAssign", diagnostics);
                        }
                        "noCommentText" => {
                            result.no_comment_text =
                                Deserializable::deserialize(&value, "noCommentText", diagnostics);
                        }
                        "noCompareNegZero" => {
                            result.no_compare_neg_zero = Deserializable::deserialize(
                                &value,
                                "noCompareNegZero",
                                diagnostics,
                            );
                        }
                        "noConfusingLabels" => {
                            result.no_confusing_labels = Deserializable::deserialize(
                                &value,
                                "noConfusingLabels",
                                diagnostics,
                            );
                        }
                        "noConfusingVoidType" => {
                            result.no_confusing_void_type = Deserializable::deserialize(
                                &value,
                                "noConfusingVoidType",
                                diagnostics,
                            );
                        }
                        "noConsoleLog" => {
                            result.no_console_log =
                                Deserializable::deserialize(&value, "noConsoleLog", diagnostics);
                        }
                        "noConstEnum" => {
                            result.no_const_enum =
                                Deserializable::deserialize(&value, "noConstEnum", diagnostics);
                        }
                        "noControlCharactersInRegex" => {
                            result.no_control_characters_in_regex = Deserializable::deserialize(
                                &value,
                                "noControlCharactersInRegex",
                                diagnostics,
                            );
                        }
                        "noDebugger" => {
                            result.no_debugger =
                                Deserializable::deserialize(&value, "noDebugger", diagnostics);
                        }
                        "noDoubleEquals" => {
                            result.no_double_equals =
                                Deserializable::deserialize(&value, "noDoubleEquals", diagnostics);
                        }
                        "noDuplicateCase" => {
                            result.no_duplicate_case =
                                Deserializable::deserialize(&value, "noDuplicateCase", diagnostics);
                        }
                        "noDuplicateClassMembers" => {
                            result.no_duplicate_class_members = Deserializable::deserialize(
                                &value,
                                "noDuplicateClassMembers",
                                diagnostics,
                            );
                        }
                        "noDuplicateJsxProps" => {
                            result.no_duplicate_jsx_props = Deserializable::deserialize(
                                &value,
                                "noDuplicateJsxProps",
                                diagnostics,
                            );
                        }
                        "noDuplicateObjectKeys" => {
                            result.no_duplicate_object_keys = Deserializable::deserialize(
                                &value,
                                "noDuplicateObjectKeys",
                                diagnostics,
                            );
                        }
                        "noDuplicateParameters" => {
                            result.no_duplicate_parameters = Deserializable::deserialize(
                                &value,
                                "noDuplicateParameters",
                                diagnostics,
                            );
                        }
                        "noEmptyInterface" => {
                            result.no_empty_interface = Deserializable::deserialize(
                                &value,
                                "noEmptyInterface",
                                diagnostics,
                            );
                        }
                        "noExplicitAny" => {
                            result.no_explicit_any =
                                Deserializable::deserialize(&value, "noExplicitAny", diagnostics);
                        }
                        "noExtraNonNullAssertion" => {
                            result.no_extra_non_null_assertion = Deserializable::deserialize(
                                &value,
                                "noExtraNonNullAssertion",
                                diagnostics,
                            );
                        }
                        "noFallthroughSwitchClause" => {
                            result.no_fallthrough_switch_clause = Deserializable::deserialize(
                                &value,
                                "noFallthroughSwitchClause",
                                diagnostics,
                            );
                        }
                        "noFunctionAssign" => {
                            result.no_function_assign = Deserializable::deserialize(
                                &value,
                                "noFunctionAssign",
                                diagnostics,
                            );
                        }
                        "noGlobalIsFinite" => {
                            result.no_global_is_finite = Deserializable::deserialize(
                                &value,
                                "noGlobalIsFinite",
                                diagnostics,
                            );
                        }
                        "noGlobalIsNan" => {
                            result.no_global_is_nan =
                                Deserializable::deserialize(&value, "noGlobalIsNan", diagnostics);
                        }
                        "noImportAssign" => {
                            result.no_import_assign =
                                Deserializable::deserialize(&value, "noImportAssign", diagnostics);
                        }
                        "noLabelVar" => {
                            result.no_label_var =
                                Deserializable::deserialize(&value, "noLabelVar", diagnostics);
                        }
                        "noMisleadingInstantiator" => {
                            result.no_misleading_instantiator = Deserializable::deserialize(
                                &value,
                                "noMisleadingInstantiator",
                                diagnostics,
                            );
                        }
                        "noMisrefactoredShorthandAssign" => {
                            result.no_misrefactored_shorthand_assign = Deserializable::deserialize(
                                &value,
                                "noMisrefactoredShorthandAssign",
                                diagnostics,
                            );
                        }
                        "noPrototypeBuiltins" => {
                            result.no_prototype_builtins = Deserializable::deserialize(
                                &value,
                                "noPrototypeBuiltins",
                                diagnostics,
                            );
                        }
                        "noRedeclare" => {
                            result.no_redeclare =
                                Deserializable::deserialize(&value, "noRedeclare", diagnostics);
                        }
                        "noRedundantUseStrict" => {
                            result.no_redundant_use_strict = Deserializable::deserialize(
                                &value,
                                "noRedundantUseStrict",
                                diagnostics,
                            );
                        }
                        "noSelfCompare" => {
                            result.no_self_compare =
                                Deserializable::deserialize(&value, "noSelfCompare", diagnostics);
                        }
                        "noShadowRestrictedNames" => {
                            result.no_shadow_restricted_names = Deserializable::deserialize(
                                &value,
                                "noShadowRestrictedNames",
                                diagnostics,
                            );
                        }
                        "noSparseArray" => {
                            result.no_sparse_array =
                                Deserializable::deserialize(&value, "noSparseArray", diagnostics);
                        }
                        "noUnsafeDeclarationMerging" => {
                            result.no_unsafe_declaration_merging = Deserializable::deserialize(
                                &value,
                                "noUnsafeDeclarationMerging",
                                diagnostics,
                            );
                        }
                        "noUnsafeNegation" => {
                            result.no_unsafe_negation = Deserializable::deserialize(
                                &value,
                                "noUnsafeNegation",
                                diagnostics,
                            );
                        }
                        "useDefaultSwitchClauseLast" => {
                            result.use_default_switch_clause_last = Deserializable::deserialize(
                                &value,
                                "useDefaultSwitchClauseLast",
                                diagnostics,
                            );
                        }
                        "useGetterReturn" => {
                            result.use_getter_return =
                                Deserializable::deserialize(&value, "useGetterReturn", diagnostics);
                        }
                        "useIsArray" => {
                            result.use_is_array =
                                Deserializable::deserialize(&value, "useIsArray", diagnostics);
                        }
                        "useNamespaceKeyword" => {
                            result.use_namespace_keyword = Deserializable::deserialize(
                                &value,
                                "useNamespaceKeyword",
                                diagnostics,
                            );
                        }
                        "useValidTypeof" => {
                            result.use_valid_typeof =
                                Deserializable::deserialize(&value, "useValidTypeof", diagnostics);
                        }
                        unknown_key => {
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                unknown_key,
                                key.range(),
                                &[
                                    "recommended",
                                    "all",
                                    "noApproximativeNumericConstant",
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
                                    "noMisleadingInstantiator",
                                    "noMisrefactoredShorthandAssign",
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
        value.deserialize(Visitor, name, diagnostics)
    }
}
