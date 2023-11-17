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
                $crate::CssSyntaxKind::CSS_ANY_FUNCTION => {
                    let $pattern = unsafe { $crate::CssAnyFunction::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_AT_KEYFRAMES => {
                    let $pattern = unsafe { $crate::CssAtKeyframes::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_AT_KEYFRAMES_BODY => {
                    let $pattern = unsafe { $crate::CssAtKeyframesBody::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_AT_MEDIA => {
                    let $pattern = unsafe { $crate::CssAtMedia::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_AT_MEDIA_QUERY => {
                    let $pattern = unsafe { $crate::CssAtMediaQuery::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_AT_MEDIA_QUERY_CONSEQUENT => {
                    let $pattern =
                        unsafe { $crate::CssAtMediaQueryConsequent::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_AT_MEDIA_QUERY_FEATURE => {
                    let $pattern = unsafe { $crate::CssAtMediaQueryFeature::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_AT_MEDIA_QUERY_FEATURE_BOOLEAN => {
                    let $pattern =
                        unsafe { $crate::CssAtMediaQueryFeatureBoolean::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_AT_MEDIA_QUERY_FEATURE_COMPARE => {
                    let $pattern =
                        unsafe { $crate::CssAtMediaQueryFeatureCompare::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_AT_MEDIA_QUERY_FEATURE_PLAIN => {
                    let $pattern =
                        unsafe { $crate::CssAtMediaQueryFeaturePlain::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_AT_MEDIA_QUERY_FEATURE_RANGE => {
                    let $pattern =
                        unsafe { $crate::CssAtMediaQueryFeatureRange::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_AT_MEDIA_QUERY_RANGE => {
                    let $pattern = unsafe { $crate::CssAtMediaQueryRange::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_ATTRIBUTE_SELECTOR => {
                    let $pattern = unsafe { $crate::CssAttributeSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BLOCK => {
                    let $pattern = unsafe { $crate::CssBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CLASS_SELECTOR => {
                    let $pattern = unsafe { $crate::CssClassSelector::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_DIMENSION => {
                    let $pattern = unsafe { $crate::CssDimension::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_KEYFRAMES_BLOCK => {
                    let $pattern = unsafe { $crate::CssKeyframesBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_KEYFRAMES_SELECTOR => {
                    let $pattern = unsafe { $crate::CssKeyframesSelector::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_NTH_MULTIPLIER => {
                    let $pattern = unsafe { $crate::CssNthMultiplier::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_RATIO => {
                    let $pattern = unsafe { $crate::CssRatio::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_UNIVERSAL_SELECTOR => {
                    let $pattern = unsafe { $crate::CssUniversalSelector::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_BOGUS_BODY => {
                    let $pattern = unsafe { $crate::CssBogusBody::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_AT_KEYFRAMES_ITEM_LIST => {
                    let $pattern = unsafe { $crate::CssAtKeyframesItemList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_AT_MEDIA_QUERY_LIST => {
                    let $pattern = unsafe { $crate::CssAtMediaQueryList::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_KEYFRAMES_SELECTOR_LIST => {
                    let $pattern = unsafe { $crate::CssKeyframesSelectorList::new_unchecked(node) };
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
