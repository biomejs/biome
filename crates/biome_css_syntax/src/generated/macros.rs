//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](biome_rowan::SyntaxNode::kind)"]
#[doc = r" of the provided [biome_rowan::SyntaxNode] and constructs the appropriate"]
#[doc = r" AstNode type for it, then execute the provided expression over it."]
#[doc = r""]
#[doc = r" # Examples"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r" map_syntax_node!(syntax_node, node => node.format())"]
#[doc = r" ```"]
#[macro_export]
macro_rules! map_syntax_node {
    ($ node : expr , $ pattern : pat => $ body : expr) => {
        match $node {
            node => match $crate::CssSyntaxNode::kind(&node) {
                $crate::CssSyntaxKind::CSS_AT_RULE => {
                    let $pattern = unsafe { $crate::CssAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_ATTRIBUTE_MATCHER => {
                    let $pattern = unsafe { $crate::CssAttributeMatcher::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_ATTRIBUTE_MATCHER_VALUE => {
                    let $pattern = unsafe { $crate::CssAttributeMatcherValue::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_ATTRIBUTE_NAME => {
                    let $pattern = unsafe { $crate::CssAttributeName::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_ATTRIBUTE_SELECTOR => {
                    let $pattern = unsafe { $crate::CssAttributeSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BINARY_EXPRESS => {
                    let $pattern = unsafe { $crate::CssBinaryExpress::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CHARSET_AT_RULE => {
                    let $pattern = unsafe { $crate::CssCharsetAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CLASS_SELECTOR => {
                    let $pattern = unsafe { $crate::CssClassSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_COLOR => {
                    let $pattern = unsafe { $crate::CssColor::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_COLOR_PROFILE_AT_RULE => {
                    let $pattern = unsafe { $crate::CssColorProfileAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_COMPLEX_SELECTOR => {
                    let $pattern = unsafe { $crate::CssComplexSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_COMPOUND_SELECTOR => {
                    let $pattern = unsafe { $crate::CssCompoundSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CONTAINER_AND_QUERY => {
                    let $pattern = unsafe { $crate::CssContainerAndQuery::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CONTAINER_AT_RULE => {
                    let $pattern = unsafe { $crate::CssContainerAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CONTAINER_NOT_QUERY => {
                    let $pattern = unsafe { $crate::CssContainerNotQuery::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CONTAINER_OR_QUERY => {
                    let $pattern = unsafe { $crate::CssContainerOrQuery::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CONTAINER_QUERY_IN_PARENS => {
                    let $pattern =
                        unsafe { $crate::CssContainerQueryInParens::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CONTAINER_SIZE_FEATURE_IN_PARENS => {
                    let $pattern =
                        unsafe { $crate::CssContainerSizeFeatureInParens::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CONTAINER_STYLE_AND_QUERY => {
                    let $pattern =
                        unsafe { $crate::CssContainerStyleAndQuery::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CONTAINER_STYLE_IN_PARENS => {
                    let $pattern =
                        unsafe { $crate::CssContainerStyleInParens::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CONTAINER_STYLE_NOT_QUERY => {
                    let $pattern =
                        unsafe { $crate::CssContainerStyleNotQuery::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CONTAINER_STYLE_OR_QUERY => {
                    let $pattern = unsafe { $crate::CssContainerStyleOrQuery::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CONTAINER_STYLE_QUERY_IN_PARENS => {
                    let $pattern =
                        unsafe { $crate::CssContainerStyleQueryInParens::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_COUNTER_STYLE_AT_RULE => {
                    let $pattern = unsafe { $crate::CssCounterStyleAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CUSTOM_PROPERTY => {
                    let $pattern = unsafe { $crate::CssCustomProperty::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DECLARATION => {
                    let $pattern = unsafe { $crate::CssDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DECLARATION_IMPORTANT => {
                    let $pattern = unsafe { $crate::CssDeclarationImportant::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DECLARATION_LIST_BLOCK => {
                    let $pattern = unsafe { $crate::CssDeclarationListBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_FONT_FACE_AT_RULE => {
                    let $pattern = unsafe { $crate::CssFontFaceAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_FONT_PALETTE_VALUES_AT_RULE => {
                    let $pattern =
                        unsafe { $crate::CssFontPaletteValuesAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_ID_SELECTOR => {
                    let $pattern = unsafe { $crate::CssIdSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_IDENTIFIER => {
                    let $pattern = unsafe { $crate::CssIdentifier::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_KEYFRAMES_AT_RULE => {
                    let $pattern = unsafe { $crate::CssKeyframesAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_KEYFRAMES_BLOCK => {
                    let $pattern = unsafe { $crate::CssKeyframesBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_KEYFRAMES_IDENT_SELECTOR => {
                    let $pattern =
                        unsafe { $crate::CssKeyframesIdentSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_KEYFRAMES_ITEM => {
                    let $pattern = unsafe { $crate::CssKeyframesItem::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_KEYFRAMES_PERCENTAGE_SELECTOR => {
                    let $pattern =
                        unsafe { $crate::CssKeyframesPercentageSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_LIST_OF_COMPONENT_VALUES_EXPRESS => {
                    let $pattern =
                        unsafe { $crate::CssListOfComponentValuesExpress::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_MEDIA_AND_CONDITION => {
                    let $pattern = unsafe { $crate::CssMediaAndCondition::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_MEDIA_AND_TYPE_QUERY => {
                    let $pattern = unsafe { $crate::CssMediaAndTypeQuery::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_MEDIA_AT_RULE => {
                    let $pattern = unsafe { $crate::CssMediaAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_MEDIA_CONDITION_IN_PARENS => {
                    let $pattern =
                        unsafe { $crate::CssMediaConditionInParens::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_MEDIA_CONDITION_QUERY => {
                    let $pattern = unsafe { $crate::CssMediaConditionQuery::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_MEDIA_FEATURE_IN_PARENS => {
                    let $pattern = unsafe { $crate::CssMediaFeatureInParens::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_MEDIA_NOT_CONDITION => {
                    let $pattern = unsafe { $crate::CssMediaNotCondition::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_MEDIA_OR_CONDITION => {
                    let $pattern = unsafe { $crate::CssMediaOrCondition::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_MEDIA_TYPE => {
                    let $pattern = unsafe { $crate::CssMediaType::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_MEDIA_TYPE_QUERY => {
                    let $pattern = unsafe { $crate::CssMediaTypeQuery::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_NAMED_NAMESPACE_PREFIX => {
                    let $pattern = unsafe { $crate::CssNamedNamespacePrefix::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_NAMESPACE => {
                    let $pattern = unsafe { $crate::CssNamespace::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_NTH_OFFSET => {
                    let $pattern = unsafe { $crate::CssNthOffset::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_NUMBER => {
                    let $pattern = unsafe { $crate::CssNumber::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PARAMETER => {
                    let $pattern = unsafe { $crate::CssParameter::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PARENTHESIZED_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::CssParenthesizedExpression::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PERCENT_DIMENSION => {
                    let $pattern = unsafe { $crate::CssPercentDimension::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PERCENTAGE => {
                    let $pattern = unsafe { $crate::CssPercentage::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR => {
                    let $pattern = unsafe {
                        $crate::CssPseudoClassFunctionCompoundSelector::new_unchecked(node)
                    };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST => {
                    let $pattern = unsafe {
                        $crate::CssPseudoClassFunctionCompoundSelectorList::new_unchecked(node)
                    };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER => {
                    let $pattern =
                        unsafe { $crate::CssPseudoClassFunctionIdentifier::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_NTH => {
                    let $pattern =
                        unsafe { $crate::CssPseudoClassFunctionNth::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST => {
                    let $pattern = unsafe {
                        $crate::CssPseudoClassFunctionRelativeSelectorList::new_unchecked(node)
                    };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_SELECTOR => {
                    let $pattern =
                        unsafe { $crate::CssPseudoClassFunctionSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST => {
                    let $pattern =
                        unsafe { $crate::CssPseudoClassFunctionSelectorList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST => {
                    let $pattern =
                        unsafe { $crate::CssPseudoClassFunctionValueList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_IDENTIFIER => {
                    let $pattern = unsafe { $crate::CssPseudoClassIdentifier::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_NTH => {
                    let $pattern = unsafe { $crate::CssPseudoClassNth::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_NTH_IDENTIFIER => {
                    let $pattern =
                        unsafe { $crate::CssPseudoClassNthIdentifier::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_NTH_NUMBER => {
                    let $pattern = unsafe { $crate::CssPseudoClassNthNumber::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_NTH_SELECTOR => {
                    let $pattern =
                        unsafe { $crate::CssPseudoClassNthSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_OF_NTH_SELECTOR => {
                    let $pattern =
                        unsafe { $crate::CssPseudoClassOfNthSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_SELECTOR => {
                    let $pattern = unsafe { $crate::CssPseudoClassSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER => {
                    let $pattern =
                        unsafe { $crate::CssPseudoElementFunctionIdentifier::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR => {
                    let $pattern =
                        unsafe { $crate::CssPseudoElementFunctionSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_ELEMENT_IDENTIFIER => {
                    let $pattern =
                        unsafe { $crate::CssPseudoElementIdentifier::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_ELEMENT_SELECTOR => {
                    let $pattern = unsafe { $crate::CssPseudoElementSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_QUERY_FEATURE_BOOLEAN => {
                    let $pattern = unsafe { $crate::CssQueryFeatureBoolean::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_QUERY_FEATURE_PLAIN => {
                    let $pattern = unsafe { $crate::CssQueryFeaturePlain::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_QUERY_FEATURE_RANGE => {
                    let $pattern = unsafe { $crate::CssQueryFeatureRange::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_QUERY_FEATURE_RANGE_COMPARISON => {
                    let $pattern =
                        unsafe { $crate::CssQueryFeatureRangeComparison::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_QUERY_FEATURE_RANGE_INTERVAL => {
                    let $pattern =
                        unsafe { $crate::CssQueryFeatureRangeInterval::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_QUERY_FEATURE_REVERSE_RANGE => {
                    let $pattern =
                        unsafe { $crate::CssQueryFeatureReverseRange::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_RATIO => {
                    let $pattern = unsafe { $crate::CssRatio::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_REGULAR_DIMENSION => {
                    let $pattern = unsafe { $crate::CssRegularDimension::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_RELATIVE_SELECTOR => {
                    let $pattern = unsafe { $crate::CssRelativeSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_ROOT => {
                    let $pattern = unsafe { $crate::CssRoot::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_RULE => {
                    let $pattern = unsafe { $crate::CssRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_RULE_LIST_BLOCK => {
                    let $pattern = unsafe { $crate::CssRuleListBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SIMPLE_FUNCTION => {
                    let $pattern = unsafe { $crate::CssSimpleFunction::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_STRING => {
                    let $pattern = unsafe { $crate::CssString::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_TYPE_SELECTOR => {
                    let $pattern = unsafe { $crate::CssTypeSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_UNIVERSAL_NAMESPACE_PREFIX => {
                    let $pattern =
                        unsafe { $crate::CssUniversalNamespacePrefix::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_UNIVERSAL_SELECTOR => {
                    let $pattern = unsafe { $crate::CssUniversalSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_URL_FUNCTION => {
                    let $pattern = unsafe { $crate::CssUrlFunction::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_URL_VALUE_RAW => {
                    let $pattern = unsafe { $crate::CssUrlValueRaw::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_VAR_FUNCTION => {
                    let $pattern = unsafe { $crate::CssVarFunction::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_VAR_FUNCTION_VALUE => {
                    let $pattern = unsafe { $crate::CssVarFunctionValue::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS => {
                    let $pattern = unsafe { $crate::CssBogus::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_AT_RULE => {
                    let $pattern = unsafe { $crate::CssBogusAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_BLOCK => {
                    let $pattern = unsafe { $crate::CssBogusBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_COMPONENT_VALUE => {
                    let $pattern = unsafe { $crate::CssBogusComponentValue::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_DECLARATION_ITEM => {
                    let $pattern = unsafe { $crate::CssBogusDeclarationItem::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_KEYFRAMES_ITEM => {
                    let $pattern = unsafe { $crate::CssBogusKeyframesItem::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_MEDIA_QUERY => {
                    let $pattern = unsafe { $crate::CssBogusMediaQuery::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_PARAMETER => {
                    let $pattern = unsafe { $crate::CssBogusParameter::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_PSEUDO_CLASS => {
                    let $pattern = unsafe { $crate::CssBogusPseudoClass::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_PSEUDO_ELEMENT => {
                    let $pattern = unsafe { $crate::CssBogusPseudoElement::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_RULE => {
                    let $pattern = unsafe { $crate::CssBogusRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_SELECTOR => {
                    let $pattern = unsafe { $crate::CssBogusSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_SUB_SELECTOR => {
                    let $pattern = unsafe { $crate::CssBogusSubSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_COMPONENT_VALUE_LIST => {
                    let $pattern = unsafe { $crate::CssComponentValueList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_COMPOUND_SELECTOR_LIST => {
                    let $pattern = unsafe { $crate::CssCompoundSelectorList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DECLARATION_LIST => {
                    let $pattern = unsafe { $crate::CssDeclarationList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_KEYFRAMES_ITEM_LIST => {
                    let $pattern = unsafe { $crate::CssKeyframesItemList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_KEYFRAMES_SELECTOR_LIST => {
                    let $pattern = unsafe { $crate::CssKeyframesSelectorList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_MEDIA_QUERY_LIST => {
                    let $pattern = unsafe { $crate::CssMediaQueryList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PARAMETER_LIST => {
                    let $pattern = unsafe { $crate::CssParameterList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_VALUE_LIST => {
                    let $pattern = unsafe { $crate::CssPseudoValueList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_RELATIVE_SELECTOR_LIST => {
                    let $pattern = unsafe { $crate::CssRelativeSelectorList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_RULE_LIST => {
                    let $pattern = unsafe { $crate::CssRuleList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SELECTOR_LIST => {
                    let $pattern = unsafe { $crate::CssSelectorList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SUB_SELECTOR_LIST => {
                    let $pattern = unsafe { $crate::CssSubSelectorList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
