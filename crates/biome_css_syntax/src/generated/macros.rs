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
                $crate::CssSyntaxKind::CSS_BINARY_EXPRESSION => {
                    let $pattern = unsafe { $crate::CssBinaryExpression::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BRACKETED_VALUE => {
                    let $pattern = unsafe { $crate::CssBracketedValue::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_COMPOSES_IMPORT_SPECIFIER => {
                    let $pattern =
                        unsafe { $crate::CssComposesImportSpecifier::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_COMPOSES_PROPERTY => {
                    let $pattern = unsafe { $crate::CssComposesProperty::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_COMPOSES_PROPERTY_VALUE => {
                    let $pattern = unsafe { $crate::CssComposesPropertyValue::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_CUSTOM_IDENTIFIER => {
                    let $pattern = unsafe { $crate::CssCustomIdentifier::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DASHED_IDENTIFIER => {
                    let $pattern = unsafe { $crate::CssDashedIdentifier::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DECLARATION => {
                    let $pattern = unsafe { $crate::CssDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DECLARATION_BLOCK => {
                    let $pattern = unsafe { $crate::CssDeclarationBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DECLARATION_IMPORTANT => {
                    let $pattern = unsafe { $crate::CssDeclarationImportant::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DECLARATION_OR_AT_RULE_BLOCK => {
                    let $pattern =
                        unsafe { $crate::CssDeclarationOrAtRuleBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DECLARATION_OR_RULE_BLOCK => {
                    let $pattern =
                        unsafe { $crate::CssDeclarationOrRuleBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DECLARATION_WITH_SEMICOLON => {
                    let $pattern =
                        unsafe { $crate::CssDeclarationWithSemicolon::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DOCUMENT_AT_RULE => {
                    let $pattern = unsafe { $crate::CssDocumentAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DOCUMENT_CUSTOM_MATCHER => {
                    let $pattern = unsafe { $crate::CssDocumentCustomMatcher::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_EMPTY_DECLARATION => {
                    let $pattern = unsafe { $crate::CssEmptyDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_FONT_FACE_AT_RULE => {
                    let $pattern = unsafe { $crate::CssFontFaceAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_FONT_FAMILY_NAME => {
                    let $pattern = unsafe { $crate::CssFontFamilyName::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_FONT_FEATURE_VALUES_AT_RULE => {
                    let $pattern =
                        unsafe { $crate::CssFontFeatureValuesAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_FONT_FEATURE_VALUES_BLOCK => {
                    let $pattern =
                        unsafe { $crate::CssFontFeatureValuesBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_FONT_FEATURE_VALUES_ITEM => {
                    let $pattern = unsafe { $crate::CssFontFeatureValuesItem::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_FONT_PALETTE_VALUES_AT_RULE => {
                    let $pattern =
                        unsafe { $crate::CssFontPaletteValuesAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_FUNCTION => {
                    let $pattern = unsafe { $crate::CssFunction::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_GENERIC_DELIMITER => {
                    let $pattern = unsafe { $crate::CssGenericDelimiter::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_GENERIC_PROPERTY => {
                    let $pattern = unsafe { $crate::CssGenericProperty::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_IMPORT_ANONYMOUS_LAYER => {
                    let $pattern = unsafe { $crate::CssImportAnonymousLayer::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_IMPORT_AT_RULE => {
                    let $pattern = unsafe { $crate::CssImportAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_IMPORT_NAMED_LAYER => {
                    let $pattern = unsafe { $crate::CssImportNamedLayer::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_IMPORT_SUPPORTS => {
                    let $pattern = unsafe { $crate::CssImportSupports::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_KEYFRAMES_SCOPE_FUNCTION => {
                    let $pattern =
                        unsafe { $crate::CssKeyframesScopeFunction::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_KEYFRAMES_SCOPE_PREFIX => {
                    let $pattern = unsafe { $crate::CssKeyframesScopePrefix::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_KEYFRAMES_SCOPED_NAME => {
                    let $pattern = unsafe { $crate::CssKeyframesScopedName::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_LAYER_AT_RULE => {
                    let $pattern = unsafe { $crate::CssLayerAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_LAYER_DECLARATION => {
                    let $pattern = unsafe { $crate::CssLayerDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_LAYER_REFERENCE => {
                    let $pattern = unsafe { $crate::CssLayerReference::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::CssListOfComponentValuesExpression::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_MARGIN_AT_RULE => {
                    let $pattern = unsafe { $crate::CssMarginAtRule::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_METAVARIABLE => {
                    let $pattern = unsafe { $crate::CssMetavariable::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_NAMESPACE_AT_RULE => {
                    let $pattern = unsafe { $crate::CssNamespaceAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_NESTED_QUALIFIED_RULE => {
                    let $pattern = unsafe { $crate::CssNestedQualifiedRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_NESTED_SELECTOR => {
                    let $pattern = unsafe { $crate::CssNestedSelector::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_PAGE_AT_RULE => {
                    let $pattern = unsafe { $crate::CssPageAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PAGE_AT_RULE_BLOCK => {
                    let $pattern = unsafe { $crate::CssPageAtRuleBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PAGE_SELECTOR => {
                    let $pattern = unsafe { $crate::CssPageSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PAGE_SELECTOR_PSEUDO => {
                    let $pattern = unsafe { $crate::CssPageSelectorPseudo::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_PERCENTAGE => {
                    let $pattern = unsafe { $crate::CssPercentage::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_POSITION_TRY_AT_RULE => {
                    let $pattern = unsafe { $crate::CssPositionTryAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PROPERTY_AT_RULE => {
                    let $pattern = unsafe { $crate::CssPropertyAtRule::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_CUSTOM_IDENTIFIER_LIST => {
                    let $pattern = unsafe {
                        $crate::CssPseudoClassFunctionCustomIdentifierList::new_unchecked(node)
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
                $crate::CssSyntaxKind::CSS_PSEUDO_ELEMENT_FUNCTION => {
                    let $pattern = unsafe { $crate::CssPseudoElementFunction::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_ELEMENT_FUNCTION_CUSTOM_IDENTIFIER => {
                    let $pattern = unsafe {
                        $crate::CssPseudoElementFunctionCustomIdentifier::new_unchecked(node)
                    };
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
                $crate::CssSyntaxKind::CSS_QUALIFIED_RULE => {
                    let $pattern = unsafe { $crate::CssQualifiedRule::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_RULE_BLOCK => {
                    let $pattern = unsafe { $crate::CssRuleBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SCOPE_AT_RULE => {
                    let $pattern = unsafe { $crate::CssScopeAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SCOPE_EDGE => {
                    let $pattern = unsafe { $crate::CssScopeEdge::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SCOPE_RANGE_END => {
                    let $pattern = unsafe { $crate::CssScopeRangeEnd::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SCOPE_RANGE_INTERVAL => {
                    let $pattern = unsafe { $crate::CssScopeRangeInterval::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SCOPE_RANGE_START => {
                    let $pattern = unsafe { $crate::CssScopeRangeStart::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_STARTING_STYLE_AT_RULE => {
                    let $pattern = unsafe { $crate::CssStartingStyleAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_STRING => {
                    let $pattern = unsafe { $crate::CssString::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SUPPORTS_AND_CONDITION => {
                    let $pattern = unsafe { $crate::CssSupportsAndCondition::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SUPPORTS_AT_RULE => {
                    let $pattern = unsafe { $crate::CssSupportsAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SUPPORTS_CONDITION_IN_PARENS => {
                    let $pattern =
                        unsafe { $crate::CssSupportsConditionInParens::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SUPPORTS_FEATURE_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::CssSupportsFeatureDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SUPPORTS_FEATURE_SELECTOR => {
                    let $pattern =
                        unsafe { $crate::CssSupportsFeatureSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SUPPORTS_NOT_CONDITION => {
                    let $pattern = unsafe { $crate::CssSupportsNotCondition::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SUPPORTS_OR_CONDITION => {
                    let $pattern = unsafe { $crate::CssSupportsOrCondition::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_TYPE_SELECTOR => {
                    let $pattern = unsafe { $crate::CssTypeSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_UNICODE_CODEPOINT => {
                    let $pattern = unsafe { $crate::CssUnicodeCodepoint::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_UNICODE_RANGE => {
                    let $pattern = unsafe { $crate::CssUnicodeRange::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_UNICODE_RANGE_INTERVAL => {
                    let $pattern = unsafe { $crate::CssUnicodeRangeInterval::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_UNICODE_RANGE_WILDCARD => {
                    let $pattern = unsafe { $crate::CssUnicodeRangeWildcard::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_UNKNOWN_BLOCK_AT_RULE => {
                    let $pattern = unsafe { $crate::CssUnknownBlockAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_UNKNOWN_DIMENSION => {
                    let $pattern = unsafe { $crate::CssUnknownDimension::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_UNKNOWN_VALUE_AT_RULE => {
                    let $pattern = unsafe { $crate::CssUnknownValueAtRule::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_VALUE_AT_RULE => {
                    let $pattern = unsafe { $crate::CssValueAtRule::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_VALUE_AT_RULE_DECLARATION_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::CssValueAtRuleDeclarationClause::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_VALUE_AT_RULE_GENERIC_PROPERTY => {
                    let $pattern =
                        unsafe { $crate::CssValueAtRuleGenericProperty::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_VALUE_AT_RULE_IMPORT_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::CssValueAtRuleImportClause::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_VALUE_AT_RULE_IMPORT_SPECIFIER => {
                    let $pattern =
                        unsafe { $crate::CssValueAtRuleImportSpecifier::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_VALUE_AT_RULE_NAMED_IMPORT_SPECIFIER => {
                    let $pattern =
                        unsafe { $crate::CssValueAtRuleNamedImportSpecifier::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_VIEW_TRANSITION_AT_RULE => {
                    let $pattern = unsafe { $crate::CssViewTransitionAtRule::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_BOGUS_CUSTOM_IDENTIFIER => {
                    let $pattern = unsafe { $crate::CssBogusCustomIdentifier::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_DECLARATION_ITEM => {
                    let $pattern = unsafe { $crate::CssBogusDeclarationItem::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_DOCUMENT_MATCHER => {
                    let $pattern = unsafe { $crate::CssBogusDocumentMatcher::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_FONT_FAMILY_NAME => {
                    let $pattern = unsafe { $crate::CssBogusFontFamilyName::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_FONT_FEATURE_VALUES_ITEM => {
                    let $pattern =
                        unsafe { $crate::CssBogusFontFeatureValuesItem::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_KEYFRAMES_ITEM => {
                    let $pattern = unsafe { $crate::CssBogusKeyframesItem::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_KEYFRAMES_NAME => {
                    let $pattern = unsafe { $crate::CssBogusKeyframesName::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_LAYER => {
                    let $pattern = unsafe { $crate::CssBogusLayer::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_MEDIA_QUERY => {
                    let $pattern = unsafe { $crate::CssBogusMediaQuery::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_PAGE_SELECTOR_PSEUDO => {
                    let $pattern =
                        unsafe { $crate::CssBogusPageSelectorPseudo::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_PARAMETER => {
                    let $pattern = unsafe { $crate::CssBogusParameter::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_PROPERTY => {
                    let $pattern = unsafe { $crate::CssBogusProperty::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_PROPERTY_VALUE => {
                    let $pattern = unsafe { $crate::CssBogusPropertyValue::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_BOGUS_SCOPE_RANGE => {
                    let $pattern = unsafe { $crate::CssBogusScopeRange::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_BOGUS_SUPPORTS_CONDITION => {
                    let $pattern =
                        unsafe { $crate::CssBogusSupportsCondition::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_UNICODE_RANGE_VALUE => {
                    let $pattern =
                        unsafe { $crate::CssBogusUnicodeRangeValue::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BOGUS_URL_MODIFIER => {
                    let $pattern = unsafe { $crate::CssBogusUrlModifier::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_UNKNOWN_AT_RULE_COMPONENT_LIST => {
                    let $pattern =
                        unsafe { $crate::CssUnknownAtRuleComponentList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_VALUE_AT_RULE_GENERIC_VALUE => {
                    let $pattern =
                        unsafe { $crate::CssValueAtRuleGenericValue::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BRACKETED_VALUE_LIST => {
                    let $pattern = unsafe { $crate::CssBracketedValueList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_COMPONENT_VALUE_LIST => {
                    let $pattern = unsafe { $crate::CssComponentValueList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_COMPOSES_CLASS_LIST => {
                    let $pattern = unsafe { $crate::CssComposesClassList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_COMPOUND_SELECTOR_LIST => {
                    let $pattern = unsafe { $crate::CssCompoundSelectorList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CUSTOM_IDENTIFIER_LIST => {
                    let $pattern = unsafe { $crate::CssCustomIdentifierList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DECLARATION_LIST => {
                    let $pattern = unsafe { $crate::CssDeclarationList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DECLARATION_OR_AT_RULE_LIST => {
                    let $pattern =
                        unsafe { $crate::CssDeclarationOrAtRuleList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DECLARATION_OR_RULE_LIST => {
                    let $pattern = unsafe { $crate::CssDeclarationOrRuleList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_DOCUMENT_MATCHER_LIST => {
                    let $pattern = unsafe { $crate::CssDocumentMatcherList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_FONT_FAMILY_NAME_LIST => {
                    let $pattern = unsafe { $crate::CssFontFamilyNameList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_FONT_FEATURE_VALUES_ITEM_LIST => {
                    let $pattern =
                        unsafe { $crate::CssFontFeatureValuesItemList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_GENERIC_COMPONENT_VALUE_LIST => {
                    let $pattern =
                        unsafe { $crate::CssGenericComponentValueList::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_LAYER_NAME_LIST => {
                    let $pattern = unsafe { $crate::CssLayerNameList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_LAYER_REFERENCE_LIST => {
                    let $pattern = unsafe { $crate::CssLayerReferenceList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_MEDIA_QUERY_LIST => {
                    let $pattern = unsafe { $crate::CssMediaQueryList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_NESTED_SELECTOR_LIST => {
                    let $pattern = unsafe { $crate::CssNestedSelectorList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PAGE_AT_RULE_ITEM_LIST => {
                    let $pattern = unsafe { $crate::CssPageAtRuleItemList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PAGE_SELECTOR_LIST => {
                    let $pattern = unsafe { $crate::CssPageSelectorList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PAGE_SELECTOR_PSEUDO_LIST => {
                    let $pattern =
                        unsafe { $crate::CssPageSelectorPseudoList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PARAMETER_LIST => {
                    let $pattern = unsafe { $crate::CssParameterList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_ELEMENT_FUNCTION_PARAMETER_LIST => {
                    let $pattern = unsafe {
                        $crate::CssPseudoElementFunctionParameterList::new_unchecked(node)
                    };
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
                $crate::CssSyntaxKind::CSS_URL_MODIFIER_LIST => {
                    let $pattern = unsafe { $crate::CssUrlModifierList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_VALUE_AT_RULE_IMPORT_SPECIFIER_LIST => {
                    let $pattern =
                        unsafe { $crate::CssValueAtRuleImportSpecifierList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_VALUE_AT_RULE_PROPERTY_LIST => {
                    let $pattern =
                        unsafe { $crate::CssValueAtRulePropertyList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
