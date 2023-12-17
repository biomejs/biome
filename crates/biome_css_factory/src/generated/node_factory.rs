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
pub fn css_at_rule(at_token: SyntaxToken, rule: AnyCssAtRule) -> CssAtRule {
    CssAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_AT_RULE,
        [
            Some(SyntaxElement::Token(at_token)),
            Some(SyntaxElement::Node(rule.into_syntax())),
        ],
    ))
}
pub fn css_attribute_matcher(
    operator_token: SyntaxToken,
    value: CssAttributeMatcherValue,
) -> CssAttributeMatcherBuilder {
    CssAttributeMatcherBuilder {
        operator_token,
        value,
        modifier_token: None,
    }
}
pub struct CssAttributeMatcherBuilder {
    operator_token: SyntaxToken,
    value: CssAttributeMatcherValue,
    modifier_token: Option<SyntaxToken>,
}
impl CssAttributeMatcherBuilder {
    pub fn with_modifier_token(mut self, modifier_token: SyntaxToken) -> Self {
        self.modifier_token = Some(modifier_token);
        self
    }
    pub fn build(self) -> CssAttributeMatcher {
        CssAttributeMatcher::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_ATTRIBUTE_MATCHER,
            [
                Some(SyntaxElement::Token(self.operator_token)),
                Some(SyntaxElement::Node(self.value.into_syntax())),
                self.modifier_token.map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn css_attribute_matcher_value(name: AnyCssAttributeMatcherValue) -> CssAttributeMatcherValue {
    CssAttributeMatcherValue::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_ATTRIBUTE_MATCHER_VALUE,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn css_attribute_name(name: CssIdentifier) -> CssAttributeNameBuilder {
    CssAttributeNameBuilder {
        name,
        namespace: None,
    }
}
pub struct CssAttributeNameBuilder {
    name: CssIdentifier,
    namespace: Option<CssNamespace>,
}
impl CssAttributeNameBuilder {
    pub fn with_namespace(mut self, namespace: CssNamespace) -> Self {
        self.namespace = Some(namespace);
        self
    }
    pub fn build(self) -> CssAttributeName {
        CssAttributeName::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_ATTRIBUTE_NAME,
            [
                self.namespace
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
            ],
        ))
    }
}
pub fn css_attribute_selector(
    l_brack_token: SyntaxToken,
    name: CssAttributeName,
    r_brack_token: SyntaxToken,
) -> CssAttributeSelectorBuilder {
    CssAttributeSelectorBuilder {
        l_brack_token,
        name,
        r_brack_token,
        matcher: None,
    }
}
pub struct CssAttributeSelectorBuilder {
    l_brack_token: SyntaxToken,
    name: CssAttributeName,
    r_brack_token: SyntaxToken,
    matcher: Option<CssAttributeMatcher>,
}
impl CssAttributeSelectorBuilder {
    pub fn with_matcher(mut self, matcher: CssAttributeMatcher) -> Self {
        self.matcher = Some(matcher);
        self
    }
    pub fn build(self) -> CssAttributeSelector {
        CssAttributeSelector::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_ATTRIBUTE_SELECTOR,
            [
                Some(SyntaxElement::Token(self.l_brack_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.matcher
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
            ],
        ))
    }
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
pub fn css_charset_at_rule(
    charset_token: SyntaxToken,
    encoding: CssString,
    semicolon_token: SyntaxToken,
) -> CssCharsetAtRule {
    CssCharsetAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CHARSET_AT_RULE,
        [
            Some(SyntaxElement::Token(charset_token)),
            Some(SyntaxElement::Node(encoding.into_syntax())),
            Some(SyntaxElement::Token(semicolon_token)),
        ],
    ))
}
pub fn css_class_selector(dot_token: SyntaxToken, name: CssIdentifier) -> CssClassSelector {
    CssClassSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CLASS_SELECTOR,
        [
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn css_color_profile_at_rule(
    color_profile_token: SyntaxToken,
    name: CssIdentifier,
    block: CssBlock,
) -> CssColorProfileAtRule {
    CssColorProfileAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COLOR_PROFILE_AT_RULE,
        [
            Some(SyntaxElement::Token(color_profile_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_complex_selector(
    left: AnyCssSelector,
    combinator_token: SyntaxToken,
    right: AnyCssSelector,
) -> CssComplexSelector {
    CssComplexSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COMPLEX_SELECTOR,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(combinator_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_compound_selector(sub_selectors: CssSubSelectorList) -> CssCompoundSelectorBuilder {
    CssCompoundSelectorBuilder {
        sub_selectors,
        nesting_selector_token: None,
        simple_selector: None,
    }
}
pub struct CssCompoundSelectorBuilder {
    sub_selectors: CssSubSelectorList,
    nesting_selector_token: Option<SyntaxToken>,
    simple_selector: Option<AnyCssSimpleSelector>,
}
impl CssCompoundSelectorBuilder {
    pub fn with_nesting_selector_token(mut self, nesting_selector_token: SyntaxToken) -> Self {
        self.nesting_selector_token = Some(nesting_selector_token);
        self
    }
    pub fn with_simple_selector(mut self, simple_selector: AnyCssSimpleSelector) -> Self {
        self.simple_selector = Some(simple_selector);
        self
    }
    pub fn build(self) -> CssCompoundSelector {
        CssCompoundSelector::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_COMPOUND_SELECTOR,
            [
                self.nesting_selector_token
                    .map(|token| SyntaxElement::Token(token)),
                self.simple_selector
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.sub_selectors.into_syntax())),
            ],
        ))
    }
}
pub fn css_container_and_query(
    left: AnyCssContainerQueryInParens,
    and_token: SyntaxToken,
    right: AnyCssContainerCombinableQuery,
) -> CssContainerAndQuery {
    CssContainerAndQuery::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_AND_QUERY,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(and_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_container_at_rule(
    container_token: SyntaxToken,
    query: AnyCssContainerQuery,
    block: CssBlock,
) -> CssContainerAtRuleBuilder {
    CssContainerAtRuleBuilder {
        container_token,
        query,
        block,
        name: None,
    }
}
pub struct CssContainerAtRuleBuilder {
    container_token: SyntaxToken,
    query: AnyCssContainerQuery,
    block: CssBlock,
    name: Option<CssIdentifier>,
}
impl CssContainerAtRuleBuilder {
    pub fn with_name(mut self, name: CssIdentifier) -> Self {
        self.name = Some(name);
        self
    }
    pub fn build(self) -> CssContainerAtRule {
        CssContainerAtRule::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_CONTAINER_AT_RULE,
            [
                Some(SyntaxElement::Token(self.container_token)),
                self.name
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.query.into_syntax())),
                Some(SyntaxElement::Node(self.block.into_syntax())),
            ],
        ))
    }
}
pub fn css_container_not_query(
    not_token: SyntaxToken,
    query: AnyCssContainerQueryInParens,
) -> CssContainerNotQuery {
    CssContainerNotQuery::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_NOT_QUERY,
        [
            Some(SyntaxElement::Token(not_token)),
            Some(SyntaxElement::Node(query.into_syntax())),
        ],
    ))
}
pub fn css_container_or_query(
    left: AnyCssContainerQueryInParens,
    or_token: SyntaxToken,
    right: AnyCssContainerCombinableQuery,
) -> CssContainerOrQuery {
    CssContainerOrQuery::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_OR_QUERY,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(or_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_container_query_in_parens(
    l_paren_token: SyntaxToken,
    query: AnyCssContainerQuery,
    r_paren_token: SyntaxToken,
) -> CssContainerQueryInParens {
    CssContainerQueryInParens::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_QUERY_IN_PARENS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(query.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_container_size_feature_in_parens(
    l_paren_token: SyntaxToken,
    query: AnyCssContainerSizeFeature,
    r_paren_token: SyntaxToken,
) -> CssContainerSizeFeatureInParens {
    CssContainerSizeFeatureInParens::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_SIZE_FEATURE_IN_PARENS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(query.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_container_style_and_query(
    left: CssContainerStyleInParens,
    and_token: SyntaxToken,
    right: AnyCssContainerStyleCombinableQuery,
) -> CssContainerStyleAndQuery {
    CssContainerStyleAndQuery::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_STYLE_AND_QUERY,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(and_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_container_style_in_parens(
    l_paren_token: SyntaxToken,
    query: AnyCssContainerStyleInParens,
    r_paren_token: SyntaxToken,
) -> CssContainerStyleInParens {
    CssContainerStyleInParens::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_STYLE_IN_PARENS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(query.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_container_style_not_query(
    not_token: SyntaxToken,
    query: CssContainerStyleInParens,
) -> CssContainerStyleNotQuery {
    CssContainerStyleNotQuery::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_STYLE_NOT_QUERY,
        [
            Some(SyntaxElement::Token(not_token)),
            Some(SyntaxElement::Node(query.into_syntax())),
        ],
    ))
}
pub fn css_container_style_or_query(
    left: CssContainerStyleInParens,
    or_token: SyntaxToken,
    right: AnyCssContainerStyleCombinableQuery,
) -> CssContainerStyleOrQuery {
    CssContainerStyleOrQuery::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_STYLE_OR_QUERY,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(or_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_container_style_query_in_parens(
    style_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    query: AnyCssContainerStyleQuery,
    r_paren_token: SyntaxToken,
) -> CssContainerStyleQueryInParens {
    CssContainerStyleQueryInParens::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CONTAINER_STYLE_QUERY_IN_PARENS,
        [
            Some(SyntaxElement::Token(style_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(query.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_counter_style_at_rule(
    counter_style_token: SyntaxToken,
    name: CssIdentifier,
    block: CssBlock,
) -> CssCounterStyleAtRule {
    CssCounterStyleAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COUNTER_STYLE_AT_RULE,
        [
            Some(SyntaxElement::Token(counter_style_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_custom_property(value: CssIdentifier) -> CssCustomProperty {
    CssCustomProperty::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_CUSTOM_PROPERTY,
        [Some(SyntaxElement::Node(value.into_syntax()))],
    ))
}
pub fn css_declaration(
    name: AnyCssDeclarationName,
    colon_token: SyntaxToken,
    value: CssComponentValueList,
) -> CssDeclarationBuilder {
    CssDeclarationBuilder {
        name,
        colon_token,
        value,
        important: None,
    }
}
pub struct CssDeclarationBuilder {
    name: AnyCssDeclarationName,
    colon_token: SyntaxToken,
    value: CssComponentValueList,
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
pub fn css_font_face_at_rule(font_face_token: SyntaxToken, block: CssBlock) -> CssFontFaceAtRule {
    CssFontFaceAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_FONT_FACE_AT_RULE,
        [
            Some(SyntaxElement::Token(font_face_token)),
            Some(SyntaxElement::Node(block.into_syntax())),
        ],
    ))
}
pub fn css_id_selector(hash_token: SyntaxToken, name: CssIdentifier) -> CssIdSelector {
    CssIdSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_ID_SELECTOR,
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
pub fn css_keyframes_at_rule(
    keyframes_token: SyntaxToken,
    name: CssIdentifier,
    css_string: CssString,
    body: CssKeyframesBody,
) -> CssKeyframesAtRule {
    CssKeyframesAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_KEYFRAMES_AT_RULE,
        [
            Some(SyntaxElement::Token(keyframes_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(css_string.into_syntax())),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
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
pub fn css_keyframes_body(
    l_curly_token: SyntaxToken,
    items: CssKeyframesItemList,
    r_curly_token: SyntaxToken,
) -> CssKeyframesBody {
    CssKeyframesBody::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_KEYFRAMES_BODY,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
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
pub fn css_media_at_rule(
    media_token: SyntaxToken,
    query_list: CssMediaQueryList,
    l_curly_token: SyntaxToken,
    body: AnyCssRule,
    r_curly_token: SyntaxToken,
) -> CssMediaAtRule {
    CssMediaAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_AT_RULE,
        [
            Some(SyntaxElement::Token(media_token)),
            Some(SyntaxElement::Node(query_list.into_syntax())),
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn css_media_query(
    condition_token: SyntaxToken,
    or_token: SyntaxToken,
    ty: AnyCssMediaQueryType,
) -> CssMediaQueryBuilder {
    CssMediaQueryBuilder {
        condition_token,
        or_token,
        ty,
        only_token: None,
        consequent: None,
    }
}
pub struct CssMediaQueryBuilder {
    condition_token: SyntaxToken,
    or_token: SyntaxToken,
    ty: AnyCssMediaQueryType,
    only_token: Option<SyntaxToken>,
    consequent: Option<CssMediaQueryConsequent>,
}
impl CssMediaQueryBuilder {
    pub fn with_only_token(mut self, only_token: SyntaxToken) -> Self {
        self.only_token = Some(only_token);
        self
    }
    pub fn with_consequent(mut self, consequent: CssMediaQueryConsequent) -> Self {
        self.consequent = Some(consequent);
        self
    }
    pub fn build(self) -> CssMediaQuery {
        CssMediaQuery::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_MEDIA_QUERY,
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
pub fn css_media_query_consequent(
    and_token: SyntaxToken,
    ty: AnyCssMediaQueryType,
) -> CssMediaQueryConsequentBuilder {
    CssMediaQueryConsequentBuilder {
        and_token,
        ty,
        condition_token: None,
    }
}
pub struct CssMediaQueryConsequentBuilder {
    and_token: SyntaxToken,
    ty: AnyCssMediaQueryType,
    condition_token: Option<SyntaxToken>,
}
impl CssMediaQueryConsequentBuilder {
    pub fn with_condition_token(mut self, condition_token: SyntaxToken) -> Self {
        self.condition_token = Some(condition_token);
        self
    }
    pub fn build(self) -> CssMediaQueryConsequent {
        CssMediaQueryConsequent::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_MEDIA_QUERY_CONSEQUENT,
            [
                Some(SyntaxElement::Token(self.and_token)),
                self.condition_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.ty.into_syntax())),
            ],
        ))
    }
}
pub fn css_media_query_feature(
    l_paren_token: SyntaxToken,
    feature: AnyCssMediaQueryFeatureType,
    r_paren_token: SyntaxToken,
) -> CssMediaQueryFeature {
    CssMediaQueryFeature::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_QUERY_FEATURE,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(feature.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_media_query_feature_boolean(
    css_identifier: CssIdentifier,
) -> CssMediaQueryFeatureBoolean {
    CssMediaQueryFeatureBoolean::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_QUERY_FEATURE_BOOLEAN,
        [Some(SyntaxElement::Node(css_identifier.into_syntax()))],
    ))
}
pub fn css_media_query_feature_compare(
    name: CssIdentifier,
    range: CssMediaQueryRange,
    value: AnyCssValue,
) -> CssMediaQueryFeatureCompare {
    CssMediaQueryFeatureCompare::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_QUERY_FEATURE_COMPARE,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(range.into_syntax())),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn css_media_query_feature_plain(
    name: CssIdentifier,
    colon_token: SyntaxToken,
    value: AnyCssValue,
) -> CssMediaQueryFeaturePlain {
    CssMediaQueryFeaturePlain::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_QUERY_FEATURE_PLAIN,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn css_media_query_feature_range(
    first_value: AnyCssValue,
    first_range: CssMediaQueryRange,
    name: CssIdentifier,
    second_value: AnyCssValue,
    second_range: CssMediaQueryRange,
) -> CssMediaQueryFeatureRange {
    CssMediaQueryFeatureRange::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_QUERY_FEATURE_RANGE,
        [
            Some(SyntaxElement::Node(first_value.into_syntax())),
            Some(SyntaxElement::Node(first_range.into_syntax())),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(second_value.into_syntax())),
            Some(SyntaxElement::Node(second_range.into_syntax())),
        ],
    ))
}
pub fn css_media_query_range(
    r_angle_token: SyntaxToken,
    l_angle_token: SyntaxToken,
    greater_than_equal_token: SyntaxToken,
    less_than_equal_token: SyntaxToken,
) -> CssMediaQueryRange {
    CssMediaQueryRange::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_QUERY_RANGE,
        [
            Some(SyntaxElement::Token(r_angle_token)),
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Token(greater_than_equal_token)),
            Some(SyntaxElement::Token(less_than_equal_token)),
        ],
    ))
}
pub fn css_named_namespace_prefix(name: CssIdentifier) -> CssNamedNamespacePrefix {
    CssNamedNamespacePrefix::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_NAMED_NAMESPACE_PREFIX,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn css_namespace(bitwise_or_token: SyntaxToken) -> CssNamespaceBuilder {
    CssNamespaceBuilder {
        bitwise_or_token,
        prefix: None,
    }
}
pub struct CssNamespaceBuilder {
    bitwise_or_token: SyntaxToken,
    prefix: Option<AnyCssNamespacePrefix>,
}
impl CssNamespaceBuilder {
    pub fn with_prefix(mut self, prefix: AnyCssNamespacePrefix) -> Self {
        self.prefix = Some(prefix);
        self
    }
    pub fn build(self) -> CssNamespace {
        CssNamespace::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_NAMESPACE,
            [
                self.prefix
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.bitwise_or_token)),
            ],
        ))
    }
}
pub fn css_nth_offset(sign_token: SyntaxToken, value: CssNumber) -> CssNthOffset {
    CssNthOffset::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_NTH_OFFSET,
        [
            Some(SyntaxElement::Token(sign_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn css_number(value_token: SyntaxToken) -> CssNumber {
    CssNumber::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_NUMBER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_parameter(css_component_value_list: CssComponentValueList) -> CssParameter {
    CssParameter::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PARAMETER,
        [Some(SyntaxElement::Node(
            css_component_value_list.into_syntax(),
        ))],
    ))
}
pub fn css_percent_dimension(value: CssNumber, unit_token: SyntaxToken) -> CssPercentDimension {
    CssPercentDimension::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PERCENT_DIMENSION,
        [
            Some(SyntaxElement::Node(value.into_syntax())),
            Some(SyntaxElement::Token(unit_token)),
        ],
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
pub fn css_pseudo_class_function_compound_selector(
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    selector: AnyCssCompoundSelector,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionCompoundSelector {
    CssPseudoClassFunctionCompoundSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR,
        [
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(selector.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_function_compound_selector_list(
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    compound_selector_list: CssCompoundSelectorList,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionCompoundSelectorList {
    CssPseudoClassFunctionCompoundSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST,
        [
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(compound_selector_list.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_function_identifier(
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    ident: CssIdentifier,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionIdentifier {
    CssPseudoClassFunctionIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER,
        [
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(ident.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_function_nth(
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    selector: AnyCssPseudoClassNthSelector,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionNth {
    CssPseudoClassFunctionNth::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_NTH,
        [
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(selector.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_function_relative_selector_list(
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    relative_selector_list: CssRelativeSelectorList,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionRelativeSelectorList {
    CssPseudoClassFunctionRelativeSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST,
        [
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(relative_selector_list.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_function_selector(
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    selector: AnyCssSelector,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionSelector {
    CssPseudoClassFunctionSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_SELECTOR,
        [
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(selector.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_function_selector_list(
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    selector_list: CssSelectorList,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionSelectorList {
    CssPseudoClassFunctionSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST,
        [
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(selector_list.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_function_value_list(
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    value_list: CssPseudoValueList,
    r_paren_token: SyntaxToken,
) -> CssPseudoClassFunctionValueList {
    CssPseudoClassFunctionValueList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST,
        [
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(value_list.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_class_identifier(name: CssIdentifier) -> CssPseudoClassIdentifier {
    CssPseudoClassIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_IDENTIFIER,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn css_pseudo_class_nth(symbol_token: SyntaxToken) -> CssPseudoClassNthBuilder {
    CssPseudoClassNthBuilder {
        symbol_token,
        sign_token: None,
        value: None,
        offset: None,
    }
}
pub struct CssPseudoClassNthBuilder {
    symbol_token: SyntaxToken,
    sign_token: Option<SyntaxToken>,
    value: Option<CssNumber>,
    offset: Option<CssNthOffset>,
}
impl CssPseudoClassNthBuilder {
    pub fn with_sign_token(mut self, sign_token: SyntaxToken) -> Self {
        self.sign_token = Some(sign_token);
        self
    }
    pub fn with_value(mut self, value: CssNumber) -> Self {
        self.value = Some(value);
        self
    }
    pub fn with_offset(mut self, offset: CssNthOffset) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn build(self) -> CssPseudoClassNth {
        CssPseudoClassNth::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_PSEUDO_CLASS_NTH,
            [
                self.sign_token.map(|token| SyntaxElement::Token(token)),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.symbol_token)),
                self.offset
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn css_pseudo_class_nth_identifier(value_token: SyntaxToken) -> CssPseudoClassNthIdentifier {
    CssPseudoClassNthIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_NTH_IDENTIFIER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_pseudo_class_nth_number(value: CssNumber) -> CssPseudoClassNthNumberBuilder {
    CssPseudoClassNthNumberBuilder {
        value,
        sign_token: None,
    }
}
pub struct CssPseudoClassNthNumberBuilder {
    value: CssNumber,
    sign_token: Option<SyntaxToken>,
}
impl CssPseudoClassNthNumberBuilder {
    pub fn with_sign_token(mut self, sign_token: SyntaxToken) -> Self {
        self.sign_token = Some(sign_token);
        self
    }
    pub fn build(self) -> CssPseudoClassNthNumber {
        CssPseudoClassNthNumber::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_PSEUDO_CLASS_NTH_NUMBER,
            [
                self.sign_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.value.into_syntax())),
            ],
        ))
    }
}
pub fn css_pseudo_class_nth_selector(
    nth: AnyCssPseudoClassNth,
) -> CssPseudoClassNthSelectorBuilder {
    CssPseudoClassNthSelectorBuilder {
        nth,
        of_selector: None,
    }
}
pub struct CssPseudoClassNthSelectorBuilder {
    nth: AnyCssPseudoClassNth,
    of_selector: Option<CssPseudoClassOfNthSelector>,
}
impl CssPseudoClassNthSelectorBuilder {
    pub fn with_of_selector(mut self, of_selector: CssPseudoClassOfNthSelector) -> Self {
        self.of_selector = Some(of_selector);
        self
    }
    pub fn build(self) -> CssPseudoClassNthSelector {
        CssPseudoClassNthSelector::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_PSEUDO_CLASS_NTH_SELECTOR,
            [
                Some(SyntaxElement::Node(self.nth.into_syntax())),
                self.of_selector
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn css_pseudo_class_of_nth_selector(
    of_token: SyntaxToken,
    selector_list: CssSelectorList,
) -> CssPseudoClassOfNthSelector {
    CssPseudoClassOfNthSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_OF_NTH_SELECTOR,
        [
            Some(SyntaxElement::Token(of_token)),
            Some(SyntaxElement::Node(selector_list.into_syntax())),
        ],
    ))
}
pub fn css_pseudo_class_selector(
    colon_token: SyntaxToken,
    class: AnyCssPseudoClass,
) -> CssPseudoClassSelector {
    CssPseudoClassSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_CLASS_SELECTOR,
        [
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(class.into_syntax())),
        ],
    ))
}
pub fn css_pseudo_element_function_identifier(
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    ident: CssIdentifier,
    r_paren_token: SyntaxToken,
) -> CssPseudoElementFunctionIdentifier {
    CssPseudoElementFunctionIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER,
        [
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(ident.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_element_function_selector(
    name: CssIdentifier,
    l_paren_token: SyntaxToken,
    selector: AnyCssSelector,
    r_paren_token: SyntaxToken,
) -> CssPseudoElementFunctionSelector {
    CssPseudoElementFunctionSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(selector.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn css_pseudo_element_identifier(name: CssIdentifier) -> CssPseudoElementIdentifier {
    CssPseudoElementIdentifier::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_ELEMENT_IDENTIFIER,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn css_pseudo_element_selector(
    double_colon_token: SyntaxToken,
    element: AnyCssPseudoElement,
) -> CssPseudoElementSelector {
    CssPseudoElementSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_ELEMENT_SELECTOR,
        [
            Some(SyntaxElement::Token(double_colon_token)),
            Some(SyntaxElement::Node(element.into_syntax())),
        ],
    ))
}
pub fn css_ratio(
    numerator: CssNumber,
    slash_token: SyntaxToken,
    denominator: CssNumber,
) -> CssRatio {
    CssRatio::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_RATIO,
        [
            Some(SyntaxElement::Node(numerator.into_syntax())),
            Some(SyntaxElement::Token(slash_token)),
            Some(SyntaxElement::Node(denominator.into_syntax())),
        ],
    ))
}
pub fn css_regular_dimension(value: CssNumber, unit: CssIdentifier) -> CssRegularDimension {
    CssRegularDimension::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_REGULAR_DIMENSION,
        [
            Some(SyntaxElement::Node(value.into_syntax())),
            Some(SyntaxElement::Node(unit.into_syntax())),
        ],
    ))
}
pub fn css_relative_selector(selector: AnyCssSelector) -> CssRelativeSelectorBuilder {
    CssRelativeSelectorBuilder {
        selector,
        combinator_token: None,
    }
}
pub struct CssRelativeSelectorBuilder {
    selector: AnyCssSelector,
    combinator_token: Option<SyntaxToken>,
}
impl CssRelativeSelectorBuilder {
    pub fn with_combinator_token(mut self, combinator_token: SyntaxToken) -> Self {
        self.combinator_token = Some(combinator_token);
        self
    }
    pub fn build(self) -> CssRelativeSelector {
        CssRelativeSelector::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_RELATIVE_SELECTOR,
            [
                self.combinator_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.selector.into_syntax())),
            ],
        ))
    }
}
pub fn css_root(rules: CssRuleList, eof_token: SyntaxToken) -> CssRootBuilder {
    CssRootBuilder {
        rules,
        eof_token,
        bom_token: None,
    }
}
pub struct CssRootBuilder {
    rules: CssRuleList,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
}
impl CssRootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn build(self) -> CssRoot {
        CssRoot::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.rules.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
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
pub fn css_size_feature_boolean(name: CssIdentifier) -> CssSizeFeatureBoolean {
    CssSizeFeatureBoolean::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SIZE_FEATURE_BOOLEAN,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn css_size_feature_plain(
    name: CssIdentifier,
    colon_token: SyntaxToken,
    value: AnyCssSizeFeatureValue,
) -> CssSizeFeaturePlain {
    CssSizeFeaturePlain::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SIZE_FEATURE_PLAIN,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn css_size_feature_range(
    left: CssIdentifier,
    comparison: CssSizeFeatureRangeComparison,
    right: AnyCssSizeFeatureValue,
) -> CssSizeFeatureRange {
    CssSizeFeatureRange::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SIZE_FEATURE_RANGE,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Node(comparison.into_syntax())),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_size_feature_range_comparison(
    operator_token: SyntaxToken,
) -> CssSizeFeatureRangeComparison {
    CssSizeFeatureRangeComparison::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SIZE_FEATURE_RANGE_COMPARISON,
        [Some(SyntaxElement::Token(operator_token))],
    ))
}
pub fn css_size_feature_range_interval(
    left: AnyCssSizeFeatureValue,
    left_comparison: CssSizeFeatureRangeComparison,
    name: CssIdentifier,
    right_comparison: CssSizeFeatureRangeComparison,
    right: AnyCssSizeFeatureValue,
) -> CssSizeFeatureRangeInterval {
    CssSizeFeatureRangeInterval::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SIZE_FEATURE_RANGE_INTERVAL,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Node(left_comparison.into_syntax())),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(right_comparison.into_syntax())),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn css_string(value_token: SyntaxToken) -> CssString {
    CssString::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_STRING,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn css_type_selector(ident: CssIdentifier) -> CssTypeSelectorBuilder {
    CssTypeSelectorBuilder {
        ident,
        namespace: None,
    }
}
pub struct CssTypeSelectorBuilder {
    ident: CssIdentifier,
    namespace: Option<CssNamespace>,
}
impl CssTypeSelectorBuilder {
    pub fn with_namespace(mut self, namespace: CssNamespace) -> Self {
        self.namespace = Some(namespace);
        self
    }
    pub fn build(self) -> CssTypeSelector {
        CssTypeSelector::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_TYPE_SELECTOR,
            [
                self.namespace
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.ident.into_syntax())),
            ],
        ))
    }
}
pub fn css_universal_namespace_prefix(star_token: SyntaxToken) -> CssUniversalNamespacePrefix {
    CssUniversalNamespacePrefix::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_UNIVERSAL_NAMESPACE_PREFIX,
        [Some(SyntaxElement::Token(star_token))],
    ))
}
pub fn css_universal_selector(star_token: SyntaxToken) -> CssUniversalSelectorBuilder {
    CssUniversalSelectorBuilder {
        star_token,
        namespace: None,
    }
}
pub struct CssUniversalSelectorBuilder {
    star_token: SyntaxToken,
    namespace: Option<CssNamespace>,
}
impl CssUniversalSelectorBuilder {
    pub fn with_namespace(mut self, namespace: CssNamespace) -> Self {
        self.namespace = Some(namespace);
        self
    }
    pub fn build(self) -> CssUniversalSelector {
        CssUniversalSelector::unwrap_cast(SyntaxNode::new_detached(
            CssSyntaxKind::CSS_UNIVERSAL_SELECTOR,
            [
                self.namespace
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.star_token)),
            ],
        ))
    }
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
pub fn css_component_value_list<I>(items: I) -> CssComponentValueList
where
    I: IntoIterator<Item = AnyCssValue>,
    I::IntoIter: ExactSizeIterator,
{
    CssComponentValueList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COMPONENT_VALUE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_compound_selector_list<I, S>(items: I, separators: S) -> CssCompoundSelectorList
where
    I: IntoIterator<Item = AnyCssCompoundSelector>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssCompoundSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_COMPOUND_SELECTOR_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_declaration_list<I, S>(items: I, separators: S) -> CssDeclarationList
where
    I: IntoIterator<Item = CssDeclaration>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssDeclarationList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_DECLARATION_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_keyframes_item_list<I>(items: I) -> CssKeyframesItemList
where
    I: IntoIterator<Item = CssKeyframesBlock>,
    I::IntoIter: ExactSizeIterator,
{
    CssKeyframesItemList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_KEYFRAMES_ITEM_LIST,
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
pub fn css_media_query_list<I, S>(items: I, separators: S) -> CssMediaQueryList
where
    I: IntoIterator<Item = CssMediaQuery>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssMediaQueryList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_MEDIA_QUERY_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_parameter_list<I, S>(items: I, separators: S) -> CssParameterList
where
    I: IntoIterator<Item = CssParameter>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssParameterList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PARAMETER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_pseudo_value_list<I, S>(items: I, separators: S) -> CssPseudoValueList
where
    I: IntoIterator<Item = AnyCssPseudoValue>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssPseudoValueList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_PSEUDO_VALUE_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn css_relative_selector_list<I, S>(items: I, separators: S) -> CssRelativeSelectorList
where
    I: IntoIterator<Item = AnyCssRelativeSelector>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = CssSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    CssRelativeSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_RELATIVE_SELECTOR_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
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
    I: IntoIterator<Item = AnyCssSelector>,
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
pub fn css_sub_selector_list<I>(items: I) -> CssSubSelectorList
where
    I: IntoIterator<Item = AnyCssSubSelector>,
    I::IntoIter: ExactSizeIterator,
{
    CssSubSelectorList::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_SUB_SELECTOR_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn css_bogus<I>(slots: I) -> CssBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogus::unwrap_cast(SyntaxNode::new_detached(CssSyntaxKind::CSS_BOGUS, slots))
}
pub fn css_bogus_at_rule<I>(slots: I) -> CssBogusAtRule
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusAtRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_AT_RULE,
        slots,
    ))
}
pub fn css_bogus_body<I>(slots: I) -> CssBogusBody
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusBody::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_BODY,
        slots,
    ))
}
pub fn css_bogus_pseudo_class<I>(slots: I) -> CssBogusPseudoClass
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusPseudoClass::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_PSEUDO_CLASS,
        slots,
    ))
}
pub fn css_bogus_pseudo_element<I>(slots: I) -> CssBogusPseudoElement
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusPseudoElement::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_PSEUDO_ELEMENT,
        slots,
    ))
}
pub fn css_bogus_rule<I>(slots: I) -> CssBogusRule
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusRule::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_RULE,
        slots,
    ))
}
pub fn css_bogus_selector<I>(slots: I) -> CssBogusSelector
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_SELECTOR,
        slots,
    ))
}
pub fn css_bogus_sub_selector<I>(slots: I) -> CssBogusSubSelector
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    CssBogusSubSelector::unwrap_cast(SyntaxNode::new_detached(
        CssSyntaxKind::CSS_BOGUS_SUB_SELECTOR,
        slots,
    ))
}
