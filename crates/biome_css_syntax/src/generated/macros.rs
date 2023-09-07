//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](rome_rowan::SyntaxNode::kind)"]
#[doc = r" of the provided [rome_rowan::SyntaxNode] and constructs the appropriate"]
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
                $crate::CssSyntaxKind::CSS_ATTRIBUTE => {
                    let $pattern = unsafe { $crate::CssAttribute::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_ATTRIBUTE_MATCHER => {
                    let $pattern = unsafe { $crate::CssAttributeMatcher::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_ATTRIBUTE_META => {
                    let $pattern = unsafe { $crate::CssAttributeMeta::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_ATTRIBUTE_MODIFIER => {
                    let $pattern = unsafe { $crate::CssAttributeModifier::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_ATTRIBUTE_NAME => {
                    let $pattern = unsafe { $crate::CssAttributeName::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_ATTRIBUTE_SELECTOR_PATTERN => {
                    let $pattern =
                        unsafe { $crate::CssAttributeSelectorPattern::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_BLOCK => {
                    let $pattern = unsafe { $crate::CssBlock::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_CLASS_SELECTOR_PATTERN => {
                    let $pattern = unsafe { $crate::CssClassSelectorPattern::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_COMBINATOR_SELECTOR_PATTERN => {
                    let $pattern =
                        unsafe { $crate::CssCombinatorSelectorPattern::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_ID_SELECTOR_PATTERN => {
                    let $pattern = unsafe { $crate::CssIdSelectorPattern::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_SELECTOR_PATTERN => {
                    let $pattern =
                        unsafe { $crate::CssPseudoClassSelectorPattern::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_PSEUDO_CLASS_SELECTOR_PATTERN_PARAMETERS => {
                    let $pattern = unsafe {
                        $crate::CssPseudoClassSelectorPatternParameters::new_unchecked(node)
                    };
                    $body
                }
                $crate::CssSyntaxKind::CSS_RATIO => {
                    let $pattern = unsafe { $crate::CssRatio::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_SELECTOR => {
                    let $pattern = unsafe { $crate::CssSelector::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_TYPE_SELECTOR_PATTERN => {
                    let $pattern = unsafe { $crate::CssTypeSelectorPattern::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_UNIVERSAL_SELECTOR_PATTERN => {
                    let $pattern =
                        unsafe { $crate::CssUniversalSelectorPattern::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_ANY_SELECTOR_PATTERN_LIST => {
                    let $pattern =
                        unsafe { $crate::CssAnySelectorPatternList::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_ATTRIBUTE_LIST => {
                    let $pattern = unsafe { $crate::CssAttributeList::new_unchecked(node) };
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
                $crate::CssSyntaxKind::CSS_RULE_LIST => {
                    let $pattern = unsafe { $crate::CssRuleList::new_unchecked(node) };
                    $body
                }
                $crate::CssSyntaxKind::CSS_SELECTOR_LIST => {
                    let $pattern = unsafe { $crate::CssSelectorList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
