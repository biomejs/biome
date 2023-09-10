//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use biome_css_syntax::{
    CssSyntaxElement as SyntaxElement, CssSyntaxNode as SyntaxNode, CssSyntaxToken as SyntaxToken,
    *,
};
use biome_rowan::AstNode;
pub fn css_any_function(css_simple_function: CssSimpleFunction) -> CssAnyFunction {
    CssAnyFunction::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_ANY_FUNCTION,
        [Some(SyntaxElement::Node(css_simple_function.into_syntax()))],
    ))
}
pub fn css_at_keyframes(
    at_token: SyntaxToken,
    keyframes_token: SyntaxToken,
    name: CssIdentifier,
    css_string: CssString,
    body: CssAtKeyframesBody,
) -> CssAtKeyframes {
    CssAtKeyframes::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_KEYFRAMES,
        [
            Some(SyntaxElement::Token(at_token)),
            Some(SyntaxElement::Token(keyframes_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(css_string.into_syntax())),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn css_at_keyframes_body(
    l_curly_token: SyntaxToken,
    items: CssAtKeyframesItemList,
    r_curly_token: SyntaxToken,
) -> CssAtKeyframesBody {
    CssAtKeyframesBody::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_KEYFRAMES_BODY,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn css_at_media(
    at_token: SyntaxToken,
    media_token: SyntaxToken,
    query_list: CssAtMediaQueryList,
    l_curly_token: SyntaxToken,
    body: AnyCssRule,
    r_curly_token: SyntaxToken,
) -> CssAtMedia {
    CssAtMedia::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_MEDIA,
        [
            Some(SyntaxElement::Token(at_token)),
            Some(SyntaxElement::Token(media_token)),
            Some(SyntaxElement::Node(query_list.into_syntax())),
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn css_at_media_query(
    condition_token: SyntaxToken,
    or_token: SyntaxToken,
    ty: AnyCssAtMediaQueryType,
) -> CssAtMediaQueryBuilder {
    CssAtMediaQueryBuilder {
        condition_token,
        or_token,
        ty,
        only_token: None,
        consequent: None,
    }
}
pub struct CssAtMediaQueryBuilder {
    condition_token: SyntaxToken,
    or_token: SyntaxToken,
    ty: AnyCssAtMediaQueryType,
    only_token: Option<SyntaxToken>,
    consequent: Option<CssAtMediaQueryConsequent>,
}
impl CssAtMediaQueryBuilder {
    pub fn with_only_token(mut self, only_token: SyntaxToken) -> Self {
        self.only_token = Some(only_token);
        self
    }
    pub fn with_consequent(mut self, consequent: CssAtMediaQueryConsequent) -> Self {
        self.consequent = Some(consequent);
        self
    }
    pub fn build(self) -> CssAtMediaQuery {
        CssAtMediaQuery::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_AT_MEDIA_QUERY,
            [
                Some(SyntaxElement::Token(self.condition_token)),
                Some(SyntaxElement::Token(self.or_token)),
                self.only_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.ty.into_syntax())),
                self.consequent
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn css_at_media_query_consequent(
    and_token: SyntaxToken,
    ty: AnyCssAtMediaQueryType,
) -> CssAtMediaQueryConsequentBuilder {
    CssAtMediaQueryConsequentBuilder {
        and_token,
        ty,
        condition_token: None,
    }
}
pub struct CssAtMediaQueryConsequentBuilder {
    and_token: SyntaxToken,
    ty: AnyCssAtMediaQueryType,
    condition_token: Option<SyntaxToken>,
}
impl CssAtMediaQueryConsequentBuilder {
    pub fn with_condition_token(mut self, condition_token: SyntaxToken) -> Self {
        self.condition_token = Some(condition_token);
        self
    }
    pub fn build(self) -> CssAtMediaQueryConsequent {
        CssAtMediaQueryConsequent::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_AT_MEDIA_QUERY_CONSEQUENT,
            [
                Some(SyntaxElement::Token(self.and_token)),
                self.condition_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.ty.into_syntax())),
            ],
        ))
    }
}
pub fn css_at_media_query_feature(
    l_paren_token: SyntaxToken,
    feature: AnyCssAtMediaQueryFeatureType,
    r_paren_token: SyntaxToken,
) -> CssAtMediaQueryFeature {
    CssAtMediaQueryFeature::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_MEDIA_QUERY_FEATURE,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(feature.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_at_media_query_feature_boolean(
    css_identifier: CssIdentifier,
) -> CssAtMediaQueryFeatureBoolean {
    CssAtMediaQueryFeatureBoolean::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_MEDIA_QUERY_FEATURE_BOOLEAN,
        [Some(SyntaxElement::Node(css_identifier.into_syntax()))],
    ))
}
pub fn css_at_media_query_feature_compare(
    name: CssIdentifier,
    range: CssAtMediaQueryRange,
    value: AnyCssValue,
) -> CssAtMediaQueryFeatureCompare {
    CssAtMediaQueryFeatureCompare::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_MEDIA_QUERY_FEATURE_COMPARE,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(range.into_syntax())),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn css_at_media_query_feature_plain(
    name: CssIdentifier,
    colon_token: SyntaxToken,
    value: AnyCssValue,
) -> CssAtMediaQueryFeaturePlain {
    CssAtMediaQueryFeaturePlain::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_MEDIA_QUERY_FEATURE_PLAIN,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn css_at_media_query_feature_range(
    first_value: AnyCssValue,
    first_range: CssAtMediaQueryRange,
    name: CssIdentifier,
    second_value: AnyCssValue,
    second_range: CssAtMediaQueryRange,
) -> CssAtMediaQueryFeatureRange {
    CssAtMediaQueryFeatureRange::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_MEDIA_QUERY_FEATURE_RANGE,
        [
            Some(SyntaxElement::Node(first_value.into_syntax())),
            Some(SyntaxElement::Node(first_range.into_syntax())),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(second_value.into_syntax())),
            Some(SyntaxElement::Node(second_range.into_syntax())),
        ],
    ))
}
pub fn css_at_media_query_range(
    r_angle_token: SyntaxToken,
    l_angle_token: SyntaxToken,
    greater_than_equal_token: SyntaxToken,
    less_than_equal_token: SyntaxToken,
) -> CssAtMediaQueryRange {
    CssAtMediaQueryRange::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_MEDIA_QUERY_RANGE,
        [
            Some(SyntaxElement::Token(r_angle_token)),
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Token(greater_than_equal_token)),
            Some(SyntaxElement::Token(less_than_equal_token)),
        ],
    ))
}
pub fn css_attribute(
    l_brack_token: SyntaxToken,
    attribute_name: CssAttributeName,
    r_brack_token: SyntaxToken,
) -> CssAttributeBuilder {
    CssAttributeBuilder {
        l_brack_token,
        attribute_name,
        r_brack_token,
        attribute_meta: None,
    }
}
pub struct CssAttributeBuilder {
    l_brack_token: SyntaxToken,
    attribute_name: CssAttributeName,
    r_brack_token: SyntaxToken,
    attribute_meta: Option<CssAttributeMeta>,
}
impl CssAttributeBuilder {
    pub fn with_attribute_meta(mut self, attribute_meta: CssAttributeMeta) -> Self {
        self.attribute_meta = Some(attribute_meta);
        self
    }
    pub fn build(self) -> CssAttribute {
        CssAttribute::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_ATTRIBUTE,
            [
                Some(SyntaxElement::Token(self.l_brack_token)),
                Some(SyntaxElement::Node(self.attribute_name.into_syntax())),
                self.attribute_meta
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
            ],
        ))
    }
}
pub fn css_attribute_matcher(
    matcher_type_token: SyntaxToken,
    exactly_or_hyphen_token: SyntaxToken,
    prefix_token: SyntaxToken,
    suffix_token: SyntaxToken,
    times_assign_token: SyntaxToken,
    eq_token: SyntaxToken,
    matcher_name: CssString,
    css_identifier: CssIdentifier,
) -> CssAttributeMatcher {
    CssAttributeMatcher::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_ATTRIBUTE_MATCHER,
        [
            Some(SyntaxElement::Token(matcher_type_token)),
            Some(SyntaxElement::Token(exactly_or_hyphen_token)),
            Some(SyntaxElement::Token(prefix_token)),
            Some(SyntaxElement::Token(suffix_token)),
            Some(SyntaxElement::Token(times_assign_token)),
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(matcher_name.into_syntax())),
            Some(SyntaxElement::Node(css_identifier.into_syntax())),
        ],
    ))
}
pub fn css_attribute_meta() -> CssAttributeMetaBuilder {
    CssAttributeMetaBuilder {
        attribute_matcher: None,
        attribute_modifier: None,
    }
}
pub struct CssAttributeMetaBuilder {
    attribute_matcher: Option<CssAttributeMatcher>,
    attribute_modifier: Option<CssAttributeModifier>,
}
impl CssAttributeMetaBuilder {
    pub fn with_attribute_matcher(mut self, attribute_matcher: CssAttributeMatcher) -> Self {
        self.attribute_matcher = Some(attribute_matcher);
        self
    }
    pub fn with_attribute_modifier(mut self, attribute_modifier: CssAttributeModifier) -> Self {
        self.attribute_modifier = Some(attribute_modifier);
        self
    }
    pub fn build(self) -> CssAttributeMeta {
        CssAttributeMeta::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_ATTRIBUTE_META,
            [
                self.attribute_matcher
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.attribute_modifier
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn css_attribute_modifier(i_token: SyntaxToken) -> CssAttributeModifier {
    CssAttributeModifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_ATTRIBUTE_MODIFIER,
        [Some(SyntaxElement::Token(i_token))],
    ))
}
pub fn css_attribute_name(css_string: CssString) -> CssAttributeName {
    CssAttributeName::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_ATTRIBUTE_NAME,
        [Some(SyntaxElement::Node(css_string.into_syntax()))],
    ))
}
pub fn css_attribute_selector_pattern(
    name: CssIdentifier,
    attribute_list: CssAttributeList,
) -> CssAttributeSelectorPattern {
    CssAttributeSelectorPattern::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_ATTRIBUTE_SELECTOR_PATTERN,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(attribute_list.into_syntax())),
        ],
    ))
}
pub fn css_block(
    l_curly_token: SyntaxToken,
    declaration_list: CssDeclarationList,
    r_curly_token: SyntaxToken,
) -> CssBlock {
    CssBlock::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BLOCK,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(declaration_list.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn css_class_selector_pattern(
    dot_token: SyntaxToken,
    name: CssIdentifier,
) -> CssClassSelectorPattern {
    CssClassSelectorPattern::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CLASS_SELECTOR_PATTERN,
        [
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn css_combinator_selector_pattern(
    left: AnyCssSelectorPattern,
    combinator_token: SyntaxToken,
    plus_token: SyntaxToken,
    bitwise_not_token: SyntaxToken,
    css_space_literal_token: SyntaxToken,
    right: AnyCssSelectorPattern,
) -> CssCombinatorSelectorPattern {
    CssCombinatorSelectorPattern::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COMBINATOR_SELECTOR_PATTERN,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(combinator_token)),
            Some(SyntaxElement::Token(plus_token)),
            Some(SyntaxElement::Token(bitwise_not_token)),
            Some(SyntaxElement::Token(css_space_literal_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_custom_property(value_token: SyntaxToken) -> CssCustomProperty {
    CssCustomProperty::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CUSTOM_PROPERTY,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_declaration(
    name: CssIdentifier,
    css_custom_property: CssCustomProperty,
    colon_token: SyntaxToken,
    value: AnyCssValue,
) -> CssDeclarationBuilder {
    CssDeclarationBuilder {
        name,
        css_custom_property,
        colon_token,
        value,
        important: None,
    }
}
pub struct CssDeclarationBuilder {
    name: CssIdentifier,
    css_custom_property: CssCustomProperty,
    colon_token: SyntaxToken,
    value: AnyCssValue,
    important: Option<CssDeclarationImportant>,
}
impl CssDeclarationBuilder {
    pub fn with_important(mut self, important: CssDeclarationImportant) -> Self {
        self.important = Some(important);
        self
    }
    pub fn build(self) -> CssDeclaration {
        CssDeclaration::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_DECLARATION,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.css_custom_property.into_syntax())),
                Some(SyntaxElement::Token(self.colon_token)),
                Some(SyntaxElement::Node(self.value.into_syntax())),
                self.important
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn css_declaration_important(
    excl_token: SyntaxToken,
    important_token: SyntaxToken,
) -> CssDeclarationImportant {
    CssDeclarationImportant::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DECLARATION_IMPORTANT,
        [
            Some(SyntaxElement::Token(excl_token)),
            Some(SyntaxElement::Token(important_token)),
        ],
    ))
}
pub fn css_dimension(value: CssNumber, unit: CssIdentifier) -> CssDimension {
    CssDimension::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DIMENSION,
        [
            Some(SyntaxElement::Node(value.into_syntax())),
            Some(SyntaxElement::Node(unit.into_syntax())),
        ],
    ))
}
pub fn css_id_selector_pattern(
    hash_token: SyntaxToken,
    name: CssIdentifier,
) -> CssIdSelectorPattern {
    CssIdSelectorPattern::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_ID_SELECTOR_PATTERN,
        [
            Some(SyntaxElement::Token(hash_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn css_identifier(value_token: SyntaxToken) -> CssIdentifier {
    CssIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_IDENTIFIER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_keyframes_block(
    selectors: CssKeyframesSelectorList,
    l_curly_token: SyntaxToken,
    declarations: CssDeclarationList,
    r_curly_token: SyntaxToken,
) -> CssKeyframesBlock {
    CssKeyframesBlock::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_KEYFRAMES_BLOCK,
        [
            Some(SyntaxElement::Node(selectors.into_syntax())),
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(declarations.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn css_keyframes_selector(
    from_token: SyntaxToken,
    to_token: SyntaxToken,
    css_percentage: CssPercentage,
) -> CssKeyframesSelector {
    CssKeyframesSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_KEYFRAMES_SELECTOR,
        [
            Some(SyntaxElement::Token(from_token)),
            Some(SyntaxElement::Token(to_token)),
            Some(SyntaxElement::Node(css_percentage.into_syntax())),
        ],
    ))
}
pub fn css_number(value_token: SyntaxToken) -> CssNumber {
    CssNumber::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_NUMBER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_parameter(any_css_value: AnyCssValue) -> CssParameter {
    CssParameter::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PARAMETER,
        [Some(SyntaxElement::Node(any_css_value.into_syntax()))],
    ))
}
pub fn css_percentage(value: CssNumber, reminder_token: SyntaxToken) -> CssPercentage {
    CssPercentage::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PERCENTAGE,
        [
            Some(SyntaxElement::Node(value.into_syntax())),
            Some(SyntaxElement::Token(reminder_token)),
        ],
    ))
}
pub fn css_pseudo_class_selector_pattern(
    colon_token: SyntaxToken,
    name: CssIdentifier,
) -> CssPseudoClassSelectorPatternBuilder {
    CssPseudoClassSelectorPatternBuilder {
        colon_token,
        name,
        parameters: None,
    }
}
pub struct CssPseudoClassSelectorPatternBuilder {
    colon_token: SyntaxToken,
    name: CssIdentifier,
    parameters: Option<CssPseudoClassSelectorPatternParameters>,
}
impl CssPseudoClassSelectorPatternBuilder {
    pub fn with_parameters(mut self, parameters: CssPseudoClassSelectorPatternParameters) -> Self {
        self.parameters = Some(parameters);
        self
    }
    pub fn build(self) -> CssPseudoClassSelectorPattern {
        CssPseudoClassSelectorPattern::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_PSEUDO_CLASS_SELECTOR_PATTERN,
            [
                Some(SyntaxElement::Token(self.colon_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn css_pseudo_class_selector_pattern_parameters(
    l_paren_token: SyntaxToken,
    parameter: AnyCssValue,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassSelectorPatternParameters {
    CssPseudoClassSelectorPatternParameters::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_SELECTOR_PATTERN_PARAMETERS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(parameter.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_ratio(numerator: CssNumber, denominator: CssNumber) -> CssRatio {
    CssRatio::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_RATIO,
        [
            Some(SyntaxElement::Node(numerator.into_syntax())),
            Some(SyntaxElement::Node(denominator.into_syntax())),
        ],
    ))
}
pub fn css_root(rules: CssRuleList, eof_token: SyntaxToken) -> CssRoot {
    CssRoot::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_ROOT,
        [
            Some(SyntaxElement::Node(rules.into_syntax())),
            Some(SyntaxElement::Token(eof_token)),
        ],
    ))
}
pub fn css_rule(prelude: CssSelectorList, block: CssBlock) -> CssRule {
    CssRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_RULE,
        [
            Some(SyntaxElement::Node(prelude.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_selector(pattern_list: CssAnySelectorPatternList) -> CssSelector {
    CssSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SELECTOR,
        [Some(SyntaxElement::Node(pattern_list.into_syntax()))],
    ))
}
pub fn css_simple_function(
    name: CssIdentifier,
    l_paren_token: SyntaxToken,
    items: CssParameterList,
    r_paren_token: SyntaxToken,
) -> CssSimpleFunction {
    CssSimpleFunction::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SIMPLE_FUNCTION,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_string(value_token: SyntaxToken) -> CssString {
    CssString::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_STRING,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_type_selector_pattern(ident: CssIdentifier) -> CssTypeSelectorPattern {
    CssTypeSelectorPattern::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_TYPE_SELECTOR_PATTERN,
        [Some(SyntaxElement::Node(ident.into_syntax()))],
    ))
}
pub fn css_universal_selector_pattern(star_token: SyntaxToken) -> CssUniversalSelectorPattern {
    CssUniversalSelectorPattern::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_UNIVERSAL_SELECTOR_PATTERN,
        [Some(SyntaxElement::Token(star_token))],
    ))
}
pub fn css_var_function(
    var_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    property: CssCustomProperty,
    r_paren_token: SyntaxToken,
) -> CssVarFunctionBuilder {
    CssVarFunctionBuilder {
        var_token,
        l_paren_token,
        property,
        r_paren_token,
        value: None,
    }
}
pub struct CssVarFunctionBuilder {
    var_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    property: CssCustomProperty,
    r_paren_token: SyntaxToken,
    value: Option<CssVarFunctionValue>,
}
impl CssVarFunctionBuilder {
    pub fn with_value(mut self, value: CssVarFunctionValue) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> CssVarFunction {
        CssVarFunction::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_VAR_FUNCTION,
            [
                Some(SyntaxElement::Token(self.var_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.property.into_syntax())),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn css_var_function_value(
    comma_token: SyntaxToken,
    value: CssIdentifier,
) -> CssVarFunctionValue {
    CssVarFunctionValue::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_VAR_FUNCTION_VALUE,
        [
            Some(SyntaxElement::Token(comma_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn css_any_selector_pattern_list<I>(items: I) -> CssAnySelectorPatternList
where
    I: IntoIterator<Item = AnyCssSelectorPattern>,
    I::IntoIter: ExactSizeIterator,
{
    CssAnySelectorPatternList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_ANY_SELECTOR_PATTERN_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_at_keyframes_item_list<I>(items: I) -> CssAtKeyframesItemList
where
    I: IntoIterator<Item = CssKeyframesBlock>,
    I::IntoIter: ExactSizeIterator,
{
    CssAtKeyframesItemList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_KEYFRAMES_ITEM_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_at_media_query_list<I, S>(items: I, separators: S) -> CssAtMediaQueryList
where
    I: IntoIterator<Item = CssAtMediaQuery>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssAtMediaQueryList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_MEDIA_QUERY_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_attribute_list<I>(items: I) -> CssAttributeList
where
    I: IntoIterator<Item = CssAttribute>,
    I::IntoIter: ExactSizeIterator,
{
    CssAttributeList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_ATTRIBUTE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_declaration_list<I>(items: I) -> CssDeclarationList
where
    I: IntoIterator<Item = CssDeclaration>,
    I::IntoIter: ExactSizeIterator,
{
    CssDeclarationList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DECLARATION_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_keyframes_selector_list<I, S>(items: I, separators: S) -> CssKeyframesSelectorList
where
    I: IntoIterator<Item = CssKeyframesSelector>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssKeyframesSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_KEYFRAMES_SELECTOR_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_parameter_list<I>(items: I) -> CssParameterList
where
    I: IntoIterator<Item = CssParameter>,
    I::IntoIter: ExactSizeIterator,
{
    CssParameterList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PARAMETER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_rule_list<I>(items: I) -> CssRuleList
where
    I: IntoIterator<Item = AnyCssRule>,
    I::IntoIter: ExactSizeIterator,
{
    CssRuleList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_RULE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_selector_list<I, S>(items: I, separators: S) -> CssSelectorList
where
    I: IntoIterator<Item = CssSelector>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SELECTOR_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_bogus<I>(slots: I) -> CssBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogus::unwrap_cast(SyntaxNode::new_detached(CssSyntaxKind::CSS_BOGUS, slots))
}
